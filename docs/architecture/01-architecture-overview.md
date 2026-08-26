# Architecture Overview

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. High-Level Shape

```

┌─────────────────────────────────────────────────────────────────┐
│                         station-app (bin)                       │
│   native shell window, routing, top-level app state             │
└───────────────┬───────────────────────────────┬─────────────────┘
                │                               │
        ┌───────▼────────┐               ┌──────▼─────────┐
        │ ui-* crates    │               │domain crates   │
        │(widgets/screens)│◄────events──►│(business logic)│
        └───────┬────────┘               └──────┬─────────┘
                │                               │
        ┌───────▼───────────────────────────────▼─────────┐
        │              integration/bridge crates          │
        │  suno-api-* | visualizer-projectm-* | audio-* | │
        │  whisper-* | ffmpeg-export-* | oauth-loopback-* │
        └───────┬───────────────────────────────┬─────────┘
                │                               │
        ┌───────▼─────────┐              ┌──────▼────────────┐
        │ local-db-sqlite │              │  external world   │
        │ keyring secrets │              │ suno.com, ffmpeg, │
        │                 │              │ projectM, whisper │
        └─────────────────┘              └───────────────────┘

```

## 2. Guiding Architectural Principles

1. **Crate = capability boundary.** Every crate has ONE clear job and a small,
   deliberate public API (its `lib.rs`). If you can't summarize a crate's purpose
   in one sentence, it's two crates.
2. **Domain logic never depends on UI.** `ui-*` crates depend on domain/service
   crates; never the reverse. This keeps the (likely to change) UI layer isolated
   and keeps domain crates independently testable/agent-friendly.
3. **All external processes/libraries are wrapped behind a bridge crate with a
   trait-based interface**, so the concrete backend (projectM, ffmpeg, whisper.cpp,
   a specific LLM provider) is swappable without touching callers.
4. **Async runtime:** `tokio`, used app-wide. UI event loop and async work are
   bridged via channels (`tokio::sync::mpsc` / `watch`) — no blocking calls on the
   UI thread, ever.
5. **Errors:** every crate defines its own `Error` enum (via `thiserror`); no
   crate returns another crate's error type directly across its public API
   boundary — wrap it. `anyhow` is allowed only in the top-level `station-app` binary
   and in test code, never in library crate public signatures.
6. **State ownership:** each domain concern owns its own state store crate
   (e.g. `library-store`, `account-store`, `pipeline-store`) backed by SQLite;
   `ui-*` crates never talk to SQLite directly, only through a store crate's API.
7. **No God Objects.** No "AppState" mega-struct holding everything. Composition
   at the top-level binary wires narrow-interface handles together.

## 3. Layered Crate Map (see 02-workspace-layout.md for the literal directory tree)

- **Layer 0 — Foundation:** error/result conventions, config loading, logging/
  tracing setup, design-token theme definitions.
- **Layer 1 — External Bridges:** Suno HTTP client, Suno auth flows, projectM FFI,
  ffmpeg process wrapper, whisper-rs wrapper, OS keyring wrapper, OAuth loopback
  server, LLM/image-gen provider adapters (thin, stubbed early).
- **Layer 2 — Domain/Stores:** account management & multi-account switching,
  library sync/cache, download manager, lyrics store & alignment, project/scene
  store (canvas + keyframes), pipeline/automation store, plugin registry (stub).
- **Layer 3 — Application Services:** orchestration logic that composes Layer 1 +
  Layer 2 into user-facing operations (e.g. "render this track as a karaoke video"
  is a service that pulls audio + lyrics + scene + calls the visualizer/export
  bridges).
- **Layer 4 — UI:** app shell/navigation, per-feature screens, shared widget
  library, theme application.
- **Layer 5 — Binary:** `station-app`, composition root only. Should contain
  almost no logic — just wiring.

Dependencies only ever point downward (Layer 4 → 3 → 2 → 1 → 0). This is
mechanically enforced by `cargo xtask check-layering`, which fails on any
upward edge; it runs in CI (.github/workflows/ci.yml `guardrails` job) as of
Phase 0. Zero exceptions — a task that seems to require an upward dependency
is a signal the logic is misplaced (see doc 18 §2.1).

