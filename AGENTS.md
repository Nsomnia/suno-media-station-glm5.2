## AGENTS.md

If you are an AI agent picking up this repository — read this file first,
then follow its pointers. Do not start writing code before reading
`docs/03-agent-constitution.md` and `docs/18-codebase-health-guardrails.md`
in full — they are binding.

## What This Project Is

See `docs/00-project-charter.md` for the full vision. One-line summary: a
native Rust desktop companion for Suno.com with advanced library, playback,
karaoke/visualizer video, and automation features not available in Suno's
official clients.

## Who's Building This

An AI coding agent (primary: GLM-5.2, occasional Google-model assist for
UI/UX-flavored subtasks), operating inside an opencode CLI + oh-my-openagent
orchestrator harness. **No human writes code on this project** — the human
is the orchestrator/reviewer only. You are expected to act as the entire
development team (see `docs/03-agent-constitution.md` §14).

## Where Everything Is

- **`docs/` — the master source of truth.** Read `README.md`'s doc index
  table first to know which doc covers what. Every doc is numbered and
  cross-referenced; do not contradict a doc without recording the change
  as an ADR in `docs/17-glossary-and-decisions-log.md`.
- **`TODO.md`** (repo root) — current task state. Check this before
  starting work, update it before finishing (mandatory, see
  `docs/03-agent-constitution.md` §10). Tasks use ASCII state marks
  (`[ ]` todo · `[~]` doing · `[!]` blocked · `[?]` needs you · `[x]`
  done-awaiting-your-verification · `[X]` verified-locked · `[-]`
  cancelled) — spec: `docs/meta/TODO-task-state-conventions.md`. The
  agent may freely modify TODO entries but must never delete completed
  rows — only the user removes finished tasks.
- **`crates/`** — the Cargo workspace, laid out per
  `docs/02-workspace-layout.md`. Directory/file names are deliberately
  verbose — trust them to describe their own contents.
- **`app/station-app/`** — the binary crate (composition root).
- **`docs/meta/`** — second-generation process docs (TODO state-mark spec,
  autonomous session protocol, audit archive).

## Non-Negotiable Rules (full detail in doc 03 & doc 18)

1. Files: ~150-200 line soft cap, 300 line hard cap.
2. Check crates.io + `gh search` for existing solutions before writing any
   non-trivial subsystem from scratch.
3. Respect the crate-layering direction (doc 01 §3) — no exceptions.
4. Every CODE task ends with: tests passing, docs in sync, `TODO.md`
   updated, a PR opened via `gh pr create` (see `docs/19-git-workflow-
   and-repo-governance.md`). Docs-only chores may commit directly to
   main without a PR.
5. Never invent a Suno API endpoint shape — `docs/06-suno-api-integration-
   contract.md` is capture-driven; halt and ask for a capture if one's
   missing.
6. When uncertain on anything architecture-level, present the human a
   short multiple-choice breakdown rather than guessing (doc 03 §9).

## Current Status

See `TODO.md` for exact current state. High-level: see `docs/04-phase-
roadmap.md` for the phase plan and which phase is currently active.

## Autonomous Operation

The orchestrator's "continue working on this project" prompt has canonical
semantics defined in `docs/meta/session-protocol-autonomous-operation.md`
(daemon loop, stop conditions). Read that doc before picking up work in an
autonomous session.
