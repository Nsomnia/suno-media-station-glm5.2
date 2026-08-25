# Data Model & Storage Schema

> **Last Updated:** 2026-08-25 · **Status:** Active

## 1. General Conventions (apply to every store crate)

- **Engine:** SQLite via `sqlx` (compile-time checked queries where
  practical). Each store crate owns exactly one logical set of tables and
  never queries another store's tables directly — cross-store composition
  happens in the `application-services` layer, not via SQL joins across
  store boundaries. (A single physical `.sqlite` file is fine for simplicity;
  the boundary is logical/crate-level, not necessarily one-file-per-store.)
- **Primary keys:** UUID v4 (`TEXT` column, stored as canonical hyphenated
  string) for all locally-originated entities. For entities mirroring a Suno
  remote resource, store Suno's own remote ID as `remote_id TEXT UNIQUE` in
  addition to a local UUID primary key — never assume Suno's ID scheme
  (could be UUID, could be a short slug, could change) until confirmed via
  doc 06 capture.
- **Timestamps:** `created_at`, `updated_at` as `TEXT` ISO-8601 UTC
  (`chrono::DateTime<Utc>` via serde), on every table. Never rely on SQLite's
  own `CURRENT_TIMESTAMP` for anything the app logic reasons about — set
  explicitly from Rust so test determinism (`deterministic-test-clock-and-
  ids`) works.
- **Migrations:** `sqlx::migrate!` with numbered migration files per store
  crate at `<crate>/migrations/NNNN_description.sql`. Migrations are
  forward-only; a schema fix is a new migration, not an edited old one
  (except during Phase 0-1 before any real user data exists in the wild).
- **Soft delete where it matters:** entities a user might want to "undo"
  removing (e.g. a downloaded track's local record, a pipeline definition)
  get a `deleted_at TEXT NULL` column rather than a hard `DELETE`, at least
  for the first version of each store — hard-delete/vacuum can be a later
  optimization once retention policy is actually decided.
- **JSON columns are allowed** for genuinely flexible/nested data (scene
  graphs, keyframe tracks, pipeline step configs) — store as `TEXT`
  containing serialized JSON via `serde_json`, with a `schema_version
  INTEGER` sibling column so future migrations can transform old JSON blobs
  rather than being stuck. Do NOT use JSON columns as a lazy substitute for
  proper relational columns on data that's actually structured/queried
  (e.g., don't JSON-blob an account's display name).

## 2. `account-profile-store`

```sql
CREATE TABLE accounts (
    id              TEXT PRIMARY KEY,       -- local UUID
    display_name    TEXT NOT NULL,
    auth_method     TEXT NOT NULL,          -- 'manual_paste' | 'embedded_webview' | 'oauth_loopback'
    keyring_ref     TEXT NOT NULL,          -- opaque handle into os-keyring-secret-storage
    suno_user_id    TEXT,                   -- remote user id once known (from profile endpoint)
    avatar_url      TEXT,
    is_active       INTEGER NOT NULL DEFAULT 0,  -- exactly one row = 1 at a time (app-enforced)
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    deleted_at      TEXT
);
```

Note: `is_active` uniqueness is enforced in the store crate's Rust logic
(transaction: clear all, then set one), not a DB constraint — SQLite partial
unique indexes could do this but the extra complexity isn't worth it for a
single-row toggle at this scale.

## 3. `suno-remote-library-cache-store`

```sql
CREATE TABLE remote_tracks (
    id                TEXT PRIMARY KEY,     -- local UUID
    account_id        TEXT NOT NULL REFERENCES accounts(id),
    remote_id         TEXT NOT NULL,        -- Suno's own track/project id
    title             TEXT NOT NULL,
    artist_or_persona TEXT,
    duration_seconds  REAL,
    cover_art_url     TEXT,
    audio_stream_url  TEXT,                 -- may be time-limited/expiring — see doc 06 notes
    tags_json         TEXT NOT NULL DEFAULT '[]',  -- JSON array of strings
    raw_metadata_json TEXT NOT NULL DEFAULT '{}',  -- full sanitized API payload, for forward-compat
    schema_version    INTEGER NOT NULL DEFAULT 1,
    synced_at         TEXT NOT NULL,
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL,
    deleted_at        TEXT,
    UNIQUE(account_id, remote_id)
);

CREATE INDEX idx_remote_tracks_account ON remote_tracks(account_id);
CREATE INDEX idx_remote_tracks_title   ON remote_tracks(title);
```

`raw_metadata_json` deliberately retains the full sanitized-of-nothing (this
is local cache, not shared — no redaction needed here) API response so that
fields not yet promoted to a real column are never lost between syncs; this
is what makes later schema evolution (Suno adds a field we now care about)
cheap — read it out of already-cached `raw_metadata_json` instead of
re-fetching everything.

Per-account **feed sync state**: Suno's feed endpoint is cursor-based (per
doc 06 §2.2), so each account needs its persisted sync cursor to resume/
continue pagination across sync runs. The simple version is chosen: a tiny
dedicated table rather than extra columns on the `accounts` row — it keeps
sync mechanics out of the profile store's concern and extends trivially if
more than one feed cursor is ever needed.

