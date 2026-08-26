# Automation Pipeline Engine Spec

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. Scope

Covers `automation-pipeline-definition-store` (doc 07 §7 schema),
`automation-batch-render-orchestrator`, and `ui-screen-automation-pipeline-
builder`. This is Phase 7 per doc 04, gated behind the Core Maintainability
Gate (doc 18 §4) same as doc 12.

## 2. Core Principle: Reuse, Never Reimplement, the Single-Track Path

The single most important architectural rule for this subsystem (stated in
doc 01 §7 and doc 04 Phase 7 and repeated here because it is the guardrail
most likely to be violated under time pressure): **the orchestrator fans
out calls to the exact same `single-track-visualizer-render-service` +
`canvas-overlay-compositing-service` + `karaoke-lyric-timing-resolution-
service` used by the one-off manual render path.** It does not contain its
own parallel rendering logic. If a bug is fixed or a feature added to the
single-track render path, automation gets it for free, automatically,
because it's the same code — this is both a correctness guarantee (what
you designed manually is what you get in bulk) and a direct defense
against doc 18's duplicate-logic guardrail (§2.2).

## 3. Pipeline Definition Model (recap + detail beyond doc 07 §7)

- **Input selector** — one of:
  - `explicit_ids`: a fixed list of `remote_track_id`s.
  - `tag`: all tracks (for the currently active account, or a specified
    account) carrying a given tag.
  - `all_in_account`: every track in an account's synced library.
  - (Additive later: saved smart-filters/search queries, once the library
    browser's filter capabilities, doc 04 Phase 1, are rich enough to
    reuse their filter representation here rather than inventing a
    second one — check for reuse opportunity when this phase begins,
    per doc 18 §2.2.)
- **Scene reference** — a single `scene_id` (doc 07 §6). All items in a
  run share one scene/template — this is intentional (that's what "brand
  consistency at scale" means); per-item scene overrides are explicitly
  out of scope for v1 (note as backlog if requested).
- **Lyric source policy** — `remote_preferred` (use remote timing if it
  exists for that track, else run Whisper), `whisper_preferred` (always
  run Whisper, ignore remote timing even if present — for consistency
  across a batch where the user doesn't trust remote timing quality),
  `remote_only` (skip/flag tracks lacking remote timing rather than
  running Whisper at all — for speed-prioritizing batches).
- **Export settings** — resolution, frame rate, codec/container (sensible
  defaults, e.g. 1080p/30fps/H.264+AAC in an MP4 container, are fine for
  v1; expose as configurable, don't hardcode invisibly), an **`encoder`
  field** using the same hardware-acceleration fallback chain as doc 09 §5
  (`videotoolbox` / `nvenc` / `qsv` / `libx264`, probed at startup with CPU
  fallback), and an output path template supporting at least
  `{track_title}`, `{account_name}`, `{date}` placeholder tokens for file
  naming/routing. The encoder choice is surfaced in the pipeline builder UI
  with a sensible default (best available HW encoder, else `libx264`).

## 4. Execution Model

```
pipeline_runs (1) ──< pipeline_run_items (many)
```

- Starting a run: resolve the input selector into a concrete track list
  **at start time** (snapshot it into `pipeline_run_items` rows
  immediately) — do NOT re-evaluate a dynamic selector (e.g. "all tracks
  tagged X") mid-run if the user adds a new matching track while a run is
  in progress; that's a separate, deliberate "run again" action, not
  silent scope-growth of an in-flight run.
- A worker pool (configurable concurrency limit — default conservatively
  low, e.g. 2-4 concurrent renders, since visualizer rendering + ffmpeg
  encoding is resource-intensive; expose as a setting for users with
  beefier machines) pulls `pending` items, marks `in_progress`, invokes the
  single-track render service (per §2), and marks `completed`/`failed`
  with `error_message` on completion.
- **Per-item failure isolation:** one item failing (bad audio file, missing
  lyrics with `remote_only` policy, ffmpeg error) logs the error against
  that `pipeline_run_item` and continues the run — never aborts the whole
  batch. A run's final status is `completed` if all items reached a
  terminal state (`completed` or `failed`), regardless of how many
  individually failed; the UI clearly surfaces a per-run success/failure
  count summary.
- **Resumability:** if the app crashes or is closed mid-run, on next
  launch the orchestrator finds any `pipeline_runs` row still `running`
  with `pending`/`in_progress` items, and offers to resume (re-queue
  `in_progress` items as `pending` first, since we can't assume they
  completed) — never silently auto-resumes without the user seeing a
  "resume interrupted run?" prompt, since a long-crashed run's environment
  (e.g. a since-removed local file) may no longer be valid.

## 5. Progress & Monitoring UI

- A run-monitor view showing: overall progress (X/N complete), a live list
  of item statuses, and the ability to cancel a run (in-progress items
  finish or are killed cleanly — killing an in-flight ffmpeg process
  cleanly, not orphaning it, is a specific implementation requirement).
- Completed runs remain browsable in history (via `pipeline_runs` rows)
  with links to output files, so a user can revisit "what did that batch
  from last week produce."

## 6. Performance Note (ties to doc 16 §6)

Phase 7's exit criteria (doc 04) explicitly requires a real 20-50 track
test run as the practical performance validation gate before claiming
"thousands of tracks" scale — resource usage (CPU/GPU/disk I/O for
concurrent visualizer-render + ffmpeg-encode workers) should be observed
and recorded in this phase's own audit notes (doc 18 §3 style) as the
baseline informing the default concurrency setting (§4) and any future
"how many tracks can this realistically batch overnight on typical
hardware" guidance surfaced to users.

## 7. Non-Goals (recap from doc 04, restated for emphasis)

- No plugin-authored pipeline steps until Phase 8 (doc 11).
- No per-item scene override in v1.
- No distributed/multi-machine rendering — single local machine, worker-
  pool concurrency only.
