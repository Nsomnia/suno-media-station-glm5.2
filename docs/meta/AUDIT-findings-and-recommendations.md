# Audit Findings & Recommendations — Original Doc Set

> **Last Updated:** 2026-08-22 · **Status:** Awaiting orchestrator decisions

Full review of `docs/00`–`20`, `99`, root `AGENTS.md`/`TODO.md`, `Cargo.toml`,
and templates, checked against (a) mechanical validity, (b) ground truth from
the predecessor prototype repo (`~/Documents/chadvis-projectm-qt/`), and
(c) the orchestrator's clarified product vision ("god mode" Suno front-end).

Each item ends with a decision box. Mark `[x]` to approve/reject, or fill in
Modify. A follow-up inference round applies all approved items in one pass.

---

## S — Scope Corrections (highest priority)

### S-1 · Charter misframes creation features as out of scope

The clarified vision is a **full-surface Suno.com desktop front-end**: every
capability the Suno API exposes is interactable, plus local-only add-ons.
Charter §3 currently says "does **not** implement music generation," which was
written to exclude *local* generation but reads as excluding *all* creation
features — contradicting "listening **and creation** front-end."

**Recommendation:** Amend doc 00: non-goal becomes "no *local* AI music
generation"; add pillar "**Full-Surface Client** — drive every user-facing
Suno capability (library, playlists, personas, generation via Suno's own
endpoints, uploads, account surfaces) from one desktop app, managed better
than the official clients." Generation always happens server-side at Suno;
Station is the better remote control.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### S-2 · Doc 06 endpoint categories miss entire API surfaces

Prototype recon shows surface areas absent from doc 06 §2: generation
(`POST /api/generate/v2-web/`, `/api/generate/lyrics/`), playlists CRUD,
personas/custom-models, audio/video uploads, trash/restore, billing/credits
read-back, omnisearch. Full seeding map:
`suno-api-ground-truth-from-prototype.md` §3.

**Recommendation:** Add doc 06 categories 2.8–2.15, each seeded with
provenance-tagged leads (status `LEAD — needs confirming capture`).

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### S-3 · Roadmap has no Creation phase

Doc 04 covers listening (1–2), karaoke data (3), video (4–5), LLM-assist (6),
automation (7) — but never *driving Suno generation* from the app, despite it
being core to "god mode" and the strongest automation story (auto-generate
songs → auto-render videos, end-to-end pipeline).

**Recommendation:** Add **"Phase 6b — Suno Creation Studio"** unlocked at the
Core Maintainability Gate alongside Phases 6/7 (same ADR-008 pattern):
generation submit/poll (`/api/generate/*`), Suno-native lyric tools, persona
picker, upload-a-take → create-song flow (`/api/uploads/audio/*`), credit
balance display before spends. A "generate tracks" pipeline step becomes a
Phase 7 additive extension.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### S-4 · Listening-parity gaps vs prototype feature set

The shipped prototype had playlists, liked/trash views, shuffle/repeat modes,
and a heuristic lyric aligner used when no timed lyrics exist. Current docs
name none of these except obliquely.

**Recommendation:** (a) Name shuffle/repeat/queue explicitly in Phase 2 exit
criteria; (b) add playlist browsing/management to Phase 1 scope (endpoints
exist; ground-truth doc §3); (c) park "heuristic aligner fallback (cheaper
than Whisper when plain lyrics exist but no timing)" in doc 99.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### S-5 · Batch rendering ignores hardware-accelerated encode

Phase 7's thousand-scale ambition needs HW encoders (VideoToolbox/NVENC/QSV).
The prototype recorded via FFmpeg with HW accel; current docs never mention
encoder selection.

**Recommendation:** Doc 13 export settings gain an `encoder` field
(`h264_videotoolbox` / `h264_nvenc` / `h264_qsv` / `libx264` fallback chain);
doc 09 notes ffmpeg encoder availability must be probed at startup.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

---

## E — Hard Errors (mechanically broken today)