## 4. UI Framework Decision

**Chosen: native Rust GUI**, immediate path = `egui` unless the agent's early
prototyping phase surfaces a hard blocker, in which case `iced` is the fallback
(both are pre-approved; pick during Phase 0 spike, document the choice + reasoning
as an ADR — see doc 17). Rationale for leading with `egui`: simpler mental model
for an LLM-driven, many-small-files codebase (immediate-mode = less hidden
state-machine complexity to keep consistent across files), and an existing Catppuccin
theme crate (`catppuccin-egui`) to bootstrap theming quickly.

The renderer-backend choice — `egui` + `glow` (OpenGL) versus `egui` + `wgpu` — is
explicitly open and must be decided by the Phase 0 spike. Rationale: projectM renders
OpenGL, so an egui+glow backend allows same-GL-context compositing of visualizer frames
(dramatically simpler interop; precedent: the predecessor prototype composited GL FBOs
in the same context), whereas egui+wgpu requires cross-API texture sharing, which is
feasible but higher-risk. Both are acceptable outcomes; whichever is chosen must be
recorded as an ADR (doc 17).

Custom bolted-on interface surfaces (e.g., a heavier web-tech canvas editor) are
explicitly allowed **later**, as an opt-in embedded surface for one specific
screen, never as a replacement for the native shell. Any such addition requires
its own ADR justifying why native widgets were insufficient for that specific
screen.

## 5. Visualizer Integration Model

`visualizer-projectm-bridge` wraps projectM's C API via `bindgen`/FFI, renders
into an offscreen texture (or shares a GL/wgpu context, backend-dependent — a
Phase 4 spike decides the exact mechanism and records it as an ADR), and exposes
a small Rust trait: feed it audio PCM frames + get back a frame texture handle +
timing metadata. The `canvas-scene-graph` compositor layers the app's overlay
elements (text/graphics/keyframed animations) on top of that texture each frame.
Export (one-off or pipeline/batch) walks frames headlessly (not tied to the live
UI framerate) and pipes raw frames to `video-export-ffmpeg`.

## 6. Lyrics/Karaoke Data Flow

```

Suno remote timed-lyrics (if available)
        │
        ▼
lyrics-store (canonical, versioned)
        │
        ├─ optional: whisper-transcription-bridge produces an independent
        │            alignment, used to fill gaps / re-time low-confidence
        │            segments / fully replace when no remote timing exists
        │
        ▼
karaoke-render-service → feeds canvas-scene-graph a per-frame "active word/line"
                          signal, which drives a built-in karaoke text style
                          (and/or a user-authored canvas element bound to it)

```

Remote timing is always the default/preferred source when present; local Whisper
is an *enhancement or fallback*, never a silent overwrite — the lyrics editor UI
must make the source (and any manual edits) visible/attributable.

## 7. Automation Pipeline Model

A pipeline is a serialized recipe: input selection (which tracks/library filter),
a scene/template reference (canvas design + keyframes), lyric-source policy,
render/export settings, and output routing (local folder / naming convention).
The `pipeline-automation-engine` crate is a queue+worker over the same
Application Services layer used for one-off renders — **automation must never
duplicate render logic**; it only fans out the same single-track render service
across many inputs with concurrency limits and resumability (crash mid-batch →
resume, don't restart from zero).

## 8. Multi-Account Model

`account-store` holds N credential profiles (each: display name, auth method,
opaque secret handle into OS keyring, cached profile metadata). Exactly one
account is "active" per app session for the primary shell, but the underlying
Suno API client is parameterized by account, so power users switching frequently
is a cheap operation (swap the active credential handle, re-key cached library
views), not a re-login.

## 9. Plugin System — Current Phase Status

Per current decision, this is **scaffolded only**: a `plugin-host-stub` crate
exists with the trait definitions and a no-op registry, but no scripting engine
is wired in yet, and no UI surface for it ships yet. This keeps the seam in the
codebase (so later work is additive, not invasive) without spending build effort
now. See doc 11 for the deferred design.
