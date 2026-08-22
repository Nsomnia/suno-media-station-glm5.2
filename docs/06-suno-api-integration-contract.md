**Suno API Integration Contract**

> **This document is capture-driven, not speculative.** Per ADR-007 (doc 17),
> no endpoint is documented here until the human orchestrator has provided a
> real traffic capture (typically via Burp Suite) of that endpoint in use.
> The agent must never invent an endpoint shape and must halt + request a
> capture (per doc 03 §7) when a needed endpoint is undocumented here.

**0. How This Doc Gets Filled In (process)**

1. Human performs the relevant action in the official Suno web/mobile client
   with Burp Suite (or similar) proxying traffic.
2. Human **redacts secrets** before sharing: replace bearer tokens, cookie
   values, API keys, and any personally-identifying fields (email, full
   name, phone) with placeholders like `<REDACTED_BEARER>`,
   `<REDACTED_COOKIE>`, `<REDACTED_EMAIL>`. Structural fields (IDs, status
   enums, timestamps, URLs to public assets) can stay real unless they
   themselves are sensitive.
3. Human pastes the sanitized request/response pair to the agent (or drops
   it in `docs/captures/raw/` as a `.http`/`.json` file — see §6).
4. Agent normalizes it into an entry below using the template in §1, adds a
   typed Rust request/response model in `suno-http-client-core`, and a
   fixture in `shared-test-support/suno-api-fixture-mocks` derived from the
   sanitized capture (fixtures are the sanitized versions — never real
   secrets — since they end up in the test suite / potentially the repo).
5. If a capture reveals a field whose meaning is unclear, the agent should
   ask rather than guess a name/type for it.

**1. Endpoint Entry Template**

Copy this block per endpoint as captures come in.

```
**<Human-readable name, e.g. "List Library Tracks">**

- **Method/Path:** `GET /api/...`
- **Auth:** Bearer required? Cookie required? Both?
- **Captured:** <date> from <client: web app / mobile app / unknown>
- **Request Headers (notable):**
  - `Header-Name: value or <REDACTED>`
- **Request Body:** (if any — JSON shape, sanitized)
- **Response Body (sanitized excerpt):**
'''json
  { }
  '''
- **Fields of Interest:**
  - `field.path` — meaning, type, nullable?
- **Rust Model Location:** `suno-http-client-core::models::...`
- **Notes/Gotchas:** pagination style, rate limits observed, error shapes
  seen, anything inconsistent between calls.
```

**2. Known Endpoint Categories (to be populated)**

These are the anticipated categories based on product requirements — each
starts EMPTY (no captures yet) and is filled in as Phase 1/3 work demands
real data. Do not populate with guessed content.

- ### 2.1 Authentication
  - Native email/password login exchange
  - Session/token refresh
  - Federated (Google) OAuth handoff shape
  - Federated (Facebook) OAuth handoff shape
  - *(status: NOT YET CAPTURED)*

- ### 2.2 Library / Projects Listing
  - List tracks/projects (pagination shape, filters, sort)
  - Track/project detail (single item)
  - Search
  - *(status: NOT YET CAPTURED)*

- ### 2.3 Track Assets
  - Audio file URL(s) (streaming vs downloadable, expiry behavior)
  - Cover art URL(s)
  - *(status: NOT YET CAPTURED)*

- ### 2.4 Timed Lyrics
  - Endpoint for lyric text + timing, if one exists as a distinct resource
  - Confirm whether timing is word-level, line-level, or both
  - *(status: NOT YET CAPTURED — critical for Phase 3 scope; if this truly
    doesn't exist, Phase 3 doc must be updated to reflect Whisper-only reality)*

- ### 2.5 Bulk / Organizational Operations
  - Tagging, playlist/collection membership, delete, rename
  - Whether bulk endpoints exist natively or must be client-side-looped
    single-item calls
  - *(status: NOT YET CAPTURED)*

- ### 2.6 Account/Profile
  - Current user profile (for display in account switcher)
  - *(status: NOT YET CAPTURED)*

- ### 2.7 Rate Limits & Error Shapes
  - Observed `429`/`5xx` behavior, retry-after headers if any
  - Standard error response envelope shape
  - *(status: NOT YET CAPTURED — fill in opportunistically whenever any
    capture happens to include an error response)*

**3. Versioning / Drift Handling**

Suno's undocumented API can change without notice. When a previously-working
endpoint starts failing in a way that looks like a shape change (not an auth
issue), the response is:

1. Request a fresh capture of that specific endpoint from the human.
2. Update this doc's entry (keep the old one struck through/archived below
   it rather than deleted, so drift is visible over time).
3. Update the corresponding Rust model + fixture.
4. Add a short entry to `docs/17-glossary-and-decisions-log.md` if the change
   was significant enough to affect architecture (e.g., pagination style
   changed entirely).

**4. Rust Client Design Notes (non-endpoint-specific)**

- `suno-http-client-core` is built on `reqwest` (async, widely used, good
  middleware ecosystem) unless a captured quirk (e.g. required HTTP/2
  fingerprinting to avoid bot detection) forces reconsideration — flag as an
  ADR if so.
- Every response type derives `serde::Deserialize` with `#[serde(rename_all
  = "camelCase")]` or explicit `#[serde(rename = "...")]` per observed field
  casing — confirm actual casing from captures rather than assuming.
- Unknown/未-mapped fields should not cause deserialization failure —
  use `#[serde(flatten)] extra: serde_json::Map<String, serde_json::Value>`
  or `#[serde(default)]` liberally, since Suno may add fields without notice
  and the client shouldn't hard-break on that.
- Client is constructed per-account (see doc 05 §5) — no global mutable auth
  state.

**5. Captures Storage Convention**

Raw sanitized capture files (optional but encouraged for anything complex)
live at:

```
docs/captures/raw/<category>/<short-description>.http
```

e.g. `docs/captures/raw/library/list-tracks-page-1.http`. These are the
*evidence trail* backing this doc's entries — keep them even after
normalizing into §2, since re-deriving an edge case later is easier with the
raw capture on hand.

**6. Current Status Summary**

**As of doc creation: zero endpoints captured.** This document is a
skeleton/process definition only. Phase 1 cannot meaningfully begin
implementation of `suno-http-client-core` beyond boilerplate/error-type
scaffolding until at least the Authentication and Library Listing categories
have real captures. This is expected and correct — flag it to the human
early in Phase 1 if captures aren't yet provided, per doc 03 §7.
