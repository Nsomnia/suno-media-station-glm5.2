# suno-media-station-glm5.2

## Doc Index

| # | Doc | What it covers |
|---|---|---|
| 00 | `docs/00-project-charter.md` | Vision, product pillars, non-goals, target users |
| 01 | `docs/01-architecture-overview.md` | High-level architecture and crate layering |
| 02 | `docs/02-workspace-layout.md` | Cargo workspace layout — crates, dirs, templates |
| 03 | `docs/03-agent-constitution.md` | Operating rules for the AI developer agent |
| 04 | `docs/04-phase-roadmap.md` | Phase plan with entry/exit criteria + maintainability gate |
| 05 | `docs/05-auth-and-multi-account.md` | Suno auth flows & multi-account design |
| 06 | `docs/06-suno-api-integration-contract.md` | Capture-driven Suno API endpoint contract |
| 07 | `docs/07-data-model-and-storage-schema.md` | Data model & storage schema across all stores |
| 08 | `docs/08-ui-ux-design-system.md` | UI/UX glass theme tokens, themes, layout conventions |
| 09 | `docs/09-visualizer-projectm-integration.md` | projectM visualizer integration spec |
| 10 | `docs/10-canvas-and-keyframe-system.md` | Overlay canvas, scene graph, keyframe animation spec |
| 11 | `docs/11-plugin-system-deferred-design.md` | Plugin system — deferred/stub design (Phase 8) |
| 12 | `docs/12-llm-and-image-gen-provider-adapters.md` | LLM text & image-gen provider adapter spec |
| 13 | `docs/13-automation-pipeline-engine.md` | Batch automation pipeline engine spec |
| 14 | `docs/14-local-playback-and-audio-engine.md` | Local playback & audio engine spec |
| 15 | `docs/15-multi-account-and-downloads-detail.md` | Multi-account & download manager detail — SUPERSEDED, folded into docs 05 & 07 |
| 16 | `docs/16-testing-strategy.md` | Testing strategy across all crates |
| 17 | `docs/17-glossary-and-decisions-log.md` | Glossary & architecture decision log (ADRs) |
| 18 | `docs/18-codebase-health-guardrails.md` | Codebase health guardrails & Core Maintainability Gate |
| 19 | `docs/19-git-workflow-and-repo-governance.md` | Branching, worktrees, PR process, AGENTS.md/TODO.md conventions |
| 20 | `docs/20-phase-0-kickoff-prompt.md` | Literal paste-ready prompt to begin implementation |
| 99 | `docs/99-ideas-backlog.md` | Parked future ideas (do not build ahead of phase) |

Also at repo root:
- **`AGENTS.md`** — front-door orientation file for any agent session (read first)
- **`TODO.md`** — living task tracker (must always be current — see doc 03 §10)

**Process docs:** `docs/meta/` holds unnumbered process/meta documents
(TODO task-state conventions, autonomous session protocol, doc-set audit
archive). Numbering policy: product specs take numbers 21+, process docs
stay unnumbered in `docs/meta/`.

**Changelog:** `CHANGELOG.md` follows the Keep-a-Changelog format and is
maintained alongside frequent commits.
