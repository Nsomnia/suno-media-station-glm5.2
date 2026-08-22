# Session Protocol — Autonomous Operation ("continue working on this project")

> **Last Updated:** 2026-08-22 · **Status:** Proposed — awaiting orchestrator decision (Audit item A-2)

The goal: the orchestrator can type **"continue working on this project"**
with zero other context, and any capable model produces correct, safe,
unattended progress for hours. This protocol is that prompt's semantics.
It borrows the proven daemon-loop patterns from the predecessor project's
`GENERAL_LLM_STARTING_PROMPT.md`, tightened by this repo's constitution
(doc 03) which always wins on conflict.

## 1. Boot Sequence (every fresh session)

1. Read, in order: `AGENTS.md` → `TODO.md` → `docs/03-agent-constitution.md`
   (if not already internalized this session) → active phase section of
   `docs/04` → any doc a pending task references.
2. Reality-check: `git status` clean? `git log --oneline -15` matches
   TODO's `Awaiting Verification` claims? Workspace builds?
   (`cargo build --workspace` once code exists.)
3. Reconcile: fix any TODO state that drifted from reality *honestly*
   (regress `[~]`→`[ ]` with note; never mark anything `[x]` that isn't).
4. If `.agent/stalled_task_log.md` exists: read it first; do not reopen a
   stalled task without a new approach.
5. Pick work: highest item in `Up Next` that is neither `[!]` nor `[?]`;
   tie-break toward items unblocking others.

## 2. The Daemon Loop

```
loop:
  task = pick_next_actionable()
  branch per docs/19 §2 (or direct commit if docs-only chore, audit A-5)
  implement → test → self-review passes (doc 03 §4)
  DoD checklist (doc 03 §5) incl. TODO update + PR (code) / commit (docs)
  immediately begin next task — no pause for permission between tasks
```

Momentum rules (from the prototype daemon directive, adopted):

- **Do not stop to ask** whether to continue to the next task. Completing one
  task and starting the next is the default heartbeat.
- **Prolific commits**: every coherent sub-step that survives build+tests gets
  committed with a conventional message (docs/19 §4). Context loss is the
  enemy; git is salvation.
- **Push cadence**: push after every merged PR / completed task so remote
  state hardens against local failure.

## 3. Absolute Stop Conditions (the only reasons to halt)

1. **Human-vision QA required**: the exit criterion can only be verified by
   eyes on a real screen (e.g. "visuals look correct") — post the exact
   manual-QA recipe into TODO `[?]` and continue with orthogonal work.
2. **Interactive auth impossible to pipe**: e.g. initial `gh` login, OS
   keyring unlock — request once via TODO `[!]`, move on.
3. **Architecture-level ambiguity** per doc 03 §9 — present the multiple-
   choice breakdown, then continue on independent tasks while awaiting the
   answer (never sit idle).
4. **Missing Suno capture** per doc 03 §7/doc 06 — log precisely which
   endpoint/action needs capturing, continue elsewhere.
5. **Intractable loop**: same fatal error persists after ~7 genuinely
   distinct approaches — invoke §4 stall protocol instead of halting outright.

## 4. Stall Protocol (fail-safe, keeps the daemon moving)

After 3 distinct failed approaches on one task:

1. Write `.agent/stalled_task_log.md`: symptom, error traces, approaches
   tried, current best hypothesis, what evidence would disambiguate.
2. Mark the task `[!]` in TODO referencing the log path.
3. Pivot to an orthogonal actionable task and keep moving.
4. Any session may retry a stalled task only with a materially different
   approach (new prior-art found, new dependency, changed plan).

## 5. Offline Resilience

- Network-dependent work blocked? Generate fixture mocks from existing
  sanitized captures/recon into `.agent/mocks/` and build against those —
  parsing/logic layers don't need live endpoints (doc 16 §2 already bans live
  calls in tests anyway).
- Tool missing? Install via brew per doc 03 §12 rather than stopping.

## 6. Context-Reset Resume Checklist

A session that dies mid-task must leave enough state that the next boot
resumes in under two minutes:

- [x] TODO row updated before every commit (state + one-line status)
- [x] Work committed incrementally (nothing lives only in the chat buffer)
- [x] Mid-thought design decisions go into the PR description or a scratch
      note file, not chat memory
- [x] If interrupted mid-edit: `git status` artifacts are either finished or
      stashed with a naming convention (`WIP-<task-slug>`), noted in TODO

## 7. Relationship to the Constitution

This protocol operationalizes doc 03; where they appear to conflict, doc 03
wins (ask-vs-infer thresholds, capture-driven API rule, scope discipline).
What this protocol adds is the *momentum* layer: never-idle scheduling, stall
handling, and resume hygiene — the pieces the constitution deliberately left
unspecified.
