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

- **Method/Path:** Clerk session-token exchange against `https://auth.suno.com/v1`
- **Auth:** browser `__session` cookie (from embedded webview login) is the root credential; everything downstream derives from it.
- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T1
- **Flow:**
  1. Browser login establishes the `__session` cookie for suno.com.
  2. `GET https://auth.suno.com/v1/client?_is_native=true` (with cookie) returns client state including `last_active_session_id`.
  3. Read `last_active_session_id` → `POST https://auth.suno.com/v1/client/sessions/{sid}/tokens`.
  4. Response yields a JWT used as the API bearer token. JWT expires ≈ 1 h but can be re-exchanged for a fresh one as long as the session cookie lives.
- **Required browser-like headers on studio-api calls:**
  - `Authorization: Bearer {jwt}`
  - `Device-Id`: persisted UUID (generate once, store in keyring/config)
  - `Browser-Token`: value issued by the site, persist alongside Device-Id
  - `Origin` / `Referer`: from suno.com
  - Browser `User-Agent`
- **API base:** `https://studio-api-prod.suno.com`
- **Notes/Gotchas:** keep this lead-status until one fresh capture confirms
  the `/v1/client` and `/tokens` exchange shapes verbatim.

### 2.2 Library / Projects Listing

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T1/T2
- **Notes/Gotchas:**
  - The library feed v3 endpoint is **POST**, not GET, and is cursor-based:
    response shape `{items, next_cursor, has_more}`.
  - Bulk metadata fetch: `GET /api/clips/get_songs_by_ids` (pass multiple clip IDs).
  - Search: `GET /api/unified/search/omnisearch`.
  - Pagination style, filters, sort order still need confirming capture.
- *(status: LEAD — no confirming capture yet)*

### 2.3 Track Assets

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T1
- **Notes/Gotchas:**
  - Audio URLs arrive directly on the clip payloads returned by the feed /
    `get_songs_by_ids` — there is no separate "get audio URL" call.
  - Dedicated download path: `GET /api/billing/clips/{clip_id}/download/`.
  - Cover art URLs also ride on clip payloads; expiry behavior unconfirmed.
- *(status: LEAD — no confirming capture yet)*

### 2.4 Timed Lyrics

- **Method/Path:** `GET /api/gen/{id}/aligned_lyrics/v2/`
- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T1
- **Response Body shape:**
  - `{ aligned_lyrics: [{word, start_time, end_time, line_index?}], language?, status }`
  - Word-level timing confirmed by recon; `line_index` presence varies — treat optional.
- **Notes/Gotchas:** this resolves the Phase 3 karaoke question — native
  word-level timed lyrics exist; Whisper remains fallback only.
- *(status: LEAD — pending one fresh confirming capture)*

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

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T2/T3
- **Endpoints:**
  - `POST /api/generate/v2-web/` — main song generation request
  - `POST /api/generate/lyrics/` — custom lyrics generation; poll via
    `GET /api/generate/lyrics/{id}` until complete
  - `POST /api/generate/concat/v2/` — extend/concat existing clips
- **Rust Model Location:** `suno-http-client-core::models::generation` (future)

### 2.9 Playlists

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T2
- **Endpoints:**
  - `GET /api/me/v2/playlists` — list user playlists
  - `POST /api/playlist/create/`
  - `POST /api/playlist/set_metadata` — rename/description changes
  - `POST /api/playlist/update_clips/` — add/remove clips from playlist
  - `POST /api/playlist/trash/`
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

- **Captured:** LEAD (prototype recon 2026-04/2026-08) — needs confirming capture · Tier T2
- **Endpoints:**
  - `GET /api/billing/info/` — credit balance / subscription info for display

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

No longer zero-endpoint: this document is now seeded with provenance-tiered
LEAD entries derived from recon docs recovered from the predecessor prototype
repo (`chadvis-projectm-qt`, ~2026-04 recon). Every such entry remains a
LEAD — implementation still requires one fresh Burp capture confirming each
endpoint before it is used, per doc 03 §7. Categories without LEAD entries
(§2.5–§2.7) remain empty pending captures. Flag gaps to the human early in
Phase 1 if confirming captures aren't yet provided.
