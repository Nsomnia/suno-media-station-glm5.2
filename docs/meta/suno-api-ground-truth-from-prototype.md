# Suno API Ground Truth Recovered from the Prototype Repo

> **Last Updated:** 2026-08-22 · **Status:** Proposed — awaiting orchestrator decision (Audit items A-3, S-2, C-1, C-2)

## 1. Provenance & Reliability Tiers

Source: `~/Documents/chadvis-projectm-qt/docs/suno_api/` (auth.md, library.md,
generation.md, billing.md, projects.md, persona.md, upload.md, social.md,
b-side.md, feature-flags.md, ENDPOINT-INVENTORY.md, ~150+ endpoints), plus
`SUNO_API_NOTES.md` dated 2026-04-21. The prototype *shipped working* auth,
library sync w/ pagination, downloads, and karaoke lyrics — so its core paths
are battle-tested; parts of ENDPOINT-INVENTORY were "generated from reference
repo scan and web research" and are less certain.

Tiering used when seeding doc 06:

| Tier | Meaning | Examples |
|---|---|---|
| T1 | Exercised by working prototype code | Clerk token exchange, feed v3 pagination, clip download, aligned lyrics |
| T2 | Documented from real traffic but not code-exercised here | uploads, playlists CRUD |
| T3 | Scan/research-derived, unverified | much of ENDPOINT-INVENTORY, B-side |

Rule: **every seeded entry enters doc 06 as `LEAD` regardless of tier** and is
promoted to `CAPTURED` only after one fresh confirming capture (ADR-007's
spirit preserved; the cold-start problem dies).

## 2. Auth Corrections (feeds audit C-1)

- Identity provider is **Clerk**, base `https://auth.suno.com/v1`.
- Flow: browser `__session` cookie → `GET /v1/client?_is_native=true` →
  read `last_active_session_id` → `POST /v1/client/sessions/{sid}/tokens`
  → JWT. JWT ≈ 1 h expiry; re-exchange works while session lives.
- JWT claims include `user_id`, `clerk_id`, `aud: "suno-api"`, `exp`.
- API base `https://studio-api-prod.suno.com`; requests carry browser-like
  headers: `Authorization: Bearer`, `Device-Id` (persisted UUID),
  `Browser-Token`, `Origin`/`Referer` = suno.com, browser UA.
- Cookie zoo exists (`__client`, `__client_uat`, analytics) — only Clerk
  cookies matter functionally.
- Implication for docs 05/06: "refresh cookie" model → "Clerk session-token
  exchange"; manual-paste flow must accept cookie strings + perform exchange.

## 3. Proposed doc 06 Category Seeds

| New § | Category | Anchor endpoints (lead status) | Tier |
|---|---|---|---|
| 2.8 | Generation | `POST /api/generate/v2-web/`, `POST /api/generate/lyrics/` + poll, `POST /api/generate/concat/v2/` | T2/T3 |
| 2.9 | Playlists | `GET /api/me/v2/playlists`, `POST /api/playlist/create/`, `/update_clips/`, `/trash/` | T2 |
| 2.10 | Personas / Custom Models | `/api/me/v2/personas`, `POST /api/persona/create/` | T3 |
| 2.11 | Uploads | `POST /api/uploads/audio/{id}/initialize-clip/`, `/upload-finish/`, poll | T2 |
| 2.12 | Trash / Restore | `/api/me/v2/trash`, `/api/gen/trash` | T3 |
| 2.13 | Billing / Credits (read-only) | `GET /api/billing/info/` | T2 |
| 2.14 | Search | `GET /api/unified/search/omnisearch` | T3 |
| 2.15 | Explicit non-goals | B-side/Labs routes, Orpheus chat API, social graph — documented as known-but-out-of-scope unless a phase pulls them in via ADR | n/a |

Existing categories get these corrections:

- §2.1 Auth → rewritten per §2 above.
- §2.2 Listing → note feed v3 is **POST, cursor-based**
  (`{items, next_cursor, has_more}`); bulk metadata via
  `GET /api/clips/get_songs_by_ids`; search endpoint named.
- §2.3 Assets → audio URLs arrive on clip payloads; dedicated download path
  `GET /api/billing/clips/{clip_id}/download/`.
- §2.4 Lyrics → **confirmed to exist**:
  `GET /api/gen/{id}/aligned_lyrics/v2/` returns word-level
  `{word, start_time, end_time, line_index?}` + `language`/`status`.

## 4. Feature Behaviors Worth Copying (prototype observations)

- Library sync of thousands of clips is paginated/cursor-driven; prototype
  exposed per-page signals rather than auto-fetching all — good UX pattern
  for our sync service progress reporting.
- Karaoke fallback chain in prototype: Suno aligned lyrics → heuristic
  aligner on plain lyrics → nothing. Our plan adds Whisper into that chain;
  consider keeping the cheap heuristic tier between remote and Whisper
  (audit S-4c).
- Recording used FFmpeg HW-accel encoders with a stats surface (encode fps,
  buffer health) — adopt both ideas for Phase 4/7 export UI (audit S-5).

## 5. Application Mechanics (once approved)

1. Copy sanitized excerpts of the prototype's suno_api docs into
   `docs/captures/raw/recon-from-chadvis/…` as the evidence trail (doc 06 §5
   convention), each file front-matter-tagged with its tier.
2. Apply §3's table into doc 06 §2 entries using the §1 template, all marked
   `Captured: LEAD (recon 2026-04/08, needs confirming capture)` except where
   the orchestrator provides fresh Burp captures during Phase 0/1.
3. Update docs 00/04/05/07 per audit S-1/S-3/S-4/C-1/C-2 in the same pass.
