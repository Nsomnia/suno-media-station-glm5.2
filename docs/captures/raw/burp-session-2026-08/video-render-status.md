# Video Render Status (T1 capture 2026-08-25)

## `GET /api/video/generate/{clip_id}/status/` (2 captures, trailing slash)

- → 200:
```json
{"status": "complete", "video_url": ""}
```
- `status` enum observed: `complete`. Empty `video_url` here means the clip's
  video render produced nothing yet/failed silently — the clip-level
  `video_url` field on the clip object remains the authoritative asset link.

## `POST /api/video_gen/pending_batches`

- **Request**: `{}`
- **Response** → 200: `{"batch_ids": []}`
- Startup poll for in-flight video generation batches (used to restore UI
  state after reload).
