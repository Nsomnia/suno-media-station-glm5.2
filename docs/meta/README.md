# Meta Docs — Audit Round (August 2026)

This directory holds the **second-generation process docs**: a full audit of the
original doc set (`docs/00`–`docs/20`, `docs/99`, root `AGENTS.md`/`TODO.md`)
plus new specs written to make fully-autonomous "continue working on this
project" operation reliable.

## Contents

| Doc | Purpose |
|---|---|
| `AUDIT-findings-and-recommendations.md` | Master findings list: hard errors, wrong guesses, scope corrections, bloat, restructure plan. Every finding has a decision checkbox for the human orchestrator. |
| `TODO-task-state-conventions.md` | Proposed ASCII state-mark system for `TODO.md` + ownership rules (agent mutates freely; only the user removes finished tasks). |
| `session-protocol-autonomous-operation.md` | The canonical boot sequence + daemon-loop protocol behind the "continue working on this project" prompt. |
| `suno-api-ground-truth-from-prototype.md` | Suno API evidence recovered from `~/Documents/chadvis-projectm-qt/` and the plan to seed doc 06 with provenance-tagged entries. |
| `github-infrastructure-plan.md` | CI workflows, PR/issue templates, branch protection, toolchain pins — the zero-to-v1 repo-ready checklist. |

## How decisions get recorded

Every actionable item in the audit doc ends with:

```
**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: <notes>
```

The human orchestrator edits these checkboxes directly (or replies in chat
with item IDs). A follow-up inference round then applies all approved changes
to the main doc set in one pass, updating cross-references and the README
index as needed.

## Status

- [x] Audit round authored, decisions approved by orchestrator
- [x] Approved changes applied to main doc set
- [ ] This directory reduced to living process docs (audit file archived)