```sql
CREATE TABLE account_sync_state (
    account_id  TEXT PRIMARY KEY REFERENCES accounts(id),
    next_cursor TEXT,               -- null = no sync performed yet / start from beginning
    synced_at   TEXT NOT NULL
);
```

## 4. `local-download-manager-store`

```sql
CREATE TABLE downloads (
    id               TEXT PRIMARY KEY,
    account_id       TEXT NOT NULL REFERENCES accounts(id),
    remote_track_id  TEXT NOT NULL REFERENCES remote_tracks(id),
    local_file_path  TEXT,                  -- null until completed
    status           TEXT NOT NULL,         -- 'queued' | 'downloading' | 'completed' | 'failed' | 'paused'
    bytes_downloaded INTEGER NOT NULL DEFAULT 0,
    bytes_total      INTEGER,
    error_message    TEXT,
    attempt_count    INTEGER NOT NULL DEFAULT 0,
    created_at       TEXT NOT NULL,
    updated_at       TEXT NOT NULL,
    completed_at     TEXT
);

CREATE INDEX idx_downloads_status ON downloads(status);
```

Operational policy (condensed from doc 15 §5, now superseded there):

- Downloads are queued with a configurable max-concurrency setting
  (default ~3).
- Failed downloads retry automatically with exponential backoff, capped at
  3 attempts (tracked via `attempt_count`); after the cap they surface a
  manual retry button rather than retrying forever.
- Partial downloads resume via HTTP range requests if the CDN supports it
  (confirm during Phase 1; otherwise restart-from-zero on retry).
- The local library root folder is user-configurable, with a sane
  per-platform default.
- File naming uses a configurable template shared with the pipeline export
  templating logic (doc 18 §2.2 reuse rule — one templating implementation,
  not two).

## 5. `lyrics-and-alignment-store`

```sql
CREATE TABLE lyric_documents (
    id               TEXT PRIMARY KEY,
    remote_track_id  TEXT NOT NULL REFERENCES remote_tracks(id),
    source           TEXT NOT NULL,   -- 'suno_remote' | 'whisper_local' | 'manual_edit'
    is_current       INTEGER NOT NULL DEFAULT 0,  -- one "current" doc per track (app-enforced)
    plain_text       TEXT NOT NULL,
    created_at       TEXT NOT NULL
);

CREATE TABLE lyric_segments (
    id                TEXT PRIMARY KEY,
    lyric_document_id TEXT NOT NULL REFERENCES lyric_documents(id),
    sequence_index    INTEGER NOT NULL,     -- order within the document
    level             TEXT NOT NULL,        -- 'line' | 'word'
    text              TEXT NOT NULL,
    start_seconds     REAL NOT NULL,
    end_seconds       REAL NOT NULL,
    confidence        REAL,                 -- present for whisper-sourced segments
    manually_edited   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_lyric_segments_doc ON lyric_segments(lyric_document_id);
```

`lyric_documents` is versioned by simply inserting a new row (never
UPDATE-in-place on the text/timing of an existing document) and flipping
`is_current` — this gives the lyrics editor full history/undo-to-source for
free, matching doc 01 §6's "never silently overwrite" requirement.

## 6. `canvas-scene-and-keyframe-store`

```sql
CREATE TABLE scenes (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    scene_json      TEXT NOT NULL,   -- full scene graph: elements, properties, keyframe tracks
    schema_version  INTEGER NOT NULL DEFAULT 1,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    deleted_at      TEXT
);
```

