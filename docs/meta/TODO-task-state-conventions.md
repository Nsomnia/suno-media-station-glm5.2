# TODO Task-State Conventions

> **Last Updated:** 2026-08-22 · **Status:** Proposed — awaiting orchestrator decision (Audit item A-1)

This spec defines the single task-tracking system for `TODO.md` (repo root).
It replaces doc 19 §7's structure and adds the ownership + state-mark rules
the orchestrator requires.

## 1. Ownership Rules (the contract)

1. **The agent may freely:** add tasks, edit task text, reorder, move tasks
   between sections, and change any state mark *except* `[X]`.
2. **Only the human orchestrator may** remove or archive completed tasks.
   The agent must never delete a line — finished work graduates to
   `[x]` (awaiting verification) and stays visible until the user verifies it
   to be perfectly complete and removes it (or marks `[X]` to keep it locked
   in place).
3. Rationale: git history is the agent's memory, but the visible `TODO.md` is
   the user's audit surface; deleting rows destroys the user's ability to spot
   check "perfectly complete" claims without archaeology.

## 2. State Marks (ASCII)

| Mark | Meaning | Who may set/advance |
|---|---|---|
| `[ ]` | Not started | both |
| `[~]` | In progress (exactly one per session ideally) | agent |
| `[!]` | Blocked — must carry `— blocked on: <reason/owner>` | both |
| `[?]` | Requires user input/decision before work can proceed | both |
| `[x]` | Completed by agent, **awaiting user verification** | agent |
| `[X]` | Verified complete & perfect by the user (locked) | user only |
| `[-]` | Cancelled / won't do — kept for history, never deleted | user only (agent proposes via comment) |

GitHub won't render custom marks as checkboxes — that's fine; this file is
read by humans and agents, not rendered as UI.

## 3. File Structure

```markdown
# TODO

> Updated at start/end of every work session (doc 03 §10).
> Marks legend: [ ] todo · [~] doing · [!] blocked · [?] needs you ·
> [x] done-awaiting-your-verification · [X] verified-locked · [-] cancelled
> Current phase: <Phase N — name> (docs/product/04-phase-roadmap.md)

## In Progress
- [~] <task> — branch: `phase-N/slug` — <one-line status note>

## Up Next (current phase, priority order)
- [ ] <task> — <source: docs/04 §Phase-N exit criterion k / docs/02 ...>

## Blocked / Needs Human Input
- [!] <task> — blocked on: <e.g. Burp capture of auth flow, docs/06 §2.1>
- [?] <task> — question for orchestrator: <the actual question>

## Awaiting Your Verification (agent says done; user confirms → remove or [X])
- [x] <task> — PR #<n> — <how to verify in one line>

## Verified Complete (user-locked; pruned only at major milestones)
- [X] <task> — PR #<n>
```

## 4. Rules of Operation

1. Update at session start (reality-check states against `git log`) and at
   every task completion — part of Definition of Done (doc 03 §5).
2. Never let `In Progress` go stale: an abandoned task regresses honestly to
   `[ ]` with a note, or `[!]` if blocked.
3. Every `[!]`/`[?]` names its unblock owner and what exactly is needed —
   the user should resolve their whole section in one sitting without
   archaeology.
4. `Awaiting Your Verification` entries carry a one-line verify recipe
   (command to run / screen to look at). No recipe = not done (doc 16 §5
   spirit).
5. Task text mirrors doc references (`docs/04 Phase N`, `docs/02` crate path)
   so any fresh context can act on it without re-deriving scope.
6. This file never contains secrets, tokens, or capture payloads.

## 5. Migration Applied on Adoption

- Existing `TODO.md` rewritten into the five-section shape above.
- Doc 19 §7 replaced by a pointer here (audit B-4).
- AGENTS.md gains a one-line pointer to this spec (audit A-5).