| ID | Location | Problem | Fix |
|---|---|---|---|
| E-1 | Root `Cargo.toml` | Markdown prose inside `[workspace.dependencies]` → invalid TOML | Move notes to `#` comments |
| E-2 | Doc 02, Cargo.toml | `"app/Suno Station-app"` — spaces invalid in Cargo package names, hostile to tooling | Rename to `app/station-app` (pkg `station-app`) across docs |
| E-3 | Crate-stub template `Cargo.toml` | Markdown prose inside `[dependencies]` → broken manifests from template | Convert to `#` comments |
| E-4 | Doc 06 §1/§4 | Corrupted fences `'''json … '''`; stray CJK char ("Unknown/未-mapped"); malformed `- ### 2.x` list-headings | Repair all three |
| E-5 | Doc 09 §5 | Self-reference "see doc 09b/§7" — no such doc | Point to same doc's §7 |
| E-6 | Doc 02 | Two tree entries ending `/.`; intro typos (`strucutre`, `LLm`, fragments) | Fix |
| E-7 | Doc 16 §2 | Env var `Suno Station_TEST_REQUIRES_LOCAL_TOOLING` contains spaces → unusable | `STATION_TEST_REQUIRES_LOCAL_TOOLING=1` |
| E-8 | Docs 19, 20 | Placeholder `Last Updated: 2024-01-01`; header convention (doc 03 §13) applied nowhere else | Apply repo-wide per T-2 |
| E-9 | `README.md` | Junk tagline above doc index; index lacks `docs/meta/` pointer | Remove; add pointer |
| E-10 | Root `Cargo.toml` | `edition = "2021"` stale for an Aug-2026 greenfield scaffold | `edition = "2024"` |
| E-11 | `.gitignore` | Missing doc 19 §9 items (`.DS_Store`, `Thumbs.db`, `*.local.toml`, editor dirs) | Add |

**Decision (E block):** [ ] Approve all   [ ] Modify per-item: ________

---

## C — Guesses Contradicted by Ground Truth or Engineering Reality

### C-1 · Auth is Clerk, not a generic "refresh cookie" (HIGH)

Docs 05/06 model auth as bearer + opaque refresh cookie. Prototype recon shows:
Clerk at `auth.suno.com/v1`; long-lived `__session` cookie; JWT obtained via
`POST /v1/client/sessions/{sid}/tokens` (≈1 h expiry, re-exchangeable while
the session cookie lives); mandatory browser-like headers (`Device-Id`,
`Browser-Token`, `Origin`, `Referer`, UA). Detail: ground-truth doc §2.

**Recommendation:** Rewrite doc 05 §4 (refresh strategy → Clerk session-token
exchange) and §2 (manual paste captures `__client` + `__session`, then performs
the exchange itself). Seed doc 06 §2.1 accordingly. Capture-driven rule stays:
these are leads until one fresh capture confirms each.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### C-2 · Word-level timed lyrics confirmed to exist (HIGH)

Doc 04 Phase 3 hedges "if Suno truly has no such endpoint." Ground truth:
`GET /api/gen/{id}/aligned_lyrics/v2/` returns word-level timing
(`{aligned_lyrics:[{word,start_time,end_time,line_index?}], language?, status}`).

**Recommendation:** Seed doc 06 §2.4 as a lead; replace the "may not exist"
hedge with "known from recon — confirm shape via fresh capture." Doc 07 §5's
schema already fits word/line levels unchanged.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### C-3 · Glassmorphism feasibility in egui is unproven (RISK)

Doc 08's glass recipe assumes backdrop blur; egui has no native backdrop-blur.
True glass needs a custom pass or falls back to flat translucency. As written,
the theme system could hit a wall mid-Phase-0 with no fallback defined.

**Recommendation:** Make doc 08 §3's glass panel recipe explicitly tiered:
Tier A true backdrop-blur (if the Phase 0 spike proves a workable technique),
Tier B translucent-fill-without-blur (always achievable). Theme tokens already
model alpha separately, so Tier B is a token-value change, not a redesign.
Record outcome in the Phase 0 ADR.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### C-4 · projectM↔wgpu texture sharing is the riskiest unknown; glow path unstudied (RISK)

Doc 01 §4 leans wgpu-backed egui and assumes shared textures with projectM's
GL output are feasible. GL-to-wgpu interop is genuinely hard; egui also runs
on `glow` (pure GL), where projectM frames live in the *same* context/API —
dramatically simpler compositing (the prototype did exactly this pattern with
Qt/GL FBOs).

**Recommendation:** Widen the Phase 0 spike mandate: evaluate
(egui+glow, same-GL-context compositing) as first-class alternative to
(egui+wgpu, texture interop), pick by evidence, record ADR. Do not pre-commit
wgpu in doc 01 §4 before the spike.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### C-5 · Smaller technical-hygiene notes (LOW, batch)

