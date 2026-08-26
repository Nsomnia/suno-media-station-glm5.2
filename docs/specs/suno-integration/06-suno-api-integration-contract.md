# Suno API Integration Contract

> **Last Updated:** 2026-08-25 · **Status:** Active

> **This document is capture-driven, not speculative.** Per ADR-007 (doc 17),
> no endpoint is documented here until the human orchestrator has provided a
> real traffic capture (typically via Burp Suite) of that endpoint in use.
> The agent must never invent an endpoint shape and must halt + request a
> capture (per doc 03 §7) when a needed endpoint is undocumented here.
> Exception: entries marked **LEAD** below are provenance-tiered leads seeded
> from recon docs recovered from the predecessor prototype repo
> (`chadvis-projectm-qt`, see `docs/captures/raw/recon-from-chadvis/`).
> A LEAD still requires one fresh confirming capture before implementation.

## 0. How This Doc Gets Filled In (process)

1. Human performs the relevant action in the official Suno web/mobile client
   with Burp Suite (or similar) proxying traffic.
2. Human **redacts secrets** before sharing: replace bearer tokens, cookie
   values, API keys, and any personally-identifying fields (email, full
   name, phone) with placeholders like `<REDACTED_BEARER>`,
   `<REDACTED_COOKIE>`, `<REDACTED_EMAIL>`. Structural fields (IDs, status
   enums, timestamps, URLs to public assets) can stay real unless they
   themselves are sensitive.
3. Human pastes the sanitized request/response pair to the agent (or drops
   it in `docs/captures/raw/` as a `.http`/`.json` file — see §5).
4. Agent normalizes it into an entry below using the template in §1, adds a
   typed Rust request/response model in `suno-http-client-core`, and a
   fixture in `shared-test-support/suno-api-fixture-mocks` derived from the
   sanitized capture (fixtures are the sanitized versions — never real
   secrets — since they end up in the test suite / potentially the repo).
5. If a capture reveals a field whose meaning is unclear, the agent should
   ask rather than guess a name/type for it.

## 1. Endpoint Entry Template

Copy this block per endpoint as captures come in.

````markdown
**<Human-readable name, e.g. "List Library Tracks">**

- **Method/Path:** `GET /api/...`
- **Auth:** Bearer required? Cookie required? Both?
- **Captured:** <date> from <client: web app / mobile app / unknown>
- **Request Headers (notable):**
  - `Header-Name: value or <REDACTED>`
- **Request Body:** (if any — JSON shape, sanitized)
- **Response Body (sanitized excerpt):**
```json
  { }
```
- **Fields of Interest:**
  - `field.path` — meaning, type, nullable?
- **Rust Model Location:** `suno-http-client-core::models::...`
- **Notes/Gotchas:** pagination style, rate limits observed, error shapes
  seen, anything inconsistent between calls.
````

## 2. Known Endpoint Categories

These are the anticipated categories based on product requirements. Entries
are either CAPTURED (real traffic capture on file) or LEAD (provenance-
tiered lead from predecessor-repo recon; needs a fresh confirming capture
before any implementation depends on it).

### 2.1 Authentication

- **Method/Path:** Clerk flow against `https://auth.suno.com/v1`
- **Auth:** browser Clerk cookies (`__client`, `__client_uat`, suffixed
  variants like `__session_Jnxw-muT`) are the root credential; everything
  downstream derives from them.
- **Captured:** T1-captured 2026-08-25 · evidence:
  `docs/captures/raw/burp-session-2026-08/clerk-auth-flow.md`
