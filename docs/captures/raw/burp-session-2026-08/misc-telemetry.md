# Misc Telemetry & App Chrome (low-value; T1 capture 2026-08-25)

Grouped briefly per orchestrator instruction. All on
`https://studio-api-prod.suno.com`, all 200/204.

## Playback telemetry (high-frequency, fire-and-forget)

- `POST /api/gen/{id}/increment_play_count/v2` — body `{"sample_factor":1}`;
  → `null`. Fired once per playback start.
- `POST /api/gen/{id}/listen_milestone` — body `{"milestone":"30s"}` (also
  other thresholds); → `null`. Fired repeatedly during playback (~30–60 s
  cadence).
- `GET /api/music_player/playbar_state` →
  `{playbar_state: "paused", song_play_time: <s>, repeat_state:
  "no-repeat", action_time: "<ISO>", song_ids_in_queue: [], volume: 100.0,
  device_id: "<redacted>", device_type: "Mac"}`
- `POST /api/music_player/playbar_state` — cross-device playbar sync; ~25
  captures, debounced every few seconds of playback state change. Body adds:
  `song_index`, `song_ids_in_queue: ["<uuid>", …]`,
  `playlist_context: null`, `device_context: null`,
  `context_id`/`context_type` ("create"). → `{"success": true}`.
  **Relevant to us**: server-side queue persistence we could adopt or ignore.

## Notifications

- `GET /api/notification/v2` → `{notified_at: "<ISO>", notifications: [
  {id, priority, updated_at, is_read, notification_type
  ("comment_mention"|"clip_create_followee"|"persona_favorite"|…),
  user_profiles: [{type, display_name, handle, avatar_image_url,
  is_verified, is_following}] (<redacted-user-data>), total_users,
  content_id, content_ancillary_id?, content_title, content_image_url,
  content_message}, "...truncated..."]}`
- `GET /api/notification/v2/badge-count` → `{"badge_count": 2}`
- `POST /api/notification/v2/clear-badge` → 204 empty

## Experiment flags

- `POST /api/statsig/experiment/` — body
  `{layer_name, parameter_name, parameter_type, parameter_default}` →
  `{success: true, data: {"<param>": "<value>"}}`
- `GET /api/statsig/experiment/forked-onboarding` →
  `{onboarding_group: "default", reason: "statsig_parameter_store"}`

## Onboarding / nudges / misc one-off GETs

- `GET /api/onboarding/current` → `{active: false, completed: false, failed: false}`
- `POST /api/onboarding/start` (body `{style_text:"", lyrics:""}`) → same
  shape + `step: {step, component, props {display_name, handle,
  avatar_image_url, aura_placeholder_url}, progress {current,total},
  skippable, can_go_back}` (props redacted)
- `GET /api/cms/nudges/publish-nudge` / `share-nudge` →
  `{slug, seen, contents: {toast_nudge: {title_text, subtitle_text,
  primary_cta {text {text}, action}}}}`
- `GET /api/modals` → `[]`
- `GET /api/labs/configs` → array of
  `{lab_id, name_override, description_override, cover_image_url,
  staff_only, enabled_ga, has_statsig_segment}`
- `GET /api/realtime/discover` →
  `{stream_url: "https://main.realtime.ably.net/sse?v=1.2&enveloped=false",
    auth: {credential: "embedded_token", jwt_header_param: "x-ably-token"}}`
  — Ably SSE endpoint for realtime updates; the bearer JWT doubles as the
  Ably token (`x-ably-token` header param seen inside the JWT header claims).
- `GET /api/challenge/progress` → challenge/credit-bonus gamification state
- `GET /api/share/stats?content_type=song` →
  `{content_type: "song", num_shared: <int>}`
- `GET /api/custom-model/pending/` → `{has_pending: false, pending_models: []}`
- `GET /api/contests/` → `{contests: [{id, name, type, description,
  start_time, end_time, base_clip_ids[], …}]}`