- Doc 07: cursor-based feed sync (feed v3) means cache store should persist
  per-account sync cursors — add one column note now, cheap later.
- Doc 12: `async_trait` fine, but note Rust native AFIT traits are standard
  since 1.75 — prefer plain `impl Trait`/AFIT unless object-safety needed;
  decide at implementation.
- Doc 14: add "probe actual downloaded format(s) in Phase 1 before locking
  decode-profile assumptions" (already half-stated).
- whisper-rs / keyring maintenance re-check notes belong at phase start (docs
  already genericize this; no change).

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

---

## B — Bloat / Merge Candidates

| ID | Item | Recommendation |
|---|---|---|
| B-1 | Doc 15 duplicates docs 05+07 heavily | Fold unique bits (account-lifecycle state diagram; download retry/resume detail) into 05/07; mark 15 `Status: Superseded` |
| B-2 | Doc 14 §5 restates doc 04 Phase 9 forward-notes | Trim to a pointer paragraph |
| B-3 | Cross-reference density ("per doc X §Y" everywhere) | Keep load-bearing refs; delete restatements that re-explain instead of point |
| B-4 | Doc 19 §7 TODO structure conflicts with new state-mark spec | Replace section with pointer to `docs/meta/TODO-task-state-conventions.md` |
| B-5 | Doc 11 kept stub-only (correct) | No change — listed to confirm it was considered, not overlooked |

**Decision (B block):** [ ] Approve all   [ ] Modify per-item: ________

---

## T — Structure & Consistency

### T-1 · Normalize heading markup repo-wide

Docs 00/01/02 use real ATX headings (`#`/`##`); the rest use bold pseudo-
headings (`**1. Vision**`). Mixed style breaks anchors/TOC tooling and looks
unpolished.

**Recommendation:** Convert every numbered doc to ATX headings in one
mechanical pass.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### T-2 · Apply the Last-Updated header convention uniformly

Doc 03 §13 mandates metadata headers; only docs 19/20 carry them (with fake
dates). Either apply everywhere or drop the convention.

**Recommendation:** Apply everywhere during the T-1 pass (real dates),
exempting `AGENTS.md`/`TODO.md` as §13 already does.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### T-3 · Numbering policy going forward

Keep existing numbers stable (all cross-references survive). New process/
meta docs live in `docs/meta/` without numbers. New product specs take next
free numbers (21+) and get README-index rows.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

### T-4 · Unspecified surfaces worth one short spec each

Global search (mentioned doc 08 §5 but never specified) and the settings-
screen inventory (scattered across five docs) each deserve a compact section
in doc 08 rather than new docs.

**Decision:** [ ] Approve   [ ] Reject   [ ] Modify: ________

---

## A — Autonomy & Infrastructure Gaps

Detailed specs live in sibling meta docs; items here only request approval to
adopt them:

- **A-1** Adopt `TODO-task-state-conventions.md`: ASCII state marks
  (`[ ]` `[~]` `[x]` `[X]` `[!]` `[?]` `[-]`), agent-may-mutate/user-only-
  removes ownership rules, rewrite of `TODO.md` and doc 19 §7.
- **A-2** Adopt `session-protocol-autonomous-operation.md`: canonical boot
  sequence behind "continue working on this project", daemon loop with stop
  conditions, stall log (`.agent/stalled_task_log.md`), offline mock cache
  (`.agent/mocks/`), context-reset resume checklist.
- **A-3** Seed doc 06 from prototype ground truth per
  `suno-api-ground-truth-from-prototype.md` (provenance-tiered leads).
- **A-4** Adopt `github-infrastructure-plan.md`: CI workflow (fmt/clippy/test/
  layering/file-cap), PR + issue templates incl. a capture-request template,
  branch protection, milestones/labels, `rust-toolchain.toml`, `deny.toml`,
  `.editorconfig`.
- **A-5** AGENTS.md amendments: add TODO legend pointer + session-protocol
  pointer; correct binary name (E-2); add explicit exemption that docs-only
  chores may commit directly to `main` without a PR (matches observed planning
  practice; code changes keep the PR requirement).

**Decision (A block):** [ ] Approve all   [ ] Modify per-item: ________