- **Flow (as observed):**
  1. Browser/social login: `POST /v1/client/sign_ins` (form-encoded,
     `strategy=oauth_google&…`) → 302 chain through
     `/social/login/google-oauth2/` → provider →
     `/social/complete/google-oauth2/`.
  2. `GET /v1/client/handshake?redirect_url=…&__clerk_api_version=…&format=nonce`
     → 302 back to suno.com with a `__clerk_handshake` JWT bridging session
     cookies onto suno.com.
  3. `GET /v1/client?__clerk_api_version=<date>&_clerk_js_version=<ver>`
     returns client state incl. `sessions[].last_active_token.jwt` — **this
     is where the API bearer comes from**.
  4. Refresh = `POST /v1/client/sessions/{sid}/touch` (empty body); its
     response embeds a fresh `last_active_token.jwt`. The prototype-recon
     path `POST /v1/client/sessions/{sid}/tokens` was NOT observed — do not
     build against it.
  5. `POST /v1/client/verify` is a Turnstile captcha heartbeat (204 empty),
     not a validity preflight. `GET /v1/environment` returns static-ish
     instance config.
- **Bearer JWT claims structure** (values redacted in capture):
  `suno.com/claims/user_id`, `https://suno.ai/claims/clerk_id`,
  `suno.com/claims/token_type: "access"`, `sun/did`, `aud: "suno-api"`,
  `sub`, `azp`, `fva`, `iat`/`exp` (~1 h), `iss`, `jit`; RS256 with
  `kid: suno-api-rs256-key-1`.
- **Required browser-like headers on studio-api calls:**
  - `Authorization: Bearer {jwt}`
  - `Device-Id`: persisted UUID (generate once, store in keyring/config)
  - `Browser-Token`: JSON-shaped value `{"token":"<base64 timestamp JSON>"}`
  - `Origin` / `Referer`: from suno.com
  - Browser `User-Agent`
