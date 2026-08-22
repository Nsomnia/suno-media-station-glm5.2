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
