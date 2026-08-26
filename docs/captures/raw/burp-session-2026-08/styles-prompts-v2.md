# Styles / Prompts v2 (T1 capture 2026-08-25)

Saved style prompts ("Styles") CRUD + AI tag enhancement.

## `GET /api/prompts/v2?per_page=100`

- **Response** → 200 (~23 KB):
```json
{"styles": [
  {"id": "<uuid>", "tags": "<style tags string>",
   "title": "<optional title>",
   "created_at": "2026-07-29T05:59:02.794Z",
   "updated_at": "2026-07-29T05:59:02.794Z"},
  "...truncated to 2..."
]}
```
- `title` absent when never set. No pagination cursor observed at 100/page.

## `POST /api/prompts/v2` — create/update a saved style

- **Request**: `{"tags":"<tags string>","id":null}` (`id` null = create;
  passing an existing id presumably updates).
- **Response** → 200: the created object
  `{"id": "<uuid>", "tags": "…", "created_at": "…", "updated_at": "…"}`

## `POST /api/prompts/upsample` (2 captures) — AI-enhance a style prompt

- **Request**:
```json
{"original_tags": "<short tags>", "lyrics": "<lyrics text or placeholder>",
 "is_instrumental": false}
```
- **Response** → 200:
```json
{"upsampled": "<expanded, detailed style description>",
 "request_id": "<uuid>"}
```
- Synchronous single-shot.

## `GET /api/prompts/suggestions`

- → 200:
```json
{
  "prompts": ["<suggested song idea>", "...10 items..."],
  "lyrics_prompts": ["<suggested lyric seed>", "...several items..."]
}
```
- Static-ish suggestion feed shown on the empty Create page.
