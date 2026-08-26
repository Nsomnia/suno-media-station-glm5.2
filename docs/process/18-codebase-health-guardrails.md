# Codebase Health Guardrails & Core Maintainability Gate

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Why This Doc Exists

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

## 2. Continuous Guardrails (checked throughout Phases 0-5, not just at the end)

### 2.1 Dependency Direction Enforcement
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

### 2.2 No Duplicate-Logic Crates
- Before creating a new crate, the agent must confirm (via a quick search of
  `docs/architecture/02-workspace-layout.md` + `rg`/grep across `crates/`) that an
  existing crate doesn't already own this responsibility. If two crates end
  up doing near-identical things (e.g. two different "resolve a track's
  audio source" implementations), that's a defect to consolidate
  immediately, not a "clean up later" item.
- Any new crate not already named in doc 02's tree requires a one-line
  addition to doc 02 in the same task that creates it — doc 02 must always
  accurately reflect the real crate tree. A crate existing in the repo but
  absent from doc 02 (or vice versa) is itself a Gate failure (§4).

### 2.3 Dead Code & Speculative Generality Ban
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

### 2.4 Complexity Budget, Not Just Line-Count Budget
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

### 2.5 Doc-Reality Sync Requirement
- Doc 03 §5's Definition of Done already requires updating relevant docs
  when reality diverges. This guardrail makes it explicit that **the docs
  in this doc-set are treated as tested artifacts, not write-once
  planning** — an inaccurate doc 02 tree, an out-of-date doc 04 exit-
  criteria claim, or an ADR log missing a real architectural decision that
  was made ad-hoc during implementation are all bugs, tracked the same as a
  code bug.

## 3. Periodic Self-Audit Ritual ("Senior Architect Pass")

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
   `docs/product/99-ideas-backlog.md` / a phase-specific "known debt" note — no
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

## 4. The Core Maintainability Gate (before Phase 6/7)

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
      recorded in `docs/product/99-ideas-backlog.md`.
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

## 5. Audit & Gate Log

*(Populated over time as phases complete — empty at doc creation.)*

| Phase | Date | Crate Count | Approx Total LOC | Layering Violations Found | Notes |
|---|---|---|---|---|---|
| 0 | 2026-08 | 51 (0 new) | ~3.1k (station-app) | 0 | See Phase 0 Audit Summary below |

### Phase 0 Audit Summary (2026-08)

- **Crates added:** 0 — the 51-crate skeleton pre-dated the phase; no new
  crates were created.
- **Implemented from stub:** `app-configuration-loader`,
  `structured-logging-and-tracing`, `design-tokens-theme-definitions`,
  `ui-shared-widget-library`, `ui-app-shell-and-navigation`, and the
  `station-app` binary (~3.1k LOC total). One deliberate stub remains:
  `error-and-result-conventions` (ADR-014).
- **Layering:** 0 violations; all 7 dependency edges point downward or
  laterally. Enforcement was upgraded from aspirational (doc 01 §3 used to
  call it a social/nice-to-have check) to a mechanical CI gate
  (`cargo xtask check-layering` in the `guardrails` job).
- **Consolidations required:** none. The dual `LoggingSettings` naming
  (config v1 section vs logging-crate runtime settings) was assessed as
  justified decoupling between layers — logged as a watch-item with rename
  candidate `LoggingSection` (see 99-ideas-backlog known debt).
- **Docs synced this round:** 4 stale crate READMEs refreshed,
  doc 01 §3 corrected, ADR entries added (incl. ADR-014), known-debt backlog
  appended.
- **Exit criteria:** independently re-verified — build/clippy/fmt/test/
  xtask guardrails green on the audit branch; compositing spike completed
  and ADR-013 accepted.
- **Gut-check (doc 02 + filenames alone):** a fresh agent context CAN
  navigate this phase's code without reading implementations — verbose
  crate/file names carry the structure.
- **Outstanding evidence:** manual human QA of the themed window is still
  pending; it is the only exit criterion not yet evidenced.
