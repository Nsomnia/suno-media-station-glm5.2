# Playlists (T1 capture 2026-08-25)

## `GET /api/playlist/me?page=1&show_trashed=false&show_sharelist=false`

- **Response** → 200 (~9 KB, 12 playlists on page 1 of 57 total):
```json
{
  "num_total_results": 57,
  "current_page": 1,
  "playlists": [
    {
      "id": "<uuid>",
      "entity_type": "playlist_schema",
      "image_url": "https://cdn2.suno.ai/….jpeg",
      "playlist_clips": [],
      "current_page": 0,
      "num_total_results": 5,
      "is_owned": true, "is_trashed": false, "is_public": true,
      "is_hidden": false, "is_discover_playlist": false,
      "user_display_name": "<redacted-display-name>",
      "user_handle": "<redacted-handle>",
      "user_avatar_image_url": "https://cdn1.suno.ai/….webp",
      "user_is_verified": false,
      "reaction": "",
      "name": "<redacted-user-data>",
      "description": "",
      "cover_is_user_set": false,
      "upvote_count": 0, "dislike_count": 0, "flag_count": 0,
      "skip_count": 0, "play_count": 22, "song_count": 5,
      "total_duration": 1509.88
    },
    "...truncated to 2..."
  ]
}
```
- Page-based pagination (`page` query param + `num_total_results`).
- Inner `playlist_clips` empty in listing; per-playlist clip fetch endpoint
  not captured this session.

## Prototype-recon playlist CRUD leads — still NOT captured

`POST /api/playlist/create/`, `/api/playlist/set_metadata`,
`/api/playlist/update_clips/`, `/api/playlist/trash/`, and the recon-listed
listing path `GET /api/me/v2/playlists` (actual observed path is
`/api/playlist/me`) remain unconfirmed.

## Related profile reads

- `GET /api/profiles/pinned-clips` → `{"pinned_clips": [ …clip objects… ]}`
- `GET /api/profiles/{handle}/info` (handle redacted) →
```json
{"user_inputted_genres": ["…", "..."],
 "section_order": ["pinned_songs", "songs", "hooks", "playlists", "personas"],
 "instagram_link": "<redacted-user-data>",
 "youtube_link": "<redacted-user-data>"}
```