- **API base:** `https://studio-api-prod.suno.com`
- **Prototype implementation evidence (pre-capture era):** the working
  C++/Qt prototype (`chadvis-projectm-qt/src/suno/SunoClient.cpp:131-196`)
  refreshed via `POST https://clerk.suno.com/v1/client/sessions/{sid}/client?_is_native=true&_clerk_js_version=5.117.0`
  with the bearer read from response `jwt` / `response.jwt`, SID discovered
  from `GET …/client?_is_native=true…` (`last_active_session_id`, fallback
  `sessions[0].id`) or from the `sid` claim of a `__session*` cookie value
  used directly as the JWT (cookie precedence: `__session` →
  `__session_Jnxw-muT` → any `__session*` prefix). Refresh was lazy-only
  (token empty), no TTL tracking; on 401 it cleared the token and required
  manual re-auth. Full analysis + verdict table:
  `docs/captures/raw/burp-session-2026-08/clerk-auth-flow.md` ("Prototype
  implementation evidence").
- **Recommended strategy** (reconciles both evidence sources — do not pick
  silently):
  1. Primary: capture-proven `GET auth.suno.com/v1/client` →
     `sessions[].last_active_token.jwt`.
  2. Refresh: capture-proven `POST …/sessions/{sid}/touch`.
  3. Fallbacks, documented-but-unverified against today's API: Clerk-standard
     `POST …/sessions/{sid}/tokens`, then prototype-era
     `POST clerk.suno.com/v1/client/sessions/{sid}/client?_is_native=true…`;
     log loudly when a fallback fires and request a fresh capture.
  4. On studio-api 401: clear bearer → one touch attempt → interactive
     re-auth; add proactive ~55 min expiry refresh (absent in prototype).
- **Notes/Gotchas:** residual uncertainty narrowed but not eliminated — the
  prototype does not prove background silent-refresh survives long sessions
  (its own scheme was lazy-refresh-on-empty plus manual re-auth), so keep
  the 401→touch→re-auth chain as the contract; host migration
  `clerk.suno.com` → `auth.suno.com` is real in captures but the old
  host+path is unverified (zero captured items for `clerk.suno.com`).

### 2.2 Library / Projects Listing

- **Captured:** T1-captured 2026-08-25 · evidence:
  `docs/captures/raw/burp-session-2026-08/feed-v3-library-listing.md`
- **Endpoints (confirmed):**
  - `POST /api/feed/v3` — primary library listing. JSON body
    `{cursor: <clip-uuid|null>, limit, filters}`; filters include
    `disliked`/`trashed` (string `"True"`/`"False"`), `workspace {presence,
    workspaceId}`, `searchText`, `user {presence, userId}`,
    `ids {presence, clipIds[]}` for bulk-by-id fetch. Response
    `{clips[], next_cursor?, has_more}` — opaque UUID cursor.
  - `POST /api/unified/feed` — profile feed; body `{feed_id,
    target_user_id, page_size}`; response `{feed: {items[], next_cursor}}`
    with numeric-offset cursor.
  - `GET /api/clips/get_songs_by_ids?ids=…&ids=…` — repeated query params;
    response `{clips[]}`.
  - `GET /api/project/me?page=N&sort=max_created_at_last_updated_clip&show_trashed=false&exclude_shared=false`
    — page-based workspace listing (`{num_total_results, current_page,
    projects[]}`).
  - `GET /api/project/default` — default workspace contents incl.
    `project_clips[{clip, relative_index, pinned}]`.
- **Notes/Gotchas:** the shared clip-object schema spans feed/v3,
  unified/feed, get_songs_by_ids, project/default and pinned-clips — model
  once. Search-as-you-type fires one request per keystroke (debounce).
- Still LEAD: `GET /api/unified/search/omnisearch` (not exercised).

### 2.3 Track Assets

- **Captured:** T1-captured 2026-08-25 (URL delivery confirmed) · evidence:
  `docs/captures/raw/burp-session-2026-08/feed-v3-library-listing.md` +
  `…/clips-relations.md`
- **Confirmed:**
  - Audio URLs ride directly on clip payloads: `audio_url` (progressive MP3
    on `cdn1.suno.ai`) plus `media_urls[]` offering an `m4a-opus` CloudFront
    variant (`delivery: "progressive"`, optional `encoding`). No separate
    "get audio URL" call.
  - Cover art on payloads: `image_url`, `image_large_url`; `video_url` for
    rendered videos (empty string until ready).
- Still LEAD: dedicated download path `GET /api/billing/clips/{clip_id}/download/`;
  CDN URL expiry behavior unconfirmed.
- Related captured: `POST /api/mango/rights` returns per-playback AES
  key/IV shapes used by the encrypted web-player variant — see §2.20.

### 2.4 Timed Lyrics

- **Method/Path:** `GET /api/gen/{id}/aligned_lyrics/v2/`
- **Captured:** T1-captured 2026-08-25 · evidence:
  `docs/captures/raw/burp-session-2026-08/aligned-lyrics-v2.md`
- **Response Body shape (corrected by capture):**
```json
{"aligned_words": [
   {"word": "Howdy, ", "success": true, "start_s": 8.697,
    "end_s": 8.976, "p_align": 0.99} ],
 "waveform_data": {}, "hoot_cer": {}, "is_streamed": false,
 "aligned_lyrics": {}}
```
  - Field names are `word`/`start_s`/`end_s` (+`success`, `p_align`) — the
    recon's `start_time`/`end_time`/`line_index` were wrong. Words carry
    trailing whitespace/newlines and `[Section]` markers; concatenating
    `word` values reconstructs the lyric text.
- **Notes/Gotchas:** this resolves the Phase 3 karaoke question — native
  word-level timed lyrics exist; Whisper remains fallback only. Companion
  endpoints `POST …/downbeats_streaming/v2` (beat grid) and
  `GET …/waveform-aggregates` (mip-mapped min/max levels 11–23) captured in
  the same file.

### 2.5 Bulk / Organizational Operations

- Tagging, playlist/collection membership, delete, rename
- Whether bulk endpoints exist natively or must be client-side-looped
  single-item calls
- *(status: NOT YET CAPTURED)*

### 2.6 Account/Profile

- Current user profile (for display in account switcher)
- *(status: NOT YET CAPTURED)*

### 2.7 Rate Limits & Error Shapes

- Observed `429`/`5xx` behavior, retry-after headers if any
- Standard error response envelope shape
- *(status: NOT YET CAPTURED — fill in opportunistically whenever any
  capture happens to include an error response)*

### 2.8 Generation

- **Captured:** `v2-web` T1-captured 2026-08-25 · evidence:
  `docs/captures/raw/burp-session-2026-08/generation-v2-web.md`; co-write
  captured via `…/cowrite-lyrics.md`
- **Endpoints:**
  - `POST /api/generate/v2-web/` — **T1-captured.** Full body shape on file:
    captcha `token` (gated by `POST /api/c/check` →
    `{required, captcha_version}`), `generation_type`, `title`, `tags`,
    `negative_tags`, `mv` (model external key), `prompt`, 
    `make_instrumental`, `metadata {create_mode, user_tier,
    create_session_token, control_sliders, …}`, cover/persona/artist/continue
    fields, client-generated `transaction_uuid`, `token_provider: 2`,
    optional `lyrics_project_id`. Response: batch object `{id, clips[]
    (status "submitted", empty audio_url until ready), metadata,
    major_model_version, status, created_at, batch_size}` — completion
    observed by polling the feed/get_songs_by_ids.
  - `POST /api/generate/cowrite-lyrics/` — **T1-captured** single-shot lyric
    co-writing (`instruction`, `selected`/`context_before`/`context_after`
    editor spans, `mode`, `references[]`, `metadata {lyrics_model,
    enable_thinking}`, optional `lyrics_project_id`) →
    `{lyrics_request_id, lyrics_id, edited_lyrics}`.
    Companion `GET /api/lyricists?limit=100`.
  - `POST /api/generate/lyrics/` + poll — still LEAD.
  - `POST /api/generate/concat/v2/` — still LEAD.
- **Rust Model Location:** `suno-http-client-core::models::generation` (future)

### 2.9 Playlists

- **Captured:** listing T1-captured 2026-08-25 · evidence:
  `docs/captures/raw/burp-session-2026-08/playlists.md`; mutations remain
  LEAD
- **Endpoints:**
  - `GET /api/playlist/me?page=N&show_trashed=false&show_sharelist=false` —
    **T1-captured** (note: recon's `GET /api/me/v2/playlists` path is wrong).
    Page-based pagination; playlist object shape on file in the capture.
  - `POST /api/playlist/create/` — LEAD
  - `POST /api/playlist/set_metadata` — LEAD
  - `POST /api/playlist/update_clips/` — LEAD
  - `POST /api/playlist/trash/` — LEAD
- Related captured reads: `GET /api/profiles/pinned-clips`,
  `GET /api/project/default/pinned-clips`, `GET /api/profiles/{handle}/info`.
- **Rust Model Location:** `suno-http-client-core::models::playlist` (future)

### 2.10 Personas / Custom Models

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T3
- **Endpoints:**
  - `GET /api/me/v2/personas`
  - `POST /api/persona/create/`

### 2.11 Uploads

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T2
- **Endpoints:**
  - `POST /api/uploads/audio/{id}/initialize-clip/`
  - `POST /api/uploads/audio/{id}/upload-finish/`
  - Poll processing status: `GET /api/uploads/audio/{id}/`

### 2.12 Trash / Restore

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T3
- **Endpoints:**
  - `GET /api/me/v2/trash`
  - `POST /api/gen/trash`

### 2.13 Billing / Credits (read-only)

- **Captured:** T1-captured 2026-08-25 · evidence:
  `docs/captures/raw/burp-session-2026-08/billing-suite.md`
- **Endpoints (all T1-captured):**
  - `GET /api/billing/info/` — subscription + credits + credit-pack catalog +
    full `plan` object incl. `usage_plan_features[]` feature-flag vocabulary
  - `GET /api/billing/usage-plans` — plan catalog (`free`, `basic`, `pro`,
    `premier`, `pro_20250501`)
  - Supporting reads: `/api/billing/usage-plan-descriptions/`,
    `/api/billing/usage-plan-faq/`,
    `/api/billing/usage-plan-web-table-comparison/`,
    `/api/billing/eligible-discounts/`, `/api/billing/conversion-tracking/`
  - `POST /api/billing/auto-reload/nudge-check` → `{show: bool}`

### 2.14 Search

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T3
- **Endpoints:**
  - `GET /api/unified/search/omnisearch` — unified search across clips,
    personas, playlists

### 2.15 Explicit Non-Goals

The following route families are known from recon but documented as
known-but-out-of-scope unless a future ADR pulls them in:

- B-side / Labs experimental routes (see `b-side.md` in the recon corpus)
- Orpheus chat API (`suno-ai--orpheus-prod-web.modal.run`)
- Social graph endpoints (follows, feed of others' creations)

### 2.16 Lyrics Projects

New category — discovered by capture. **T1-captured 2026-08-25** · evidence:
`docs/captures/raw/burp-session-2026-08/lyrics-projects-crud.md`

- `GET /api/lyrics-projects?limit=50|100&sort=updated_at&cursor=<opaque>` —
  keyset pagination; cursor is base64 JSON
  `{field, value, id}`; response `{projects[], next_cursor?}`
- `POST /api/lyrics-projects` — create: `{"title": "…"}` → full project
  object (`{id, title, lyrics, created_at, updated_at}`)
- `POST /api/lyrics-projects/{id}/flush` — debounced autosave:
  `{"lyrics": "…"}` → `{"updated_at"}`
- Deletion/update-by-id endpoints not yet captured.

### 2.17 Styles / Prompts v2

New category — discovered by capture. **T1-captured 2026-08-25** · evidence:
`docs/captures/raw/burp-session-2026-08/styles-prompts-v2.md`

- `GET /api/prompts/v2?per_page=100` → `{styles: [{id, tags, title?,
  created_at, updated_at}]}`
- `POST /api/prompts/v2` — create/update saved style:
  `{"tags": "…", "id": null}` → style object
- `POST /api/prompts/upsample` — AI tag enhancement:
  `{"original_tags", "lyrics", "is_instrumental"}` →
  `{"upsampled", "request_id"}`
- `GET /api/prompts/suggestions` → `{prompts[], lyrics_prompts[]}`

### 2.18 Personalization & User Config

New category — discovered by capture. **T1-captured 2026-08-25** · evidence:
`docs/captures/raw/burp-session-2026-08/session-user-config.md`

- `GET /api/session/` — app bootstrap: user profile + authoritative model
  catalog (`models[]`: `external_key`, `max_lengths`, `capabilities[]`,
  `allowed_condition_combinations`, feature flags)
- `GET /api/user/metadata`, `GET /api/user/get_user_session_id/`,
  `GET /api/user/tos_acceptance`
- `POST /api/user/user_config/` — read (empty `{}` body) of persisted prefs;
  mutation shape unconfirmed
- `GET /api/personalization/memory` (AI style-profile TLDRs),
  `GET /api/personalization/settings`

### 2.19 Video Render Status

New category — discovered by capture. **T1-captured 2026-08-25** · evidence:
`docs/captures/raw/burp-session-2026-08/video-render-status.md`

- `GET /api/video/generate/{clip_id}/status/` → `{status, video_url}`
- `POST /api/video_gen/pending_batches` (`{}`) → `{batch_ids[]}` — startup
  poll for in-flight video renders.

### 2.20 Clip Relations & Rights

New category — discovered by capture. **T1-captured 2026-08-25** · evidence:
`docs/captures/raw/burp-session-2026-08/clips-relations.md`

- `GET /api/clips/{id}/attribution` → `{source_clips[]}`
- `GET /api/clips/parent?clip_id=` → `{is_public}`
- `GET /api/clips/remixes?clip_id=&limit=` → `{remixes[], has_more}`;
  `GET /api/clips/remixes/count?clip_id=` → `{count, is_capped}`
- `GET /api/clips/get_similar/?id=` → `{similar_clips[]}` (large)
- `GET /api/gen/{id}/comments?order=most_liked` →
  `{results[], allow_comment, total_count}`
- `POST /api/mango/rights` — body
  `{content_params {content_id, content_type}}` → per-playback
  `{key, iv}` crypto material for the encrypted player variant.

### 2.21 Playback Telemetry / App Chrome / Realtime

Low-value but captured for completeness (**T1-captured 2026-08-25**) ·
evidence: `docs/captures/raw/burp-session-2026-08/misc-telemetry.md`

- Playback telemetry: `POST /api/gen/{id}/increment_play_count/v2`,
  `POST /api/gen/{id}/listen_milestone` (`{"milestone":"30s"}`)
- Server-side playbar sync: `GET/POST /api/music_player/playbar_state`
- Notifications: `GET /api/notification/v2`, `…/badge-count`,
  `POST …/clear-badge` (204)
- Experiments: `POST /api/statsig/experiment/`,
  `GET /api/statsig/experiment/{name}`
- Misc one-offs: onboarding (`current`/`start`), cms nudges, `/api/modals`,
  `/api/labs/configs`, `/api/challenge/progress`, `/api/share/stats`,
  `/api/custom-model/pending/`, `/api/contests/`
- Realtime: `GET /api/realtime/discover` → Ably SSE stream URL + auth shape
  (bearer JWT doubles as the Ably token via `x-ably-token`).

## 3. Versioning / Drift Handling

Suno's undocumented API can change without notice. When a previously-working
endpoint starts failing in a way that looks like a shape change (not an auth
issue), the response is:

1. Request a fresh capture of that specific endpoint from the human.
2. Update this doc's entry (keep the old one struck through/archived below
   it rather than deleted, so drift is visible over time).
3. Update the corresponding Rust model + fixture.
4. Add a short entry to `docs/architecture/17-glossary-and-decisions-log.md` if the change
   was significant enough to affect architecture (e.g., pagination style
   changed entirely).

## 4. Rust Client Design Notes (non-endpoint-specific)

- `suno-http-client-core` is built on `reqwest` (async, widely used, good
  middleware ecosystem) unless a captured quirk (e.g. required HTTP/2
  fingerprinting to avoid bot detection) forces reconsideration — flag as an
  ADR if so.
- Every response type derives `serde::Deserialize` with `#[serde(rename_all
  = "camelCase")]` or explicit `#[serde(rename = "...")]` per observed field
  casing — confirm actual casing from captures rather than assuming.
- Unknown/unmapped fields should not cause deserialization failure —
  use `#[serde(flatten)] extra: serde_json::Map<String, serde_json::Value>`
  or `#[serde(default)]` liberally, since Suno may add fields without notice
  and the client shouldn't hard-break on that.
- Client is constructed per-account (see doc 05 §5) — no global mutable auth
  state.

## 5. Captures Storage Convention

Raw sanitized capture files (optional but encouraged for anything complex)
live at:

```
docs/captures/raw/<category>/<short-description>.http
```

e.g. `docs/captures/raw/library/list-tracks-page-1.http`. These are the
*evidence trail* backing this doc's entries — keep them even after
normalizing into §2, since re-deriving an edge case later is easier with the
raw capture on hand.

Recon documents recovered from the predecessor prototype repo live at
`docs/captures/raw/recon-from-chadvis/` — these back the LEAD entries above
and are leads, not captures (see the README in that directory).

## 6. Current Status Summary

As of 2026-08-25, this document is backed by a real T1 capture session
(`docs/captures/raw/burp-session-2026-08/`): auth flow, library listing,
generation, co-write lyrics, timed lyrics + downbeats + waveform, lyrics
projects, billing suite, playlists (listing), clip relations/rights, video
render status, styles v2, personalization/config, and telemetry are all
T1-captured and safe to implement against. Still LEAD-only: uploads (§2.11),
trash/restore (§2.12), search omnisearch (§2.14), generation `lyrics`/
`concat` paths (§2.8), playlist mutations (§2.9), personas listing (§2.10),
and error-shape envelope (§2.7 — no non-2xx captured yet). Flag gaps to the
human early in Phase 1 before depending on any LEAD entry.
