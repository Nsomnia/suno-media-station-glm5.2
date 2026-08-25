# Project Charter — "Suno Station" (working name)

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Vision

Suno Station is a native, Rust-built desktop companion to Suno.com that gives power users
capabilities the official web/mobile clients don't offer — full remote library control,
local library parity, AI-assisted lyric/art generation, real-time audio-reactive
visualizer "music videos" with karaoke-grade synced lyrics, a design canvas for on-brand
overlays, keyframe animation, an automation pipeline for batch-producing content at
scale, and a long-horizon path toward a built-in lightweight DAW for recording source
audio destined for Suno.

Suno Station is a full-surface listening AND creation front-end for Suno.com — every
user-facing capability the Suno API exposes is interactable from the app, plus local-only
add-on features (visualizer videos, karaoke, recording).

Suno Station is *not* a Suno replacement or a music generation engine. It is a control-plane
and creative-production layer built on top of Suno's remote data plus local media
tooling.

## 2. Product Pillars

1. **Remote Library Mastery** — everything the official client can do to a user's Suno
   library, done faster, in bulk, and across multiple accounts.
2. **Local-Remote Parity** — download once, then browse/play/organize local copies with
   the same UX as remote, indistinguishable to the user.
3. **Visualizer Music Videos** — projectM-powered audio-reactive rendering, exportable
   as video, with optional karaoke-synced lyric overlay (Suno-provided timing enhanced
   or replaced by local Whisper alignment).
4. **Brand Canvas** — a design surface for placing/animating text & graphics over the
   visualizer output; keyframe system; built-in effect library.
5. **Automation at Scale** — once a "look" is dialed in manually, let it run
   unattended across hundreds/thousands of tracks (batch karaoke videos, batch
   "advertisement visualizer" videos, etc).
6. **Creative Assist (secondary, later)** — pluggable text-LLM (lyrics ideation) and
   image-gen (cover art) adapters, calling out to remote APIs or local
   OpenAI-compatible/ComfyUI-style servers. Not core-path early work.
7. **Extensibility** — plugin system (scripting first, WASM later) so end users can
   extend effects/automation without a Suno Station release.
8. **Full-Surface Client** — drive every user-facing Suno capability (library, playlists,
   personas, generation via Suno's own server-side endpoints, uploads, account surfaces)
   from one desktop app, managed better than the official clients.

## 3. Non-Goals (explicit, revisit only via ADR)

- Suno Station does **not** implement music generation *locally* — song creation always
  happens server-side via Suno's own API, which Station drives as a first-class client
  feature. No local AI audio/diffusion inference.
- Suno Station does **not** attempt to be a full multitrack DAW at v1. Recording is
  "capture a take for upload to Suno," not "produce an album." JUCE-equivalent DAW
  work is an explicit late-phase stretch goal (Phase 9), not core scope.
- Suno Station does **not** ship a real-time diffusion image model. Image generation is an
  adapter over remote APIs or an existing local server (ComfyUI/A1111-style), never a
  from-scratch Rust inference stack (unless a future ADR revisits this).
- Suno Station does **not** attempt to reverse-engineer Suno's *generation* pipeline —
  only its account/library/asset/lyrics REST & realtime surfaces needed for client
  functionality.
- No cloud backend of our own. Suno Station is a local-first desktop app; any server-side
  component (if ever needed for OAuth relay) is minimal and documented separately.

## 4. Target Users

- **Primary:** Suno power users producing many tracks who want faster library
  ops, better lyric/karaoke handling, and shareable visualizer videos.
- **Secondary:** Small creators/agencies producing branded "AI music" content at
  volume (the automation pipeline is squarely for this group).
- **Tertiary:** Multi-account jugglers (agencies managing several client Suno
  accounts) needing fast account switching.

## 5. Platform Targets

Windows, macOS, Linux — desktop parity from day one where feasible. If a phase's
work is genuinely platform-blocked (e.g., a webview quirk), document the gap in that
phase's spec rather than silently dropping a platform.

## 6. Technology Spine (see 01-architecture-overview.md for detail)

- **Language:** Rust, workspace-of-crates.
- **UI:** native Rust GUI (egui or iced — decision recorded in 01), no Electron/web
  wrapper for the primary shell. Web-tech may be *bolted on* later for isolated
  sub-surfaces (e.g. a canvas editor) if native proves insufficient — never as the
  foundation.
- **Visualizer:** projectM (C++ lib) via FFI bridge crate.
- **Audio:** symphonia (decode), cpal (I/O), rodio-style mixer for playback;
  whisper-rs for local transcription/alignment.
- **Video export:** shell out to system `ffmpeg` binary.
- **Storage:** SQLite via sqlx; OS keyring for secrets.
- **LLM Dev Process:** built by an AI coding agent (GLM-5.2 primary, occasional
  Google-model assist for UI/UX-flavored tasks) operating inside an agentic CLI
  harness (opencode + oh-my-openagent/"sysaphus" orchestrator), with the human as
  orchestrator/reviewer, not line-by-line author. See 03-agent-constitution.md.

## 7. Definition of "Done" for the Project (north star, not a v1 checklist)

A user can: log into 1+ Suno accounts (any auth method), browse/search/bulk-manage
their remote library, download tracks locally with full local playback parity,
pull or generate karaoke-grade timed lyrics, design a branded overlay in the canvas
editor with keyframed animation, render a one-off visualizer video, then convert
that manual recipe into an automation pipeline and batch-render hundreds of videos
unattended — all inside one native, fast, good-looking (Catppuccin/Monokai-themed,
glass-panel) desktop app.

## 8. How These Docs Are Used

This doc set is the **master set of truths** for the project. Every subsequent
prompt handed to the coding agent should reference the relevant doc(s) by filename
and must not contradict them. If reality forces a contradiction, the correct action
is: stop, raise it to the human orchestrator, update the doc via ADR
(see 17-glossary-and-decisions-log.md), *then* continue coding. Docs are updated
deliberately, not silently drifted from.
