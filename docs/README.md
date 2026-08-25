# Documentation Wiki Hub

This directory is the master source of truth for the project. Docs are
organized **by purpose** into subdirectories; filenames keep their stable
numeric prefixes (`NN-name.md`) so existing "doc N §M" references remain
valid regardless of location.

**Numbering convention:** doc numbers are permanent. A reference like
"doc 06 §2.4" always means the file whose name starts with `06-`, wherever
it lives. Product specs take numbers 21+; process/meta docs stay unnumbered
in `meta/`.

## Start Here

| Doc | What it covers |
|---|---|
| [00 — Project Charter](product/00-project-charter.md) | Vision, product pillars, non-goals, target users |
| [04 — Phase Roadmap](product/04-phase-roadmap.md) | Phase plan with entry/exit criteria + maintainability gate |
| [03 — Agent Constitution](process/03-agent-constitution.md) | Binding operating rules for the AI developer agent |

## Product

| Doc | What it covers |
|---|---|
| [00 — Project Charter](product/00-project-charter.md) | Vision, product pillars, non-goals, target users |
| [04 — Phase Roadmap](product/04-phase-roadmap.md) | Phase plan with entry/exit criteria + maintainability gate |
| [99 — Ideas Backlog](product/99-ideas-backlog.md) | Parked future ideas (do not build ahead of phase) |

## Architecture

| Doc | What it covers |
|---|---|
| [01 — Architecture Overview](architecture/01-architecture-overview.md) | High-level architecture and crate layering |
| [02 — Workspace Layout](architecture/02-workspace-layout.md) | Cargo workspace layout — crates, dirs, templates |
| [17 — Glossary & Decisions Log](architecture/17-glossary-and-decisions-log.md) | Glossary & architecture decision log (ADRs) |

## Specifications

### Suno Integration — [`specs/suno-integration/`](specs/suno-integration/)

| Doc | What it covers |
|---|---|
| [05 — Auth & Multi-Account](specs/suno-integration/05-auth-and-multi-account.md) | Suno auth flows & multi-account design |
| [06 — Suno API Integration Contract](specs/suno-integration/06-suno-api-integration-contract.md) | Capture-driven Suno API endpoint contract |

### Data & Storage — [`specs/data-and-storage/`](specs/data-and-storage/)

| Doc | What it covers |
|---|---|
| [07 — Data Model & Storage Schema](specs/data-and-storage/07-data-model-and-storage-schema.md) | Data model & storage schema across all stores |

### UI/UX — [`specs/ui-ux/`](specs/ui-ux/)

| Doc | What it covers |
|---|---|
| [08 — UI/UX Design System](specs/ui-ux/08-ui-ux-design-system.md) | Glass theme tokens, themes, layout conventions |

### Audio Playback — [`specs/audio-playback/`](specs/audio-playback/)

| Doc | What it covers |
|---|---|
| [14 — Local Playback & Audio Engine](specs/audio-playback/14-local-playback-and-audio-engine.md) | Local playback & audio engine spec |

### Visuals & Video — [`specs/visuals-and-video/`](specs/visuals-and-video/)

| Doc | What it covers |
|---|---|
| [09 — Visualizer projectM Integration](specs/visuals-and-video/09-visualizer-projectm-integration.md) | projectM visualizer integration spec |
| [10 — Canvas & Keyframe System](specs/visuals-and-video/10-canvas-and-keyframe-system.md) | Overlay canvas, scene graph, keyframe animation spec |

### Automation — [`specs/automation/`](specs/automation/)

| Doc | What it covers |
|---|---|
| [13 — Automation Pipeline Engine](specs/automation/13-automation-pipeline-engine.md) | Batch automation pipeline engine spec |

### Extensibility — [`specs/extensibility/`](specs/extensibility/)

| Doc | What it covers |
|---|---|
| [11 — Plugin System (Deferred Design)](specs/extensibility/11-plugin-system-deferred-design.md) | Plugin system — deferred/stub design (Phase 8) |
| [12 — LLM & Image-Gen Provider Adapters](specs/extensibility/12-llm-and-image-gen-provider-adapters.md) | LLM text & image-gen provider adapter spec |

### Superseded — [`specs/superseded/`](specs/superseded/)

| Doc | What it covers |
|---|---|
| [15 — Multi-Account & Downloads Detail](specs/superseded/15-multi-account-and-downloads-detail.md) | SUPERSEDED — folded into docs 05 & 07; kept for history |

## Phases

Stage-by-stage implementation guides live in one directory per phase:
`phases/phase-N-<slug>/`. See [`phases/README.md`](phases/README.md) for the
convention and current status.

| Dir | Contents |
|---|---|
| [`phase-0-foundation/`](phases/phase-0-foundation/) | [20 — Phase 0 Kickoff Prompt](phases/phase-0-foundation/20-phase-0-kickoff-prompt.md) — literal paste-ready prompt to begin implementation |

## Process (binding rules)

| Doc | What it covers |
|---|---|
| [03 — Agent Constitution](process/03-agent-constitution.md) | Operating rules for the AI developer agent |
| [16 — Testing Strategy](process/16-testing-strategy.md) | Testing strategy across all crates |
| [18 — Codebase Health Guardrails](process/18-codebase-health-guardrails.md) | Health guardrails & Core Maintainability Gate |
| [19 — Git Workflow & Repo Governance](process/19-git-workflow-and-repo-governance.md) | Branching, worktrees, PR process, AGENTS.md/TODO.md conventions |

## Meta / Audit Archive

Unnumbered second-generation process documents live in [`meta/`](meta/) —
see its [README](meta/README.md): TODO task-state conventions, autonomous
session protocol, GitHub infrastructure plan, API ground-truth notes, and
the doc-set audit archive.

## Captures

Suno API captures (raw ground-truth evidence for the capture-driven contract,
doc 06) live in [`captures/`](captures/raw/recon-from-chadvis/README.md).

## Templates

Crate/file templates used when scaffolding new code live in
[`templates/`](templates/crate-stub-template/) — currently the
crate-stub template referenced by the workspace layout (doc 02).
