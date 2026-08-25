# Phase Roadmap

> **Last Updated:** 2026-08-25 · **Status:** Active

## §0. Phasing Is a Guide, Not a Blind Queue — Core Maintainability Gate

Phases 0-5 constitute **"Core."** Phases 6 (LLM Creative Integrations), 6b
(Suno Creation Studio), and 7 (Automation Pipelines) are **not allowed to
begin** until Core passes the **Core Maintainability Gate** defined in
`docs/process/18-codebase-health-guardrails.md`.
This is a deliberate, named checkpoint — not implicit — specifically because
the predecessor C++/Qt project reached ~25k LOC of unmaintainable spaghetti
by the time it got this far, and LLM-integration features were part of what
got bolted on under that decay (see ADR-008). We are not repeating that.

Order of Core phases (0→5) stays sequential as written below. What's
flexible is what happens **after** Core: once the Gate passes, the human
orchestrator chooses whether Phase 6 (LLM/image-gen), Phase 6b (Creation
Studio), or Phase 7 (Automation) goes next based on product priority at that
time — all three are equally "unlocked," none is forced to wait on the others.

---

## Phase Roadmap — Entry/Exit Criteria

Each phase below lists: **Goal**, **Primary crates touched/created** (paths per
doc 02), **Entry criteria** (must be true before starting), **Exit criteria**
(definition of "phase done," in addition to the universal per-task DoD in doc
03 §5), **Key risks / required spikes**, and **Explicit non-goals** (things
that will tempt scope creep — named so the agent doesn't wander into them).

Phases are sequential by default but not strictly blocking — e.g. Phase 4
(Visualizer) could start once Phase 2 (Playback) is stable even if Phase 3
(Lyrics) is still wrapping up, if the human orchestrator chooses to reorder.
Docs are written phase-by-phase as prompts; this roadmap doc itself is written
in full now so later phase docs can reference "we are here" against the whole
arc.

---

## Phase 0 — Foundation

**Goal:** A compiling, running, empty-but-structurally-complete app shell with
theming, logging, config, and the full workspace skeleton in place.

### Crates
- All of `foundation/*`
- Workspace root `Cargo.toml`, `app/station-app` binary (opens a themed empty
  window with nav placeholder)
- Stub `lib.rs`/`README.md` for every crate in doc 02's full tree (even ones
  not implemented until much later)

**Entry criteria:** This doc set is finalized and approved by the human.

### Exit criteria
- `cargo build --workspace` succeeds with the full crate tree present.
- App launches to a themed (Catppuccin default) empty window with a nav shell
  and a working theme-switcher (proves the design-token → UI plumbing works
  end to end).
- Structured logging writes to a rotating local log file + stdout.
- Config loader reads a TOML config file from the OS-appropriate config dir,
  with sane defaults if absent.
- Egui-vs-iced spike decision made and recorded as an ADR (doc 17) if it
  wasn't already obviously egui.

**Risks/spikes:** Per audit C-4/C-3, the compositing spike mandate is widened:
it must evaluate BOTH (a) egui+glow same-context compositing of a
projectM-rendered frame AND (b) egui+wgpu cross-API texture sharing, before
any renderer-backend commitment is made (see doc 01 §4). The spike must also
probe glassmorphism feasibility — true backdrop-blur vs the Tier-B translucent
fallback per doc 08 §3. Record the outcome as an ADR (doc 17). If this spike
reveals a hard blocker, resolve it here, not in Phase 4.

**Non-goals:** No real Suno connectivity, no real audio, no real visualizer
output yet — this phase is purely scaffolding + the compositing spike.

---

## Phase 1 — Suno Core (Accounts, Auth, Remote Library, Downloads)

**Goal:** Full multi-account Suno auth, and the ability to browse/search the
remote library (tracks and playlists) and download tracks locally.

### Crates
- `external-bridges/suno-http-client-core`
- `external-bridges/suno-auth-manual-token-paste`
- `external-bridges/suno-auth-embedded-webview-login`
- `external-bridges/suno-auth-oauth-loopback-google-fb`
- `external-bridges/os-keyring-secret-storage`
- `domain-stores/account-profile-store`
- `domain-stores/suno-remote-library-cache-store`
- `domain-stores/local-download-manager-store`
- `application-services/suno-library-sync-service`
- `application-services/suno-bulk-library-operations-service`
- `application-services/track-download-orchestration-service`
- `ui/ui-screen-account-management`
- `ui/ui-screen-remote-library-browser`

