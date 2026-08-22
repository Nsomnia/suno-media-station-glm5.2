## Conversation

### 🧑‍💻 User

The user is taking a c++ and qt + qlm project and taking lessons learned during its test-bed development process to now re-create it with modern llm models and using the rust language alongside either a direct rust ui/ux framework/library or wrapping it around another language. The project is a suno.com front-end with advanced features not possible, or not easily possible, via the offical web and mobile clients as well as adding many features that are directly possibler or by bootstrapping features locally on-top of the remote data. LLM integration will allow for easy generation of textual elements such as lyrics or image models for song art. The ability for full remote suno library control alongside local downloads from suno remote in addition to all the same features for local audio playback will have a seamless experience. As note eventually automation will another aspect of the program such that the end-user can setup a pipeline and have it crate thousands of these karaoke or animated advertisment visualizer music videos once they have their work dialed in. The other main feature of the initial project and eventual product is using projectM to create one-off or automated visualizer centered "music videos" as well as using available timed lyrical data from the suno remote and an optional local whisper model to enhace available timed lyrics for a karaoke/"sing along" abiliy in the videos. The ability to have a canvas for placing and designing textual and graphical elements to render on-top of the visualizer will allow high ai music brand customization and will enable animations of these elements with a keyframe system as well as built-in effects. A plugin system defined will allow further end-user customization. The ability to record audio to then be used in suno projects can be enhanced by providing a form of digital audio workstation perhaps eventually evolving to use or recreate somerthing like the JUCE library. The ui should be modern and glassy with a initial theme(s) choice of Catppuccin varities or other popular theme(s) such as monokai. Means of handling LLM for optimal performance is critical:

classes should be highly specific not totalling more than some hundred lines of code each and focusing on doing one piece of specific logic well with highly nested directories to organize these many classes which if using with class filename and directory names having verbose names that explain exactly what they are then this also allows for better LLM performnance?
only an LLM will be writting code and thus must handle all rolls of the development team from junior code monkey to senior lead develeoper and project lead whereas the userr will simply be the orchestrator as well as handling minor tasks
the llm model should be advicedto make tool calls or execute gh github-cli commands on the users host system to find example code to borrow from or to find libraries and other pre-existing code instead of reinventing the wheel.
the model will be likely operating in the opencode cli agent coding package with the oh-my-openagent sysaphus orchestrator
glm-5.2 seems like the likely model for the majory of the projects development as it is free right now and the user has good success during their investagory previous exploratory project to develop these feature ideas using the older glm-5 model so this should be much superior
the ability to have multiple suno accounts that can be switched between easily may be benfital to power-users
suno auth can be done via jwt/bearer token and refresh cookies found in the users browser with a js snippet plus a loopback network localhost means of allowing google and facebook sign-in
User Requirments
Use this information and go back-and-forth with the usrer or provide choices wherre appropriate to develop this project at a high-level,. MNavigate through conversation and decision making with the user providing options to select form wheere needed to outkline this entire project so that the entire stack is known and so that when the user is happy with what we have outlined then any number of prompts can be written to provide as markdown documents to an LLm inside an agentic coding harness package to then go step by step developing. We will worry about git workftree, workflow, branch, tags, releasess, and such once the project is outlined.

Do not output all the docs yet first verify any questions you have to the user first and when satisfied and the user instructs to start the complete project prompt then we can really get consuming tokens.

### 🤖 Assistant — claude-sonnet-5-high

**Suno-Rust Rewrite — Architecture Discovery (Round 1)**

Understood — no docs yet. Below is a structured set of the fork-in-the-road decisions I need from you, plus a set of "sane defaults" I'll assume unless you object. Once we lock these, I'll draft the actual planning doc set (vision, architecture, per-phase specs, agent constitution, testing strategy) as markdown files ready to feed the coding agent.

I've grouped things into **Critical Path** (please answer these — everything else cascades from them) and **Secondary/Defaults** (skim and flag disagreements only).

---

**PART 1 — Critical Path Decisions**

**1. UI / Rendering Architecture ⭐ (the linchpin decision)**

This determines how projectM's real-time GL output, the freeform canvas/keyframe editor, and the "glassy" aesthetic all fit together.

