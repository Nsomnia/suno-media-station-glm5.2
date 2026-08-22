**Glossary & Architecture Decision Log (ADR)**

**Glossary**

- **Suno Station** — working project name (this app).
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

---

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