**Entry criteria:** Phase 0 done. At least one real Burp Suite capture of a
Suno login + library-list + track-detail flow provided by the human (see doc
06) so `suno-http-client-core` isn't built against guesses.

### Exit criteria
- User can add an account via manual token paste (bootstrap path) AND via the
  embedded-webview flow for Suno-native login AND via loopback OAuth for at
  least one federated provider (Google or Facebook — whichever the human can
  test against first).
- User can add a 2nd account and switch the "active account" in the UI, with
  the library browser correctly re-scoping.
- Remote library browser lists tracks/projects, supports search/filter, and
  at least one bulk operation (e.g. bulk tag or bulk delete — whichever the
  captured traffic supports first).
- Download manager can queue and complete downloads of remote tracks to a
  local folder, tracked in `local-download-manager-store`, with resumable/
  retryable behavior on failure.
- Playlist browsing/management works: the user can list their playlists,
  view a playlist's tracks, create/rename/trash a playlist, and add/remove
  clips — built from doc 06 §2.9 leads once captured.
- Token refresh (for the manual/webview paths) is handled automatically when
  a request 401s, without forcing re-login, where Suno's API supports it —
  document in doc 06 if it doesn't and a manual re-auth is required instead.

**Risks/spikes:** Confirm whether Suno's refresh-cookie approach actually
yields a working silent-refresh flow, or if bearer tokens simply expire and
require re-login/re-paste periodically — this determines UX (silent vs. a
"please re-auth" prompt) and must be captured in doc 06, not assumed.

**Non-goals:** No local playback yet (download only, no player UI beyond
maybe a bare "play in system default app" stopgap). No lyrics handling yet.

---

## Phase 2 — Local Playback Parity + Basic Recording

**Goal:** Local files downloaded in Phase 1 play back with a full-featured
player UI indistinguishable in capability from browsing remote (queue,
shuffle, seek, volume, gapless-ish behavior where feasible); basic
audio-take recording exists.

### Crates
- `external-bridges/audio-decode-symphonia-bridge`
- `external-bridges/audio-io-cpal-bridge`
- `domain-stores/recorded-audio-take-store`
- `application-services/local-playback-parity-service`
- `application-services/audio-recording-capture-service`
- `ui/ui-screen-local-library-browser`
- `ui/ui-screen-recording-studio` (minimal: device select, record, stop, save
  take, playback the take — not a DAW yet)

**Entry criteria:** Phase 1 done; at least a handful of real downloaded
tracks available for testing playback.

### Exit criteria
- Local library browser plays local files with standard transport controls,
  queue management (including a shuffle toggle and repeat modes: off/all/one),
  and volume, matching what the remote browser's "preview
  play" offers (so switching between remote-preview and local-file playback
  feels the same to the user).
- Recording studio screen can select an input device, record a take, and
  save it locally with metadata (date, device, duration) in
  `recorded-audio-take-store`.
- No audio dropouts/glitches under normal use — basic manual QA pass, no
  formal perf benchmark required yet.

**Non-goals:** No mixing/multitrack (that's Phase 9). No auto-upload of takes
to Suno yet (log it in `99-ideas-backlog.md` if tempting to add early).

---

## Phase 3 — Lyrics / Karaoke Data

**Goal:** Timed lyrics pulled from Suno where available, enhanced/aligned
locally via Whisper where needed, editable in a dedicated UI.

### Crates
- `external-bridges/whisper-transcription-bridge`
- `domain-stores/lyrics-and-alignment-store`
- `application-services/karaoke-lyric-timing-resolution-service`
- `ui/ui-screen-lyrics-editor`

**Entry criteria:** Phase 1 (remote data access) and Phase 2 (local audio
access) done; a confirming capture of the aligned-lyrics endpoint
(`GET /api/gen/{id}/aligned_lyrics/v2/`, known from recon per doc 06 §2.4)
provided.

### Exit criteria
- For a track with Suno-provided timed lyrics, the app displays them
  correctly time-synced during local playback.
- For a track without them (or with sparse/low-confidence timing), the user
  can trigger local Whisper transcription+alignment and review/accept the
  result.
- Lyrics editor allows manual correction of word/line timing and text, with
  edits versioned (never silently overwriting the original source) per doc 01
  §6.
- At least one exported artifact (even a plain `.lrc`/`.srt` file) proves the
  timing data round-trips correctly — useful for later video export too.

**Non-goals:** No visualizer/video rendering integration yet — that's Phase
4/5's job to consume this data.

---

## Phase 4 — Visualizer (projectM)

**Goal:** Live audio-reactive visualizer preview inside the app, plus a
first one-off render-to-video pipeline via ffmpeg (no overlay/canvas yet —
raw visualizer output only).

### Crates
- `external-bridges/visualizer-projectm-ffi-bindings`
- `external-bridges/visualizer-projectm-frame-bridge`
- `external-bridges/video-export-ffmpeg-process`
- `application-services/single-track-visualizer-render-service`
- `ui/ui-screen-visualizer-preview`

**Entry criteria:** Phase 0's compositing spike succeeded (or its findings
have reshaped this phase's approach via ADR). Phase 2 done (need decoded PCM
to feed the visualizer).

