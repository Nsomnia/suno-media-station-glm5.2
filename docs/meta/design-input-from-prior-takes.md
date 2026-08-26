# Design Input From Prior Takes (Sibling Project Survey)

> **Last Updated:** 2026-08-25 · **Status:** Active
>
> Distilled from a survey of every prior attempt at this same product across
> `~/Documents` (chadvis reference impl, SUNBEAM seed set, sunoPulse iced
> mockups, five model-benchmark re-takes, Tauri frontends, QML shell mockups)
> and the owner's GitHub repos. This doc preserves their best ideas so the
> canonical rewrite inherits them without re-deriving. Nothing here overrides
> the binding docs — items needing spec changes enter via
> [doc 99](../product/99-ideas-backlog.md) and an ADR when promoted.

## 0. The Cross-Cutting Stall Pattern (read this first)

Every take that stopped, stopped at the **same wall**: the native
GL/projectM bridge (Tauri frontend stalled at "libprojectM FFI pending",
chadvis fought Qt6 RHI, the C++ glm5.2 sibling stalled post-scaffold), while
every take that stayed at seed-doc stage produced nothing. Consequence for
this repo, already encoded in [Phase 0](../product/04-phase-roadmap.md):
**the rendering-host decision must be spiked and ADR'd before feature work**,
and the mock-first seam (§6 below) keeps everything else unblocked while the
GL pipeline is conquered.

## 1. Deterministic Export Pipeline

Fixed-timestep headless render loop: frame index is the export master clock;
timestamp = `n / fps` (integer math, never float accumulation); exact
`1/fps` PCM slices; RGBA frames piped to an ffmpeg process. Screen-capture
recording is formally rejected ("we own the PCM"). **Capability gate:**
projectM's `projectm_set_frame_time()` / FBO render entry points exist only
in 4.2+ master — Phase 0's spike must probe symbols at build/runtime and
record the result ([doc 09 §4-5](../specs/visuals-and-video/09-visualizer-projectm-integration.md)).

## 2. projectM Landmine Checklist (carry into FFI crate docs)

From two independent implementations: PCM count = samples *per channel* ·
feed silence while paused (silent-audio watchdog keeps presets animating) ·
the playlist is a **separate C library — wrap it, register the failed-preset
callback for auto-skip** · set texture paths before loading presets ·
projectM leaves GL state dirty → wrap calls in a state guard · RAII all
string frees · mesh size is the CPU quality dial · target GL 3.3 core.

## 3. Rendering-Surface Decision Record (input to the Phase 0 ADR)

Ranked by accumulated evidence across takes: (1) dedicated native GL window
hosting the visualizer with the toolkit HUD composited inside it — the only
path proven end-to-end (chadvis final form after QQuickItem and
QQuickFramebufferObject both broke under Qt6 RHI); (2) egui-glow same-context
compositing; (3) Slint as sanctioned single-window fallback; rejected with
reasons: iced (wgpu↔GL texture sharing fragile), GL-inside-webview,
Electron, screen-capture recording.

## 4. Overlay Timeline Editor (best-in-survey creation-studio layout)

Typed tracks (`audio | text | karaoke | image | preset`) — notably **preset
segments**: visualizer preset changes as draggable timeline clips. Per-clip
properties (x/y/scale/opacity/fontSize/content); keyframes with easing enum
including `hold`; side KeyframeInspector pane; zoomable, docked ≥30vh
timeline under the viewport. Feeds
[doc 10](../specs/visuals-and-video/10-canvas-and-keyframe-system.md) via
doc 99 promotion.

## 5. Bounded Overlay Expression Language

Pure functions over a fixed variable set (`t, bass, mid, treb, vol, beat,
line_index, progress`). Declarative effects JSON first (fade/color/scale/
karaoke-progress), threshold triggers later. **No Turing-complete scripting
in v1** — plugins (doc 11/Rhai) arrive only in Phase 8.

## 6. Mock-First Development Seam

One trait boundary per external dependency, answered by both real backends
and mocks: scripted canned-response HTTP transport tests for Suno;
procedurally synthesized demo audio so the whole UI runs with zero native
deps. This is how previous attempts kept UI velocity while the native
pipeline was unbuilt — make it a first-class requirement of service crates
(doc 01 §2.3), not an afterthought.

## 7. Library Sync Schema Law + Search + Politeness

- **`remote_*` vs `local_*` column separation**: never store locally-derived
  data where sync overwrites ([doc 07](../specs/data-and-storage/07-data-model-and-storage-schema.md)).
- Resumable cursor sync state machine (Idle→Probe→PullDelta→Reconcile→PushLocal);
  conflict rows surfaced in UI, not auto-resolved.
- FTS index over titles/prompts/lyrics/user tags.
- Global politeness limiter ≤1 rps sustained / ≤5 burst, honoring
  `Retry-After` (chadvis's serial deque is the precedent).
- Distinct surfacing of Suno error classes: 401 → one transparent refresh-
  retry; 429 = out of credits; 430 = too frequent.
- Song relationship taxonomy (8 verbs): cover_of / remix_of / stem_of /
  extended_from / cropped_from / video_export_of / inspired_by (+derivation
  DAG view) — pairs with clip lineage in the prior-art doc.

## 8. Settings Single Source of Truth

One schema definition generates TOML serde + CLI flags + UI form; precedence
defaults < file < CLI < live-writeback; debounced autosave (2 s precedent);
versioned config modules for breaking changes. Every knob exists in all three
surfaces or it doesn't exist.

## 9. Karaoke/Lyric Toolchain

Whisper timing reconciled against official aligned lyrics via DTW-style
alignment with per-word confidence displayed linter-style; IDE-like editor
(line gutter, click-to-seek, drag timing handles, mini waveform strip);
LRC/enhanced-LRC/SRT round-trip exporters; word-highlight lower-third overlay
with previous-line ghost; karaoke text rendered as just another overlay layer
(one styling system).

## 10. Visual Identity & Widget Kit

Catppuccin Mocha token theme with semantic colors/radius/spacing/type scales
is proven across four independent takes — keep as default
([doc 08](../specs/ui-ux/08-ui-ux-design-system.md)). Proven widget ideas to
re-create natively: rotary volume knob, waveform scrubber with hover-time
tooltip, glowing star ratings, procedural cover-art hues from track metadata,
export-progress HUD over fullscreen viz, persistent transport bar with
contextual jump buttons, TagChip prompt building, credits-remaining readout,
stems rack (mute/solo per stem), generation V1/V2 result cards.

## 11. Smaller Gems

Prompt Vault with versioning + `negative_prompt` field (real specimens exist
in `~/Documents/*.rtf` for validation) · capability-probe feature flags for
Suno b-side/experimental features · factory encoder presets (`youtube1080p60`,
`discord8mb`, …) · append-only idea-graveyard discipline (this repo uses
[doc 99](../product/99-ideas-backlog.md)) · fuzzy did-you-mean CLI flags ·
keyboard shortcuts Space/N/P/R/M convention.

Source map for all claims: survey report of 2026-08-25 covering
`OVERVIEW-OF-CHADVIS.log.md`, `claudesonner5_sunoprojectmvos.rtf`,
`sunoPulse-rust-iced-mockup`, `suno-client-projectm-video-creator-*` (five
variants), `suno-desktop-client-and-projectm-music-video-creator/.agents/docs/*`,
`suno-studio-frontend-and-projectm-visualizer-karaoke-video-creator`,
`qt-qml-responsive-desktop-mobile-esque-ui-mockup`.
