# Co-write Lyrics (T1 capture 2026-08-25)

## `POST /api/generate/cowrite-lyrics/`

- **Request** (JSON):
```json
{
  "selected": "",
  "context_before": "",
  "context_after": "",
  "instruction": "<natural-language instruction>",
  "title": "",
  "style": "",
  "mode": "apply_user_request",
  "references": [],
  "num_variants": null,
  "lyricist_id": null,
  "metadata": {"lyrics_model": "default", "enable_thinking": false},
  "create_session_token": null,
  "lyrics_project_id": null
}
```
- Fields support in-editor co-writing: `selected`/`context_before`/
  `context_after` carry the editor selection and surrounding text; `mode` is
  an instruction-mode enum (`apply_user_request` observed).
- **Response** → 200:
```json
{
  "lyrics_request_id": "<uuid>",
  "lyrics_id": "<uuid>",
  "edited_lyrics": "<full lyric text with [Section] tags>"
}
```
- Single-shot request/response (no polling observed for this call).

## `GET /api/lyricists?limit=100`

- **Response** → 200: `{"lyricists": []}` (empty for this account; structure
  implies selectable persona-lyricists with ids usable as `lyricist_id`).