### Exit criteria
- Live preview screen shows projectM visuals reacting to a locally playing
  track in real time, with preset switching.
- User can trigger a one-off "render to video" for a track: headless
  (non-realtime-bound) frame walk, piped to ffmpeg, producing a playable
  output video file with audio muxed in correctly (sync verified manually).
- Reasonable preset management (bundle a small default preset pack, allow
  pointing at a folder of additional `.milk` presets if the user has them).

**Risks/spikes:** Headless/offscreen rendering at export time (decoupled from
display refresh) is a distinct code path from the live-preview path — budget
explicit design time for this rather than assuming the preview path trivially
also serves export.

**Non-goals:** No overlay/canvas text or keyframes yet (Phase 5). No batch/
automation yet (Phase 7).

---

## Phase 5 — Canvas Overlay + Keyframe System

**Goal:** A design canvas for placing text/graphic elements over the
visualizer output, with a keyframe animation system and a small built-in
effects library, feeding into both live preview and export.

### Crates
- `domain-stores/canvas-scene-and-keyframe-store`
- `application-services/canvas-overlay-compositing-service`
- `ui/ui-screen-canvas-scene-editor`

**Entry criteria:** Phase 4 done (need a visualizer frame to composite over).
Phase 3 done if karaoke-bound elements are in scope for this phase's first
cut (recommended: yes — a "karaoke text" element type bound to the lyrics
timing service is a flagship feature, not an afterthought).

### Exit criteria
- User can add text and basic graphic (image/shape) elements to a scene,
  position/scale/rotate them freeform on a canvas overlaying a live visualizer
  preview.
- Keyframe timeline lets the user set property values (position, opacity,
  scale, color, etc.) at specific times with interpolation between them
  (at minimum linear + one easing curve).
- At least one built-in "karaoke text" element type exists, bound to the
  Phase 3 lyric timing data, correctly highlighting active words/lines.
- A small built-in effects library exists (e.g., fade-in/out, simple particle
  or glow effect) selectable per element.
- Scenes save/load via `canvas-scene-and-keyframe-store` and apply correctly
  to both live preview and the Phase 4 export path (single render code path
  per doc 01 §7's automation principle, applied here too — no "preview-only"
  vs "export-only" scene interpretation divergence).

**Non-goals:** No plugin-authored custom elements/effects yet (Phase 8). No
batch automation yet (Phase 7).

---

## Phase 6 — LLM Creative Integrations

**Goal:** Optional, pluggable text-LLM (lyric ideation) and image-gen (cover/
brand art) assist features, as adapters over remote APIs or a local
OpenAI-compatible/ComfyUI-style server.

### Crates
- `external-bridges/llm-text-provider-adapter`
- `external-bridges/image-gen-provider-adapter`
- Small UI touch-points inside existing screens (lyrics editor gets an
  "assist" panel; canvas editor gets an "generate art asset" action) rather
  than a dedicated new screen.

**Entry criteria:** Phases 3 and 5 done (this phase augments them, doesn't
stand alone). Explicitly deprioritized per ADR-006 — do not pull this phase
forward ahead of 1-5/7 without an explicit human decision to do so.

### Exit criteria
- At least one text-LLM provider adapter (OpenAI-compatible base, since it
  covers the widest surface) working end-to-end for a "suggest lyric lines"
  or "suggest a lyric edit" assist action.
- At least one image-gen adapter working end-to-end for "generate a piece of
  cover/brand art" producing an asset usable directly as a canvas element.
- Provider credentials managed via the same secret-storage approach as Suno
  auth (`os-keyring-secret-storage`), configured in settings.

**Non-goals:** No local on-device diffusion/inference model bundling. No
attempt to support every provider — one solid OpenAI-compatible path first,
others additive later.

---

## Phase 6b — Suno Creation Studio

**Goal:** Drive Suno's server-side creation features from the app — submit
generations and poll them to completion, Suno-native lyric tools, persona
selection, upload-a-take→create-song flow, credit-aware spending display.
This is the "creation front-end" half of the Full-Surface Client pillar
(doc 00).

