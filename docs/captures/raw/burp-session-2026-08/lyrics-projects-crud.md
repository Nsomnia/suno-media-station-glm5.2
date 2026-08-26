# Lyrics Projects CRUD (T1 capture 2026-08-25)

Server-persisted lyric drafts backing the lyrics editor and co-write.

## `GET /api/lyrics-projects?limit=50|100&sort=updated_at[&cursor=<opaque>]` (8 captures)

- **Response** → 200 (up to ~464 KB):
```json
{
  "projects": [
    {"id": "<uuid>", "title": "<redacted-user-data>",
     "lyrics": "<full lyric text>",
     "created_at": "2026-08-26T02:47:12.250Z",
     "updated_at": "2026-08-26T02:47:25.335Z"},
    "...truncated to 2..."
  ],
  "next_cursor": "<base64 cursor>"
}
```
- Cursor is base64 JSON decoding to structure:
  `{"field": "updated_at", "value": "<ISO ts>", "id": "<uuid>"}` — keyset
  pagination on (updated_at, id). Pass back verbatim as the `cursor` query
  param. Absent on last page.

## `POST /api/lyrics-projects` (3 captures)

- **Request**: `{"title":"<initial title>"}`
- **Response** → 200: full project object (`id`, `title`, `lyrics: ""`,
  `created_at`, `updated_at`). Creating a project auto-saves it server-side;
  observed immediately before generation with `lyrics_project_id` linking.

## `POST /api/lyrics-projects/{id}/flush` (15 captures)

- Debounced autosave: client flushes editor state every ~30–60s and after edits.
- **Request**: `{"lyrics": "<entire lyric text>"}`
- **Response** → 200: `{"updated_at": "2026-08-26T02:30:25.734Z"}`
- No DELETE/PUT captured this session; update appears to be flush-only
  (title set at creation). Deletion endpoint unknown — still a gap.
