# Visualizer (projectM) Integration Spec

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Scope

Covers `visualizer-projectm-ffi-bindings`, `visualizer-projectm-frame-bridge`,
and their consumption by `single-track-visualizer-render-service` (live
preview) and the export path (headless render, feeding
`video-export-ffmpeg-process`). Canvas overlay compositing (text/graphics on
top of the visualizer frame) is doc 10's concern, not this one — this doc
stops at "produce a visualizer frame," doc 10 picks up "composite things
onto that frame."

## 2. Why projectM, Why FFI

projectM is a mature, widely-used, MilkDrop-preset-compatible C/C++ library —
per doc 03 §3's prior-art mandate, using it (rather than reimplementing
audio-reactive shader visualization from scratch in Rust) is the correct
call; this is exactly the kind of "don't reinvent the wheel" case the
constitution names explicitly.

## 3. Crate Split Rationale

- **`visualizer-projectm-ffi-bindings`** — the *only* crate allowed to
  contain `unsafe` FFI calls into projectM's C API (via `bindgen`-generated
  raw bindings, hand-wrapped into a minimal safe-ish surface: init/destroy a
  projectM instance, feed PCM, render-to-target, load preset, list presets).
  Kept intentionally thin and boring — its job is "make projectM callable
  from Rust safely," nothing more.
- **`visualizer-projectm-frame-bridge`** — the actual integration logic:
  owns the trait consumed by the rest of the app, manages the render-target
  strategy (live preview vs. headless export — see §5), preset
  selection/management, and timing/frame-pacing logic. Depends on the FFI
  crate but exposes zero `unsafe`/raw-pointer types in its own public API.

```rust
// illustrative shape, frame-bridge crate's public trait
pub trait VisualizerEngine {
    fn load_preset(&mut self, preset: PresetHandle) -> Result<(), VisualizerError>;
    fn list_available_presets(&self) -> Vec<PresetHandle>;
    fn feed_audio_frame(&mut self, pcm: &AudioFrame) -> Result<(), VisualizerError>;
    /// Renders one frame; caller controls when this is invoked — live preview
    /// calls it per display-refresh, export calls it per output-video-frame,
    /// decoupled from real time (see §5).
    fn render_frame(&mut self, target: &mut RenderTarget) -> Result<FrameMetadata, VisualizerError>;
}
```

## 4. Build/Packaging Concerns (address early — Phase 0/4 spike)

- projectM ships as a C++ library — building/linking it from a Rust build
  requires either: (a) a system-installed projectM the user brings
  themselves (simplest, but adds an install-dependency for end users), or
  (b) vendoring/building it via a `build.rs` + `cmake`/`cc` crate pipeline
  (more self-contained, heavier build times, more cross-platform build
  complexity). **Decision deferred to the Phase 0 compositing spike** (doc
  04 Phase 0) — record the outcome as an ADR once decided; do not silently
  pick one mid-implementation.
- Preset files (`.milk`) — confirm projectM's default preset pack's license
  before bundling any presets in `assets/projectm-presets/`; if unclear,
  ship with zero bundled presets and require the user to point at their own
  preset folder in settings, documented clearly in the UI rather than
  risking a license issue.

## 5. Live Preview vs. Headless Export — Two Render Paths, One Engine Trait

This is the most important design decision in this doc, flagged as a risk
in doc 04 Phase 4:

- **Live preview path:** `render_frame` called once per UI display refresh,
  audio fed from the currently-playing track in real time via
  `audio-io-cpal-bridge`'s output tap, rendered into a texture shared with
  the UI framework's renderer (mechanism decided by the Phase 0 spike —
  likely a `wgpu` shared texture if using `egui`/`eframe`'s wgpu backend).
- **Headless export path:** NOT tied to real time or display refresh at
  all. Audio is walked frame-by-frame from the decoded file at the
  **output video's** frame rate (e.g. 30fps → advance PCM position by
  1/30s worth of samples per iteration), `render_frame` is called into an
  **offscreen render target** (no display presentation), and the resulting
  frame is immediately piped to `video-export-ffmpeg-process` as a raw
  frame on stdin (or written to a temp frame buffer, whichever
  `video-export-ffmpeg-process`'s design settles on — see §7 of this doc).
  This path can run **faster or slower than real-time** depending on
  machine performance — that's the whole point (batch rendering thousands
  of tracks shouldn't be real-time-bound).
- **Hardware-accelerated encode requirement:** ffmpeg encoder availability
  is probed at startup with a fallback chain — `h264_videotoolbox` (macOS)
  → `h264_nvenc` (NVIDIA) → `h264_qsv` (Intel QSV) → `libx264` (CPU
  fallback). Batch automation at scale depends on HW encoders being
  available and preferred; the predecessor prototype already rendered via
  FFmpeg hardware acceleration, so this is proven practice, not aspiration.
  The same chain is reused by pipeline export settings (doc 13 §3).
- Both paths use the *same* `VisualizerEngine` trait and the *same*
  `canvas-overlay-compositing-service` (doc 10) for consistency — the only
  difference is what drives the frame-advance loop and where the frame
  ends up (screen vs. ffmpeg pipe). This shared-path requirement is what
  guarantees "what you designed in preview is exactly what you get in
  export," which is a named product requirement, not an implementation
  nicety.

## 6. Preset Management

- `PresetHandle` refers to a `.milk` file path + display name; a settings
  screen lets users point at one or more local preset folders.
- Preset switching mid-playback (live preview) should be smooth — no crash/
  black-frame on switch; projectM natively supports blending between
  presets, expose that as a config option (blend duration) rather than
  hardcoding it.
- For automation pipelines (Phase 7), a scene/pipeline can pin a specific
  preset (or a preset-rotation policy) — that's stored in the scene JSON
  (doc 07 §6), not in this crate.

## 7. Audio Feed Details

- `AudioFrame` = a fixed-size PCM buffer (format/sample-rate normalized
  before reaching this crate — normalization is `audio-decode-symphonia-
  bridge`'s job, not the visualizer bridge's).
- projectM expects roughly real-time-cadence PCM chunks for its beat-
  detection internals to behave correctly even in the headless path — if
  headless export feeding "as fast as possible" produces beat-detection
  artifacts (projectM's internal analysis assuming wall-clock time), this
  is a known risk to validate in the Phase 4 spike; the fallback if it's a
  real problem is disabling/reducing projectM's live beat-detection
  reliance in favor of pre-analyzed beat/onset data fed in a way projectM
  accepts (confirm from projectM's own docs/source what's supported)
  rather than guessing — another concrete "check prior art / actual docs
  before assuming" case per doc 03 §3.

## 8. Error Handling

`VisualizerError` should distinguish (at minimum): preset-load failure
(bad/corrupt `.milk` file — recoverable, skip/report that preset),
engine-init failure (missing/incompatible projectM install — fatal,
surfaced clearly at app-startup or feature-first-use, with actionable
guidance per §4's packaging decision), and render-frame failure (should be
rare/fatal-per-render, logged with full context for debugging).
