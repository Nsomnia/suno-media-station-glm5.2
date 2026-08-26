# AGENTS.md

If you are an AI agent picking up this repository — read this file first,
then follow its pointers. Do not start writing code before reading
[the Agent Constitution](docs/process/03-agent-constitution.md) and
[the Codebase Health Guardrails](docs/process/18-codebase-health-guardrails.md)
in full — they are **binding**. If any task instruction conflicts with them,
they win; flag the conflict to the human orchestrator.

## What This Project Is

A native Rust desktop companion for [Suno.com](https://suno.com) with
advanced library management, local playback parity, karaoke/visualizer music-video
creation, and batch automation features not available in Suno's official
clients. Full vision: [Project Charter (doc 00)](docs/product/00-project-charter.md).

It is a ground-up Rust rewrite of a working but structurally decayed C++/Qt
prototype (`~/Documents/chadvis-projectm-qt`). Before touching any subsystem,
read the [Predecessor Post-Mortem](docs/process/predecessor-postmortem-chadvis-qt.md):
it lists which prototype logic to port faithfully and which defects must never
be replicated.

## Who Works Here

**No human writes code.** AI coding agents do all implementation, review,
testing, documentation, and release work inside an opencode CLI +
oh-my-opencode orchestrator harness. The human is the orchestrator/reviewer
only. You are expected to self-perform every dev-team role (constitution §14).
The harness may delegate lanes to specialist sub-agents; write scopes must not
overlap between concurrent agents.

## Where Everything Is

| Path | What it is |
|---|---|
| [`docs/README.md`](docs/README.md) | **Wiki hub — start here** for the full doc map |
| [`TODO.md`](TODO.md) | Living task tracker; check before starting, update before finishing (constitution §10). State-mark spec: [`meta/TODO-task-state-conventions.md`](docs/meta/TODO-task-state-conventions.md) |
| `crates/` | Cargo workspace, laid out per [Workspace Layout (doc 02)](docs/architecture/02-workspace-layout.md). Names are deliberately verbose — trust paths to describe contents |
| `app/station-app/` | Binary crate; composition root only |
| `xtask/` | Repo guardrail tooling: `cargo xtask check-layering`, `check-file-caps` |
| `assets/` | Themes, projectM presets, icons, shaders |
| `reference-scratchpad/` | Gitignored clones of reference repos studied for prior art (constitution §3/§12); browse freely, never commit |
| `docs/captures/` | Raw Suno API traffic captures — the only valid API ground truth |

## Non-Negotiable Rules (full detail in doc 03 & doc 18)

1. **File size:** ~150–200 line soft cap, 300 line hard cap per `.rs` file.
   Split *before* continuing past it. Deep nesting + verbose names are correct style here.
2. **Prior-art-first:** search crates.io + `gh search repos`/`gh search code`
   before writing any non-trivial subsystem from scratch.
3. **Layering:** dependencies point only downward
   `ui → application-services → domain-stores → external-bridges → foundation`
   ([doc 01 §3](docs/architecture/01-architecture-overview.md)). Enforced by
   `cargo xtask check-layering` — zero exceptions.
4. **Definition of Done per code task:** builds warning-free · clippy clean ·
   minimal tests · size caps respected · docs updated in sync · `TODO.md`
   updated · PR opened via `gh pr create` ([doc 19](docs/process/19-git-workflow-and-repo-governance.md)).
   Docs-only chores may commit directly to `main`.
5. **Never invent a Suno API shape.** The contract
   ([doc 06](docs/specs/suno-integration/06-suno-api-integration-contract.md))
   is capture-driven. Missing capture → halt and request one via the
   `capture-needed` issue template.
6. **Architecture-level uncertainty** → present the human a short
   multiple-choice breakdown (options + tradeoffs + your recommendation),
   don't guess (constitution §9).

## Verification Commands (run before claiming anything done)

```sh
cargo build --workspace && cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check && cargo test --workspace
cargo xtask check-layering && cargo xtask check-file-caps
# `cargo deny check` removed from CI by orchestrator decision 2026-08-26
# (license gating deferred); deny.toml kept for optional local runs.
```

CI runs exactly these on every push/PR — keep `main` green at all times.

## Current Status & Autonomous Operation

- Exact live state: [`TODO.md`](TODO.md). Phase plan:
  [Phase Roadmap (doc 04)](docs/product/04-phase-roadmap.md).
- The orchestrator prompt *"continue working on this project"* has canonical
  semantics in [`meta/session-protocol-autonomous-operation.md`](docs/meta/session-protocol-autonomous-operation.md)
  (boot sequence, daemon loop, stop conditions). Read it before any autonomous session.
- Contradicting or extending any doc requires an ADR in
  [doc 17](docs/architecture/17-glossary-and-decisions-log.md).

## Subtree Orientation Files

Short `AGENTS.md` files exist where conventions genuinely differ:

- [`crates/external-bridges/AGENTS.md`](crates/external-bridges/AGENTS.md) — FFI/unsafe concentration rules
- [`crates/ui/AGENTS.md`](crates/ui/AGENTS.md) — UI-framework decision + screen-crate conventions
- [`xtask/AGENTS.md`](xtask/AGENTS.md) — guardrail-tooling expectations

Each points back to authoritative docs rather than restating them. When adding
a subtree with distinct conventions, add its orientation file in the same task.
