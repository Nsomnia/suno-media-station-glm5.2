# Clip Relations & Rights (T1 capture 2026-08-25)

All `GET` unless noted; `https://studio-api-prod.suno.com`. Small payloads.

## `GET /api/clips/{id}/attribution` (13 captures — one per clip on open)

- → 200: `{"source_clips": []}` (empty for original clips; populated shape
  not observed — presumably lists clips this clip was derived from).

## `GET /api/clips/parent?clip_id=<uuid>`

- → 200: `{"is_public": false}` — visibility gate for the parent/origin clip.

## `GET /api/clips/remixes?clip_id=<uuid>&limit=20`

- → 200: `{"remixes": [], "has_more": false}` (no remixes existed for the
  sampled clips; element shape presumably the standard clip object).

## `GET /api/clips/remixes/count?clip_id=<uuid>`

- → 200: `{"count": 0, "is_capped": false}`

## `GET /api/clips/get_similar/?id=<uuid>` (trailing slash, 2 captures)

- → 200 (large, ~308 KB):
```json
{"similar_clips": [ { …standard clip object… }, "...truncated to 2..." ]}
```

## `GET /api/gen/{id}/comments?order=most_liked`

- → 200: `{"results": [], "allow_comment": true, "total_count": 0}`
- Non-empty comment shape not captured.

## `POST /api/mango/rights` (5 captures — fired per clip on playback)

- **Request**: `{"content_params":{"content_id":"<clip uuid>","content_type":"clip"}}`
- **Response** → 200:
```json
{"key": "<redacted base64, 32-byte shape>", "iv": "<redacted base64, 32-byte shape>"}
```
- Returns per-playback symmetric key material (AES key + IV shapes) used by
  the web player for stream decryption. Fired immediately before/with
  `increment_play_count`. A desktop client doing plain progressive MP3
  download likely does not need this; documented because it gates any future
  use of the encrypted `m4a-opus` CDN variant.
