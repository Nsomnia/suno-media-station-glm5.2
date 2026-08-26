# Library Listing — feed/v3, unified/feed, get_songs_by_ids, project/me (T1 capture 2026-08-25)

All calls to `https://studio-api-prod.suno.com`. Common headers on every
request (see README redaction policy): `Authorization: Bearer <redacted-jwt>`,
`Browser-Token: <redacted> {"token":"<base64 timestamp JSON>"}`,
`Device-Id: <redacted-uuid>`, `Origin/Referer: https://suno.com`, browser UA.
Every endpoint below was preceded by an identical-shape `OPTIONS` preflight
(200) — CORS is enforced; a native client must handle or avoid triggering it.

## `POST /api/feed/v3` — primary library listing (20 captures)

- **Request** (JSON):
```json
{
  "cursor": "02a69052-01d6-4e4c-a72c-9669d42f8729",
  "limit": 20,
  "filters": {
    "disliked": "False", "trashed": "False",
    "fromStudioProject": {"presence": "False"},
    "stem": {"presence": "False"},
    "stemComplement": "False",
    "workspace": {"presence": "True", "workspaceId": "default"},
    "searchText": "yakuza"
  }
}
```
- Filter variants observed:
  - text search: `"searchText": "<term>"` (client-side debounced per keystroke)
  - by user: `"user": {"presence": "True", "userId": "<redacted-user-id>"}`
  - by ids (bulk fetch): `"ids": {"presence": "True", "clipIds": ["<uuid>", …]}`
    with matching top-level `"limit"` equal to id count
  - first page uses `"cursor": null` or omits it
  - boolean-ish filter values are the *strings* `"True"`/`"False"`.
- **Response** → 200:
```json
{
  "clips": [ { …clip object… }, { … }, "...truncated to 2 of ~20..." ],
  "next_cursor": "c6c3215f-195d-4b31-b594-c5c0192535e9",
  "has_more": true
}
```
- Pagination: opaque **clip-UUID cursor** (the last clip's id), passed back as
  `cursor`. When a page exhausts results, `next_cursor` is absent and
  `has_more: false`.
- Clip object keys (superset observed): `status` (`submitted|complete`),
  `title`, `play_count`, `upvote_count`, `allow_comments`, `is_verified`,
  `id`, `entity_type: "song_schema"`, `video_url`, `audio_url`,
  `media_urls[] {url, content_type: "m4a-opus"|"mp3", delivery:
  "progressive", encoding?}`, `image_url`, `image_large_url`,
  `major_model_version` ("v5.5"), `model_name` ("chirp-fenix"),
  `metadata {tags, negative_tags?, prompt, …}`, plus on generation responses:
  `handle`, `display_name`, `user_id`, `is_public`, `is_trashed`, `is_liked`,
  `batch_index`, `created_at`, `action_config`, `has_hook`, `is_persona_root`.

## `POST /api/unified/feed` — profile-style feed

- **Request**: `{"feed_id":"user_songs","target_user_id":"<redacted-user-id>","page_size":20}`
- **Response** → 200:
```json
{"feed": {
   "feed_id": "user_songs",
   "feed_container_type": "synthetic_playlist",
   "feed_title": "Songs",
   "items": [ {"content_id": "<uuid>", "logging_context":
               {"recommendation_item_id": ""}, "content_type": "clip",
               "content_item": { …same clip object… } },
              "...truncated…" ],
   "next_cursor": "20" }}
```
- Cursor here is numeric-offset-style ("20"), unlike feed/v3's UUID cursor.

## `GET /api/clips/get_songs_by_ids?ids=<uuid>&ids=<uuid>&…`

- Repeated `ids` query params (20 in captured call). Response → 200:
  `{"clips": [ …clip objects… ]}` (~187 KB for 20 clips).

## `GET /api/project/me?page=1&sort=max_created_at_last_updated_clip&show_trashed=false&exclude_shared=false`

- Workspace listing. Response → 200:
```json
{"num_total_results": 55, "current_page": 1,
 "projects": [
   {"id": "default", "name": "My Workspace", "description": "Workspace for unassigned clips",
    "clip_count": 3073, "last_updated_clip": "2026-08-25T20:14:37.690Z", "shared": false},
   {"id": "<uuid>", "name": "<redacted-user-data>", "description": "",
    "clip_count": 166, "last_updated_clip": "…", "shared": false,
    "created_at": "2025-05-16T03:48:55.565Z"},
   "...truncated to 2..."]}
```
- Page-based pagination (`page` + `num_total_results`), not cursor.

## `GET /api/project/default` — default workspace contents

- Response → 200 (large): `{"id": "default", "name": "My Workspace",
  "description": "Workspace for unassigned clips", "project_clips":
  [{"clip": {…clip object…}, "relative_index": 0, "pinned": false},
  "...truncated..."], "is_owned": true, "is_trashed": false,
  "clip_count": <int>, "current_page": <int>, "shared": false}`

## Observed behaviors / gotchas

- feed/v3 search fires one request per keystroke — client should debounce.
- The clip object shape is shared across feed/v3, unified/feed,
  get_songs_by_ids, project/default, pinned-clips, get_similar — model once.
- `OPTIONS` preflights were captured for every endpoint family; all 200 with
  permissive CORS toward `https://suno.com`.