### Crates
- `external-bridges/suno-generation-client`
- `external-bridges/suno-upload-client`
- `domain-stores/generation-job-store`
- `ui/ui-screen-creation-studio`

**Entry criteria:** Core Maintainability Gate passed (doc 18 §4) — unlocked
alongside Phases 6/7 per doc 04 §0; plus confirming captures for doc 06 §2.8
(generation) and §2.11 (uploads).

### Exit criteria
- User can submit a generation (prompt/style/persona/instrumental toggles as
  the captured API allows), watch job status poll to completion, and see
  resulting clips land in the library cache.
- Lyric-generation assist works end-to-end.
- Upload flow takes a locally recorded take through initialize/upload-finish/
  poll to produce a usable clip.
- Current credit balance is fetched and shown before any spend-confirm dialog.

**Non-goals:** No local generation/inference; no purchase/checkout flows
(billing read-only, doc 06 §2.13); no bulk auto-generation until the Phase 7
pipeline step exists.

---

## Phase 7 — Automation Pipelines

**Goal:** Turn a manually-dialed-in single-track recipe (scene + lyric policy
+ export settings) into a saved pipeline that batch-processes many tracks
unattended, at the scale of hundreds/thousands.

### Crates
- `domain-stores/automation-pipeline-definition-store`
- `application-services/automation-batch-render-orchestrator`
- `ui/ui-screen-automation-pipeline-builder`

**Entry criteria:** Phases 1, 2, 3, 4, 5 all done — this phase is pure
orchestration over already-working single-track capability, per doc 01 §7.

### Exit criteria
- User can define a pipeline: input set (e.g., "all tracks tagged X" or an
  explicit list), a scene template, a lyric-source policy (remote-preferred /
  whisper-preferred / remote-only), export settings, output naming/routing.
- Pipeline runs as a background job queue with a configurable concurrency
  limit, visible progress per item, and per-item error isolation (one failing
  track doesn't abort the batch).
- Crash/interrupt mid-run is resumable — already-completed items aren't
  redone, per doc 01 §7.
- A test run of at least ~20-50 tracks completes successfully end-to-end as
  the phase's practical validation (before claiming "thousands" scale is
  proven).

**Non-goals:** No plugin-authored pipeline steps yet (Phase 8) — pipeline
step types are a fixed, built-in set for now.

---

## Phase 8 — Plugin System

**Goal:** Promote `plugin-host-stub` into a working scripting-based plugin
system (Rhai preferred per doc 03/doc 11), letting users author custom canvas
effects and/or automation pipeline steps without a Suno Station release.

### Crates
- Replace `external-bridges/plugin-host-stub` internals (keep its public
  trait shape stable where possible) with a real Rhai-backed host.
- Touch points in `ui-screen-canvas-scene-editor` (custom effect hook) and
  `ui-screen-automation-pipeline-builder` (custom pipeline step hook).

**Entry criteria:** Phases 5 and 7 done (plugins hook into both).

### Exit criteria
- A documented, minimal plugin API (Rhai script receives/returns well-defined
  data — e.g., a per-frame property-modifier function for a canvas effect)
  with at least 2 example plugins shipped as references.
- Sandboxing: a plugin cannot access the filesystem/network/process beyond
  what's explicitly exposed to it.
- WASM tier remains a documented future extension point (doc 11), not built
  in this phase unless a real need surfaces.

**Non-goals:** No plugin marketplace/distribution infra — local file-based
plugin loading is sufficient for this phase.

---

## Phase 9 — DAW Evolution

**Goal:** Grow the Phase 2 basic recorder toward a lightweight multitrack
digital audio workstation sufficient for producing source audio for Suno
projects, evaluating how far to go toward a JUCE-equivalent.

**Crates:** New `application-services`/`ui` crates TBD at phase-start (not
pre-named here — this phase is far enough out that premature crate-naming
would likely misjudge the real shape needed; name them when the phase doc is
written).

**Entry criteria:** All prior phases stable; explicit human decision to
invest in this phase (it's the most open-ended/optional in the roadmap).

**Exit criteria:** Defined in that phase's own doc when written — at minimum,
multitrack recording, basic mixing (gain/pan/basic EQ), and export to a
format Suno's upload accepts.

**Non-goals (likely permanent, revisit via ADR only):** Not attempting to be
a general-purpose DAW competing with Ableton/Reaper/etc — scope stays
tethered to "produce good source audio for Suno projects."
