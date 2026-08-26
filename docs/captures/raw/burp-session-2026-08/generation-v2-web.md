# Generation — POST /api/generate/v2-web/ (T1 capture 2026-08-25)

## `POST /api/c/check` — captcha requirement gate

- **Request**: `{"ctype":"generation"}`
- **Response** → 200: `{"required": true, "captcha_version": 2}`
- Generation requests must carry a fresh captcha token in the `token` field
  (see below). Check this gate before generating.

## `POST /api/generate/v2-web/`

- Full request body structure (values sanitized; long text truncated):

```json
{
  "token": "<redacted-captcha-token>",
  "generation_type": "TEXT",
  "title": "<short title>",
  "tags": "<style tags string>",
  "negative_tags": "",
  "mv": "chirp-fenix",
  "prompt": "<lyrics/prompt string>",
  "make_instrumental": false,
  "user_uploaded_images_b64": null,
  "metadata": {
    "web_client_pathname": "/chat",
    "is_max_mode": false,
    "is_mumble": false,
    "create_mode": "custom",
    "user_tier": "<plan uuid>",
    "create_session_token": "<uuid>",
    "disable_volume_normalization": false,
    "control_sliders": { "…": "…" }
  },
  "override_fields": [],
  "cover_clip_id": null,
  "cover_start_s": null,
  "cover_end_s": null,
  "persona_id": null,
  "artist_clip_id": null,
  "artist_start_s": null,
  "artist_end_s": null,
  "continue_clip_id": null,
  "continued_aligned_prompt": null,
  "continue_at": null,
  "transaction_uuid": "<uuid generated client-side>",
  "token_provider": 2,
  "lyrics_project_id": "<uuid or null>"
}
```

- **Response** → 200:
```json
{
  "id": "<batch uuid>",
  "clips": [
    { "status": "submitted", "title": "…", "id": "<clip uuid>",
      "entity_type": "song_schema", "video_url": "", "audio_url": "",
      "major_model_version": "v5.5", "model_name": "chirp-fenix",
      "batch_index": 0, "metadata": {"tags": "…", "prompt": "…"},
      "...same clip schema as feed/v3..." },
    { "...second clip of batch..." }
  ],
  "metadata": { },
  "major_model_version": "v5.5",
  "status": "submitted",
  "created_at": "…",
  "batch_size": 2
}
```
- Clips start as `"submitted"` with empty `audio_url`; completion is observed
  by polling the library feed / `get_songs_by_ids` until
  `status == "complete"` and `audio_url` populates.
- Model availability + per-model limits (`max_lengths`, capabilities) come
  from `GET /api/session/` → `models[]` (see session-user-config.md).

## Prototype-recon endpoints NOT exercised this capture

- `POST /api/generate/lyrics/` (+ poll), `POST /api/generate/concat/v2/` —
  remain LEADs. Note lyric *co-writing* now has its own captured endpoint:
  see [cowrite-lyrics.md](cowrite-lyrics.md).
