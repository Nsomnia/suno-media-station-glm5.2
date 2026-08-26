# Session, User & Config (T1 capture 2026-08-25)

Identity-adjacent reads/writes on `https://studio-api-prod.suno.com`.
All identity values redacted per README policy.

## `GET /api/session/` — bootstrap call (fired on every app load)

- **Response** → 200 (~6 KB), top keys: `user`, `models`, plus more.
```json
{
  "user": {
    "email": "<redacted-email>",
    "username": "<redacted-email>",
    "id": "<redacted-user-id>",
    "clerk_id": "<redacted clerk user_… id>",
    "display_name": "<redacted-display-name>",
    "handle": "<redacted-handle>",
    "avatar_image_url": "https://cdn1.suno.ai/….webp",
    "profile_description": "<redacted-user-data>",
    "is_handle_updated": true,
    "birthday": "<redacted-user-data>",
    "is_vip": false, "is_trusted_vip": false,
    "accepted_vip_program_tos": false,
    "total_clips": 7999
  },
  "models": [
    {"can_use": true,
     "max_lengths": {"title": 100, "prompt": 5000, "tags": 1000,
                      "negative_tags": 1000, "gpt_description_prompt": 3000},
     "name": "v4.5", "external_key": "chirp-auk", "major_version": 5,
     "description": "Intelligent prompts",
     "is_default_free_model": false, "is_default_model": false,
     "badges": ["pro"], "model_badges": [{"display_name": "PRO", "...colors..."}],
     "capabilities": ["generate","extend","upload_extend","cover_extend",
       "artist_extend","artist_cover","artist_consistency","infill",
       "infill_intro","infill_outro","artist_infill","cover_infill",
       "upsample","cover"],
     "features": ["create_control_sliders","tag_upsample","reuse_styles_lyrics"],
     "allowed_condition_combinations": [["extend"],["persona","extend"],["cover"],"..."],
     "id": "<uuid>"},
    {"...v4 (chirp-v4)..."}
  ]
}
```
- This is the authoritative model-catalog source (`mv` values for generation
  come from `external_key`; limits from `max_lengths`).

## Bearer JWT claim structure (shape only — from Authorization header, redacted)

Header: RS256, `kid: suno-api-rs256-key-1`. Payload claims observed:
`suno.com/claims/user_id` (uuid), `https://suno.ai/claims/clerk_id`
(`user_…`), `suno.com/claims/token_type: "access"`, `sun/did` (numeric),
`aud: "suno-api"`, `sub` (= clerk id), `azp: "https://suno.com"`, `fva:
[0,-1]`, `iat`/`exp` (~1 h lifetime), `iss: "https://auth.suno.com"`, `jit`.

## Other user endpoints

| Endpoint | Response |
|---|---|
| `GET /api/user/metadata` | `{user_id: "<redacted>", user_plan_key: "premier", joined_date: "<epoch-ms string>", is_vip: false}` |
| `GET /api/user/get_user_session_id/` | `{session_id: "<opaque hex>"}` (Suno-side session token, distinct from Clerk session ids) |
| `GET /api/user/tos_acceptance` | `{has_accepted_tos: true, has_accepted_tos_timestamp: "<ISO>"}` |

## `POST /api/user/user_config/`

- **Request**: `{}` (empty object returns current config; field-patch shape
  unconfirmed)
- **Response** → 200:
```json
{"shown_creation_tour": true, "shown_claim_username": true,
 "has_accepted_timbaland_terms": false, "preferred_tags": [],
 "has_set_remix_perm": true, "creation_song_language": "",
 "has_set_video_cover_hook_perm": false,
 "dismissed_banners_web": ["hooks_promo_banner"],
 "tooltip_progression": 0,
 "notification_preferences": {"post_engagement": true,
                               "followed_user_post": true},
 "shown_challenges_tooltip": false, "completed_onboarding_flow": false,
 "publish_remix_default": true, "publish_pin_default": false,
 "publish_comment_default": true, "player_play_related": true,
 "player_play_next_queued": true, "show_credits": false,
 "data_restrictions_uploads": false, "data_restrictions_voices": false,
 "data_restrictions_derivatives": false, "data_restrictions_eligible": false}
```

## Personalization

| Endpoint | Response |
|---|---|
| `GET /api/personalization/memory` | `{generated_tldr: "<style profile text>", user_tldr: "<same, editable copy>", has_user_tldr: true}` — AI-generated personalization profile feeding generation |
| `GET /api/personalization/settings` | `{"styles_augmentation": false}` |
