# Burp Capture Session — 2026-08

> **Provenance:** live Burp Suite XML item exports provided by the human
> orchestrator on 2026-08-25 (capture timestamps 2026-08-25 ~19:02–20:57 MDT).
> Source exports (outside the repo, read-only): `~/Documents/suno-burp-exports/`
> (`studio-api-prod.suno.com`, `suno.com`, `auth.suno.com`; duplicate copy of the
> first at `~/Documents/studio-api-prod.suno.com`). Small/empty exports
> (`audiophile.suno.ai`, `clerk.suno.com`, `studio-api.prod.suno.com`, `blah`)
> contained nothing of evidentiary value.
>
> These markdown files are **sanitized extractions**, not the raw XML. The raw
> XML stays outside the repo because it contains live credentials.

## Coverage & Item Counts

| Host | Items | API relevance |
|---|---|---|
| `studio-api-prod.suno.com` | 306 | Primary Suno API — all §2.x evidence in doc 06 |
| `suno.com` | 292 | Next.js app shell / static assets / page loads only — no new API surface |
| `auth.suno.com` | 18 | Clerk auth flow (13 non-static items) |

All captured responses were `2xx` (mostly 200, two 204s, plus auth-flow 302
redirects). No error-shape captures in this session (doc 06 §2.7 remains open).

## Redaction Policy Applied

The source exports contain live credentials. Before any content entered this
directory, the following were removed/replaced:

- `Authorization` header values → recorded as presence + shape only
  (`Bearer <redacted-jwt>`); JWT claim *structure* documented with placeholder
  values only.
- `Cookie` request headers and `Set-Cookie` response header values → header
  NAME/presence recorded only (e.g. "4 Set-Cookie headers returned").
- `Browser-Token` values → shape only (`{"token":"<base64 timestamp JSON>"}`).
- All JWT strings (`eyJ…`) → `<redacted-jwt>` (including inside URLs and
  response bodies).
- Clerk identifiers (`client_*`, `session_*`, `signin_*`) and captcha tokens →
  `<redacted-*>`.
- User identity: emails → `<redacted-email>`; handles → `<redacted-handle>`;
  display names → `<redacted-display-name>`; user UUIDs → `<redacted-user-id>`;
  device IDs → `<redacted-device-id>`; Stripe subscription ids →
  `<redacted-subscription-id>`; birthday/social links/profile descriptions →
  removed.
- Third-party user data embedded in notification/feed examples → replaced with
  `<redacted-user-data>` or generic placeholders.
- Per-request crypto material from `mango/rights` (`key`/`iv`) → shapes only.
- Long lyric/prompt text truncated to short excerpts (own-account creative
  content, kept minimal as a matter of hygiene, not secrecy).

Content clip UUIDs, playlist/project IDs, model names, CDN URLs, and enum
values were kept real (structural, non-sensitive).

## Index of Extracted Endpoint Files

| File | Endpoint group |
|---|---|
| [`clerk-auth-flow.md`](clerk-auth-flow.md) | `auth.suno.com` Clerk sign-in/handshake/client/touch/verify/environment |
| [`feed-v3-library-listing.md`](feed-v3-library-listing.md) | `POST /api/feed/v3`, `POST /api/unified/feed`, `GET /api/clips/get_songs_by_ids`, `GET /api/project/me`, `GET /api/project/default` |
| [`generation-v2-web.md`](generation-v2-web.md) | `POST /api/generate/v2-web/`, `POST /api/c/check` (captcha gate) |
| [`cowrite-lyrics.md`](cowrite-lyrics.md) | `POST /api/generate/cowrite-lyrics/`, `GET /api/lyricists` |
| [`aligned-lyrics-v2.md`](aligned-lyrics-v2.md) | `GET /api/gen/{id}/aligned_lyrics/v2/`, `POST …/downbeats_streaming/v2`, `GET …/waveform-aggregates` |
| [`lyrics-projects-crud.md`](lyrics-projects-crud.md) | `GET/POST /api/lyrics-projects`, `POST /api/lyrics-projects/{id}/flush` |
| [`billing-suite.md`](billing-suite.md) | `/api/billing/*` (info, usage-plans ×4, eligible-discounts, conversion-tracking, auto-reload/nudge-check) |
| [`playlists.md`](playlists.md) | `GET /api/playlist/me`, profile pinned-clips |
| [`clips-relations.md`](clips-relations.md) | attribution, parent, remixes(+count), get_similar, comments, mango/rights |
| [`video-render-status.md`](video-render-status.md) | `GET /api/video/generate/{id}/status/`, `POST /api/video_gen/pending_batches` |
| [`styles-prompts-v2.md`](styles-prompts-v2.md) | `GET/POST /api/prompts/v2`, `POST /api/prompts/upsample`, `GET /api/prompts/suggestions` |
| [`session-user-config.md`](session-user-config.md) | `/api/session/`, user metadata/session-id/tos/user_config, personalization/* |
| [`misc-telemetry.md`](misc-telemetry.md) | listen_milestone, increment_play_count, playbar_state, notifications, statsig, onboarding, nudges, labs, realtime/discover, misc one-off GETs |

See also: prior-art leads this session confirms/corrects are tracked in
[`docs/meta/suno-api-ground-truth-from-prototype.md`](../../../meta/suno-api-ground-truth-from-prototype.md)
(addendum table) and applied to
[`docs/specs/suno-integration/06-suno-api-integration-contract.md`](../../../specs/suno-integration/06-suno-api-integration-contract.md).