| Option | Description | Pros | Cons |
|---|---|---|---|
| **A. Native Rust GUI** — `egui` or `iced` (wgpu-backed) | Single process/GPU context. Visualizer output becomes a shared texture rendered directly inside the same UI surface. | Cleanest integration with projectM; `iced::widget::canvas` is purpose-built for a freeform scene editor; `catppuccin-egui` already exists. | "Glassy" polish requires manual shader/blur work; smaller LLM training corpus than web stacks. |
| **B. Tauri hybrid** — Rust core + web frontend (React/Svelte + TS) for chrome, canvas editor, settings; visualizer rendered in a positioned native child window | Best LLM code-gen quality (huge web training data), trivial glassmorphism via CSS `backdrop-filter`, tons of open-source timeline/canvas editors to crib from (Konva, Fabric, GSAP). | Embedding a real-time OpenGL/wgpu surface *seamlessly* inside/behind a webview cross-platform is genuinely hard (child-window compositing tricks à la OBS browser dock/Discord overlay). |
| **C. `gpui`** (Zed's framework) | Proven "glassy," modern, GPU-native, ships in a real production app. | Small ecosystem, sparse docs → less LLM leverage, no existing theme crates to borrow. |

**My lean:** A, since the visualizer is the *center of gravity* of nearly every screen (player, canvas editor, automation preview) — keeping it native-first avoids the hardest integration problem. But B is legitimate if you value dev velocity/aesthetics over that complexity and are willing to solve the embedding problem once, early.

**Question:** Pick A / B / C. If A, do you have a lean toward `egui` (immediate-mode, simpler, more manual state mgmt) vs `iced` (Elm-architecture, better fit for a stateful app with many small widget-classes, built-in canvas widget)?

---

**2. Code Organization Strategy ⭐**

You explicitly want small, highly-specific, verbosely-named files for LLM performance. Two ways to achieve that in Rust:

- **A.** Single/few crates with a deeply nested `src/` module tree.
- **B.** A **Cargo workspace of many small domain crates** — e.g. `suno-api`, `suno-auth-jwt-cookie`, `suno-auth-oauth-loopback`, `audio-playback-engine`, `audio-recording-daw-lite`, `visualizer-projectm-bridge`, `canvas-scene-graph`, `canvas-keyframe-animation`, `plugin-host-rhai`, `plugin-host-wasm`, `llm-provider-openai-compatible`, `image-gen-provider-adapter`, `whisper-transcription-bridge`, `video-export-ffmpeg`, `pipeline-automation-engine`, `theme-catppuccin`, `local-db-sqlite`, `ui-shell`, etc.

**Recommendation: B.** Each crate = a clean compile boundary + explicit public API (`lib.rs` as contract) + isolated tests. This maps almost 1:1 to "one crate ≈ one ticket the agent can hold entirely in context," which directly serves your stated goal.

**Questions:**
- Confirm B (or A)?
- What's your target max-LOC-per-file? I'd propose a **soft cap ~150–200 lines, hard cap ~300** (refactor/split beyond that) — agree or give me a number.

---

**3. Plugin System Runtime**

- **A. WASM** (wasmtime/wasmer, component model) — sandboxed, cross-platform, "modern," heavier to build tooling for.
- **B. Native dylibs** (Rust trait ABI via `abi_stable`) — fast but fragile across compiler versions, unsafe, no sandbox.
- **C. Embedded scripting** — `Rhai` (pure Rust, no C toolchain, easy sandbox) or Lua (`mlua`) for user-authored effects/automation logic.
- **D. Tiered:** Rhai/scripting for lightweight user hooks (custom effects, automation steps) shipped early, **WASM plugin host added later** for heavier compiled plugins.

**Recommendation: D.** Start simple (scripting), earn the right to add the heavier WASM host in a later phase.

**Question:** Approve D? Preference between Rhai vs Lua for the scripting tier?

---

**4. Suno Auth Capture Method**

- **A. Manual:** user runs a bookmarklet/JS snippet in their logged-in browser, copies bearer token + refresh cookie into the app.
- **B. Embedded webview login:** app opens a hidden/managed webview (via `wry`, usable standalone even if main UI is native) pointed at suno.com, intercepts network traffic/cookies to grab the token automatically.
- **C. Both** — B as polished default, A as manual/power-user/debug fallback.

**Recommendation: C.**

For Google/Facebook sign-in: confirm the **loopback approach** — spin a temp `127.0.0.1:PORT` HTTP server, open the OAuth URL (system browser or embedded webview) with `redirect_uri` pointed at loopback, capture the resulting code/token there. This works independent of A/B/C above.

**Question:** Approve C + loopback OAuth as described (matches your prior project's approach)?

---

**5. Roadmap / Phasing (strawman — please edit)**

I recommend we **document architecture for all phases up front**, but only write implementation prompts phase-by-phase.

- **Phase 0 – Foundation:** workspace scaffold, theme system, app shell, local DB, secrets/multi-account storage, agent "constitution" doc.
- **Phase 1 – Suno Core:** auth (manual + webview + OAuth loopback), multi-account switch, remote library browse/search/sync, local download manager.
- **Phase 2 – Playback:** local playback engine at feature parity with remote, basic record-and-upload.
- **Phase 3 – Lyrics/Karaoke:** Suno timed-lyrics fetch, local Whisper enhancement/alignment, lyric editor.
- **Phase 4 – Visualizer:** projectM bridge, preset management, audio-reactive render, first one-off render-to-video via ffmpeg.
- **Phase 5 – Canvas/Overlay + Keyframes:** scene editor for text/graphics over the visualizer, timeline, built-in effects.
- **Phase 6 – LLM Creative Integrations:** pluggable text-LLM (lyrics assist) and image-gen (art) adapters + in-app assist UI.
- **Phase 7 – Automation Pipelines:** pipeline builder + batch engine (thousands of videos, unattended).
- **Phase 8 – Plugin System:** Rhai hooks → WASM plugin host.
- **Phase 9 – DAW Evolution:** multitrack recording, evaluate JUCE-equivalent needs.

**Question:** Reorder/merge/split/add anything? Any phase you want pulled forward (e.g., visualizer before full library management)?

---

**6. In-App LLM & Image-Gen Provider Strategy**

- **Text:** adapter pattern with an **OpenAI-compatible base adapter** (covers OpenAI, OpenRouter, local Ollama/LM Studio, many others via one client) + optional first-class Anthropic/Gemini adapters.
- **Image (song art):** pluggable adapters — remote APIs (OpenAI Images, Stability, Ideogram, etc.) **and/or** calling a local existing server (ComfyUI/A1111 HTTP API) rather than reimplementing diffusion inference in Rust. Native local inference (`candle`) could be a much later phase if ever.

**Question:** Approve this adapter strategy? Any specific providers you want guaranteed day one?

---

**PART 2 — Secondary Decisions (defaults, flag if you disagree)**

- **Local DB:** SQLite via `sqlx` — library cache, lyrics, projects, pipeline defs, plugin registry.
- **Secrets/multi-account:** OS keyring (`keyring` crate) with encrypted-file fallback; N stored credential profiles, easy switcher.
- **Audio:** `symphonia` (decode) + `cpal` (I/O) + a thin mixer/`rodio` for playback and basic recording; full DAW mixing graph deferred to Phase 9.
- **Whisper:** `whisper-rs` bindings, user-selectable model size; used to *enhance/align*, never to fully replace Suno-provided timing when it exists.
- **Video export:** shell out to `ffmpeg` binary for muxing/encoding (simplest, most reliable); reconsider bindings only if pipeline throughput demands it.
- **Theme implementation:** swappable design-token system (colors/radii/blur) — concrete mechanics depend on Part 1 Q1 answer. Ships with Catppuccin (4 flavors) + Monokai at minimum.
- **Platforms:** Windows/macOS/Linux desktop parity, unless you want one prioritized first.
- **Testing:** per-crate unit tests + integration tests against recorded/mocked Suno API fixtures (never live endpoints in CI) + snapshot tests for canvas/keyframe serialization.
- **Agent Operating Rules ("constitution") doc:** I'll draft this to cover LOC limits, naming/nesting conventions, mandatory `gh`/crates.io prior-art search before implementing any non-trivial subsystem, self-review "junior pass → senior lead pass" ritual, and definition-of-done checklists per task.

---

**What I need from you now**

Answer the six Part 1 questions (letter picks or your own alternative are both fine), sanity-check the roadmap, and flag anything in Part 2. Once locked, I'll produce the full markdown doc set for the agentic harness — but only when you say go.

### 🧑‍💻 User

Native rust Gui will work and we can always shift to custom interface additions bolted on.  The cargo  workspace option to organize.  Max LOC while not strict should indeed Be 300. Plugin system can be ignored and kept empty boilerplate for now. Regarding the with capture bteppscijg the WebView option with capturing a callback local url might be preferred unless the WebView idea used the user's host system Brower as Google logins don't work  in electron packages for example nor qt versions of WebView the phasing seems fine and the user will provide real suno traffick captures From burpsuite when needed. In app LLM chef and media generation will be another example of a feature for far down the deveo pipeline there doesn't need much focus yet. Every assumption you made at the end of your output is sound for choices and thus you lay now write all these doc's for the user to copy and paste info their git repository. the use of will be using the glm-5.2 model which often is accented by occasional other models as inference allows with Google models sometimes used for ui/UX work.  Output until token pilots reached and unless you end output stating everything for this stage has been output then the user will respond with "continue" until seeing such. LLM management ideas can be sprinkled in where appropriate such as allowing thinking and any other tokens that are not to be directly readnl by an end-user to use "caveman speak" for token efficiency. Let's make this project slap due to starting off with top quality documentation as the projects master set of truths and guidance!

### 🤖 Assistant — claude-sonnet-5-high

Good — the OAuth clarification is an important catch, so let me lock that in explicitly before the docs, since it changes the auth doc's design:

**Auth mechanism split (final):**
- **Suno native email/password login** → embedded webview (`wry`) pointed at suno.com's own login page, network/cookie interception grabs bearer + refresh cookie. Suno's own page isn't blocking embedded user-agents (or if it starts to, manual JS-snippet fallback covers it).
- **Google / Facebook federated login** → **system default browser** (not embedded webview) via the `open`/`webbrowser` crate, with a temporary `127.0.0.1:PORT` loopback HTTP server as the OAuth `redirect_uri` to catch the callback. This is the standard "installed app" OAuth pattern and is exactly what avoids Google's `disallowed_useragent` block.
- **Manual JS-snippet paste** → always-available fallback/power-user/debug path, and the primary path while you're bootstrapping with Burp-captured traffic before the automated flows are wired up.

This is captured in the docs below. Now generating the full doc set — copy each block into the indicated file path in your repo. I'll keep going across multiple replies; I will explicitly say **"— END OF DOC BATCH, ALL DOCS FOR THIS STAGE DELIVERED —"** when complete. Otherwise reply `continue`.

---

```
docs/00-project-charter.md
```
```markdown
**Project Charter — "Sonora" (working name)**

> Working name used throughout these docs: **Sonora**. Rename freely at repo init —
> it's a find/replace across doc-set, not a design decision.

**1. Vision**

Sonora is a native, Rust-built desktop companion to Suno.com that gives power users
capabilities the official web/mobile clients don't offer — full remote library control,
local library parity, AI-assisted lyric/art generation, real-time audio-reactive
visualizer "music videos" with karaoke-grade synced lyrics, a design canvas for on-brand
overlays, keyframe animation, an automation pipeline for batch-producing content at
scale, and a long-horizon path toward a built-in lightweight DAW for recording source
audio destined for Suno.

Sonora is *not* a Suno replacement or a music generation engine. It is a control-plane
and creative-production layer built on top of Suno's remote data plus local media
tooling.

**2. Product Pillars**

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
   extend effects/automation without a Sonora release.

**3. Non-Goals (explicit, revisit only via ADR)**

- Sonora does **not** implement music generation. It is a client/production tool
  around Suno's generated audio.
- Sonora does **not** attempt to be a full multitrack DAW at v1. Recording is
  "capture a take for upload to Suno," not "produce an album." JUCE-equivalent DAW
  work is an explicit late-phase stretch goal (Phase 9), not core scope.
- Sonora does **not** ship a real-time diffusion image model. Image generation is an
  adapter over remote APIs or an existing local server (ComfyUI/A1111-style), never a
  from-scratch Rust inference stack (unless a future ADR revisits this).
- Sonora does **not** attempt to reverse-engineer Suno's *generation* pipeline —
  only its account/library/asset/lyrics REST & realtime surfaces needed for client
  functionality.
- No cloud backend of our own. Sonora is a local-first desktop app; any server-side
  component (if ever needed for OAuth relay) is minimal and documented separately.

**4. Target Users**

- **Primary:** Suno power users producing many tracks who want faster library
  ops, better lyric/karaoke handling, and shareable visualizer videos.
- **Secondary:** Small creators/agencies producing branded "AI music" content at
  volume (the automation pipeline is squarely for this group).
- **Tertiary:** Multi-account jugglers (agencies managing several client Suno
  accounts) needing fast account switching.

**5. Platform Targets**

Windows, macOS, Linux — desktop parity from day one where feasible. If a phase's
work is genuinely platform-blocked (e.g., a webview quirk), document the gap in that
phase's spec rather than silently dropping a platform.

**6. Technology Spine (see 01-architecture-overview.md for detail)**

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

**7. Definition of "Done" for the Project (north star, not a v1 checklist)**

A user can: log into 1+ Suno accounts (any auth method), browse/search/bulk-manage
their remote library, download tracks locally with full local playback parity,
pull or generate karaoke-grade timed lyrics, design a branded overlay in the canvas
editor with keyframed animation, render a one-off visualizer video, then convert
that manual recipe into an automation pipeline and batch-render hundreds of videos
unattended — all inside one native, fast, good-looking (Catppuccin/Monokai-themed,
glass-panel) desktop app.

**8. How These Docs Are Used**

This doc set is the **master set of truths** for the project. Every subsequent
prompt handed to the coding agent should reference the relevant doc(s) by filename
and must not contradict them. If reality forces a contradiction, the correct action
is: stop, raise it to the human orchestrator, update the doc via ADR
(see 17-glossary-and-decisions-log.md), *then* continue coding. Docs are updated
deliberately, not silently drifted from.
```

---

```
docs/01-architecture-overview.md
```
```markdown
**Architecture Overview**

**1. High-Level Shape**

```

┌─────────────────────────────────────────────────────────────────┐
│                         sonora-app (bin)                        │
│   native shell window, routing, top-level app state              │
└───────────────┬───────────────────────────────┬─────────────────┘
                │                               │
        ┌───────▼────────┐              ┌───────▼────────┐
        │   ui-* crates    │              │  domain crates   │
        │ (widgets/screens)│◄────events──►│ (business logic) │
        └───────┬────────┘              └───────┬────────┘
                │                               │
        ┌───────▼────────────────────────────────▼────────┐
        │              integration/bridge crates            │
        │  suno-api-* | visualizer-projectm-* | audio-* |    │
        │  whisper-* | ffmpeg-export-* | oauth-loopback-*    │
        └───────┬────────────────────────────────┬────────┘
                │                               │
        ┌───────▼────────┐              ┌───────▼────────┐
        │  local-db-sqlite │              │  external world  │
        │  keyring secrets │              │ suno.com, ffmpeg,│
        │                  │              │ projectM, whisper │
        └──────────────────┘              └──────────────────┘

```

**2. Guiding Architectural Principles**

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
   boundary — wrap it. `anyhow` is allowed only in the top-level `sonora-app` binary
   and in test code, never in library crate public signatures.
6. **State ownership:** each domain concern owns its own state store crate
   (e.g. `library-store`, `account-store`, `pipeline-store`) backed by SQLite;
   `ui-*` crates never talk to SQLite directly, only through a store crate's API.
7. **No God Objects.** No "AppState" mega-struct holding everything. Composition
   at the top-level binary wires narrow-interface handles together.

**3. Layered Crate Map (see 02-workspace-layout.md for the literal directory tree)**

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
- **Layer 5 — Binary:** `sonora-app`, composition root only. Should contain
  almost no logic — just wiring.

Dependencies only ever point downward (Layer 4 → 3 → 2 → 1 → 0). This is enforced
socially/via code review checklist for now; a `cargo-deny`/custom lint check to
enforce it mechanically is a nice-to-have backlog item, not a blocker.

**4. UI Framework Decision — RECORDED**

**Chosen: native Rust GUI**, immediate path = `egui` unless the agent's early
prototyping phase surfaces a hard blocker, in which case `iced` is the fallback
(both are pre-approved; pick during Phase 0 spike, document the choice + reasoning
as an ADR — see doc 17). Rationale for leading with `egui`: simpler mental model
for an LLM-driven, many-small-files codebase (immediate-mode = less hidden
state-machine complexity to keep consistent across files), mature `wgpu` backend
(needed for compositing projectM's GL/wgpu output as a texture into the same
frame), and existing Catppuccin theme crate (`catppuccin-egui`) to bootstrap
theming quickly.

Custom bolted-on interface surfaces (e.g., a heavier web-tech canvas editor) are
explicitly allowed **later**, as an opt-in embedded surface for one specific
screen, never as a replacement for the native shell. Any such addition requires
its own ADR justifying why native widgets were insufficient for that specific
screen.

**5. Visualizer Integration Model**

`visualizer-projectm-bridge` wraps projectM's C API via `bindgen`/FFI, renders
into an offscreen texture (or shares a GL/wgpu context, backend-dependent — a
Phase 4 spike decides the exact mechanism and records it as an ADR), and exposes
a small Rust trait: feed it audio PCM frames + get back a frame texture handle +
timing metadata. The `canvas-scene-graph` compositor layers the app's overlay
elements (text/graphics/keyframed animations) on top of that texture each frame.
Export (one-off or pipeline/batch) walks frames headlessly (not tied to the live
UI framerate) and pipes raw frames to `video-export-ffmpeg`.

**6. Lyrics/Karaoke Data Flow**

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

**7. Automation Pipeline Model**

A pipeline is a serialized recipe: input selection (which tracks/library filter),
a scene/template reference (canvas design + keyframes), lyric-source policy,
render/export settings, and output routing (local folder / naming convention).
The `pipeline-automation-engine` crate is a queue+worker over the same
Application Services layer used for one-off renders — **automation must never
duplicate render logic**; it only fans out the same single-track render service
across many inputs with concurrency limits and resumability (crash mid-batch →
resume, don't restart from zero).

**8. Multi-Account Model**

`account-store` holds N credential profiles (each: display name, auth method,
opaque secret handle into OS keyring, cached profile metadata). Exactly one
account is "active" per app session for the primary shell, but the underlying
Suno API client is parameterized by account, so power users switching frequently
is a cheap operation (swap the active credential handle, re-key cached library
views), not a re-login.

**9. Plugin System — Current Phase Status**

Per current decision, this is **scaffolded only**: a `plugin-host-stub` crate
exists with the trait definitions and a no-op registry, but no scripting engine
is wired in yet, and no UI surface for it ships yet. This keeps the seam in the
codebase (so later work is additive, not invasive) without spending build effort
now. See doc 11 for the deferred design.
```

---

```
docs/02-workspace-layout.md
```
```markdown
**Cargo Workspace Layout**

This is the literal directory structure to scaffold in Phase 0. Every crate's
`lib.rs` should begin with a doc comment restating its ONE job (copy the
"Purpose" line below into it). Directories are intentionally deep/verbose per
the LLM-context-legibility goal — prefer a clear nested path over a clever short
one.

```

sonora/
├── Cargo.toml                         # workspace root
├── docs/                              # this doc set (source of truth)
├── crates/
│   │
│   ├── foundation/
│   │   ├── error-and-result-conventions/      # shared Error trait helpers, thiserror macros
│   │   ├── app-configuration-loader/          # reads/writes app config (TOML), env overrides
│   │   ├── structured-logging-and-tracing/    # tracing subscriber setup, log file rotation
│   │   └── design-tokens-theme-definitions/   # Catppuccin/Monokai token structs, no UI code
│   │
│   ├── external-bridges/
│   │   ├── suno-http-client-core/             # low-level authenticated HTTP client to suno.com
│   │   ├── suno-auth-manual-token-paste/       # parse pasted JS-snippet JWT+cookie blob
│   │   ├── suno-auth-embedded-webview-login/   # wry-based Suno login capture
│   │   ├── suno-auth-oauth-loopback-google-fb/ # system-browser + localhost callback OAuth
│   │   ├── os-keyring-secret-storage/          # keyring crate wrapper, encrypted-file fallback
│   │   ├── visualizer-projectm-ffi-bindings/   # raw bindgen output + safe wrapper boundary
│   │   ├── visualizer-projectm-frame-bridge/   # PCM-in, texture-out trait + implementation
│   │   ├── audio-decode-symphonia-bridge/      # file → PCM decode wrapper
│   │   ├── audio-io-cpal-bridge/               # device I/O, playback + input capture
│   │   ├── whisper-transcription-bridge/       # whisper-rs wrapper, model mgmt
│   │   ├── video-export-ffmpeg-process/        # spawn/pipe-to ffmpeg, progress parsing
│   │   ├── llm-text-provider-adapter/          # trait + OpenAI-compatible impl (stub-priority: low)
│   │   ├── image-gen-provider-adapter/         # trait + remote/local-server impls (stub-priority: low)
│   │   └── plugin-host-stub/                   # trait defs + no-op registry only
│   │
│   ├── domain-stores/
│   │   ├── account-profile-store/              # multi-account CRUD + active-account switching
│   │   ├── suno-remote-library-cache-store/     # cached remote track/project metadata
│   │   ├── local-download-manager-store/        # download queue, local file bookkeeping
│   │   ├── lyrics-and-alignment-store/          # remote+whisper timed lyrics, versioned edits
│   │   ├── canvas-scene-and-keyframe-store/      # scene graph + keyframe track persistence
│   │   ├── automation-pipeline-definition-store/ # pipeline recipes, run history
│   │   └── recorded-audio-take-store/            # locally recorded takes metadata
│   │
│   ├── application-services/
│   │   ├── suno-library-sync-service/           # orchestrates cache-store <-> http-client
│   │   ├── suno-bulk-library-operations-service/ # bulk tag/delete/organize across tracks
│   │   ├── track-download-orchestration-service/ # drives download-manager-store end to end
│   │   ├── local-playback-parity-service/        # unifies remote-stream vs local-file playback
│   │   ├── karaoke-lyric-timing-resolution-service/ # merges remote+whisper per lyrics-flow doc
│   │   ├── single-track-visualizer-render-service/ # one-off "render this track as video"
│   │   ├── canvas-overlay-compositing-service/    # combines visualizer frame + scene graph
│   │   ├── automation-batch-render-orchestrator/  # fan-out of render-service across pipeline
│   │   └── audio-recording-capture-service/       # mic capture -> take-store -> (future upload)
│   │
│   ├── ui/
│   │   ├── ui-app-shell-and-navigation/         # window, top nav/routing, layout skeleton
│   │   ├── ui-shared-widget-library/            # buttons/cards/glass-panel primitives, themed
│   │   ├── ui-screen-account-management/        # add/switch/remove Suno accounts
│   │   ├── ui-screen-remote-library-browser/    # search/browse/bulk-ops on Suno library
│   │   ├── ui-screen-local-library-browser/     # local downloads browser/player
│   │   ├── ui-screen-lyrics-editor/             # karaoke timing review/edit UI
│   │   ├── ui-screen-visualizer-preview/        # live preview + one-off render controls
│   │   ├── ui-screen-canvas-scene-editor/       # freeform placement + keyframe timeline UI
│   │   ├── ui-screen-automation-pipeline-builder/ # pipeline authoring + run monitor UI
│   │   ├── ui-screen-settings-and-theming/      # theme picker, provider keys, prefs
│   │   └── ui-screen-recording-studio/          # basic capture UI (Phase 9 grows this)
│   │
│   └── shared-test-support/
│       ├── suno-api-fixture-mocks/              # recorded/mocked HTTP fixtures for tests
│       └── deterministic-test-clock-and-ids/    # test helpers for time/uuid determinism
│
├── app/
│   └── sonora-app/                    # the binary crate; composition root only
│       ├── Cargo.toml
│       └── src/main.rs
│
├── assets/
│   ├── themes/                        # catppuccin/monokai token json/toml sources
│   └── projectm-presets/              # bundled default presets (if redistributable)
│
└── xtask/                             # cargo-xtask style dev tooling (codegen, fixture capture helpers)

```

**Notes on This Layout**

- **Not every crate listed above needs code on day one.** Phase 0 scaffolds the
  *whole workspace skeleton* with each crate compiling as an empty stub
  (`lib.rs` with the purpose doc-comment and maybe one placeholder type) so the
  directory structure — the thing that gives the LLM agent navigable, guessable
  paths — exists before deep feature work starts. Filling them in happens per
  phase per doc 04.
- **Naming convention:** `kebab-case`, `noun-phrase-describing-the-one-job`,
  no abbreviations unless industry-standard (`ffi`, `io`, `db`). This is
  deliberate over-verbosity for agent legibility, not a style preference to
  fight.
- **300-line soft/hard cap** applies to files (`*.rs`), not crates. A crate
  like `ui-screen-canvas-scene-editor` will contain MANY files/submodules — that
  is expected and correct; keep nesting deeper (e.g.
  `src/keyframe_track/interpolation_curve_editor_widget.rs`) rather than
  growing one file.
- Any crate not yet implemented ships with a `README.md` stub stating its
  planned phase, so `gh`/repo browsing by the agent surfaces intent even for
  empty crates.
```

---

```
docs/03-agent-constitution.md
```
```markdown
**Agent Constitution — Operating Rules for the AI Developer**

This document governs HOW the coding agent (primary model: GLM-5.2, occasional
Google-model assist for UI/UX-flavored subtasks, operating inside an
opencode-CLI + oh-my-openagent "sysaphus" orchestrator harness) must work on this
codebase. The human is the orchestrator/reviewer, not a line-by-line author.
Treat this doc as binding; if a task instruction conflicts with this doc, this
doc wins and the conflict should be flagged back to the human.

**1. File & Function Size Discipline**

- **Soft cap: ~150–200 lines per `.rs` file. Hard cap: 300 lines.** If a file is
  approaching the hard cap, STOP and split it (extract a submodule, a new type,
  a new file) before continuing the feature — do not finish-then-refactor "later."
- Functions: prefer under ~40 lines. A function needing more is almost always
  asking to be decomposed into named helper functions (even private, single-call
  ones) — this is not wasted effort, it is documentation via naming.
- One `struct`/`enum`/`trait` "concept" per file where practical. A file named
  `interpolation_curve_editor_widget.rs` should be about that widget, not a grab
  bag.

**2. Naming & Directory Discipline**

- Verbose, descriptive, unambiguous names for files, directories, crates, and
  public types. Optimize for "an agent with no prior context can guess this
  file's contents from its path alone." Prefer
  `karaoke_lyric_timing_resolution_service.rs` over `lyrics.rs`.
- Deep nesting is *good* here, not a smell — group by feature/domain first,
  then by concern within it.
- Every crate's `lib.rs` (or `mod.rs` for a major module) starts with a doc
  comment: one sentence of purpose, then (if non-trivial) a short "why this
  exists / what it explicitly does NOT do" note.

**3. Prior-Art-First Mandate**

Before implementing any non-trivial subsystem (roughly: anything bigger than a
single small function), the agent MUST first check for existing solutions
rather than writing from scratch:

1. Search **crates.io** (via `cargo search` or docs.rs browsing) for an
   existing, maintained crate that solves the problem.
2. Use the **`gh` GitHub CLI** on the user's host system to search for example
   implementations, reference code, or entire libraries to borrow patterns
   from (`gh search repos`, `gh search code`, `gh repo clone <ref> --depth 1`
   into a scratch/tmp dir for inspection, never committed as-is).
3. Only write bespoke code when (2) turns up nothing suitable, or when a
   dependency would be inappropriate (license conflict, abandoned, huge
   transitive dep bloat, or wrong problem shape).
4. When borrowing patterns (not code verbatim) from an external repo, note the
   source (repo + rough concept borrowed) in a code comment or the PR
   description — attribution + a future-debugging breadcrumb, not legal
   copy-paste.
5. When adding a new external dependency, briefly justify it (in the commit
   message or PR description): what it replaces writing by hand, its
   maintenance status, its license.

This mandate exists because reinventing wheels burns tokens and produces worse,
less-battle-tested code than a mature crate/pattern. Skipping this step on
anything beyond trivial glue code is a process violation, not a style nitpick.

**4. Roles the Agent Must Self-Perform**

Since only the LLM writes code, each meaningful unit of work should pass
through these self-review "hats" before being presented as done — briefly, not
as theater:

1. **Junior implementer pass:** make it work, satisfy the spec/doc.
2. **Senior reviewer pass:** re-read the diff critically — error handling
   completeness, edge cases, does it violate any rule in this doc, is a simpler
   approach available, does it match the architecture doc's layering rules
   (doc 01 §2/§3)?
3. **Lead/project-owner pass:** does this change stay within the current
   phase's scope (doc 04)? Does it introduce scope creep, an undiscussed
   dependency, or an undocumented architectural decision that needs an ADR
   (doc 17)?

This is a lightweight internal checklist, not three separate slow passes with
full re-generation — but each should leave a trace (e.g., a short "self-review
notes" section in the task's summary/PR description) so the human orchestrator
can spot-check reasoning, not just the diff.

**5. Definition of Done (per task/ticket)**

A task is not done until:

- [ ] It compiles (`cargo build --workspace`) with zero warnings introduced.
- [ ] `cargo clippy --workspace --all-targets` is clean for touched crates.
- [ ] New/changed public behavior has at least a minimal unit test (domain/
      service/bridge crates) or is explicitly noted as UI-only/manual-QA
      (UI crates may lean on manual verification + later snapshot tests).
- [ ] File(s) touched respect the size caps in §1; if not, split before
      declaring done.
- [ ] No layering violation per doc 01 §2/§3 (UI doesn't reach into stores
      directly, domain doesn't import UI crates, etc).
- [ ] The relevant doc (02 workspace layout, or the phase's feature doc) still
      accurately describes what was built; update it if reality diverged
      (small, deliberate doc edits are expected and good).
- [ ] A short human-readable summary of what changed and why is produced
      (commit message and/or PR description) — written for a human skimming,
      not just a diff.

**6. Token & Reasoning Efficiency**

- **Internal reasoning/scratch/thinking-stream tokens** (anything not meant to
  be read directly by the end user or committed to docs/comments/commit
  messages) should be written in **terse, compressed "caveman speak"** —
  drop articles/filler words, use fragments, abbreviate aggressively. This is
  purely a token-economy technique for the model's own working/thinking
  channel and must NEVER leak into: code comments, doc-comments, commit
  messages, PR descriptions, UI strings, or any of the docs in this doc set.
  Those all remain fully professional, grammatical, and clear — this rule is
  about the invisible scratchpad only.
- Prefer referencing doc filenames/section numbers over re-explaining
  architecture inline when reasoning — the docs are the shared long-term
  memory; don't re-derive them each task.
- When a task is large, break it into an explicit internal checklist first
  (still terse/caveman in the scratch channel) rather than free-associating —
  cheaper to steer if it goes wrong, cheaper to resume after a context reset.
- Favor incremental, single-concern commits/diffs over sprawling multi-concern
  ones — smaller diffs are cheaper to review (by the agent's own senior-pass
  and by the human) and cheaper to re-attempt if wrong.

**7. Tool-Use Expectations**

- The agent is expected to actually invoke tools/shell (`gh`, `cargo`,
  `rg`/`grep`, filesystem reads) rather than guessing file contents or
  hallucinating API shapes — especially for anything touching the real Suno
  API surface, where ground truth comes from Burp-captured traffic the human
  provides (see doc 06) rather than invented assumptions.
- Never invent a Suno API endpoint's shape from thin air. If a needed endpoint
  isn't yet documented in doc 06, stop and ask the human orchestrator to
  capture it, rather than guessing a payload shape and shipping speculative
  code against it.
- When uncertain about a cross-cutting architectural choice not already
  decided in these docs, prefer asking (cheap) over assuming-and-redoing
  (expensive) — but for purely local implementation details fully within an
  already-decided architecture, proceed autonomously; the human is not meant
  to be a bottleneck for every line.

**8. Scope Discipline**

- Work strictly within the current phase's doc (04 + the specific feature doc)
  unless explicitly told to jump ahead. Seeing a good future feature idea
  mid-task → note it (e.g. append to a `docs/99-ideas-backlog.md`), don't build
  it now.
- The plugin system (doc 11) and LLM/image-gen adapters (doc 12) are
  intentionally stub-only until their phases arrive — do not "helpfully"
  flesh them out early; empty, well-documented boilerplate is the correct
  state for them pre-phase.
```

---

```
docs/17-glossary-and-decisions-log.md
```
```markdown
**Glossary & Architecture Decision Log (ADR)**

**Glossary**

- **Sonora** — working project name (this app).
- **Bridge crate** — a crate whose sole job is wrapping an external process/
  library/service behind a small trait, isolating the rest of the codebase from
  its concrete API.
- **Store crate** — a crate owning persistence + CRUD-ish logic for one domain
  concern, backed by SQLite.
- **Service crate** — orchestration logic composing bridges + stores into a
  user-meaningful operation.
- **Scene graph** — the canvas editor's in-memory/serialized tree of overlay
  elements (text, graphics, groups) plus their keyframe tracks.
- **Pipeline** — a saved automation recipe: inputs + scene template + lyric
  policy + export settings, runnable in batch.
- **Take** — a locally recorded audio clip captured for potential upload to
  Suno.

**ADR Log**

Format: `ADR-NNN: Title — Status — Date`. Keep entries short; link to the doc
section they amend if applicable. Add new entries as decisions are made or
revisited during development — this is a living doc.

**ADR-001: Native Rust UI over Tauri/web-hybrid**
- **Status:** Accepted
- **Context:** Needed a UI approach that integrates cleanly with a real-time
  projectM visualizer surface while remaining feasible for LLM-driven
  development.
- **Decision:** Native Rust GUI (egui primary, iced fallback), per doc 01 §4.
  Web-tech may be bolted on later for isolated sub-surfaces only, via ADR.
- **Consequence:** Visualizer texture compositing is native/wgpu-based, no
  cross-process/webview GL embedding problem to solve.

**ADR-002: Cargo workspace of many small crates over monolith**
- **Status:** Accepted
- **Context:** LLM code-gen performance benefits from small, single-purpose,
  well-named compile units.
- **Decision:** Workspace-of-crates per doc 02, organized by layer
  (foundation / external-bridges / domain-stores / application-services / ui).
- **Consequence:** More Cargo.toml boilerplate, offset by clearer boundaries
  and independent testability.

**ADR-003: File size caps — 300 line hard cap**
- **Status:** Accepted
- **Decision:** Soft cap ~150-200 lines, hard cap 300 lines per `.rs` file.
- **Consequence:** Enforced via agent constitution (doc 03 §1) self-discipline;
  a future lint/CI check may automate this.

**ADR-004: Plugin system deferred to stub-only**
- **Status:** Accepted
- **Decision:** `plugin-host-stub` crate holds trait definitions/no-op
  registry only; Rhai-scripting tier and later WASM tier are Phase 8 work
  (doc 04, doc 11).
- **Consequence:** Seam exists in the architecture now; no functional plugin
  execution until Phase 8.

**ADR-005: Auth mechanism split by login type**
- **Status:** Accepted
- **Context:** Google/Facebook OAuth blocks embedded webview user-agents
  (`disallowed_useragent`); Electron/Qt-webview equivalents hit the same wall.
- **Decision:** Suno native email/password → embedded webview (`wry`) cookie/
  token interception. Google/Facebook → system default browser + localhost
  loopback HTTP server callback (standard "installed app" OAuth flow). Manual
  JS-snippet paste always available as fallback and as the bootstrap path
  while working from Burp-captured traffic. See doc 05.
- **Consequence:** Three auth-bridge crates instead of one unified webview
  approach; more code, but each path actually works for its login type.

**ADR-006: LLM/image-gen adapters and plugin system are low-priority stubs**
- **Status:** Accepted
- **Decision:** Both get trait-only scaffolding early if convenient, full
  implementation pushed to their designated late phases (doc 04 Phase 6/8).
- **Consequence:** Early phases focus entirely on library/playback/
  visualizer/canvas/automation core value.

**ADR-007: Suno API contract is capture-driven, not guessed**
- **Status:** Accepted
- **Decision:** doc 06 (Suno API integration contract) is populated
  incrementally from real Burp Suite traffic captures the human orchestrator
  provides, not from agent speculation about endpoint shapes.
- **Consequence:** `suno-http-client-core` development is necessarily
  iterative/gap-filled rather than fully spec'd upfront; per doc 03 §7, the
  agent must stop and request a capture rather than invent a payload shape.
```

---

**continue** — next batch covers doc 04 (full phase roadmap with entry/exit criteria), doc 05 (auth & multi-account detail), doc 06 (Suno API contract skeleton), and doc 02-appendix (Cargo.toml workspace skeleton + crate stub template) unless you want me to reorder priority.

### 🧑‍💻 User

Continue

### 🤖 Assistant — claude-sonnet-5-high

Continuing with the phase roadmap, auth deep-dive, the Suno API contract template, and the literal workspace scaffold files.

---

```
docs/04-phase-roadmap.md
```
```markdown
**Phase Roadmap — Entry/Exit Criteria**

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

**Phase 0 — Foundation**

**Goal:** A compiling, running, empty-but-structurally-complete app shell with
theming, logging, config, and the full workspace skeleton in place.

**Crates:**
- All of `foundation/*`
- Workspace root `Cargo.toml`, `app/sonora-app` binary (opens a themed empty
  window with nav placeholder)
- Stub `lib.rs`/`README.md` for every crate in doc 02's full tree (even ones
  not implemented until much later)

**Entry criteria:** This doc set is finalized and approved by the human.

**Exit criteria:**
- `cargo build --workspace` succeeds with the full crate tree present.
- App launches to a themed (Catppuccin default) empty window with a nav shell
  and a working theme-switcher (proves the design-token → UI plumbing works
  end to end).
- Structured logging writes to a rotating local log file + stdout.
- Config loader reads a TOML config file from the OS-appropriate config dir,
  with sane defaults if absent.
- Egui-vs-iced spike decision made and recorded as an ADR (doc 17) if it
  wasn't already obviously egui.

**Risks/spikes:** Confirming projectM's rendering approach *can* share a
texture with the chosen UI framework's renderer — do a minimal spike (even
just rendering a solid-color quad from a separate GL/wgpu context into an
egui texture) before committing further, since this is the architecture's
riskiest integration point. If this spike reveals a hard blocker, resolve it
here, not in Phase 4.

**Non-goals:** No real Suno connectivity, no real audio, no real visualizer
output yet — this phase is purely scaffolding + the compositing spike.

---

**Phase 1 — Suno Core (Accounts, Auth, Remote Library, Downloads)**

**Goal:** Full multi-account Suno auth, and the ability to browse/search the
remote library and download tracks locally.

**Crates:**
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

**Exit criteria:**
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

**Phase 2 — Local Playback Parity + Basic Recording**

**Goal:** Local files downloaded in Phase 1 play back with a full-featured
player UI indistinguishable in capability from browsing remote (queue,
shuffle, seek, volume, gapless-ish behavior where feasible); basic
audio-take recording exists.

**Crates:**
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

**Exit criteria:**
- Local library browser plays local files with standard transport controls,
  queue management, and volume, matching what the remote browser's "preview
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

**Phase 3 — Lyrics / Karaoke Data**

**Goal:** Timed lyrics pulled from Suno where available, enhanced/aligned
locally via Whisper where needed, editable in a dedicated UI.

**Crates:**
- `external-bridges/whisper-transcription-bridge`
- `domain-stores/lyrics-and-alignment-store`
- `application-services/karaoke-lyric-timing-resolution-service`
- `ui/ui-screen-lyrics-editor`

**Entry criteria:** Phase 1 (remote data access) and Phase 2 (local audio
access) done; a Burp capture of Suno's timed-lyrics endpoint (if one exists
and is discoverable) provided — if Suno truly has no such endpoint, this
phase leans entirely on Whisper and doc 06 should say so explicitly.

**Exit criteria:**
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

**Phase 4 — Visualizer (projectM)**

**Goal:** Live audio-reactive visualizer preview inside the app, plus a
first one-off render-to-video pipeline via ffmpeg (no overlay/canvas yet —
raw visualizer output only).

**Crates:**
- `external-bridges/visualizer-projectm-ffi-bindings`
- `external-bridges/visualizer-projectm-frame-bridge`
- `external-bridges/video-export-ffmpeg-process`
- `application-services/single-track-visualizer-render-service`
- `ui/ui-screen-visualizer-preview`

**Entry criteria:** Phase 0's compositing spike succeeded (or its findings
have reshaped this phase's approach via ADR). Phase 2 done (need decoded PCM
to feed the visualizer).

**Exit criteria:**
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

**Phase 5 — Canvas Overlay + Keyframe System**

**Goal:** A design canvas for placing text/graphic elements over the
visualizer output, with a keyframe animation system and a small built-in
effects library, feeding into both live preview and export.

**Crates:**
- `domain-stores/canvas-scene-and-keyframe-store`
- `application-services/canvas-overlay-compositing-service`
- `ui/ui-screen-canvas-scene-editor`

**Entry criteria:** Phase 4 done (need a visualizer frame to composite over).
Phase 3 done if karaoke-bound elements are in scope for this phase's first
cut (recommended: yes — a "karaoke text" element type bound to the lyrics
timing service is a flagship feature, not an afterthought).

**Exit criteria:**
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

**Phase 6 — LLM Creative Integrations**

**Goal:** Optional, pluggable text-LLM (lyric ideation) and image-gen (cover/
brand art) assist features, as adapters over remote APIs or a local
OpenAI-compatible/ComfyUI-style server.

**Crates:**
- `external-bridges/llm-text-provider-adapter`
- `external-bridges/image-gen-provider-adapter`
- Small UI touch-points inside existing screens (lyrics editor gets an
  "assist" panel; canvas editor gets an "generate art asset" action) rather
  than a dedicated new screen.

**Entry criteria:** Phases 3 and 5 done (this phase augments them, doesn't
stand alone). Explicitly deprioritized per ADR-006 — do not pull this phase
forward ahead of 1-5/7 without an explicit human decision to do so.

**Exit criteria:**
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

**Phase 7 — Automation Pipelines**

**Goal:** Turn a manually-dialed-in single-track recipe (scene + lyric policy
+ export settings) into a saved pipeline that batch-processes many tracks
unattended, at the scale of hundreds/thousands.

**Crates:**
- `domain-stores/automation-pipeline-definition-store`
- `application-services/automation-batch-render-orchestrator`
- `ui/ui-screen-automation-pipeline-builder`

**Entry criteria:** Phases 1, 2, 3, 4, 5 all done — this phase is pure
orchestration over already-working single-track capability, per doc 01 §7.

**Exit criteria:**
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

**Phase 8 — Plugin System**

**Goal:** Promote `plugin-host-stub` into a working scripting-based plugin
system (Rhai preferred per doc 03/doc 11), letting users author custom canvas
effects and/or automation pipeline steps without a Sonora release.

**Crates:**
- Replace `external-bridges/plugin-host-stub` internals (keep its public
  trait shape stable where possible) with a real Rhai-backed host.
- Touch points in `ui-screen-canvas-scene-editor` (custom effect hook) and
  `ui-screen-automation-pipeline-builder` (custom pipeline step hook).

**Entry criteria:** Phases 5 and 7 done (plugins hook into both).

**Exit criteria:**
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

**Phase 9 — DAW Evolution**

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
```

---

```
docs/05-auth-and-multi-account.md
```
```markdown
**Authentication & Multi-Account Design**

**1. Why Auth Is Split Three Ways**

Suno's login surface covers native email/password AND federated login
(Google/Facebook). These cannot share one capture mechanism:

- **Native login** happens entirely on suno.com's own pages — an embedded
  webview can render this and intercept the resulting network traffic/cookies
  without issue, since Suno itself isn't blocking embedded rendering (Suno's
  page, not Google's).
- **Federated login (Google/Facebook)** hits the identity provider's own
  consent screens, which actively detect and block known embedded-webview
  user-agents (Google's `disallowed_useragent` error is the classic symptom —
  the same failure Electron and Qt `QWebEngineView`-based tools hit). The only
  reliable path is the **system's real default browser**, with the OAuth
  `redirect_uri` pointed at a temporary localhost server the app spins up to
  catch the callback ("installed application" OAuth pattern).
- **Manual paste** (bearer JWT + refresh cookie captured via a browser JS
  snippet, or from a Burp Suite capture) is always available as a fallback
  and is the **primary bootstrap path** during early development, since it
  requires no auth-flow code at all to start integrating against real Suno
  data.

**2. Crate Responsibilities**

- `suno-auth-manual-token-paste` — parses a pasted blob (JWT + cookie string,
  format TBD from first real capture) into a normalized `SunoCredential`
  struct. No network activity of its own.
- `suno-auth-embedded-webview-login` — opens a `wry`-based webview at Suno's
  login URL, watches outgoing requests/response `Set-Cookie` headers (or a
  post-login redirect target) for the bearer token + refresh cookie, then
  closes the webview and hands back a `SunoCredential`.
- `suno-auth-oauth-loopback-google-fb` — starts a short-lived
  `127.0.0.1:{ephemeral-or-fixed-port}` HTTP listener, opens the federated
  provider's OAuth URL in the **system default browser** via the `open` (or
  `webbrowser`) crate, receives the callback (auth code and/or token
  depending on Suno's flow shape — TBD from capture), exchanges/relays as
  needed, and hands back a `SunoCredential`. Shuts the listener down
  immediately after success or timeout.
- All three produce the same normalized output type so
  `suno-http-client-core` and `account-profile-store` never need to know
  which method produced a given account's credential.

```rust
// illustrative shape only — finalize once first real capture is in hand
pub struct SunoCredential {
    pub bearer_token: String,
    pub refresh_cookie: Option<String>,
    pub obtained_via: AuthMethod, // ManualPaste | EmbeddedWebview | OAuthLoopback
    pub captured_at: chrono::DateTime<chrono::Utc>,
}
```

**3. Storage & Security**

- `SunoCredential` values are never stored in SQLite in plaintext. They go
  into the OS keyring via `os-keyring-secret-storage`, keyed by an opaque
  account-id; `account-profile-store` (SQLite) holds only the account-id,
  display name, auth method, and non-sensitive cached profile metadata
  (username/avatar/etc, for display purposes).
- Encrypted-file fallback (for platforms/environments without a usable OS
  keyring, e.g. some Linux headless setups) uses a locally-generated key
  file with restrictive filesystem permissions — documented as a reduced-
  security mode in the settings UI, not silently equivalent to keyring
  storage.
- Logs must never contain full tokens/cookies. `structured-logging-and-
  tracing` setup should include a redaction layer/newtype
  (`SecretString`-style, e.g. via the `secrecy` crate) so accidental
  `{:?}`-formatting of a credential struct doesn't leak it into logs.

**4. Token Refresh Strategy**

- On any Suno API call returning 401, `suno-http-client-core` attempts a
  refresh using the stored refresh cookie (mechanism TBD — confirm from
  capture whether this is a distinct refresh endpoint or simply re-hitting
  a session endpoint with the cookie attached).
- If refresh succeeds, the new bearer token replaces the stored one
  transparently and the original request is retried once.
- If refresh fails or no refresh cookie is available for that account, the
  UI surfaces a clear "please re-authenticate this account" prompt scoped to
  that specific account (not a global logout) — critical in a multi-account
  app where other accounts may still have valid sessions.
- **This entire section is provisional until doc 06 has a real capture of
  Suno's refresh behavior.** Do not implement speculative refresh-endpoint
  code before that capture exists (per doc 03 §7).

**5. Multi-Account Switching Mechanics**

- `account-profile-store` tracks all known accounts + which one is "active"
  for the current app session.
- Switching accounts:
  1. Swap the active credential handle used to construct/parameterize the
     `suno-http-client-core` instance (the client is per-account-scoped, not
     a single global singleton with mutable auth state).
  2. Re-key/refresh the views backed by `suno-remote-library-cache-store`
     (which is itself keyed by account-id, so cached data for inactive
     accounts isn't discarded — switching back is instant/cheap, not a
     re-fetch).
  3. Local downloads (`local-download-manager-store`) are also account-
     scoped in their metadata (a downloaded track remembers which account it
     came from) even though the files themselves live in one shared local
     library folder — this matters for re-sync/dedup logic later.
- UI: an account switcher control (avatar/name dropdown or similar, exact
  design deferred to doc 08) always visible in the app shell, not buried in
  settings — this is a named power-user feature, not an edge case.

**6. Bootstrap Workflow During Development**

Until the embedded-webview and OAuth-loopback flows are built and verified,
all Suno integration work proceeds against credentials obtained via:

1. Human performs a real login in their own browser (or via Burp Suite as a
   proxy) and captures the resulting requests.
2. Human provides the **sanitized** capture (see doc 06 §1 for the
   redaction rule) to the agent as the ground truth for both the API
   contract AND to manually test `suno-auth-manual-token-paste` end to end.
3. `suno-auth-manual-token-paste`'s UI (a simple paste box in the account
   management screen) is therefore effectively a **Phase 1 priority-one**
   deliverable, even though it's the "least glamorous" of the three auth
   methods — everything else depends on it existing first for testing.
```

---

```
docs/06-suno-api-integration-contract.md
```
```markdown
**Suno API Integration Contract**

> **This document is capture-driven, not speculative.** Per ADR-007 (doc 17),
> no endpoint is documented here until the human orchestrator has provided a
> real traffic capture (typically via Burp Suite) of that endpoint in use.
> The agent must never invent an endpoint shape and must halt + request a
> capture (per doc 03 §7) when a needed endpoint is undocumented here.

**0. How This Doc Gets Filled In (process)**

1. Human performs the relevant action in the official Suno web/mobile client
   with Burp Suite (or similar) proxying traffic.
2. Human **redacts secrets** before sharing: replace bearer tokens, cookie
   values, API keys, and any personally-identifying fields (email, full
   name, phone) with placeholders like `<REDACTED_BEARER>`,
   `<REDACTED_COOKIE>`, `<REDACTED_EMAIL>`. Structural fields (IDs, status
   enums, timestamps, URLs to public assets) can stay real unless they
   themselves are sensitive.
3. Human pastes the sanitized request/response pair to the agent (or drops
   it in `docs/captures/raw/` as a `.http`/`.json` file — see §6).
4. Agent normalizes it into an entry below using the template in §1, adds a
   typed Rust request/response model in `suno-http-client-core`, and a
   fixture in `shared-test-support/suno-api-fixture-mocks` derived from the
   sanitized capture (fixtures are the sanitized versions — never real
   secrets — since they end up in the test suite / potentially the repo).
5. If a capture reveals a field whose meaning is unclear, the agent should
   ask rather than guess a name/type for it.

**1. Endpoint Entry Template**

Copy this block per endpoint as captures come in.

```

**<Human-readable name, e.g. "List Library Tracks">**

- **Method/Path:** `GET /api/...`
- **Auth:** Bearer required? Cookie required? Both?
- **Captured:** <date> from <client: web app / mobile app / unknown>
- **Request Headers (notable):**
  - `Header-Name: value or <REDACTED>`
- **Request Body:** (if any — JSON shape, sanitized)
- **Response Body (sanitized excerpt):**
  ```json
  { }
  ```
- **Fields of Interest:**
  - `field.path` — meaning, type, nullable?
- **Rust Model Location:** `suno-http-client-core::models::...`
- **Notes/Gotchas:** pagination style, rate limits observed, error shapes
  seen, anything inconsistent between calls.

```

**2. Known Endpoint Categories (to be populated)**

These are the anticipated categories based on product requirements — each
starts EMPTY (no captures yet) and is filled in as Phase 1/3 work demands
real data. Do not populate with guessed content.

- ### 2.1 Authentication
  - Native email/password login exchange
  - Session/token refresh
  - Federated (Google) OAuth handoff shape
  - Federated (Facebook) OAuth handoff shape
  - *(status: NOT YET CAPTURED)*

- ### 2.2 Library / Projects Listing
  - List tracks/projects (pagination shape, filters, sort)
  - Track/project detail (single item)
  - Search
  - *(status: NOT YET CAPTURED)*

- ### 2.3 Track Assets
  - Audio file URL(s) (streaming vs downloadable, expiry behavior)
  - Cover art URL(s)
  - *(status: NOT YET CAPTURED)*

- ### 2.4 Timed Lyrics
  - Endpoint for lyric text + timing, if one exists as a distinct resource
  - Confirm whether timing is word-level, line-level, or both
  - *(status: NOT YET CAPTURED — critical for Phase 3 scope; if this truly
    doesn't exist, Phase 3 doc must be updated to reflect Whisper-only reality)*

- ### 2.5 Bulk / Organizational Operations
  - Tagging, playlist/collection membership, delete, rename
  - Whether bulk endpoints exist natively or must be client-side-looped
    single-item calls
  - *(status: NOT YET CAPTURED)*

- ### 2.6 Account/Profile
  - Current user profile (for display in account switcher)
  - *(status: NOT YET CAPTURED)*

- ### 2.7 Rate Limits & Error Shapes
  - Observed `429`/`5xx` behavior, retry-after headers if any
  - Standard error response envelope shape
  - *(status: NOT YET CAPTURED — fill in opportunistically whenever any
    capture happens to include an error response)*

**3. Versioning / Drift Handling**

Suno's undocumented API can change without notice. When a previously-working
endpoint starts failing in a way that looks like a shape change (not an auth
issue), the response is:

1. Request a fresh capture of that specific endpoint from the human.
2. Update this doc's entry (keep the old one struck through/archived below
   it rather than deleted, so drift is visible over time).
3. Update the corresponding Rust model + fixture.
4. Add a short entry to `docs/17-glossary-and-decisions-log.md` if the change
   was significant enough to affect architecture (e.g., pagination style
   changed entirely).

**4. Rust Client Design Notes (non-endpoint-specific)**

- `suno-http-client-core` is built on `reqwest` (async, widely used, good
  middleware ecosystem) unless a captured quirk (e.g. required HTTP/2
  fingerprinting to avoid bot detection) forces reconsideration — flag as an
  ADR if so.
- Every response type derives `serde::Deserialize` with `#[serde(rename_all
  = "camelCase")]` or explicit `#[serde(rename = "...")]` per observed field
  casing — confirm actual casing from captures rather than assuming.
- Unknown/未-mapped fields should not cause deserialization failure —
  use `#[serde(flatten)] extra: serde_json::Map<String, serde_json::Value>`
  or `#[serde(default)]` liberally, since Suno may add fields without notice
  and the client shouldn't hard-break on that.
- Client is constructed per-account (see doc 05 §5) — no global mutable auth
  state.

**5. Captures Storage Convention**

Raw sanitized capture files (optional but encouraged for anything complex)
live at:

```

docs/captures/raw/<category>/<short-description>.http

```

e.g. `docs/captures/raw/library/list-tracks-page-1.http`. These are the
*evidence trail* backing this doc's entries — keep them even after
normalizing into §2, since re-deriving an edge case later is easier with the
raw capture on hand.

**6. Current Status Summary**

**As of doc creation: zero endpoints captured.** This document is a
skeleton/process definition only. Phase 1 cannot meaningfully begin
implementation of `suno-http-client-core` beyond boilerplate/error-type
scaffolding until at least the Authentication and Library Listing categories
have real captures. This is expected and correct — flag it to the human
early in Phase 1 if captures aren't yet provided, per doc 03 §7.
```

---

```
Cargo.toml
```
```toml
[workspace]
resolver = "2"
members = [
    "app/sonora-app",

    "crates/foundation/error-and-result-conventions",
    "crates/foundation/app-configuration-loader",
    "crates/foundation/structured-logging-and-tracing",
    "crates/foundation/design-tokens-theme-definitions",

    "crates/external-bridges/suno-http-client-core",
    "crates/external-bridges/suno-auth-manual-token-paste",
    "crates/external-bridges/suno-auth-embedded-webview-login",
    "crates/external-bridges/suno-auth-oauth-loopback-google-fb",
    "crates/external-bridges/os-keyring-secret-storage",
    "crates/external-bridges/visualizer-projectm-ffi-bindings",
    "crates/external-bridges/visualizer-projectm-frame-bridge",
    "crates/external-bridges/audio-decode-symphonia-bridge",
    "crates/external-bridges/audio-io-cpal-bridge",
    "crates/external-bridges/whisper-transcription-bridge",
    "crates/external-bridges/video-export-ffmpeg-process",
    "crates/external-bridges/llm-text-provider-adapter",
    "crates/external-bridges/image-gen-provider-adapter",
    "crates/external-bridges/plugin-host-stub",

    "crates/domain-stores/account-profile-store",
    "crates/domain-stores/suno-remote-library-cache-store",
    "crates/domain-stores/local-download-manager-store",
    "crates/domain-stores/lyrics-and-alignment-store",
    "crates/domain-stores/canvas-scene-and-keyframe-store",
    "crates/domain-stores/automation-pipeline-definition-store",
    "crates/domain-stores/recorded-audio-take-store",

    "crates/application-services/suno-library-sync-service",
    "crates/application-services/suno-bulk-library-operations-service",
    "crates/application-services/track-download-orchestration-service",
    "crates/application-services/local-playback-parity-service",
    "crates/application-services/karaoke-lyric-timing-resolution-service",
    "crates/application-services/single-track-visualizer-render-service",
    "crates/application-services/canvas-overlay-compositing-service",
    "crates/application-services/automation-batch-render-orchestrator",
    "crates/application-services/audio-recording-capture-service",

    "crates/ui/ui-app-shell-and-navigation",
    "crates/ui/ui-shared-widget-library",
    "crates/ui/ui-screen-account-management",
    "crates/ui/ui-screen-remote-library-browser",
    "crates/ui/ui-screen-local-library-browser",
    "crates/ui/ui-screen-lyrics-editor",
    "crates/ui/ui-screen-visualizer-preview",
    "crates/ui/ui-screen-canvas-scene-editor",
    "crates/ui/ui-screen-automation-pipeline-builder",
    "crates/ui/ui-screen-settings-and-theming",
    "crates/ui/ui-screen-recording-studio",

    "crates/shared-test-support/suno-api-fixture-mocks",
    "crates/shared-test-support/deterministic-test-clock-and-ids",
]

[workspace.package]
edition = "2021"
license = "UNLICENSED"
publish = false

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
tracing = "0.1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
secrecy = { version = "0.8", features = ["serde"] }

**NOTE: pin exact versions once Phase 0 dependency selection is finalized.**
**Placeholder ranges above are a starting point, not a locked decision.**
```

---

```
docs/templates/crate-stub-template/Cargo.toml
```
```toml
[package]
name = "REPLACE_ME_kebab_case_crate_name"
version = "0.1.0"
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
**Add only what this crate's ONE job actually needs.**
**Prefer workspace.dependencies entries where applicable.**
```

---

```
docs/templates/crate-stub-template/src/lib.rs
```
```rust
//! Purpose: <ONE sentence — what this crate does>.
//!
//! This crate does NOT: <one line, if useful — what's explicitly out of
//! scope for this crate, to prevent scope-creep into a neighboring crate's
//! responsibility>.
//!
//! Status: STUB — scaffolded in Phase 0, implementation scheduled for
//! Phase <N> per docs/04-phase-roadmap.md.

// Intentionally empty beyond this doc-comment until its phase begins.
```

---

```
docs/templates/crate-stub-template/README.md
```
```markdown
**<crate-name>**

**Purpose:** <one sentence, mirrors the lib.rs doc-comment>

**Layer:** foundation | external-bridges | domain-stores |
application-services | ui | shared-test-support

**Planned phase:** Phase <N> — see `docs/04-phase-roadmap.md`

**Public API status:** not yet implemented (stub)

**Key dependencies (planned):** <crates/libraries this will lean on>

**Depended on by (planned):** <which other crates will consume this one>
```

---

**continue** — next batch: doc 07 (data model & storage schema across all stores), doc 08 (UI/UX design system — glass theme tokens, Catppuccin/Monokai spec, layout conventions), and doc 16 (testing strategy) unless you'd like a different order.

### 🧑‍💻 User

Continue.

### 🤖 Assistant — claude-sonnet-5-high

Continuing with data model/storage schema, the UI/UX design system, and testing strategy.

---

```
docs/07-data-model-and-storage-schema.md
```
```markdown
**Data Model & Storage Schema**

**1. General Conventions (apply to every store crate)**

- **Engine:** SQLite via `sqlx` (compile-time checked queries where
  practical). Each store crate owns exactly one logical set of tables and
  never queries another store's tables directly — cross-store composition
  happens in the `application-services` layer, not via SQL joins across
  store boundaries. (A single physical `.sqlite` file is fine for simplicity;
  the boundary is logical/crate-level, not necessarily one-file-per-store.)
- **Primary keys:** UUID v4 (`TEXT` column, stored as canonical hyphenated
  string) for all locally-originated entities. For entities mirroring a Suno
  remote resource, store Suno's own remote ID as `remote_id TEXT UNIQUE` in
  addition to a local UUID primary key — never assume Suno's ID scheme
  (could be UUID, could be a short slug, could change) until confirmed via
  doc 06 capture.
- **Timestamps:** `created_at`, `updated_at` as `TEXT` ISO-8601 UTC
  (`chrono::DateTime<Utc>` via serde), on every table. Never rely on SQLite's
  own `CURRENT_TIMESTAMP` for anything the app logic reasons about — set
  explicitly from Rust so test determinism (`deterministic-test-clock-and-
  ids`) works.
- **Migrations:** `sqlx::migrate!` with numbered migration files per store
  crate at `<crate>/migrations/NNNN_description.sql`. Migrations are
  forward-only; a schema fix is a new migration, not an edited old one
  (except during Phase 0-1 before any real user data exists in the wild).
- **Soft delete where it matters:** entities a user might want to "undo"
  removing (e.g. a downloaded track's local record, a pipeline definition)
  get a `deleted_at TEXT NULL` column rather than a hard `DELETE`, at least
  for the first version of each store — hard-delete/vacuum can be a later
  optimization once retention policy is actually decided.
- **JSON columns are allowed** for genuinely flexible/nested data (scene
  graphs, keyframe tracks, pipeline step configs) — store as `TEXT`
  containing serialized JSON via `serde_json`, with a `schema_version
  INTEGER` sibling column so future migrations can transform old JSON blobs
  rather than being stuck. Do NOT use JSON columns as a lazy substitute for
  proper relational columns on data that's actually structured/queried
  (e.g., don't JSON-blob an account's display name).

**2. `account-profile-store`**

```sql
CREATE TABLE accounts (
    id              TEXT PRIMARY KEY,       -- local UUID
    display_name    TEXT NOT NULL,
    auth_method     TEXT NOT NULL,          -- 'manual_paste' | 'embedded_webview' | 'oauth_loopback'
    keyring_ref     TEXT NOT NULL,          -- opaque handle into os-keyring-secret-storage
    suno_user_id    TEXT,                   -- remote user id once known (from profile endpoint)
    avatar_url      TEXT,
    is_active       INTEGER NOT NULL DEFAULT 0,  -- exactly one row = 1 at a time (app-enforced)
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    deleted_at      TEXT
);
```

Note: `is_active` uniqueness is enforced in the store crate's Rust logic
(transaction: clear all, then set one), not a DB constraint — SQLite partial
unique indexes could do this but the extra complexity isn't worth it for a
single-row toggle at this scale.

**3. `suno-remote-library-cache-store`**

```sql
CREATE TABLE remote_tracks (
    id                TEXT PRIMARY KEY,     -- local UUID
    account_id        TEXT NOT NULL REFERENCES accounts(id),
    remote_id         TEXT NOT NULL,        -- Suno's own track/project id
    title             TEXT NOT NULL,
    artist_or_persona TEXT,
    duration_seconds  REAL,
    cover_art_url     TEXT,
    audio_stream_url  TEXT,                 -- may be time-limited/expiring — see doc 06 notes
    tags_json         TEXT NOT NULL DEFAULT '[]',  -- JSON array of strings
    raw_metadata_json TEXT NOT NULL DEFAULT '{}',  -- full sanitized API payload, for forward-compat
    schema_version    INTEGER NOT NULL DEFAULT 1,
    synced_at         TEXT NOT NULL,
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL,
    deleted_at        TEXT,
    UNIQUE(account_id, remote_id)
);

CREATE INDEX idx_remote_tracks_account ON remote_tracks(account_id);
CREATE INDEX idx_remote_tracks_title   ON remote_tracks(title);
```

`raw_metadata_json` deliberately retains the full sanitized-of-nothing (this
is local cache, not shared — no redaction needed here) API response so that
fields not yet promoted to a real column are never lost between syncs; this
is what makes later schema evolution (Suno adds a field we now care about)
cheap — read it out of already-cached `raw_metadata_json` instead of
re-fetching everything.

**4. `local-download-manager-store`**

```sql
CREATE TABLE downloads (
    id               TEXT PRIMARY KEY,
    account_id       TEXT NOT NULL REFERENCES accounts(id),
    remote_track_id  TEXT NOT NULL REFERENCES remote_tracks(id),
    local_file_path  TEXT,                  -- null until completed
    status           TEXT NOT NULL,         -- 'queued' | 'downloading' | 'completed' | 'failed' | 'paused'
    bytes_downloaded INTEGER NOT NULL DEFAULT 0,
    bytes_total      INTEGER,
    error_message    TEXT,
    attempt_count    INTEGER NOT NULL DEFAULT 0,
    created_at       TEXT NOT NULL,
    updated_at       TEXT NOT NULL,
    completed_at     TEXT
);

CREATE INDEX idx_downloads_status ON downloads(status);
```

**5. `lyrics-and-alignment-store`**

```sql
CREATE TABLE lyric_documents (
    id               TEXT PRIMARY KEY,
    remote_track_id  TEXT NOT NULL REFERENCES remote_tracks(id),
    source           TEXT NOT NULL,   -- 'suno_remote' | 'whisper_local' | 'manual_edit'
    is_current       INTEGER NOT NULL DEFAULT 0,  -- one "current" doc per track (app-enforced)
    plain_text       TEXT NOT NULL,
    created_at       TEXT NOT NULL
);

CREATE TABLE lyric_segments (
    id                TEXT PRIMARY KEY,
    lyric_document_id TEXT NOT NULL REFERENCES lyric_documents(id),
    sequence_index    INTEGER NOT NULL,     -- order within the document
    level             TEXT NOT NULL,        -- 'line' | 'word'
    text              TEXT NOT NULL,
    start_seconds     REAL NOT NULL,
    end_seconds       REAL NOT NULL,
    confidence        REAL,                 -- present for whisper-sourced segments
    manually_edited   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_lyric_segments_doc ON lyric_segments(lyric_document_id);
```

`lyric_documents` is versioned by simply inserting a new row (never
UPDATE-in-place on the text/timing of an existing document) and flipping
`is_current` — this gives the lyrics editor full history/undo-to-source for
free, matching doc 01 §6's "never silently overwrite" requirement.

**6. `canvas-scene-and-keyframe-store`**

```sql
CREATE TABLE scenes (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    scene_json      TEXT NOT NULL,   -- full scene graph: elements, properties, keyframe tracks
    schema_version  INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    deleted_at      TEXT
);
```

Deliberately a single JSON-blob-per-scene design rather than normalizing
elements/keyframes into rows: scene graphs are read/written wholesale by the
canvas editor (load-edit-save the whole tree), rarely queried piecemeal by
SQL, and their shape will churn a lot during Phase 5 development — a rigid
relational schema would fight that. `schema_version` + a small in-Rust
migration function (`fn migrate_scene_json(v: u32, json: Value) -> Value`)
handles evolution. Revisit this decision via ADR only if a real need for
SQL-level querying into scene internals emerges (e.g., "find all scenes
using effect X" at scale).

Illustrative (non-binding) `scene_json` shape:
```json
{
  "elements": [
    {
      "id": "uuid",
      "kind": "text | image | shape | karaoke_text",
      "base_properties": { "x": 0, "y": 0, "rotation": 0, "scale": 1, "opacity": 1 },
      "kind_specific": { "...": "..." },
      "keyframe_tracks": [
        { "property": "opacity", "points": [ { "t": 0.0, "value": 0.0, "easing": "linear" }, { "t": 1.0, "value": 1.0, "easing": "ease_out" } ] }
      ],
      "effects": [ { "type": "fade_in_out", "params": { } } ]
    }
  ],
  "canvas_settings": { "width": 1920, "height": 1080 }
}
```

**7. `automation-pipeline-definition-store`**

```sql
CREATE TABLE pipelines (
    id                TEXT PRIMARY KEY,
    name              TEXT NOT NULL,
    input_selector_json TEXT NOT NULL,  -- e.g. {"kind":"tag","tag":"promo"} | {"kind":"explicit_ids","ids":[...]}
    scene_id          TEXT NOT NULL REFERENCES scenes(id),
    lyric_source_policy TEXT NOT NULL,  -- 'remote_preferred' | 'whisper_preferred' | 'remote_only'
    export_settings_json TEXT NOT NULL, -- resolution, fps, codec, output path template
    schema_version    INTEGER NOT NULL DEFAULT 1,
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL,
    deleted_at        TEXT
);

CREATE TABLE pipeline_runs (
    id            TEXT PRIMARY KEY,
    pipeline_id   TEXT NOT NULL REFERENCES pipelines(id),
    status        TEXT NOT NULL,   -- 'running' | 'completed' | 'failed' | 'cancelled'
    started_at    TEXT NOT NULL,
    finished_at   TEXT
);

CREATE TABLE pipeline_run_items (
    id              TEXT PRIMARY KEY,
    pipeline_run_id TEXT NOT NULL REFERENCES pipeline_runs(id),
    remote_track_id TEXT NOT NULL REFERENCES remote_tracks(id),
    status          TEXT NOT NULL,  -- 'pending' | 'in_progress' | 'completed' | 'failed'
    error_message   TEXT,
    output_file_path TEXT,
    started_at      TEXT,
    finished_at     TEXT
);

CREATE INDEX idx_run_items_run ON pipeline_run_items(pipeline_run_id);
CREATE INDEX idx_run_items_status ON pipeline_run_items(status);
```

`pipeline_run_items` existing as durable rows (not just in-memory queue
state) is what makes crash-resumability (doc 04 Phase 7 exit criteria)
possible: on restart, resume any run with unfinished items rather than
re-running the whole pipeline.

**8. `recorded-audio-take-store`**

```sql
CREATE TABLE recorded_takes (
    id              TEXT PRIMARY KEY,
    local_file_path TEXT NOT NULL,
    input_device_name TEXT,
    duration_seconds  REAL,
    sample_rate       INTEGER,
    channels          INTEGER,
    notes             TEXT,
    created_at        TEXT NOT NULL,
    deleted_at        TEXT
);
```

Deliberately minimal for Phase 2; Phase 9's DAW work will likely add a
`recording_sessions` / `tracks_within_session` layer above this rather than
overloading this table — that decision is deferred to Phase 9's own doc.

**9. Cross-Store Composition Examples (illustrative, not literal SQL)**

These live in `application-services` code, never as cross-store SQL:

- **Karaoke render service** reads `remote_tracks` (for the audio source),
  `lyric_documents`/`lyric_segments` (current doc for that track), and a
  `scenes` row referencing a `karaoke_text` element — combining three
  stores' data at the Rust level.
- **Automation orchestrator** reads a `pipelines` row, resolves its
  `input_selector_json` against `remote_tracks`, creates a `pipeline_runs` +
  `pipeline_run_items` set, then for each item invokes the same single-track
  render service Phase 4/5 already built.
```

---

```
docs/08-ui-ux-design-system.md
```
```markdown
**UI/UX Design System**

**1. Aesthetic Direction**

**"Modern glass."** Translucent, blurred-background panels with soft
borders and subtle depth (layered elevation via blur + slight shadow, not
skeuomorphic gloss), floating over content — well suited to a visualizer-
centric app where the visualizer itself is often the most visually "loud"
element and UI chrome should feel like it's sitting *above* the show, not
competing with it.

**2. Theme Inventory (ship-day-one set)**

- **Catppuccin Latte** (light)
- **Catppuccin Frappé** (dark, muted)
- **Catppuccin Macchiato** (dark, richer)
- **Catppuccin Mocha** (dark, default) — **this is the app's default theme**
- **Monokai** (dark, high-contrast accent colors — for users who want a more
  "classic dev tool" punch of color over Catppuccin's softness)

Each theme is a `DesignTokens` value (see §3) produced from the
`design-tokens-theme-definitions` crate; for Catppuccin, source the actual
published palette values from the official `catppuccin/catppuccin` palette
spec (via `gh`/crates.io — a `catppuccin` Rust crate already exists and
should be used rather than hand-transcribing hex codes) per doc 03 §3's
prior-art mandate. Monokai's palette should similarly be sourced from an
existing well-known reference rather than invented.

**3. Design Token Model**

`design-tokens-theme-definitions` is pure data — no UI framework
dependency — so it can be consumed by whatever the UI layer ends up being
(egui now, potentially something else for a bolted-on surface later).

```rust
pub struct DesignTokens {
    pub name: &'static str,             // "Catppuccin Mocha", "Monokai", ...
    pub is_dark: bool,

    pub color_background_base: Rgba,
    pub color_background_elevated: Rgba,   // glass-panel fill (pre-alpha; alpha applied separately)
    pub color_surface_glass_alpha: f32,    // translucency amount for glass panels
    pub color_border_subtle: Rgba,
    pub color_text_primary: Rgba,
    pub color_text_secondary: Rgba,
    pub color_text_disabled: Rgba,
    pub color_accent_primary: Rgba,        // primary interactive accent
    pub color_accent_secondary: Rgba,
    pub color_success: Rgba,
    pub color_warning: Rgba,
    pub color_danger: Rgba,

    pub radius_small: f32,
    pub radius_medium: f32,
    pub radius_large: f32,

    pub spacing_unit: f32,   // base spacing unit; other spacing = multiples of this

    pub blur_radius_glass_panel: f32,
    pub elevation_shadow_opacity: f32,
}
```

- **Spacing scale:** multiples of `spacing_unit` (default 4.0px-equivalent):
  1x, 2x, 3x, 4x, 6x, 8x, 12x, 16x — components reference the multiple, not a
  raw pixel value, so density/scaling adjustments are a one-token change.
- **Radius scale:** small (inputs/buttons/chips), medium (cards/panels),
  large (modals/major containers).
- **Glass panel recipe (applies to: side nav, floating toolbars, modal
  dialogs, the canvas editor's property inspector):** background =
  `color_background_elevated` at `color_surface_glass_alpha`, backdrop blur
  = `blur_radius_glass_panel`, 1px border at `color_border_subtle`, subtle
  drop shadow at `elevation_shadow_opacity`. Content directly over the live
  visualizer (in preview/canvas-editor screens) uses glass panels; content
  over a flat background (library browser, settings) may use a lighter/flat
  elevated surface instead — glass is a deliberate accent for
  visualizer-adjacent chrome, not applied blindly everywhere.

**4. Typography**

- One primary UI font (proportional, geometric/humanist sans — e.g. Inter
  or similar widely-available open font, bundled) for all UI chrome.
- One monospace font (for any code/JSON display, e.g. in settings/debug
  views) — e.g. JetBrains Mono or similar.
- Type scale (relative, tokens not raw pixels): `caption`, `body`,
  `body_emphasis`, `heading_small`, `heading_medium`, `heading_large`,
  `display` (used sparingly — e.g. a big now-playing title).
- Karaoke/lyric text on the canvas is a **separate concern** from UI
  typography — it's user-customizable per-scene (font, size, color, style)
  via the canvas editor, not tied to the app's own theme tokens.

**5. Layout Conventions**

- **App shell:** persistent left-side nav (collapsible) for top-level
  sections (Library, Player/Visualizer, Canvas Editor, Automation, Studio,
  Settings) + a top bar hosting the account switcher (always visible, per
  doc 05 §5) and global search.
- **Screen composition:** each `ui-screen-*` crate owns one top-level
  screen; screens compose widgets from `ui-shared-widget-library`, never
  duplicate a widget's implementation locally — if a screen needs a widget
  that doesn't exist yet, add it to the shared library crate, don't inline
  a one-off copy.
- **Responsive behavior:** desktop-first, but the app shell should tolerate
  reasonable window resizing gracefully (collapsing nav to icons-only below
  a width threshold, etc) — full mobile/tablet responsiveness is explicitly
  out of scope (this is a desktop app).
- **Modal usage:** sparing — prefer inline panels/drawers over modal dialogs
  where the task isn't truly blocking (e.g., "edit track tags" = inline
  popover, not a modal; "confirm delete" = modal, since it's a genuine
  interrupt).

**6. Motion / Animation Guidelines**

- UI chrome transitions (panel open/close, hover states, theme switch)
  should be quick and subtle — target ~120-200ms, ease-out — never
  block interaction waiting for an animation to finish.
- The canvas editor's own keyframe animation system (doc 04 Phase 5) is a
  **user-authored content feature**, entirely separate from this section's
  "UI chrome polish" concern — don't conflate the two when implementing.
- Respect a system-level "reduce motion" preference where the OS/platform
  exposes one; fall back to instant transitions for chrome (not for
  user-authored canvas content, which always plays as designed).

**7. Iconography**

- Use a single consistent open-source icon set (evaluate via prior-art
  search per doc 03 §3 — e.g. `lucide` or `phosphor` icon sets have Rust-
  friendly SVG/font distributions) rather than mixing icon sources.
- Icons follow the same accent/neutral color tokens as text — no
  hardcoded icon colors outside the token system.

**8. Accessibility Baseline (v1 minimum bar)**

- All interactive elements reachable via keyboard (tab order sane, no
  keyboard traps).
- Color tokens chosen/verified for reasonable contrast on both light
  (Latte) and dark (Mocha/Frappé/Macchiato/Monokai) themes — spot-check
  text-on-glass-panel contrast specifically, since translucency can degrade
  contrast versus the palette's "on solid background" intent.
- No motion-only signaling for critical state (e.g., download failure must
  also have a text/icon indicator, not just a color/animation change).
- Full screen-reader support is a stretch goal, not a v1 blocker, given
  egui's current maturity in that area — note this honestly rather than
  overclaiming.

**9. Component Naming Convention (for `ui-shared-widget-library`)**

Match the verbose-naming philosophy from doc 03: `glass_panel_container.rs`,
`primary_accent_button.rs`, `track_list_row_card.rs`,
`account_switcher_dropdown.rs`, `keyframe_timeline_ruler.rs`, etc — not
`panel.rs`, `button.rs`, `row.rs`.
```

---

```
docs/16-testing-strategy.md
```
```markdown
**Testing Strategy**

**1. Philosophy**

Given an LLM writes all code, tests are the primary safety net against
silent regressions and hallucinated behavior — they are not optional
polish. Every domain/service/bridge crate ships tests alongside its
implementation in the same task, per the Definition of Done (doc 03 §5),
not as a follow-up ticket.

**2. Test Levels by Layer**

- **Foundation crates:** straightforward unit tests (config parsing edge
  cases, token redaction actually redacting, theme token completeness).
- **External-bridge crates:** unit tests against fakes/mocks of the external
  system wherever possible (e.g., `suno-http-client-core` tested against
  `suno-api-fixture-mocks`, not live suno.com — **live-network calls are
  banned in the automated test suite entirely**, per doc 03 §7 and doc 06's
  capture-driven approach; live calls only happen via manual, human-run
  exploratory sessions when capturing new endpoints). Process-wrapper bridges
  (`video-export-ffmpeg-process`, `whisper-transcription-bridge`) get a thin
  integration test gated behind a feature flag/env var
  (`SONORA_TEST_REQUIRES_LOCAL_TOOLING=1`) so CI-less/tool-less environments
  can skip them without failing the default `cargo test --workspace` run.
- **Domain-store crates:** integration tests against a real (temp-file or
  in-memory) SQLite instance via `sqlx`'s test utilities — migrations run,
  CRUD round-trips verified, including the "versioned/never-overwrite"
  behaviors (e.g. lyric document history) and soft-delete behavior.
- **Application-service crates:** integration tests composing fake/stub
  bridges + real (temp) stores to verify orchestration logic (e.g., "does
  the karaoke render service correctly prefer remote lyrics over whisper
  when both exist") without needing the full real external stack.
- **UI crates:** primarily manual QA during early phases (egui's testing
  story is weaker than backend Rust). From Phase 5 onward, add **snapshot
  tests for serializable state** the UI produces (scene graph JSON,
  keyframe track JSON) even if the visual rendering itself isn't
  snapshot-tested — this catches "the editor silently corrupts saved data"
  bugs, which are the highest-value UI-adjacent bugs to catch automatically.

**3. Fixture & Mock Data Conventions**

- `suno-api-fixture-mocks` holds sanitized, real-capture-derived JSON
  fixtures (see doc 06 §0) organized to mirror doc 06's endpoint categories
  (`fixtures/library/list_tracks_page_1.json`, etc). A small
  `MockSunoHttpClient` (implementing the same trait `suno-http-client-core`
  exposes) serves these fixtures for tests, letting service/UI-logic tests
  run without any real HTTP dependency.
- `deterministic-test-clock-and-ids` provides a fixed/injectable clock and
  a seeded/sequential UUID generator so store-layer tests produce
  reproducible `created_at`/`id` values instead of asserting against
  wall-clock time or random UUIDs (which would make assertions flaky/
  awkward).
- Fixtures are sanitized per doc 06 §0's redaction rule even though they
  live only in the repo (not shared externally) — treat "never commit a
  real secret" as a hard rule regardless of the repo's visibility, since
  visibility/hosting policy may change later.

**4. What CI Enforces (mechanics finalized later per project instructions,**
   principles fixed now)

Actual CI/git workflow tooling is explicitly deferred (per the user's
instruction to worry about worktrees/branches/releases later), but the
**bar CI will eventually enforce** is fixed now so development proceeds
toward it from day one:

- `cargo build --workspace` — zero errors, zero new warnings.
- `cargo clippy --workspace --all-targets -- -D warnings` — clean.
- `cargo test --workspace` — all pass, live-network-requiring and local-
  tooling-requiring tests properly gated/skipped as described in §2.
- `cargo fmt --check` — consistent formatting (standard rustfmt defaults
  unless a documented project-wide `rustfmt.toml` override is added early
  and recorded via ADR).
- A file-size-cap check (custom `xtask` script, per doc 02) flags any `.rs`
  file over the 300-line hard cap (doc 03 §1) — this can start as advisory/
  warning-only in `xtask` and later become a hard CI gate once the codebase
  is large enough for it to matter.

**5. Manual QA Checklists**

For each phase (doc 04), the phase's own feature doc (written when that
phase begins, per the project's step-by-step prompting plan) should include
a short manual QA checklist specific to that phase's exit criteria — e.g.
Phase 1's checklist walks through adding an account via all three auth
methods, switching accounts, and confirming library re-scoping. These
checklists are how the human orchestrator spot-verifies agent-claimed
"exit criteria met" status without needing to read every diff line-by-line.

**6. Performance/Load Testing (light touch until it matters)**

- No formal perf benchmarking suite before Phase 7 (Automation). Phase 7's
  own exit criteria (a real 20-50 track batch run, doc 04) IS the first
  real load test, and should be treated as such — capture timing/resource
  usage observations from that run into that phase's doc as a baseline for
  later "thousands of tracks" scale work.
- Audio playback (Phase 2) and visualizer rendering (Phase 4) get informal
  "does it glitch/drop frames under normal manual use" checks rather than
  automated benchmarks initially — formalize only if a real problem
  surfaces.

**7. Security-Adjacent Testing**

- Redaction logic (doc 05 §3, `secrecy`-wrapped credential types) gets an
  explicit unit test asserting that `Debug`/`Display` formatting of a
  credential-bearing struct never contains the raw secret substring — this
  is cheap to test and guards against a genuinely damaging class of bug
  (accidental token leakage into logs).
- Keyring storage round-trip (`os-keyring-secret-storage`) gets an
  integration test on each target platform's actual keyring where CI
  environment permits, with a documented manual-test fallback note for
  platforms/CI runners where a real OS keyring isn't available headlessly.
```

---

**continue** — next batch: doc 09 (visualizer/projectM integration spec detail), doc 10 (canvas & keyframe system detail spec), doc 11 (plugin system deferred design — full detail for when Phase 8 arrives), and doc 12 (LLM/image-gen adapter spec) unless you'd like reprioritization.

### 🧑‍💻 User

Continue. Llm chat and image generation will be next phase after the core is fleshed out to production grade and codebase is maintainable—whoch likely even sooner to avoid 25k LOC of spaghetti again like the c++ and qt version

### 🤖 Assistant — claude-sonnet-5-high

Good call — that's exactly the kind of thing worth locking in explicitly rather than letting drift happen silently. Two things before continuing the remaining topic docs:

1. **Roadmap amendment** (recorded properly, not just conversationally)
2. **A new guardrails doc** dedicated to preventing the 25k-LOC-spaghetti outcome — since "small files in nested dirs" alone doesn't guarantee a maintainable *system*, only maintainable *files*. This is the more important addition than the reordering itself.

---

```
docs/17-glossary-and-decisions-log.md  — APPEND
```
```markdown
**ADR-008: LLM Chat & Image-Gen priority moved earlier; explicit codebase-health gate added**
- **Status:** Accepted
- **Context:** Prior C++/Qt prototype grew to ~25k LOC of spaghetti before
  these features were reached. Two causes identified: (1) feature work
  outpaced architectural discipline, (2) creative/LLM-integration features
  were bolted on late without a clean seam, encouraging shortcuts.
- **Decision:**
  1. Doc 04's Phase 6 (LLM Creative Integrations) is **re-sequenced to
     immediately follow whichever of Phases 1-5 constitutes "core" being
     production-grade and maintainable** — not rigidly fixed at position 6
     by number. Practically: Phases 1-5 (accounts/library, playback,
     lyrics, visualizer, canvas) are the "core" that must clear the new
     **Core Maintainability Gate** (doc 18) before Phase 6 (LLM/image-gen)
     OR Phase 7 (automation) proceeds — whichever the human orchestrator
     chooses to tackle next at that point. Phase numbers in doc 04 are
     retained as-written for reference, but doc 04 §0 is amended (below)
     to state this gate explicitly rather than treating phase numbers as a
     strict queue.
  2. A new doc, `docs/18-codebase-health-guardrails.md`, defines the actual
     mechanical/process checks that constitute "production grade and
     maintainable" — not just vibes — and mandates a **Core Maintainability
     Gate audit** before Phase 6/7 begins.
- **Consequence:** The agent must treat the Phase 5→6/7 boundary as a hard
  stop requiring an explicit audit pass, not a soft suggestion.
```

---

```
docs/04-phase-roadmap.md  — INSERT NEW §0 AT TOP OF FILE
```
```markdown
**§0. Phasing Is a Guide, Not a Blind Queue — Core Maintainability Gate**

Phases 0-5 constitute **"Core."** Phases 6 (LLM Creative Integrations) and 7
(Automation Pipelines) are **not allowed to begin** until Core passes the
**Core Maintainability Gate** defined in `docs/18-codebase-health-guardrails.md`.
This is a deliberate, named checkpoint — not implicit — specifically because
the predecessor C++/Qt project reached ~25k LOC of unmaintainable spaghetti
by the time it got this far, and LLM-integration features were part of what
got bolted on under that decay (see ADR-008). We are not repeating that.

Order of Core phases (0→5) stays sequential as written below. What's
flexible is what happens **after** Core: once the Gate passes, the human
orchestrator chooses whether Phase 6 (LLM/image-gen) or Phase 7
(Automation) goes next based on product priority at that time — both are
equally "unlocked," neither is forced to wait on the other.

---
```

*(the rest of doc 04 follows unchanged below this insert — original phase content stands as delivered)*

---

```
docs/18-codebase-health-guardrails.md
```
```markdown
**Codebase Health Guardrails & Core Maintainability Gate**

**1. Why This Doc Exists**

Small files + deep nesting + verbose names (doc 02, doc 03) prevent
*individual-file* spaghetti. They do **not** by themselves prevent
*system-level* spaghetti: circular crate dependencies, a "core" crate that
quietly becomes a dumping ground, duplicated logic scattered across
lookalike crates, dead/half-finished code paths accumulating, or an
ever-growing pile of one-off exceptions to doc 01's layering rules. The
predecessor C++/Qt project's ~25k LOC problem was this second kind of decay,
not a "files were too long" problem. This doc defines concrete, checkable
guardrails against that specific failure mode, and the **Core Maintainability
Gate** that must be passed before Phase 6/7 work begins (per ADR-008).

**2. Continuous Guardrails (checked throughout Phases 0-5, not just at the end)**

**2.1 Dependency Direction Enforcement**
- Doc 01 §3's layering (`ui → application-services → domain-stores →
  external-bridges → foundation`) must hold with **zero exceptions**. Any
  task whose implementation seems to require an upward dependency (e.g. a
  domain-store needing something from application-services) is a signal the
  logic is misplaced — stop and relocate it, don't add the illegal edge.
- `xtask` gains a `check-layering` command (build this in Phase 0, even as a
  simple Cargo.toml-dependency-graph parser) that fails if any crate depends
  on a crate from a "higher" layer than itself. Run it as part of the
  Definition of Done (doc 03 §5) for every task from Phase 0 onward — not
  deferred to "later CI," since catching a layering violation at commit
  time is cheap and catching it after 50 more commits build on top of it is
  expensive.

**2.2 No Duplicate-Logic Crates**
- Before creating a new crate, the agent must confirm (via a quick search of
  `docs/02-workspace-layout.md` + `rg`/grep across `crates/`) that an
  existing crate doesn't already own this responsibility. If two crates end
  up doing near-identical things (e.g. two different "resolve a track's
  audio source" implementations), that's a defect to consolidate
  immediately, not a "clean up later" item.
- Any new crate not already named in doc 02's tree requires a one-line
  addition to doc 02 in the same task that creates it — doc 02 must always
  accurately reflect the real crate tree. A crate existing in the repo but
  absent from doc 02 (or vice versa) is itself a Gate failure (§4).

**2.3 Dead Code & Speculative Generality Ban**
- No `#[allow(dead_code)]` left in committed code as a permanent fixture —
  either the code is used, or it's deleted. Temporary scaffolding during a
  multi-step task is fine; it must be resolved by that task's completion.
- No "just in case" abstraction: don't introduce a trait with only one
  implementation "for future flexibility" unless a second implementation is
  concretely planned in the *current or next* phase doc. Speculative
  generality is exactly the kind of thing that reads as "senior engineering"
  but actually adds indirection an LLM-driven codebase can't afford —
  prefer the concrete type until a second real need proves the
  abstraction's shape.
- Every crate created via the Phase 0 stub sweep (doc 02) that reaches its
  designated phase and gets implemented should have its stub `README.md`
  status line updated from "not yet implemented" to reflect reality in the
  same task — stale status docs are a form of dead documentation-code.

**2.4 Complexity Budget, Not Just Line-Count Budget**
- Doc 03's 300-line file cap catches *size*, not *complexity*. Additionally
  watch for and split on:
  - A function/method with more than ~4 levels of nested control flow
    (if/match/loop nesting) — flatten via early returns or extraction.
  - A `match`/`if-else` chain handling more than ~6-7 arms of genuinely
    distinct behavior — consider a lookup table, trait dispatch, or
    splitting the concern.
  - A struct with more than ~10-12 fields — likely two structs pretending
    to be one (classic "God struct" smell even inside a small file).
  - A crate whose `lib.rs` public API surface (public fns/types) exceeds
    what a one-paragraph purpose statement can honestly describe — if
    describing the crate needs "and also," it's doing two jobs.

**2.5 Doc-Reality Sync Requirement**
- Doc 03 §5's Definition of Done already requires updating relevant docs
  when reality diverges. This guardrail makes it explicit that **the docs
  in this doc-set are treated as tested artifacts, not write-once
  planning** — an inaccurate doc 02 tree, an out-of-date doc 04 exit-
  criteria claim, or an ADR log missing a real architectural decision that
  was made ad-hoc during implementation are all bugs, tracked the same as a
  code bug.

**3. Periodic Self-Audit Ritual ("Senior Architect Pass")**

At the end of **every phase** (0 through 5 at minimum; recommended
thereafter too), before marking that phase's exit criteria as met, the agent
performs a dedicated audit pass distinct from the per-task self-review in
doc 03 §4:

1. Re-read doc 01 (architecture) and doc 02 (workspace layout) fully against
   the actual current crate tree and dependency graph.
2. Run `check-layering` (§2.1) and a simple LOC/complexity scan across all
   crates touched in the phase.
3. Grep for `TODO`/`FIXME`/`unwrap()`-in-non-test-code left behind, and
   either resolve or explicitly log each remaining one in
   `docs/99-ideas-backlog.md` / a phase-specific "known debt" note — no
   silent accumulation.
4. Produce a short **Phase Audit Summary** (a few paragraphs, added to that
   phase's own doc when it exists, or to this doc's §5 log otherwise)
   covering: crate count added, any layering exceptions found and how
   resolved, any duplicate-logic consolidations performed, any docs
   updated to match reality, and an honest gut-check statement of "would a
   new contributor (or a fresh agent context) be able to navigate this
   phase's code using only doc 02 + file/dir names?"

This ritual is intentionally positioned as "architect reviewing the
sprint," not "developer finishing a ticket" — it's the mechanism that
catches system-level drift the per-task self-review (doc 03 §4) isn't
scoped to catch.

**4. The Core Maintainability Gate (before Phase 6/7)**

Beyond the per-phase audits (§3), a single consolidated Gate review happens
once Phase 5 is complete, before Phase 6 or 7 begins. The Gate **passes**
only when all of the following are true:

- [ ] `check-layering` passes clean across the entire workspace.
- [ ] No crate exceeds the complexity/size budgets in §2.4 without a
      documented, deliberate exception (and exceptions should be rare/zero).
- [ ] Doc 02's crate tree exactly matches the real `crates/` directory
      (no drift, no orphans, no undocumented additions).
- [ ] Every phase 0-5 doc's exit criteria are re-verified true against
      current `main`, not just true-at-the-time-they-were-written (a
      regression introduced by a later phase would otherwise go unnoticed).
- [ ] `cargo tree --workspace --duplicates` reviewed — no surprising
      duplicate-version dependency bloat.
- [ ] The 4 Phase Audit Summaries (§3) exist and their "known debt" items
      have been triaged: fixed, or explicitly deferred with a reason
      recorded in `docs/99-ideas-backlog.md`.
- [ ] A **total workspace LOC and crate-count snapshot** is recorded in this
      doc's §5 log, specifically to track growth rate over time — if Core
      alone is already approaching the predecessor project's ~25k LOC
      danger zone, that's a signal to slow down and consolidate *before*
      adding Phase 6/7 features on top, not a coincidence to ignore.
- [ ] Human orchestrator has done a spot-check pass (guided by the Phase
      Audit Summaries) and explicitly signs off — this Gate is not
      self-certified by the agent alone, given it's the specific checkpoint
      created *because* of a prior project's uncaught decay.

If the Gate fails on any item, the response is a **consolidation task list**
(fix the failing items) before any Phase 6/7 feature work starts — this is
allowed to take real effort; it is the whole point of the Gate.

**5. Audit & Gate Log**

*(Populated over time as phases complete — empty at doc creation.)*

| Phase | Date | Crate Count | Approx Total LOC | Layering Violations Found | Notes |
|---|---|---|---|---|---|
| — | — | — | — | — | *(no phases completed yet)* |
```

---

Now continuing the originally-planned batch: visualizer, canvas/keyframe, and plugin deep-dive specs. Image/LLM adapter doc (12) will note its new earlier-priority status per ADR-008.

---

```
docs/09-visualizer-projectm-integration.md
```
```markdown
**Visualizer (projectM) Integration Spec**

**1. Scope**

Covers `visualizer-projectm-ffi-bindings`, `visualizer-projectm-frame-bridge`,
and their consumption by `single-track-visualizer-render-service` (live
preview) and the export path (headless render, feeding
`video-export-ffmpeg-process`). Canvas overlay compositing (text/graphics on
top of the visualizer frame) is doc 10's concern, not this one — this doc
stops at "produce a visualizer frame," doc 10 picks up "composite things
onto that frame."

**2. Why projectM, Why FFI**

projectM is a mature, widely-used, MilkDrop-preset-compatible C/C++ library —
per doc 03 §3's prior-art mandate, using it (rather than reimplementing
audio-reactive shader visualization from scratch in Rust) is the correct
call; this is exactly the kind of "don't reinvent the wheel" case the
constitution names explicitly.

**3. Crate Split Rationale**

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

**4. Build/Packaging Concerns (address early — Phase 0/4 spike)**

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

**5. Live Preview vs. Headless Export — Two Render Paths, One Engine Trait**

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
  `video-export-ffmpeg-process`'s design settles on — see doc 09b/§7).
  This path can run **faster or slower than real-time** depending on
  machine performance — that's the whole point (batch rendering thousands
  of tracks shouldn't be real-time-bound).
- Both paths use the *same* `VisualizerEngine` trait and the *same*
  `canvas-overlay-compositing-service` (doc 10) for consistency — the only
  difference is what drives the frame-advance loop and where the frame
  ends up (screen vs. ffmpeg pipe). This shared-path requirement is what
  guarantees "what you designed in preview is exactly what you get in
  export," which is a named product requirement, not an implementation
  nicety.

**6. Preset Management**

- `PresetHandle` refers to a `.milk` file path + display name; a settings
  screen lets users point at one or more local preset folders.
- Preset switching mid-playback (live preview) should be smooth — no crash/
  black-frame on switch; projectM natively supports blending between
  presets, expose that as a config option (blend duration) rather than
  hardcoding it.
- For automation pipelines (Phase 7), a scene/pipeline can pin a specific
  preset (or a preset-rotation policy) — that's stored in the scene JSON
  (doc 07 §6), not in this crate.

**7. Audio Feed Details**

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

**8. Error Handling**

`VisualizerError` should distinguish (at minimum): preset-load failure
(bad/corrupt `.milk` file — recoverable, skip/report that preset),
engine-init failure (missing/incompatible projectM install — fatal,
surfaced clearly at app-startup or feature-first-use, with actionable
guidance per §4's packaging decision), and render-frame failure (should be
rare/fatal-per-render, logged with full context for debugging).
```

---

```
docs/10-canvas-and-keyframe-system.md
```
```markdown
**Canvas & Keyframe System Spec**

**1. Scope**

Covers `canvas-scene-and-keyframe-store` (doc 07 §6 schema), the
`ui-screen-canvas-scene-editor` UI, and `canvas-overlay-compositing-service`
(the runtime logic that turns a saved scene + a visualizer frame + current
playback time into a final composited frame, consumed by both live preview
and export per doc 09 §5).

**2. Core Concepts**

- **Scene** — a saved document: a list of **elements**, each with base
  properties, kind-specific data, keyframe tracks, and effects (doc 07 §6
  JSON shape). A scene is reusable across multiple tracks (that's what
  makes automation pipelines, doc 04 Phase 7, meaningful — one scene
  "template" applied to many tracks).
- **Element** — one placeable thing on the canvas. Kinds at v1: `text`,
  `image`, `shape` (basic rect/ellipse, for simple graphic accents),
  `karaoke_text` (special: bound to the current track's resolved lyric
  timing, doc 03/doc 07 §5, rather than static content).
- **Keyframe track** — a per-property timeline: a named property (e.g.
  `opacity`, `x`, `y`, `scale`, `rotation`, `color`) plus an ordered list of
  `(time, value, easing)` points. Rendering at time `t` = interpolate
  between the two surrounding points using the specified easing (or hold
  the last value's `easing` behavior semantics — clarify start/end
  boundary behavior during implementation and document the choice).
- **Effect** — a named, parameterized visual treatment applied to an
  element (e.g. `fade_in_out`, `glow`, a simple particle burst) — distinct
  from keyframes in that effects are pre-built, parameterized behaviors a
  user picks and tunes (start/end time, intensity, color), not manually
  keyframed point-by-point. Internally an effect MAY be implemented as
  "synthesizes keyframe tracks under the hood" — that's an acceptable
  implementation detail, not a user-facing distinction they need to
  understand.

**3. Interpolation & Easing (v1 minimum set)**

- `linear`
- `ease_in`, `ease_out`, `ease_in_out` (standard cubic easing — use an
  existing crate, e.g. a small easing-functions crate, per doc 03 §3, don't
  hand-derive cubic bezier math from scratch).
- `step` (hold previous value until the next keyframe, then jump — useful
  for e.g. karaoke word-highlight-style discrete changes).

Additional easing curves are additive later (backlog item), not a v1
blocker.

**4. Karaoke Text Element — Detail**

This is the flagship element type, so it gets explicit spec treatment
rather than being lumped into generic "text":

- Bound to a `remote_track_id`'s current `lyric_document` (doc 07 §5) at
  render time — NOT to a copy of the lyric data baked into the scene
  (a scene is reusable across tracks; the lyrics must resolve per-track at
  render time, whether that's live preview of Track A or a pipeline batch
  running Track A, B, C sequentially through the *same* scene).
- Style properties (font, size, color for "upcoming," "active," "sung"
  word states, highlight animation style — e.g. simple color-swap vs.
  scale-pulse on the active word) are scene-level configuration on the
  `karaoke_text` element, NOT per-track — this is exactly the
  "brand customization" product goal: design the karaoke look once, it
  applies consistently across every track run through that scene/pipeline.
- Granularity: support both line-level and word-level highlighting modes
  (doc 07 §5's `lyric_segments.level` distinguishes these) — user picks
  which mode a given `karaoke_text` element uses; word-level requires
  word-level timing data to exist (falls back to line-level gracefully with
  a UI indicator if only line-level timing is available for that track).

**5. Canvas Editor UX (high-level; detailed wireframing happens in-phase)**

- Freeform drag/resize/rotate manipulation of elements directly on a
  live-visualizer-backed canvas (so the user sees real context while
  designing, not a blank/mock background).
- A property inspector panel (glass panel per doc 08 §3) for the selected
  element's properties and effect list.
- A keyframe timeline panel (bottom-docked, standard NLE-style — playhead,
  per-property tracks, draggable keyframe points) — this is a substantial
  UI subsystem in its own right; expect `ui-screen-canvas-scene-editor` to
  be one of the largest crates in the workspace by file count (not by
  individual file size — the 300-line cap still applies per-file; this
  crate will simply have MANY files: `keyframe_timeline_ruler.rs`,
  `keyframe_point_drag_handle.rs`, `property_track_row.rs`,
  `element_selection_outline_overlay.rs`, etc).
- Snapping/alignment guides for placement (nice-to-have, not a v1 exit
  criterion — note in Phase 5's doc if deferred).

**6. Compositing Runtime (`canvas-overlay-compositing-service`)**

Given: a visualizer frame texture (doc 09), a scene, a current playback
time `t`, and (for karaoke elements) resolved lyric data —

1. For each element, evaluate all its keyframe tracks + active effects at
   time `t` to produce a concrete property set (position, opacity, etc).
2. Render each element (text/image/shape/karaoke-text) into the same
   target the visualizer frame occupies, in element list order (simple
   z-order = list order; explicit z-index property is a possible backlog
   refinement if list-order proves insufficient in practice).
3. Return the fully composited frame.

This must be **framerate-and-realtime-agnostic** just like doc 09 §5's
visualizer engine — called once per live-preview display refresh in one
path, once per output-video-frame in the headless export path, driven by
`t` values the caller provides rather than this service tracking time
itself. This service should have **zero knowledge of whether it's being
called from preview or export** — that separation of concerns is what
guarantees preview/export visual parity (doc 09 §5's named requirement).

**7. Performance Consideration**

Text rendering (especially per-word karaoke styling, potentially with
per-frame color/scale changes) at video-export framerates across
potentially thousands of automated renders (Phase 7) needs to be
reasonably efficient — prefer an existing mature Rust text-rendering
approach (e.g. via the chosen UI framework's own text/vector rendering
facilities, or a dedicated crate like `cosmic-text`/`ab_glyph` if the UI
framework's own text stack isn't easily usable in the headless/offscreen
export context) over a naive re-layout-every-frame-from-scratch approach if
profiling in Phase 5/7 shows it's a bottleneck. Do not over-optimize
prematurely — get it correct first, per doc 18 §2.3's anti-speculative-
generality guidance; this note exists so it's not forgotten if/when it does
become a real bottleneck at automation scale.
```

---

```
docs/11-plugin-system-deferred-design.md
```
```markdown
**Plugin System — Deferred Design (Phase 8)**

**1. Current Status (Phases 0-7)**

`plugin-host-stub` exists purely as trait scaffolding + a no-op registry —
per doc 03 §8 and ADR-004, this is intentional and should NOT be fleshed
out ahead of Phase 8. This doc records the *intended* design now so the
seam left in earlier phases (see §4) is shaped correctly, without actually
building the implementation early.

**2. Tiered Runtime Decision (recap of earlier decision)**

- **Tier 1 (Phase 8 initial): Rhai scripting.** Pure-Rust embedded scripting
  engine, no external toolchain/compiler needed by end users, naturally
  sandboxed (no filesystem/network access unless explicitly exposed via
  registered host functions). Chosen over Lua (`mlua`) for being pure-Rust
  (simpler cross-platform distribution, no C dependency) — confirm this
  reasoning still holds via a quick prior-art check at Phase 8 start in
  case the ecosystem has shifted.
- **Tier 2 (later, only if real need emerges): WASM component host** via
  `wasmtime`, for users wanting to author plugins in other languages or
  needing capabilities beyond what a scripting sandbox comfortably allows
  (e.g. genuinely heavy compute). Not built until a concrete Tier-1
  limitation is hit in practice — per doc 18 §2.3's anti-speculative-
  generality rule, do not build Tier 2 "just in case."

**3. Planned Extension Points (Phase 8 targets)**

- **Canvas custom effect hook:** a Rhai script implementing a
  per-frame property-modifier function — given an element's current
  computed properties + time `t`, returns modified properties. Registered
  as a new "effect type" alongside the built-ins (doc 10 §2).
  ```
  // illustrative Rhai function signature
  fn apply(properties, t) {
      // read/modify a fixed, documented set of numeric/string properties
      // return the modified properties map
  }
  ```
- **Automation pipeline custom step hook:** a Rhai script that can run as
  an additional step in a pipeline (doc 07 §7) — e.g. custom output-file
  naming logic, or a conditional skip rule — given a well-defined read-only
  context object (track metadata, run info) and returning a simple
  decision/value, not arbitrary host control.

**4. What Earlier Phases Must NOT Do (guardrail against premature coupling)**

- Earlier phases (5, 7) may reference `plugin-host-stub`'s trait types as
  extension seams (e.g. an `Effect` enum in the scene JSON schema having a
  `Custom(PluginRef)` variant that's simply unused/unreachable until Phase
  8) — this is fine and encouraged as forward-compatible schema design.
- Earlier phases must NOT implement any actual script execution, sandbox
  logic, or plugin-file-loading — that's 100% Phase 8 scope. If a Phase 5/7
  task feels tempted to "just quickly support a simple script hook already
  since the seam is right there," that's scope creep per doc 03 §8 — note
  it in `docs/99-ideas-backlog.md` and move on.

**5. Sandbox & Trust Model (Phase 8 requirement, recorded now for clarity)**

- A plugin script has access ONLY to explicitly-registered host functions/
  data — no ambient filesystem, network, or process access via Rhai's
  default engine configuration (Rhai is sandboxed-by-default unless you
  register additional capabilities — confirm this is still accurate for
  whatever Rhai version is current at Phase 8 time).
- Plugins are loaded from local files the user explicitly points the app
  at (a plugins folder in the app's config directory) — no auto-discovery
  from arbitrary locations, no remote plugin fetching/marketplace (doc 04
  Phase 8 non-goals).
- A misbehaving plugin (infinite loop, error) must not hang or crash the
  host app — Rhai supports execution step limits/timeouts; Phase 8's
  implementation must configure and test this explicitly as a hard
  requirement, not an afterthought.

**6. Example Plugins to Ship (Phase 8 exit criteria per doc 04)**

- A simple custom canvas effect (e.g. a "strobe/flash on beat" property
  modifier) — demonstrates the effect hook.
- A simple custom pipeline step (e.g. "skip tracks shorter than N
  seconds") — demonstrates the pipeline hook.
Both ship as documented, readable example scripts (not obfuscated/minified)
specifically so end users can learn the API by reading them.
```

---

**continue** — next batch: doc 12 (LLM text + image-gen provider adapter spec, reflecting its ADR-008-adjusted earlier priority), doc 13 (automation pipeline engine detail), doc 14 (local playback/DAW-adjacent audio engine spec), and doc 15 (multi-account + downloads deep detail) — then I'll do a final consolidation/index doc (doc 99 ideas backlog + a top-level README/index) to close out the set.

### 🧑‍💻 User

Continue. ❤️

### 🤖 Assistant — claude-sonnet-5-high

---

```
docs/12-llm-and-image-gen-provider-adapters.md
```
```markdown
**LLM Text & Image-Gen Provider Adapters**

**0. Priority Status (per ADR-008)**

This capability was originally slotted as "Phase 6, after everything else."
Per ADR-008, it is now positioned as **one of the two options unlocked
immediately after the Core Maintainability Gate (doc 18 §4) passes** —
i.e., it can be tackled right after Phases 0-5 are genuinely
production-grade, potentially *before* Phase 7 (Automation), at the human
orchestrator's discretion. This doc's design is written to be buildable at
that point without depending on Phase 7 existing yet. Where a design note
below assumes automation exists, it's flagged explicitly as a Phase-7-or-
later enhancement, not a hard dependency.

**1. Scope & Philosophy**

Sonora does not implement LLM inference or image diffusion itself — it
provides **adapters** over remote APIs and/or local OpenAI-compatible or
ComfyUI/A1111-style servers the user already runs. This keeps the crate
thin, keeps Sonora's own maintenance burden low (per doc 18's core
philosophy of avoiding scope-driven decay), and lets users bring whichever
provider/model they already pay for or self-host.

Two independent adapter surfaces:
- **Text generation** — lyric ideation/assist, applied inside the lyrics
  editor (doc 04 Phase 3 / doc 07 §5) as an "assist" panel, not a
  standalone chat window (see §5 on scope boundary).
- **Image generation** — cover/brand art assist, applied inside the canvas
  scene editor (doc 10) as a "generate art asset" action producing a usable
  `image` element source.

**2. Text Provider Adapter (`llm-text-provider-adapter`)**

```rust
#[async_trait::async_trait]
pub trait TextGenerationProvider: Send + Sync {
    async fn generate(
        &self,
        request: TextGenerationRequest,
    ) -> Result<TextGenerationResponse, TextProviderError>;
}

pub struct TextGenerationRequest {
    pub system_prompt: Option<String>,
    pub user_prompt: String,
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

pub struct TextGenerationResponse {
    pub text: String,
    pub provider_name: String,
    pub raw_metadata_json: serde_json::Value, // token usage etc, kept for forward-compat
}
```

- **v1 concrete implementation:** a single `OpenAiCompatibleTextProvider`
  hitting any endpoint implementing the widely-adopted `/v1/chat/
  completions`-style contract — this alone covers OpenAI itself,
  OpenRouter (multi-model proxy), Ollama, LM Studio, and most self-hosted
  local inference servers, maximizing coverage for minimal adapter code
  (prior-art-first mandate, doc 03 §3, in action).
- **Configuration (per doc 08 settings screen):** base URL, API key
  (stored via `os-keyring-secret-storage`, same pattern as Suno auth per
  doc 05 §3), model name/identifier, and optional default
  system-prompt-per-use-case (a "lyric assist" system prompt distinct from
  any future non-lyric use case's prompt).
- **Additional providers (Anthropic/Gemini native APIs)** are additive
  future implementations of the same trait — not required for v1, listed
  here so the trait shape anticipates them (e.g. `raw_metadata_json`
  absorbing provider-specific response quirks rather than the trait itself
  needing provider-specific fields).

**3. Image Provider Adapter (`image-gen-provider-adapter`)**

```rust
#[async_trait::async_trait]
pub trait ImageGenerationProvider: Send + Sync {
    async fn generate(
        &self,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, ImageProviderError>;
}

pub struct ImageGenerationRequest {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub seed: Option<u64>,
}

pub struct ImageGenerationResponse {
    pub image_bytes: Vec<u8>,      // PNG, normalized regardless of provider's native format
    pub provider_name: String,
    pub raw_metadata_json: serde_json::Value,
}
```

- **v1 concrete implementations (pick at least one during Phase
  implementation, both are reasonable day-one targets):**
  - `RemoteApiImageProvider` — a straightforward REST-call adapter for a
    well-documented hosted API (e.g. OpenAI Images or Stability's API —
    confirm current API shape via their official docs at implementation
    time rather than assuming, since these APIs change).
  - `ComfyUiHttpImageProvider` — calls a user's already-running local
    ComfyUI server's HTTP API with a pre-defined or user-supplied workflow
    graph. This is the "power user with a GPU and existing local setup"
    path and is strongly aligned with the project's general "don't
    reimplement what a mature local tool already does" philosophy.
- Output is always normalized to PNG bytes at the adapter boundary so
  `canvas-scene-and-keyframe-store`/the canvas editor never needs to know
  which provider produced an image asset.

**4. Cost/Rate Awareness (light touch, not a hard requirement)**

- Both adapters should surface provider errors distinctly for
  auth-failure vs. rate-limit vs. generic-failure (`TextProviderError`/
  `ImageProviderError` enums), so the UI can show an actionable message
  ("check your API key" vs "rate limited, try again shortly") rather than
  a generic failure toast.
- No built-in spend-tracking/budgeting UI at v1 — that's a reasonable
  backlog item (`docs/99-ideas-backlog.md`) if user demand emerges, not a
  day-one requirement.

**5. Explicit Scope Boundary — Not a General Chat Assistant**

This is a **feature-embedded assist tool** (a button inside the lyrics
editor / canvas editor that calls out and inserts a result), not a
general-purpose chat interface living as its own app section. This
boundary is deliberate: a full chat UI is a much larger surface (history,
threading, multi-turn context management, etc) that isn't a named product
pillar (doc 00 §2) and risks becoming exactly the kind of scope creep doc
18 exists to prevent. If a genuine need for a broader chat surface emerges
later, that requires a fresh product decision + ADR, not an organic
expansion of this adapter crate.

**6. Testing Approach**

Per doc 16 §2, no live-network calls in the automated test suite — both
providers are tested against fixture-based fakes
(`shared-test-support/suno-api-fixture-mocks`'s sibling pattern, or a small
dedicated fixture module within each adapter crate if reuse doesn't fit
cleanly) verifying request construction and response normalization. Live
provider verification is a manual QA step (doc 16 §5-style checklist) using
the human orchestrator's own API key/local server during that phase's
development.

**7. Interaction with Automation Pipelines (Phase 7, forward-reference only)**

Once Phase 7 exists, a pipeline step type "generate art asset for track
using prompt template X" or "generate a lyric variant" becomes a natural,
additive pipeline step built on these same adapters — but this doc's v1
scope is the manual, single-action assist UI only. Do not build the
pipeline-step integration as part of this phase unless Phase 7 already
exists and the human orchestrator explicitly asks for that combined work.
```

---

```
docs/13-automation-pipeline-engine.md
```
```markdown
**Automation Pipeline Engine Spec**

**1. Scope**

Covers `automation-pipeline-definition-store` (doc 07 §7 schema),
`automation-batch-render-orchestrator`, and `ui-screen-automation-pipeline-
builder`. This is Phase 7 per doc 04, gated behind the Core Maintainability
Gate (doc 18 §4) same as doc 12.

**2. Core Principle: Reuse, Never Reimplement, the Single-Track Path**

The single most important architectural rule for this subsystem (stated in
doc 01 §7 and doc 04 Phase 7 and repeated here because it is the guardrail
most likely to be violated under time pressure): **the orchestrator fans
out calls to the exact same `single-track-visualizer-render-service` +
`canvas-overlay-compositing-service` + `karaoke-lyric-timing-resolution-
service` used by the one-off manual render path.** It does not contain its
own parallel rendering logic. If a bug is fixed or a feature added to the
single-track render path, automation gets it for free, automatically,
because it's the same code — this is both a correctness guarantee (what
you designed manually is what you get in bulk) and a direct defense
against doc 18's duplicate-logic guardrail (§2.2).

**3. Pipeline Definition Model (recap + detail beyond doc 07 §7)**

- **Input selector** — one of:
  - `explicit_ids`: a fixed list of `remote_track_id`s.
  - `tag`: all tracks (for the currently active account, or a specified
    account) carrying a given tag.
  - `all_in_account`: every track in an account's synced library.
  - (Additive later: saved smart-filters/search queries, once the library
    browser's filter capabilities, doc 04 Phase 1, are rich enough to
    reuse their filter representation here rather than inventing a
    second one — check for reuse opportunity when this phase begins,
    per doc 18 §2.2.)
- **Scene reference** — a single `scene_id` (doc 07 §6). All items in a
  run share one scene/template — this is intentional (that's what "brand
  consistency at scale" means); per-item scene overrides are explicitly
  out of scope for v1 (note as backlog if requested).
- **Lyric source policy** — `remote_preferred` (use remote timing if it
  exists for that track, else run Whisper), `whisper_preferred` (always
  run Whisper, ignore remote timing even if present — for consistency
  across a batch where the user doesn't trust remote timing quality),
  `remote_only` (skip/flag tracks lacking remote timing rather than
  running Whisper at all — for speed-prioritizing batches).
- **Export settings** — resolution, frame rate, codec/container (sensible
  defaults, e.g. 1080p/30fps/H.264+AAC in an MP4 container, are fine for
  v1; expose as configurable, don't hardcode invisibly), and an output
  path template supporting at least `{track_title}`, `{account_name}`,
  `{date}` placeholder tokens for file naming/routing.

**4. Execution Model**

```
pipeline_runs (1) ──< pipeline_run_items (many)
```

- Starting a run: resolve the input selector into a concrete track list
  **at start time** (snapshot it into `pipeline_run_items` rows
  immediately) — do NOT re-evaluate a dynamic selector (e.g. "all tracks
  tagged X") mid-run if the user adds a new matching track while a run is
  in progress; that's a separate, deliberate "run again" action, not
  silent scope-growth of an in-flight run.
- A worker pool (configurable concurrency limit — default conservatively
  low, e.g. 2-4 concurrent renders, since visualizer rendering + ffmpeg
  encoding is resource-intensive; expose as a setting for users with
  beefier machines) pulls `pending` items, marks `in_progress`, invokes the
  single-track render service (per §2), and marks `completed`/`failed`
  with `error_message` on completion.
- **Per-item failure isolation:** one item failing (bad audio file, missing
  lyrics with `remote_only` policy, ffmpeg error) logs the error against
  that `pipeline_run_item` and continues the run — never aborts the whole
  batch. A run's final status is `completed` if all items reached a
  terminal state (`completed` or `failed`), regardless of how many
  individually failed; the UI clearly surfaces a per-run success/failure
  count summary.
- **Resumability:** if the app crashes or is closed mid-run, on next
  launch the orchestrator finds any `pipeline_runs` row still `running`
  with `pending`/`in_progress` items, and offers to resume (re-queue
  `in_progress` items as `pending` first, since we can't assume they
  completed) — never silently auto-resumes without the user seeing a
  "resume interrupted run?" prompt, since a long-crashed run's environment
  (e.g. a since-removed local file) may no longer be valid.

**5. Progress & Monitoring UI**

- A run-monitor view showing: overall progress (X/N complete), a live list
  of item statuses, and the ability to cancel a run (in-progress items
  finish or are killed cleanly — killing an in-flight ffmpeg process
  cleanly, not orphaning it, is a specific implementation requirement).
- Completed runs remain browsable in history (via `pipeline_runs` rows)
  with links to output files, so a user can revisit "what did that batch
  from last week produce."

**6. Performance Note (ties to doc 16 §6)**

Phase 7's exit criteria (doc 04) explicitly requires a real 20-50 track
test run as the practical performance validation gate before claiming
"thousands of tracks" scale — resource usage (CPU/GPU/disk I/O for
concurrent visualizer-render + ffmpeg-encode workers) should be observed
and recorded in this phase's own audit notes (doc 18 §3 style) as the
baseline informing the default concurrency setting (§4) and any future
"how many tracks can this realistically batch overnight on typical
hardware" guidance surfaced to users.

**7. Non-Goals (recap from doc 04, restated for emphasis)**

- No plugin-authored pipeline steps until Phase 8 (doc 11).
- No per-item scene override in v1.
- No distributed/multi-machine rendering — single local machine, worker-
  pool concurrency only.
```

---

```
docs/14-local-playback-and-audio-engine.md
```
```markdown
**Local Playback & Audio Engine Spec**

**1. Scope**

Covers `audio-decode-symphonia-bridge`, `audio-io-cpal-bridge`,
`local-playback-parity-service` (Phase 2), `recorded-audio-take-store` +
`audio-recording-capture-service` (Phase 2's recording slice), and
forward-notes for Phase 9's DAW evolution. This is the "make local files
feel exactly as good to use as remote streaming" layer, plus the seed of
audio capture.

**2. Decode Layer (`audio-decode-symphonia-bridge`)**

- Wraps `symphonia` (pure-Rust, actively maintained, wide format support —
  confirmed prior-art choice per doc 03 §3) to decode local audio files
  (whatever formats Suno downloads actually arrive as — confirm via doc 06
  capture/observation during Phase 1, likely MP3 and/or WAV/FLAC) into a
  normalized PCM stream: fixed sample format (e.g. `f32` interleaved),
  explicit sample rate and channel count metadata attached to every
  decoded chunk.
- This crate's public API should be a simple pull-based iterator/stream
  abstraction (`fn next_chunk(&mut self) -> Option<PcmChunk>`) — consumers
  (playback engine, visualizer feed, Whisper transcription, headless
  export's audio walk) all pull from the same normalized shape, so
  **decode logic exists in exactly one place** despite being consumed by
  four different downstream features (doc 18 §2.2 duplicate-logic
  guardrail, directly relevant here).

**3. Playback Engine (`audio-io-cpal-bridge` + `local-playback-parity-**
   service`)

- `audio-io-cpal-bridge` wraps `cpal` for output device enumeration/
  selection and the actual audio callback loop; kept thin (device I/O
  only), mirroring the FFI-bridge-crate pattern from doc 09 §3 (thin raw
  wrapper crate + a fuller-logic crate above it).
- `local-playback-parity-service` owns: play/pause/seek/volume/queue
  state, gapless-ish handling between queued tracks (buffer-ahead the next
  track's initial decode while the current one nears its end — "gapless-
  ish" because true sample-accurate gapless across arbitrary formats is a
  stretch goal, not a hard v1 requirement; note this honestly rather than
  overclaiming per doc 08 §8's accessibility-honesty precedent), and the
  unification of "playing a local file" vs. "previewing a remote stream
  URL" behind one player interface so the UI doesn't need two separate
  player implementations (doc 04 Phase 2 exit criteria: parity between
  remote-preview and local-file playback UX).
- This service is also the audio tap point for the visualizer's live
  preview feed (doc 09 §5) — it exposes a way to subscribe to the
  currently-playing PCM stream (a broadcast channel or similar) rather
  than the visualizer needing its own separate playback/decode instance
  for the same audio.

**4. Recording Capture (`audio-recording-capture-service`, Phase 2 slice)**

- Minimal v1: enumerate input devices (via the same `audio-io-cpal-bridge`
  crate, input-side), record to a local WAV (or similar lossless/simple
  format — avoid needing an encoder dependency for v1, PCM WAV is
  trivially correct and sufficient), save via `recorded-audio-take-store`
  (doc 07 §8) with basic metadata.
- No real-time monitoring effects, no punch-in/out, no multitrack — this
  is "hit record, capture a clean take, stop, save, play it back" only.
  Explicitly deferred to Phase 9: mixing, multitrack, effects processing,
  any JUCE-equivalent DSP work.

**5. Phase 9 Forward-Notes (non-binding, for future phase-doc authoring)**

When Phase 9 begins, its own doc should define concrete crate names rather
than inheriting these placeholder ideas verbatim — but worth recording now
so the phase isn't started from a blank slate:

- A multitrack session concept will likely need its own store (sessions
  containing multiple `recorded_takes`-like clips arranged on a timeline
  with per-clip gain/offset).
- Mixing (gain, pan, basic EQ) is real-time DSP — evaluate existing Rust
  DSP crates (`fundsp`, or similar) before hand-rolling filter math, per
  doc 03 §3.
- "Recreate something like JUCE" (per the original product notes) is an
  extremely large undertaking if taken literally (JUCE is a mature,
  decades-refined C++ audio framework) — Phase 9's actual doc should scope
  this down explicitly to "the specific DSP/mixing capabilities Sonora's
  recording-for-Suno use case needs," not "build a general audio framework
  competing with JUCE," per doc 00 §3's non-goals precedent. Flag this
  explicitly to the human orchestrator when Phase 9 planning begins, since
  it's the single largest scope-inflation risk in the entire roadmap.

**6. Error Handling & Device-Change Resilience**

- Both playback and recording paths must handle a device disconnecting
  mid-use gracefully (e.g. Bluetooth headphones dropping, USB interface
  unplugged) — surface a clear UI notification and fall back to system
  default device rather than silently hanging or crashing the audio
  callback thread. This is a common real-world annoyance worth explicit
  handling rather than an edge case to skip.
```

---

```
docs/15-multi-account-and-downloads-detail.md
```
```markdown
**Multi-Account & Download Manager — Detail**

**1. Scope**

Expands on doc 05 (auth mechanics) and doc 07 §§2-4 (schema) with the
operational/UX detail needed to implement Phase 1's account-switching and
download-manager features concretely.

**2. Account Lifecycle States**

An account (`accounts` row, doc 07 §2) moves through:

```
added (credential captured, profile not yet fetched)
   → active_valid (profile fetched, API calls succeeding)
   → needs_reauth (401s that refresh couldn't resolve, per doc 05 §4)
   → removed (soft-deleted, deleted_at set)
```

The UI must clearly distinguish `active_valid` from `needs_reauth` in the
account switcher (doc 08 §5) — e.g. a small warning badge on that
account's entry — since a multi-account power user needs to know at a
glance which of their several accounts requires attention, without it
blocking use of their other, still-valid accounts.

**3. Adding an Account — Flow**

1. User initiates "Add Account" from the account management screen
   (`ui-screen-account-management`).
2. User picks a method: **Paste Token** (always available), **Suno Login**
   (embedded webview), or **Sign in with Google/Facebook** (system browser
   + loopback).
3. On successful credential capture (any method), the app immediately
   makes one lightweight authenticated call (the account/profile endpoint,
   once captured per doc 06 §2.6) to (a) verify the credential actually
   works before committing it, and (b) populate `display_name`/
   `avatar_url`/`suno_user_id` — never save an account row with unverified
   credentials.
4. New account is added to `account-profile-store`; user chooses whether
   to make it active immediately or just add it alongside existing
   accounts.
5. First library sync for a newly added account happens automatically in
   the background (via `suno-library-sync-service`) so switching to it
   later doesn't present an empty library while a first sync churns.

**4. Switching Accounts — UX Detail (expands doc 05 §5)**

- Switching is a single click/selection in the always-visible account
  switcher (doc 08 §5) — no re-navigation, no loading-screen interstitial
  if that account's library cache already exists (instant swap); a brief,
  clearly-indicated background refresh may still occur to catch up on
  remote changes since last sync, but the UI should render the last-known
  cached state immediately rather than blanking while that refresh
  completes.
- Any in-progress operation scoped to the *previous* active account (e.g.
  a download queued from that account) continues running in the
  background — switching active account for browsing purposes must not
  cancel unrelated in-flight operations tied to another account.

**5. Download Manager — Operational Detail**

- Downloads are queued (not immediately fired) with the concurrency model
  matching what doc 13 §4 established for automation renders — a
  configurable max-concurrent-downloads setting (default modest, e.g. 3),
  since simultaneous large downloads can saturate bandwidth or hit
  server-side rate limits (watch for this in Phase 1's real usage and
  document observed behavior in doc 06 §2.7 if Suno's CDN pushes back).
- **Retry policy:** failed downloads (`status = 'failed'`) get an
  exponential-backoff automatic retry up to a small cap (e.g. 3 attempts,
  tracked via `attempt_count`), then surface as needing manual user
  action (a visible "retry" button) rather than retrying forever silently.
- **Resumability:** partial downloads (`bytes_downloaded` < `bytes_total`)
  should resume via HTTP range requests if the audio CDN supports them
  (confirm via capture/testing in Phase 1 — if not supported, downloads
  restart from zero on retry, which is acceptable but should be noted in
  doc 06 as an observed CDN limitation rather than assumed).
- **Storage location:** a user-configurable local library root folder
  (settings screen), with a sane per-platform default (e.g. platform
  "Music" folder or an app-specific subfolder — let the user decide which
  convention they prefer at setup, don't force one silently).
- **File naming:** a configurable template (mirroring doc 13 §3's export
  path template mechanism — reuse the same tiny templating logic if
  practical, per doc 18 §2.2's duplicate-logic guardrail, rather than
  writing two separate path-templating implementations for downloads vs.
  pipeline exports).

**6. Bulk Library Operations — Scope for Phase 1**

`suno-bulk-library-operations-service` initially supports whatever the
first real doc 06 capture reveals is *actually* bulk-capable server-side
(e.g. bulk tag, bulk delete) — if the real API only supports single-item
operations, this service still provides a "bulk" UX by looping
client-side with clear per-item progress/failure reporting (same
per-item-isolation principle as doc 13 §4's pipeline runs — don't let one
item's failure abort a 200-item bulk tag operation). Document explicitly
in doc 06 §2.5 which reality applies once known, rather than assuming
native bulk-endpoint support upfront.

**7. Data Ownership Clarification (ties to doc 07 §3-4)**

- `remote_tracks` rows are account-scoped cache entries — the same
  underlying Suno track visible to two different local accounts (unlikely
  but possible, e.g. a track shared/duplicated across accounts) gets two
  separate cache rows, not a deduplicated shared row — simplicity over
  premature cross-account deduplication (which would require confidently
  identifying "same track" across accounts, itself non-trivial and not a
  named product requirement).
- `downloads` rows reference a specific `remote_tracks` row (thus
  implicitly a specific account), even though the resulting
  `local_file_path` lives in one shared local library folder — if the
  same audio is downloaded once per account by coincidence, that's two
  local files for now (a future dedup-by-content-hash optimization is a
  reasonable backlog item, not a v1 concern).
```

---

```
docs/99-ideas-backlog.md
```
```markdown
**Ideas Backlog**

A running log of good ideas surfaced during development that are
explicitly OUT of current scope (per doc 03 §8's scope-discipline rule).
Nothing here is committed to — this is a parking lot, reviewed
periodically (e.g. at Core Maintainability Gate time, doc 18 §4, and at
the start of any new phase) to decide what (if anything) graduates into an
actual phase/doc update.

Format: `- [ ] <idea> — (surfaced during: <phase/task>, date if useful)`

**Parked Ideas**

- [ ] Auto-upload recorded takes (Phase 2) directly to a Suno project —
      deferred, Phase 2 is capture-and-save-locally only.
- [ ] Cross-account content-hash deduplication for downloaded audio files
      — deferred, not a v1 concern (doc 15 §7).
- [ ] Snapping/alignment guides in the canvas editor — deferred, not a
      Phase 5 exit criterion (doc 10 §5).
- [ ] Additional easing curve types beyond the v1 minimum set — additive,
      low priority (doc 10 §3).
- [ ] Spend/budget tracking UI for LLM/image-gen provider usage — deferred
      (doc 12 §4), only if real user demand emerges.
- [ ] Saved smart-filters/search queries reused as pipeline input
      selectors — deferred until library browser filtering (Phase 1) is
      mature enough to reuse cleanly (doc 13 §3).
- [ ] Per-item scene overrides within a single automation pipeline run —
      deferred, v1 pipelines are one-scene-per-run only (doc 13 §3).
- [ ] Plugin marketplace/distribution infrastructure — explicitly deferred
      indefinitely; local file-based plugin loading only (doc 04 Phase 8
      non-goals, doc 11 §5).
- [ ] WASM plugin host (Tier 2) — build only if a concrete Tier-1 (Rhai)
      limitation is actually hit in practice (doc 11 §2).
- [ ] Full mobile/tablet responsive layout — explicitly out of scope,
      desktop-first app (doc 08 §5).
- [ ] Full screen-reader accessibility support — stretch goal, not a v1
      blocker given current egui maturity (doc 08 §8).

**Review Log**

*(Populated as backlog reviews happen — empty at doc-set creation.)*

| Date | Reviewed By | Items Graduated to a Phase Doc | Items Removed (no longer relevant) |
|---|---|---|---|
| — | — | — | — |
```

---

```
README.md  (repo root — index into the doc set)
```
```markdown
**Sonora**

A native Rust desktop companion for Suno.com: full remote library control,
local playback parity, karaoke-grade synced lyrics (Suno-provided + local
Whisper enhancement), projectM-powered audio-reactive visualizer "music
videos," a freeform brand-canvas with keyframe animation, batch automation
pipelines, and (later) LLM-assisted lyric/art generation and a lightweight
recording studio.

This repository is developed primarily by an AI coding agent (GLM-5.2,
occasional Google-model assist for UI/UX-flavored subtasks) operating
inside an opencode CLI + oh-my-openagent orchestrator harness, with a human
as orchestrator/reviewer. **`docs/03-agent-constitution.md` and
`docs/18-codebase-health-guardrails.md` are binding operating rules for any
agent working in this repo — read them before writing code.**

**Doc Set Index (source of truth — read in this order)**

| # | Doc | Purpose |
|---|---|---|
| 00 | `docs/00-project-charter.md` | Vision, pillars, non-goals, definition of done |
| 01 | `docs/01-architecture-overview.md` | Layering rules, crate map shape, key design decisions |
| 02 | `docs/02-workspace-layout.md` | Literal Cargo workspace / directory tree |
| 03 | `docs/03-agent-constitution.md` | **Binding** agent operating rules (size caps, prior-art mandate, DoD) |
| 04 | `docs/04-phase-roadmap.md` | Phase-by-phase plan, entry/exit criteria, Core Maintainability Gate |
| 05 | `docs/05-auth-and-multi-account.md` | Suno auth (3 methods) + multi-account mechanics |
| 06 | `docs/06-suno-api-integration-contract.md` | **Capture-driven** Suno API contract (empty until real captures provided) |
| 07 | `docs/07-data-model-and-storage-schema.md` | SQLite schema for every store crate |
| 08 | `docs/08-ui-ux-design-system.md` | Glass theme, Catppuccin/Monokai tokens, layout conventions |
| 09 | `docs/09-visualizer-projectm-integration.md` | projectM FFI integration, preview vs. headless export |
| 10 | `docs/10-canvas-and-keyframe-system.md` | Canvas editor, keyframes, karaoke text element |
| 11 | `docs/11-plugin-system-deferred-design.md` | Phase 8 plugin design (stub-only until then) |
| 12 | `docs/12-llm-and-image-gen-provider-adapters.md` | Lyric/art-gen provider adapters (priority per ADR-008) |
| 13 | `docs/13-automation-pipeline-engine.md` | Batch rendering engine (reuses single-track render path) |
| 14 | `docs/14-local-playback-and-audio-engine.md` | Decode/playback/recording engine |
| 15 | `docs/15-multi-account-and-downloads-detail.md` | Account lifecycle + download manager operational detail |
| 16 | `docs/16-testing-strategy.md` | Test levels, fixture/mock conventions, CI bar |
| 17 | `docs/17-glossary-and-decisions-log.md` | Glossary + ADR log (living doc) |
| 18 | `docs/18-codebase-health-guardrails.md` | Anti-spaghetti guardrails + Core Maintainability Gate |
| 99 | `docs/99-ideas-backlog.md` | Deliberately-parked ideas (living doc) |

**Getting Started (once docs are approved and coding begins)**

Work proceeds phase-by-phase per doc 04, with each phase's implementation
driven by prompts derived from the relevant docs above — never freehand
against the codebase without doc grounding. Git workflow (branches,
worktrees, tags, releases) is intentionally undefined here per project
decision — establish it at Phase 0 kickoff as a lightweight addendum, not
a blocker to starting this doc set.

**Status**

📋 **Planning complete, implementation not yet started.** This doc set is
the master source of truth. Any code contradicting these docs should be
treated as a bug in either the code or the doc — resolve via updating one
to match the other (with an ADR in doc 17 if it's a real decision change),
never by silent drift.
```

---

— **END OF DOC BATCH, ALL DOCS FOR THIS STAGE DELIVERED —**

That's the complete planning doc set: charter, architecture, workspace layout, agent constitution, phase roadmap (with the Core Maintainability Gate amendment), auth design, API contract skeleton, data schema, UI/UX system, visualizer spec, canvas/keyframe spec, plugin deferred design, LLM/image-gen adapter spec, automation engine spec, audio engine spec, account/download detail, testing strategy, ADR log, ideas backlog, and a root README index.

A few things worth a quick gut-check from you before you start feeding these into the coding agent:

1. **Naming** — "Sonora" was just a placeholder working name I picked for reference-consistency across docs. Rename now (cheap, it's a find/replace) or keep it as a codename and decide branding later?
2. **Phase 0 kickoff prompt** — want me to draft the actual first implementation prompt (referencing docs 00-03, 08, 09§4 spike, 18) that you'd paste into the agent to kick off scaffolding, or are you set to write that yourself now that the doc set exists?
3. Anything across this whole set that reads as over-engineered or under-specified now that you see it all together?

### 🧑‍💻 User

1 - The user will find and replace from the chat history output json with their selected project name.
2 - Ensuring this is strictly or nearly so focusing on having the model setup the project for full project lifespans upkeep and maintenance including git ops such as workspaces, branches, pull-requests, ensuring documentation is layed out beforehand for agents to read whether AGENTS.md anywhere and that a globalk TODO.md is kept either in its own file or within another markdown document within chat context with gaurentee. Reminding proper programming paradigms are followed and the idea of keeping files highly granular to allow room-to-grow and so directory and file names on their own describe exactly what a class does for example, using multiple choice or inferring with the user whenever not sure, keeping files versioned with last modified dates where appropriate, reminding model that no human will be writing code and the user is instead only orchestrating so they must handle everything as the complete dev team, noting there is no token usage limits within sane reason using currently free model on openrouter, opencode, and kilocode, that the users local tools may be used or installed with brew as well as cloned from git and compiled/installed with this including the `gh` github cli which is great for searching for git repos to clone for reference code/logic as well as libraries (dont reinvent the wheel), and anything else your much superior knowledge deems useful in this MASSIVE and AMAZING outline you have nearly solely outlined.
3 - The user will infer about this once they read over the codes and get them split into proper markdown documents in the currently empty project repository.

### 🤖 Assistant — claude-sonnet-5-high

Excellent — these are exactly the "full lifecycle" gaps worth closing before any code gets written. Adding: an appendix to the agent constitution (decision protocol, AGENTS.md/TODO.md mandates, tooling permissions, the "no hard token ceiling but context still finite" clarification, full-SDLC role coverage), a dedicated git/repo-governance doc, root-level `AGENTS.md` and `TODO.md` templates, and the literal Phase 0 kickoff prompt.

---

```
docs/03-agent-constitution.md  — APPEND §9 ONWARD
```
```markdown
**9. Decision-Making Protocol: Ask vs. Infer**

Not every ambiguity warrants stopping to ask the human orchestrator — but
silently guessing on consequential decisions is equally wrong. Use this
test:

- **Architecture-level, hard-to-reverse, or contradicts/extends a doc?**
  → STOP. Present the human a short **multiple-choice** breakdown (2-4
  labeled options, each with a one-line tradeoff, plus your own
  recommended pick) — mirror the exact style used to originally scope this
  project (docs 00-18 were built this way; keep using it for the same
  reason: it's fast for the human to answer and forces the agent to have
  already thought through the tradeoffs before asking). Do not proceed
  until answered. Record the outcome as a new ADR (doc 17) once decided.
- **Local implementation detail, fully within an already-decided
  architecture, cheaply reversible?** → Infer a reasonable choice, proceed,
  and note the inference briefly in the task's commit/PR description
  ("Note: chose X naming/shape for Y, not specified in docs, easily
  changed if wrong") so the human can correct it cheaply on review without
  it having blocked progress.
- **When genuinely 50/50 even after research** (checked docs, checked
  prior art per §3) — default to asking rather than guessing. Being asked
  slightly too often is a cheap failure mode; silently baking in a wrong
  consequential assumption across dozens of downstream files is an
  expensive one.

**10. AGENTS.md & TODO.md — Mandatory, Always-Current**

- **`AGENTS.md`** (repo root, plus optionally one per major subtree — see
  doc 19 §6) is the front-door orientation file for any agent (this one, a
  future session of this one, or a different model entirely) picking up
  this codebase cold. It must always accurately reflect current reality:
  where the docs live, what phase the project is in, what the immediate
  next steps are, and pointers to the constitution/guardrail docs. Treat a
  stale `AGENTS.md` as a bug, same as a stale doc 02 crate tree (doc 18
  §2.5).
- **`TODO.md`** (repo root) is the living task tracker — the single place
  a human or agent looks to answer "what's in flight, what's next, what's
  blocked." It must be updated **at the start and end of every work
  session/task**, without exception — this is part of the Definition of
  Done (§5) from this point forward: a task is not "done" if `TODO.md`
  still shows it as pending/in-progress. Structure and template are
  defined in doc 19 §7.
- Both files are **guaranteed-maintained artifacts**, not best-effort —
  they exist specifically so a fresh agent context (after a reset, a
  model switch, or a long gap) can resume orchestration correctly without
  the human having to re-explain project state from memory.

**11. Token/Compute Budget Reality — No Hard Cost Ceiling, Context Still Finite**

The primary development model runs on a free tier (OpenRouter/opencode/
kilocode-hosted, GLM-5.2 primary), so **token *cost* is not a constraint to
optimize for** within sane bounds — do not shortcut research depth, test
thoroughness, self-review rigor (§4), or prior-art search (§3) to "save
tokens" in the cost sense. Be as thorough as the task genuinely warrants.

This does **not** eliminate the value of the caveman-speak internal
scratchpad convention (§6) — that convention exists for **context-window
economy and reasoning throughput**, not dollar-cost avoidance, and remains
worthwhile: a model's context window is still finite regardless of price,
and terse internal reasoning leaves more effective context available for
actually-relevant file contents, doc excerpts, and tool output. So:
**think efficiently (terse scratch channel), but don't work superficially
(skip steps) to save money that isn't actually a limiting factor here.**
When in doubt, prefer the more thorough path.

**12. Local Tooling Acquisition Permissions**

The agent is explicitly permitted, and encouraged where it genuinely
serves a task, to:

- Install developer tooling on the user's host machine via **Homebrew**
  (`brew install ...`) — e.g. `ffmpeg`, `sqlite`, build dependencies for
  `projectM` or `whisper.cpp`, etc.
- **Clone reference repositories via `gh repo clone` / `git clone`** into a
  scratch/tmp directory to study implementation patterns or vendor/build a
  needed C/C++ dependency (e.g. building `projectM` from source if no
  suitable system package exists) — never committed into this repo
  verbatim; either build against it as an external dependency, or extract
  learnings/patterns per the attribution note in §3.
- Use **`gh` for repo/code search** (`gh search repos`, `gh search code`)
  as the default first move before writing any non-trivial subsystem from
  scratch (§3) — this is not optional flourish, it is the mandated
  first step.
- Run arbitrary local build/inspect commands (`cmake`, `cargo install`,
  `pkg-config`, etc.) as needed to get a dependency working, rather than
  stopping at "this might require system setup I can't do" — the agent
  has shell access and should use it.
- When a tool/library installation could have side effects the human might
  not want (e.g. installing a large toolchain, modifying global system
  state beyond the project), briefly state what's about to be
  installed/why before doing it — not a full stop-and-ask per §9's test
  (this is typically a low-consequence, reversible action), just a
  transparency note in the task's output.

**13. File & Documentation Versioning Convention**

- Every file in `docs/` carries a header metadata line directly under its
  H1 title:
  ```
  > **Last Updated:** YYYY-MM-DD · **Status:** Draft | Active | Superseded
  ```
  Update the date whenever a doc's *content* meaningfully changes (not for
  trivial typo fixes). `Status: Superseded` is used rather than deleting a
  doc outright if a later decision fully replaces it — link to its
  replacement in that case.
- Source code (`.rs` files) does **not** duplicate a "last modified" stamp
  in-file — git history is the authoritative, always-accurate source of
  truth for that, and an in-file timestamp would only ever go stale and
  mislead. Where "freshness" matters for a generated artifact (e.g. a
  crate's `README.md` stub's "Public API status" line, per doc 18 §2.3),
  keep that specific line current as part of the relevant task's DoD
  instead of a blanket timestamp convention.
- `TODO.md` and `AGENTS.md` (§10) are the two files expected to change
  most frequently and are explicitly exempted from needing a "last
  updated" header — their freshness is enforced by the DoD requirement
  itself (§5, §10), not by a date stamp.

**14. Full Dev-Team Role Coverage (expands §4)**

Because no human writes code on this project, the agent must consciously
rotate through **every** role a real team would staff, not just
implementer/reviewer (§4). Depending on the task at hand, explicitly adopt
the relevant hat:

- **Product/Project Lead:** keeps work aligned to doc 00's pillars and doc
  04's phase scope; owns saying "no, that's backlog" (§8) and updating
  `TODO.md`/`AGENTS.md` (§10).
- **Software Architect:** owns doc 01/02 layering integrity and the doc 18
  guardrails; performs the end-of-phase Senior Architect Pass (doc 18 §3).
- **Senior/Junior Engineer:** implements per §1-§3 of this doc.
- **QA/Test Engineer:** ensures doc 16's test levels are actually satisfied
  per task, not just "code compiles."
- **DevOps/Release Engineer:** owns git hygiene per doc 19 — branches,
  worktrees, PR creation via `gh`, and (later) tagging/release mechanics.
- **Technical Writer:** keeps every doc-reality-sync obligation (doc 18
  §2.5, §13 above) actually current — documentation is a first-class
  deliverable of every task, not an afterthought squeezed in if time
  allows.
- **Security-minded reviewer:** specifically for anything touching
  credentials/secrets (doc 05 §3, doc 16 §7) — treat this as a distinct
  mandatory lens, not folded silently into general code review.

A single task may only need 2-3 of these hats actively; the point is
*conscious* coverage — don't let "nobody's job" gaps (a classic cause of
real-world project decay, and a contributor to the predecessor project's
spaghetti per ADR-008) exist just because no human was assigned that
function.
```

---

```
docs/19-git-workflow-and-repo-governance.md
```
```markdown
**Git Workflow & Repository Governance**

> **Last Updated:** 2024-01-01 · **Status:** Active

**1. Why This Matters Even With a Sole Human Orchestrator**

No human writes code here, but git history is still the project's
permanent audit trail, rollback mechanism, and the primary artifact the
human orchestrator actually reviews (rather than re-reading every doc on
every task). Good git hygiene is therefore not bureaucratic overhead — it
is the human's main lever for staying in control of an AI-authored
codebase without reading every line.

**2. Branching Model**

**Trunk-based, short-lived feature branches.** `main` is always expected
to pass the CI bar (doc 16 §4) — nothing is merged that doesn't build,
lint clean, and pass tests.

- **Branch naming:** `phase-<N>/<crate-or-feature-slug>`, mirroring doc
  04's phase numbers and doc 02's crate names for instant legibility —
  e.g. `phase-0/foundation-workspace-scaffold`,
  `phase-1/suno-auth-manual-token-paste`,
  `phase-5/canvas-keyframe-timeline-ui`. For cross-cutting work not tied
  to one crate, use `phase-<N>/<short-description>`.
- **One branch per meaningfully-scoped task**, not one giant branch per
  phase — small branches/PRs are easier for the human to spot-check and
  easier to revert individually if one is wrong, consistent with doc 03
  §6's small-diff preference.
- Branches are deleted after merge (locally and remotely) to keep the
  branch list a true reflection of active work, not history.

**3. Worktrees**

Use `git worktree` when working on more than one branch's checkout
simultaneously is useful (e.g. the orchestrator harness running a
parallel sub-task while another is mid-review, or keeping a long-lived
Phase-0-spike branch checked out separately from day-to-day Phase 1 work).

- **Convention:** worktrees live as sibling directories to the main repo
  checkout: `../sonora-worktrees/<branch-slug>/` (adjust `sonora` to the
  user's actual chosen project name). Never nest a worktree inside the
  main checkout's own tree.
- Not mandatory for every task — a normal `git checkout -b` is perfectly
  fine for straightforward sequential work. Reach for a worktree
  specifically when genuine parallelism is happening.

**4. Commit Conventions**

- **Conventional-commit-style prefixes:** `feat:`, `fix:`, `docs:`,
  `refactor:`, `test:`, `chore:`, `perf:` — enables clean changelog
  generation later and gives the human a fast scan of intent.
- **Small, single-concern commits** (doc 03 §6) — a commit should be
  independently understandable and, ideally, independently revertible.
- **Commit body** (not just the subject line) briefly states *why*, not
  just *what*, when the "why" isn't obvious from the diff alone — this is
  where a §9-style "inferred X, easily changed if wrong" note belongs when
  applicable.

**5. Pull Requests — Used Even for Sole-Maintainer Merges**

Every task-branch gets a PR opened via `gh pr create` before merging to
`main`, even though the human orchestrator is typically the only reviewer/
merger. This is deliberate, not theater:

- The **PR description is the structured self-review artifact** doc 03
  §4 requires (junior/senior/lead-pass notes) — it's the fastest way for
  the human to spot-check reasoning without reading the full diff first.
- It creates a natural checkpoint for the human to say "wait, no" before
  something lands on `main`, without slowing down agent throughput (the
  agent can keep working on the next branch while a PR awaits a glance).
- It gives future-agent-sessions (§10 in doc 03) a searchable history of
  *why* things were built a certain way, via `gh pr list`/`gh pr view`,
  which is cheaper to query than re-reading raw commit diffs.

**PR description template:**
```markdown
**What & Why**
<1-3 sentences>

**Docs Referenced**
<which docs/sections this work implements/follows — e.g. doc 04 Phase 1,
doc 07 §2>

**Self-Review Notes**
- Junior pass: <brief>
- Senior pass: <brief — edge cases considered, layering check per doc 01>
- Lead pass: <scope check per doc 04/doc 03 §8; any inferred decisions
  per doc 03 §9 flagged here>

**Testing**
<what tests were added/run, per doc 16>

**Docs Updated**
<list any docs touched to stay in sync, per doc 18 §2.5, or "none needed">
```

Merge strategy: squash-merge (keeps `main`'s history clean and matches the
"one branch = one logical change" convention above) unless a PR
genuinely contains multiple commits worth preserving individually (rare —
prefer splitting into multiple PRs instead when that's the case).

**6. `AGENTS.md` — Structure & Placement**

- **Root `AGENTS.md`** (template below) is the mandatory front-door file —
  every agent session should read it first, before diving into `docs/`.
  It stays short and points outward rather than duplicating content.
- **Nested `AGENTS.md` files** are optional and should exist only where a
  subtree has genuinely distinct conventions worth flagging close to the
  code (e.g. `crates/external-bridges/AGENTS.md` reminding that
  `unsafe`/FFI code is concentrated in specific crates per doc 09 §3, or
  `crates/ui/AGENTS.md` reminding of the UI-framework decision from doc
  01 §4). A nested file should be short (a paragraph or two) and always
  link back to the authoritative doc rather than restating it — avoid
  doc 18 §2.5-style drift risk by never having two files claim to be the
  source of truth for the same fact.

**7. `TODO.md` — Structure & Update Discipline**

Root-level, single file (not per-crate — a scattered multi-file TODO
system defeats the "one place to look" purpose). Structure:

```markdown
**TODO**

> Updated at the start/end of every work session — see docs/03 §10.
> Current phase: <Phase N — name> (docs/04-phase-roadmap.md)

**In Progress**
- [ ] <task> — branch: `phase-N/...` — <one-line status>

**Up Next (this phase)**
- [ ] <task, mirrors the phase doc's remaining exit-criteria items>

**Blocked / Needs Human Input**
- [ ] <task> — blocked on: <e.g. "needs a Burp capture of endpoint X, see doc 06">

**Recently Completed (rolling short list, not full history — git log is history)**
- [x] <task> — PR #<n>
```

Rules:
- **Never let "In Progress" contain a stale entry** — if a session ends
  mid-task, the entry stays with an honest status note; if it's actually
  done, it moves to "Recently Completed" with its PR link, immediately.
- "Blocked / Needs Human Input" is the primary channel for surfacing
  doc-03-§9-style stop-and-ask items and doc-06-style capture requests —
  the human should be able to open just this file and know exactly what's
  waiting on them.
- Keep "Recently Completed" short (e.g. last 10-15 items) — prune older
  entries; they remain fully discoverable via `git log`/merged PRs, this
  section is a quick-glance convenience only, not an archive.

**8. Tags & Releases (lightweight policy for now)**

Full release engineering is intentionally deferred (per original project
scope decision), but a minimal placeholder policy avoids total ambiguity:

- No tags/releases during Phase 0-1 (nothing user-facing to release yet).
- First tag (`v0.1.0`) is reasonable once Phase 2 (local playback parity)
  is functionally complete — the first point where "a person could
  actually use this for something" is true. Revisit/formalize actual
  semver policy and changelog generation at that point via an ADR (doc
  17), rather than deciding it speculatively now.

**9. `.gitignore` Baseline (Phase 0 task)**

Ensure standard Rust (`/target`, `Cargo.lock` handling — commit
`Cargo.lock` for a binary/application project, which this is, per Cargo's
own guidance for applications vs. libraries), OS-cruft (`.DS_Store`,
`Thumbs.db`), editor-cruft, and local secrets/config
(`*.local.toml`, any accidentally-created credential-dump files) are
git-ignored from the very first commit.
```

---

```
AGENTS.md
```
```markdown
**AGENTS.md**

If you are an AI agent picking up this repository — read this file first,
then follow its pointers. Do not start writing code before reading
`docs/03-agent-constitution.md` and `docs/18-codebase-health-guardrails.md`
in full — they are binding.

**What This Project Is**

See `docs/00-project-charter.md` for the full vision. One-line summary: a
native Rust desktop companion for Suno.com with advanced library, playback,
karaoke/visualizer video, and automation features not available in Suno's
official clients.

**Who's Building This**

An AI coding agent (primary: GLM-5.2, occasional Google-model assist for
UI/UX-flavored subtasks), operating inside an opencode CLI + oh-my-openagent
orchestrator harness. **No human writes code on this project** — the human
is the orchestrator/reviewer only. You are expected to act as the entire
development team (see `docs/03-agent-constitution.md` §14).

**Where Everything Is**

- **`docs/` — the master source of truth.** Read `README.md`'s doc index
  table first to know which doc covers what. Every doc is numbered and
  cross-referenced; do not contradict a doc without recording the change
  as an ADR in `docs/17-glossary-and-decisions-log.md`.
- **`TODO.md`** (repo root) — current task state. Check this before
  starting work, update it before finishing (mandatory, see
  `docs/03-agent-constitution.md` §10).
- **`crates/`** — the Cargo workspace, laid out per
  `docs/02-workspace-layout.md`. Directory/file names are deliberately
  verbose — trust them to describe their own contents.

**Non-Negotiable Rules (full detail in doc 03 & doc 18)**

1. Files: ~150-200 line soft cap, 300 line hard cap.
2. Check crates.io + `gh search` for existing solutions before writing any
   non-trivial subsystem from scratch.
3. Respect the crate-layering direction (doc 01 §3) — no exceptions.
4. Every task ends with: tests passing, docs in sync, `TODO.md` updated,
   a PR opened via `gh pr create` (see `docs/19-git-workflow-and-repo-
   governance.md`).
5. Never invent a Suno API endpoint shape — `docs/06-suno-api-integration-
   contract.md` is capture-driven; halt and ask for a capture if one's
   missing.
6. When uncertain on anything architecture-level, present the human a
   short multiple-choice breakdown rather than guessing (doc 03 §9).

**Current Status**

See `TODO.md` for exact current state. High-level: see `docs/04-phase-
roadmap.md` for the phase plan and which phase is currently active.
```

---

```
TODO.md
```
```markdown
**TODO**

> Updated at the start/end of every work session — see
> `docs/03-agent-constitution.md` §10 and `docs/19-git-workflow-and-repo-
> governance.md` §7.
>
> Current phase: **Phase 0 — Foundation** (`docs/04-phase-roadmap.md`)

**In Progress**
- [ ] *(empty — project not yet started)*

**Up Next (this phase)**
- [ ] Initialize git repo, `.gitignore`, root `Cargo.toml` workspace skeleton
      (docs/02, docs/19 §9)
- [ ] Scaffold every crate stub per docs/02 (empty `lib.rs` + `README.md`
      per docs/templates/crate-stub-template)
- [ ] Implement `structured-logging-and-tracing`, `app-configuration-
      loader`, `design-tokens-theme-definitions` (Catppuccin + Monokai)
- [ ] `sonora-app` binary: themed empty window + nav shell + working
      theme-switcher
- [ ] egui-vs-iced spike + projectM texture-compositing feasibility spike
      (docs/01 §4, docs/09 §4-5) → record decision as ADR
- [ ] `xtask check-layering` command (docs/18 §2.1)
- [ ] End-of-phase Senior Architect Pass + Phase Audit Summary (docs/18 §3)

**Blocked / Needs Human Input**
- [ ] Need first sanitized Burp Suite capture(s) of Suno auth + library-
      list + track-detail flows to unblock meaningful Phase 1 work on
      `suno-http-client-core` (docs/06)

**Recently Completed**
- [x] Full planning doc set (docs/00-18, 99) authored and approved by human
```

---

```
docs/20-phase-0-kickoff-prompt.md
```
```markdown
**Phase 0 Kickoff Prompt (Literal, Paste-Ready)**

> **Last Updated:** 2024-01-01 · **Status:** Active

This is the literal prompt text to paste into the agentic coding harness
(opencode + oh-my-openagent) to begin implementation. Paste as-is, or
lightly adjust the project name if you haven't already find-replaced
"Sonora" throughout the repo.

---

```text
You are the sole development team for this project — no human will write
any code. The human is your orchestrator and reviewer only. Before doing
anything else, read these files in full, in order:

1. AGENTS.md
2. docs/00-project-charter.md
3. docs/01-architecture-overview.md
4. docs/02-workspace-layout.md
5. docs/03-agent-constitution.md  (binding operating rules — internalize
   fully: file size caps, prior-art-first mandate via crates.io/gh search,
   the ask-vs-infer decision protocol, AGENTS.md/TODO.md maintenance
   requirements, full dev-team role coverage, and the note that token cost
   is not a constraint here — be as thorough as each task genuinely
   warrants, don't cut corners for token economy)
6. docs/04-phase-roadmap.md  (note §0's Core Maintainability Gate)
7. docs/08-ui-ux-design-system.md
8. docs/09-visualizer-projectm-integration.md  (note §4 build/packaging
   decision and §5's live-preview-vs-headless-export design)
9. docs/16-testing-strategy.md
10. docs/18-codebase-health-guardrails.md  (binding — the whole reason
    this doc set is this thorough is to avoid the predecessor project's
    ~25k LOC spaghetti outcome; take this doc as seriously as doc 03)
11. docs/19-git-workflow-and-repo-governance.md
12. TODO.md

Then execute Phase 0 exactly as scoped in docs/04-phase-roadmap.md's
"Phase 0 — Foundation" section, in this order:

1. Git/repo setup: initialize the repository if not already, set up
   .gitignore per doc 19 §9, confirm AGENTS.md and TODO.md exist and are
   accurate (they should already exist from planning — verify, don't
   duplicate).
2. Scaffold the ENTIRE Cargo workspace tree exactly as specified in
   docs/02-workspace-layout.md — every crate listed gets a real directory,
   a Cargo.toml, and a stub lib.rs + README.md per
   docs/templates/crate-stub-template, even crates whose real
   implementation is phases away. The workspace must compile
   (`cargo build --workspace`) with this stub tree in place before you
   move on.
3. Implement the Layer 0 foundation crates for real (not stubs):
   error-and-result-conventions, app-configuration-loader,
   structured-logging-and-tracing, design-tokens-theme-definitions
   (Catppuccin Mocha/Frappé/Macchiato/Latte + Monokai — source real
   palette values from an existing `catppuccin` crate / known-good
   Monokai reference, per the prior-art mandate, don't hand-transcribe
   hex codes from memory).
4. Do the required Phase 0 spike (doc 01 §4, doc 09 §4-5): validate that
   your chosen UI framework (egui, per current lean — confirm or switch to
   iced if a real blocker surfaces) can share/composite a GPU texture
   produced by a separate rendering context, since this de-risks the
   entire visualizer architecture. Record the outcome — including the
   projectM build/packaging decision (system-installed vs. vendored via
   build.rs) — as a new ADR in docs/17.
5. Build `sonora-app`: a themed, empty window with a left-nav shell
   (sections can be placeholder/disabled for now) and a working
   theme-switcher proving the design-token pipeline works end to end.
6. Build the `xtask` `check-layering` command per docs/18 §2.1.
7. For every piece of work, follow the full Definition of Done (docs/03
   §5) — tests, clippy, layering check, doc-sync, and a PR opened via
   `gh pr create` per docs/19 §5 — do not batch all of Phase 0 into one
   giant PR; split by logical unit (e.g., one PR for workspace scaffold,
   one for the foundation crates, one for the spike, one for the app
   shell, one for xtask).
8. Update TODO.md continuously as you go (docs/19 §7) — never let it go
   stale mid-session.
9. At the end of Phase 0, perform the Senior Architect Pass (docs/18 §3)
   and produce a Phase Audit Summary. Do not mark Phase 0 complete in
   TODO.md until this pass is done and its findings (if any) are resolved
   or explicitly logged in docs/99-ideas-backlog.md.

Ground rules while you work:
- Never guess a Suno API endpoint's shape — none should be needed in
  Phase 0 anyway, but if you find yourself tempted to stub something
  Suno-shaped, stop; that's Phase 1's job and depends on real captures
  (doc 06).
- Use `gh search repos` / `gh search code` / crates.io before writing any
  non-trivial logic from scratch — this includes, e.g., checking for an
  existing `catppuccin` Rust crate, existing easing-function crates,
  existing wgpu-texture-sharing examples, before hand-rolling any of it.
- Use `brew`, `git clone`, and local compilation as needed to get
  dependencies (e.g. projectM) working on this machine — you have
  permission to install tooling per docs/03 §12.
- When you hit a genuinely architecture-level ambiguity not resolved by
  the docs, stop and present the human a short multiple-choice breakdown
  with your recommendation (docs/03 §9) rather than guessing. For minor,
  reversible implementation details, make a reasonable call and note it.
- You are not token-constrained in any meaningful sense here (free-tier
  model) — be thorough. Do not skip tests, skip the prior-art search, or
  skip a self-review pass to save effort.

Begin with step 1.
```
```

---

```
README.md  — UPDATE DOC INDEX TABLE (append rows, insert after row 18)
```
```markdown
| 19 | `docs/19-git-workflow-and-repo-governance.md` | Branching, worktrees, PR process, AGENTS.md/TODO.md conventions |
| 20 | `docs/20-phase-0-kickoff-prompt.md` | Literal paste-ready prompt to begin implementation |

Also at repo root:
- **`AGENTS.md`** — front-door orientation file for any agent session (read first)
- **`TODO.md`** — living task tracker (must always be current — see doc 03 §10)
```

---

— **END OF DOC BATCH, ALL DOCS FOR THIS STAGE DELIVERED —**

That closes the full-lifecycle governance gap: git branching/worktrees/PR discipline, `AGENTS.md` + `TODO.md` as guaranteed-current living artifacts, the ask-vs-infer decision protocol, explicit tooling permissions (brew/git-clone/`gh`), the "no cost ceiling but context still finite" clarification, full-SDLC role coverage, doc versioning convention, and a literal paste-ready Phase 0 kickoff prompt.

The doc set (00–20 + 99, plus root `AGENTS.md`/`TODO.md`) is now complete end-to-end — from vision through architecture through anti-spaghetti guardrails through git ops through the actual first prompt. Once you've dropped these into your repo and swapped "Sonora" for your real project name, doc 20 is what you paste to kick everything off. 🎉
