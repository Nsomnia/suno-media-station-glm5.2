**Git Workflow & Repository Governance**

> **Last Updated:** 2024-01-01 · **Status:** Active

**1. Why This Matters Even With a Sole Human Orchestrator**

No human writes code here, but git history is still the project's
permanent audit trail, rollback mechanism, and the primary artifact the
human orchestrator actually reviews (rather than re-reading every doc on
every task). Good git hygiene is therefore not bureaucratic overhead — it
is the human's main lever for staying in control of an AI-authored
codebase without reading every line.

**2. Branching Model**

**Trunk-based, short-lived feature branches.** `main` is always expected
to pass the CI bar (doc 16 §4) — nothing is merged that doesn't build,
lint clean, and pass tests.

- **Branch naming:** `phase-<N>/<crate-or-feature-slug>`, mirroring doc
  04's phase numbers and doc 02's crate names for instant legibility —
  e.g. `phase-0/foundation-workspace-scaffold`,
  `phase-1/suno-auth-manual-token-paste`,
  `phase-5/canvas-keyframe-timeline-ui`. For cross-cutting work not tied
  to one crate, use `phase-<N>/<short-description>`.
- **One branch per meaningfully-scoped task**, not one giant branch per
  phase — small branches/PRs are easier for the human to spot-check and
  easier to revert individually if one is wrong, consistent with doc 03
  §6's small-diff preference.
- Branches are deleted after merge (locally and remotely) to keep the
  branch list a true reflection of active work, not history.

**3. Worktrees**

Use `git worktree` when working on more than one branch's checkout
simultaneously is useful (e.g. the orchestrator harness running a
parallel sub-task while another is mid-review, or keeping a long-lived
Phase-0-spike branch checked out separately from day-to-day Phase 1 work).

- **Convention:** worktrees live as sibling directories to the main repo
  checkout: `../Suno Station-worktrees/<branch-slug>/` (adjust `Suno Station` to the
  user's actual chosen project name). Never nest a worktree inside the
  main checkout's own tree.
- Not mandatory for every task — a normal `git checkout -b` is perfectly
  fine for straightforward sequential work. Reach for a worktree
  specifically when genuine parallelism is happening.

**4. Commit Conventions**

- **Conventional-commit-style prefixes:** `feat:`, `fix:`, `docs:`,
  `refactor:`, `test:`, `chore:`, `perf:` — enables clean changelog
  generation later and gives the human a fast scan of intent.
- **Small, single-concern commits** (doc 03 §6) — a commit should be
  independently understandable and, ideally, independently revertible.
- **Commit body** (not just the subject line) briefly states *why*, not
  just *what*, when the "why" isn't obvious from the diff alone — this is
  where a §9-style "inferred X, easily changed if wrong" note belongs when
  applicable.

**5. Pull Requests — Used Even for Sole-Maintainer Merges**

Every task-branch gets a PR opened via `gh pr create` before merging to
`main`, even though the human orchestrator is typically the only reviewer/
merger. This is deliberate, not theater:

- The **PR description is the structured self-review artifact** doc 03
  §4 requires (junior/senior/lead-pass notes) — it's the fastest way for
  the human to spot-check reasoning without reading the full diff first.
- It creates a natural checkpoint for the human to say "wait, no" before
  something lands on `main`, without slowing down agent throughput (the
  agent can keep working on the next branch while a PR awaits a glance).
- It gives future-agent-sessions (§10 in doc 03) a searchable history of
  *why* things were built a certain way, via `gh pr list`/`gh pr view`,
  which is cheaper to query than re-reading raw commit diffs.

**PR description template:**
```markdown
**What & Why**
<1-3 sentences>

**Docs Referenced**
<which docs/sections this work implements/follows — e.g. doc 04 Phase 1,
doc 07 §2>

**Self-Review Notes**
- Junior pass: <brief>
- Senior pass: <brief — edge cases considered, layering check per doc 01>
- Lead pass: <scope check per doc 04/doc 03 §8; any inferred decisions
  per doc 03 §9 flagged here>

**Testing**
<what tests were added/run, per doc 16>

**Docs Updated**
<list any docs touched to stay in sync, per doc 18 §2.5, or "none needed">
```

Merge strategy: squash-merge (keeps `main`'s history clean and matches the
"one branch = one logical change" convention above) unless a PR
genuinely contains multiple commits worth preserving individually (rare —
prefer splitting into multiple PRs instead when that's the case).

**6. `AGENTS.md` — Structure & Placement**

- **Root `AGENTS.md`** (template below) is the mandatory front-door file —
  every agent session should read it first, before diving into `docs/`.
  It stays short and points outward rather than duplicating content.
- **Nested `AGENTS.md` files** are optional and should exist only where a
  subtree has genuinely distinct conventions worth flagging close to the
  code (e.g. `crates/external-bridges/AGENTS.md` reminding that
  `unsafe`/FFI code is concentrated in specific crates per doc 09 §3, or
  `crates/ui/AGENTS.md` reminding of the UI-framework decision from doc
  01 §4). A nested file should be short (a paragraph or two) and always
  link back to the authoritative doc rather than restating it — avoid
  doc 18 §2.5-style drift risk by never having two files claim to be the
  source of truth for the same fact.

**7. `TODO.md` — Structure & Update Discipline**

Root-level, single file (not per-crate — a scattered multi-file TODO
system defeats the "one place to look" purpose). Structure:

```markdown
**TODO**

> Updated at the start/end of every work session — see docs/03 §10.
> Current phase: <Phase N — name> (docs/04-phase-roadmap.md)

**In Progress**
- [ ] <task> — branch: `phase-N/...` — <one-line status>

**Up Next (this phase)**
- [ ] <task, mirrors the phase doc's remaining exit-criteria items>

**Blocked / Needs Human Input**
- [ ] <task> — blocked on: <e.g. "needs a Burp capture of endpoint X, see doc 06">

**Recently Completed (rolling short list, not full history — git log is history)**
- [x] <task> — PR #<n>
```

Rules:
- **Never let "In Progress" contain a stale entry** — if a session ends
  mid-task, the entry stays with an honest status note; if it's actually
  done, it moves to "Recently Completed" with its PR link, immediately.
- "Blocked / Needs Human Input" is the primary channel for surfacing
  doc-03-§9-style stop-and-ask items and doc-06-style capture requests —
  the human should be able to open just this file and know exactly what's
  waiting on them.
- Keep "Recently Completed" short (e.g. last 10-15 items) — prune older
  entries; they remain fully discoverable via `git log`/merged PRs, this
  section is a quick-glance convenience only, not an archive.

**8. Tags & Releases (lightweight policy for now)**

Full release engineering is intentionally deferred (per original project
scope decision), but a minimal placeholder policy avoids total ambiguity:

- No tags/releases during Phase 0-1 (nothing user-facing to release yet).
- First tag (`v0.1.0`) is reasonable once Phase 2 (local playback parity)
  is functionally complete — the first point where "a person could
  actually use this for something" is true. Revisit/formalize actual
  semver policy and changelog generation at that point via an ADR (doc
  17), rather than deciding it speculatively now.

**9. `.gitignore` Baseline (Phase 0 task)**

Ensure standard Rust (`/target`, `Cargo.lock` handling — commit
`Cargo.lock` for a binary/application project, which this is, per Cargo's
own guidance for applications vs. libraries), OS-cruft (`.DS_Store`,
`Thumbs.db`), editor-cruft, and local secrets/config
(`*.local.toml`, any accidentally-created credential-dump files) are
git-ignored from the very first commit.
