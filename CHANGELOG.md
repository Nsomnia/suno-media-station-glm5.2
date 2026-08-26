# Changelog

All notable changes to this project are documented in this file.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); versioning
per docs/19 §8 (first tag `v0.1.0` at Phase 2 completion). Granular history
lives in the commit log — this file is the human-skimmable summary.

## [Unreleased]

### Planning & Documentation

- Full planning doc set authored (docs/00–20, 99): charter, architecture,
  workspace layout, agent constitution, phase roadmap with entry/exit
  criteria, auth/multi-account design, capture-driven Suno API contract,
  storage schemas, UI/UX design system, per-subsystem specs (visualizer,
  canvas/keyframes, plugins-deferred, LLM/image-gen adapters, automation,
  playback/audio, downloads), testing strategy, health guardrails with the
  Core Maintainability Gate, git governance, and the Phase 0 kickoff prompt.
- Chat-session export (`docs/.agent/PROMPTS/`) extracted into per-file docs
  via `scripts/extract-agent-docs.py`; one commit per document.
- Meta audit round (`docs/meta/`): full findings review with approved
  corrections — scope amendments, 11 mechanical fixes, ground-truth
  corrections, bloat merges, structure normalization, autonomy/infrastructure
  adoption plan (ADR-012).

### Added

- **Scope:** Full-Surface Client pillar (ADR-009) — creation features via
  Suno's server-side API are first-class; new **Phase 6b — Suno Creation
  Studio** in the roadmap; four new crates in the workspace tree
  (`suno-generation-client`, `suno-upload-client`, `generation-job-store`,
  `ui-screen-creation-studio`); doc 06 gains endpoint categories 2.8–2.15.
- **Ground truth:** Suno API recon corpus imported to
  `docs/captures/raw/recon-from-chadvis/` with provenance tiers (T1/T2/T3);
  every entry is a LEAD until one fresh confirming capture (ADR-007 spirit).
- **Corrections:** auth model corrected to Clerk session-token exchange
  (ADR-010); word-level aligned-lyrics endpoint confirmed to exist; egui
  glow-vs-wgpu renderer decision opened as a mandated Phase 0 spike;
  glassmorphism recipe tiered A/B with guaranteed Tier-B baseline.
- **Process:** TODO.md state-mark system ([ ] [~] [!] [?] [x] [X] [-]) with
  agent-mutates/user-only-removes ownership (ADR-011,
  `docs/meta/TODO-task-state-conventions.md`); autonomous session protocol
  (`docs/meta/session-protocol-autonomous-operation.md`) defining the daemon
  loop behind "continue working on this project".
- **Infrastructure:** GitHub Actions CI (fmt/clippy/test matrix/guardrails/
  cargo-deny) and tag-gated release workflow; PR template matching doc 19 §5;
  issue templates including a structured Burp capture-request form;
  `rust-toolchain.toml`, `.editorconfig`, `deny.toml`; completed `.gitignore`.
- Root `Cargo.toml` made valid TOML (edition 2024); binary crate renamed to
  `station-app`; crate-stub template repaired.

### Changed

- Doc set normalized to ATX headings with Last-Updated metadata headers.
- Doc 15 superseded — content folded into docs 05 & 07 (retained for history).
- README junk tagline removed; full doc index table maintained.
- **Docs reorganized into a categorized wiki taxonomy** — `product/`,
  `architecture/`, `specs/<domain>/`, `phases/phase-N-<slug>/`, `process/`;
  numeric filename prefixes kept so "doc N §M" references stay valid; hub
  (`docs/README.md`) + per-category index pages added; root README is now a
  compact landing page delegating to the hub.
- Root/nested `AGENTS.md` refreshed as hyperlinked front door; `CLAUDE.md`
  provided as a symlink.

### Added (2026-08-25 repo-readiness round)

- **Workspace scaffolded end-to-end:** 51 library-crate stubs + `station-app`
  binary + std-only `xtask` guardrails (`check-layering`,
  `check-file-caps`) — build/clippy/fmt/test all green on day one.
- **Predecessor post-mortem** (`docs/process/predecessor-postmortem-chadvis-qt.md`):
  port-faithfully table + do-NOT-replicate list mapped to guardrails.
- **Prior-art knowledge base**: `docs/meta/reference-architecture-prior-art.md`
  (owner's own repos + termusic/projectM/Karaoke Mugen/UltraStar Play
  patterns) and `docs/meta/design-input-from-prior-takes.md` (mined ideas
  incl. deterministic export spec input, projectM landmine checklist, and the
  "everyone stalls at the GL bridge" stall-pattern warning).
- **Repo hygiene kit:** dependabot (cargo+actions), CODEOWNERS, SECURITY.md,
  CONTRIBUTING.md, markdown link-check workflow, cargo-deny advisories/bans
  split, nested AGENTS.md orientation files, gitignored
  `reference-scratchpad/` with ten cloned reference repos.

## [Unreleased-in-planning] — Phase 0

Workspace scaffold, foundation crates, app shell with theming, compositing
spike, xtask guardrails — see docs/04 Phase 0 and TODO.md.