Deliberately a single JSON-blob-per-scene design rather than normalizing
elements/keyframes into rows: scene graphs are read/written wholesale by the
canvas editor (load-edit-save the whole tree), rarely queried piecemeal by
SQL, and their shape will churn a lot during Phase 5 development — a rigid
relational schema would fight that. `schema_version` + a small in-Rust
migration function (`fn migrate_scene_json(v: u32, json: Value) -> Value`)
handles evolution. Revisit this decision via ADR only if a real need for
SQL-level querying into scene internals emerges (e.g., "find all scenes
using effect X" at scale).

Illustrative (non-binding) `scene_json` shape:
```json
{
  "elements": [
    {
      "id": "uuid",
      "kind": "text | image | shape | karaoke_text",
      "base_properties": { "x": 0, "y": 0, "rotation": 0, "scale": 1, "opacity": 1 },
      "kind_specific": { "...": "..." },
      "keyframe_tracks": [
        { "property": "opacity", "points": [ { "t": 0.0, "value": 0.0, "easing": "linear" }, { "t": 1.0, "value": 1.0, "easing": "ease_out" } ] }
      ],
      "effects": [ { "type": "fade_in_out", "params": { } } ]
    }
  ],
  "canvas_settings": { "width": 1920, "height": 1080 }
}
```

## 7. `automation-pipeline-definition-store`

```sql
CREATE TABLE pipelines (
    id                TEXT PRIMARY KEY,
    name              TEXT NOT NULL,
    input_selector_json TEXT NOT NULL,  -- e.g. {"kind":"tag","tag":"promo"} | {"kind":"explicit_ids","ids":[...]}
    scene_id          TEXT NOT NULL REFERENCES scenes(id),
    lyric_source_policy TEXT NOT NULL,  -- 'remote_preferred' | 'whisper_preferred' | 'remote_only'
    export_settings_json TEXT NOT NULL, -- resolution, fps, codec, output path template
    schema_version    INTEGER NOT NULL DEFAULT 1,
    created_at        TEXT NOT NULL,
    updated_at        TEXT NOT NULL,
    deleted_at        TEXT
);

CREATE TABLE pipeline_runs (
    id            TEXT PRIMARY KEY,
    pipeline_id   TEXT NOT NULL REFERENCES pipelines(id),
    status        TEXT NOT NULL,   -- 'running' | 'completed' | 'failed' | 'cancelled'
    started_at    TEXT NOT NULL,
    finished_at   TEXT
);

CREATE TABLE pipeline_run_items (
    id              TEXT PRIMARY KEY,
    pipeline_run_id TEXT NOT NULL REFERENCES pipeline_runs(id),
    remote_track_id TEXT NOT NULL REFERENCES remote_tracks(id),
    status          TEXT NOT NULL,  -- 'pending' | 'in_progress' | 'completed' | 'failed'
    error_message   TEXT,
    output_file_path TEXT,
    started_at      TEXT,
    finished_at     TEXT
);

CREATE INDEX idx_run_items_run ON pipeline_run_items(pipeline_run_id);
CREATE INDEX idx_run_items_status ON pipeline_run_items(status);
```

`pipeline_run_items` existing as durable rows (not just in-memory queue
state) is what makes crash-resumability (doc 04 Phase 7 exit criteria)
possible: on restart, resume any run with unfinished items rather than
re-running the whole pipeline.

## 8. `recorded-audio-take-store`

```sql
CREATE TABLE recorded_takes (
    id              TEXT PRIMARY KEY,
    local_file_path TEXT NOT NULL,
    input_device_name TEXT,
    duration_seconds  REAL,
    sample_rate       INTEGER,
    channels          INTEGER,
    notes             TEXT,
    created_at        TEXT NOT NULL,
    deleted_at        TEXT
);
```

Deliberately minimal for Phase 2; Phase 9's DAW work will likely add a
`recording_sessions` / `tracks_within_session` layer above this rather than
overloading this table — that decision is deferred to Phase 9's own doc.

## 9. Cross-Store Composition Examples (illustrative, not literal SQL)

These live in `application-services` code, never as cross-store SQL:

- **Karaoke render service** reads `remote_tracks` (for the audio source),
  `lyric_documents`/`lyric_segments` (current doc for that track), and a
  `scenes` row referencing a `karaoke_text` element — combining three
  stores' data at the Rust level.
- **Automation orchestrator** reads a `pipelines` row, resolves its
  `input_selector_json` against `remote_tracks`, creates a `pipeline_runs` +
  `pipeline_run_items` set, then for each item invokes the same single-track
  render service Phase 4/5 already built.
