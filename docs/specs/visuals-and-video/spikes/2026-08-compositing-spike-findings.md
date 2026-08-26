# Compositing Spike Findings — projectM-in-egui (glow vs wgpu)

> **Last Updated:** 2026-08-26 · **Status:** Active (spike complete)

Phase 0 exit criterion (doc 04): evaluate **(a)** egui+glow same-context
compositing of a projectM-rendered frame AND **(b)** egui+wgpu cross-API
texture sharing, probe glassmorphism feasibility (true backdrop-blur vs Tier-B
fallback per doc 08 §3), and confirm projectM 4.x symbol availability
(`projectm_set_frame_time`, FBO render path). This spike discharges all four.
Outcome recorded as ADR-013 (Proposed) in doc 17.

## 1. Method

Throwaway prototype (gitignored `reference-scratchpad/spike-egui-projectm/`):
eframe 0.34 with the `glow` renderer, rendering libprojectM-4 into an app-owned
FBO fed by procedurally synthesized stereo PCM (no audio device), compositing
the FBO's color texture fullscreen under egui chrome via
`register_native_texture`, plus a `backdrop-blur-egui` grab-pass frosted-glass
test panel over the live visualizer. projectM binding:
`projectm-rs` vendored at commit with `projectm-sys 1.2.3`.

Research phase (@librarian report, 2026-08-26) established the API landscape;
the prototype verified it by compilation and execution.

## 2. Results

### 2.1 projectM 4.x symbols — CONFIRMED ✅

Verified present in the vendored libprojectM source headers:

| Symbol | Header | Role |
|---|---|---|
| `projectm_opengl_render_frame_fbo(handle, fbo_id)` | `render_opengl.h` | Renders into our own FBO — the compositing primitive |
| `projectm_set_frame_time(handle, seconds)` | `parameters.h` | Deterministic timestamps (also the video-export path) |
| `projectm_pcm_add_float(handle, samples, count, channels)` | `audio.h` | Interleaved float PCM feeding |

**Version caveat (important):** the release-pinned `projectm-rs` submodule
(`v4.1.2`) does NOT expose `render_frame_fbo` / `set_frame_time`; the vendor
tree had to bump libprojectM to master (`4.x-dev`). When implementing
`visualizer-projectm-ffi-bindings` in Phase 5, pin a known-good master commit,
or upstream the missing safe-wrapper coverage and track it.

### 2.2 Same-context GL compositing — WORKS ✅

The prototype renders projectM into an app-owned FBO each frame, registers the
color texture once via egui_glow's native-texture registration, and draws it as
a fullscreen image primitive UNDER the tint wash, glass panel, and UI widgets.
Strict painter-order layering confirmed: visualizer → tint → frost → text →
buttons. **Human-orchestrator visual QA confirmed:** window opens, projectM
rendering visibly active, frosted-glass test widget composited over it.

### 2.3 Glassmorphism — TRUE backdrop blur works on the glow path ✅

`backdrop-blur-egui` 0.2.x (`grab-pass` feature) grabbed the live framebuffer
region behind the panel and composited blurred+tinted output — the Tier-A
"true backdrop-blur" option from doc 08 §3 is feasible without custom shader
work. Fallback (DIY framebuffer-copy blur) was implemented but not needed.
Caveats for Phase 5: the crate pins specific egui versions (currently 0.34) and
is pre-release — pin exact versions and wrap behind our own widget so an
egui upgrade doesn't ripple into screens.

### 2.4 egui+wgpu cross-API path — REJECTED ❌

No public wgpu API imports GL textures; on macOS this is structural (GL via
CGL, wgpu via Metal; zero-copy would need unsafe IOSurface/hal work). The CPU
readback fallback (~500 MB/s at 1080p60 plus sync stalls and added latency)
is a non-starter for fullscreen glass-composited visuals. Decision rationale
fully captured in ADR-013.

### 2.5 Known runtime caveats (honest record)

- The cmake-built debug dylib (`libprojectM-4d.4.dylib`) needs to be
  discoverable at load time; launching the raw binary from a bare shell hit a
  dyld "no LC_RPATH" error. Normal launch paths (via cargo/IDE, or adding an
  rpath link-arg) resolve it. This is throwaway-spike packaging noise, not an
  architecture problem — the real FFI crate will own proper linking.
- Automated fps instrumentation did not survive the provider outages that
  interrupted this spike; performance was not formally measured. Rendering was
  smooth to the eye at default settings; formal perf budget checks are
  deferred to Phase 5's first integration milestone.

## 3. Verdict

**egui + glow** — high confidence. Zero-copy projectM compositing, working
true-backdrop-blur glass, and deterministic-timestamp video-export primitives
all verified on the actual target platform (macOS). See ADR-013.
