# Recon Docs from chadvis-projectm-qt (Provenance & Trust Tiers)

## Provenance

These markdown files are recon documents recovered from the predecessor
prototype repo **chadvis-projectm-qt**, dated approximately **2026-04**
(with updates through 2026-08). They document reverse-engineered Suno API
endpoints observed in the production web app and related services. They were
copied here (with a secret-scan/redaction pass applied — no live JWTs or
long token/cookie values were found) so that this project has a provenance-
trail for the LEAD entries in `docs/06-suno-api-integration-contract.md`.

`SOURCE-README.md` is the original index README from the source repo,
renamed to avoid colliding with this file.

## Reliability Tiers

Every endpoint mentioned below carries one of three provenance tiers:

- **T1 — exercised by prototype code.** The predecessor prototype called
  these successfully at runtime. Examples: the Clerk cookie→JWT exchange
  (`POST /v1/client/sessions/{sid}/tokens`, `GET /v1/client?_is_native=true`),
  feed v3 cursor pagination, clip download
  (`/api/billing/clips/{clip_id}/download/`), aligned lyrics
  (`/api/gen/{id}/aligned_lyrics/v2/`).
- **T2 — documented from real traffic.** Observed via real request/response
  inspection but not exercised end-to-end by prototype code. Examples:
  uploads flow, playlists CRUD, billing info.
- **T3 — scan-derived / inventory-only.** Listed in endpoint inventories or
  subdomain scans without verified traffic evidence. Examples: most of
  `ENDPOINT-INVENTORY.md`, B-side/Labs routes.

## Trust Rule

**Every entry here is a LEAD until one fresh Burp capture confirms it.**
No implementation may depend on an endpoint documented only in these files;
per doc 03 §7 and `docs/06-suno-api-integration-contract.md`, a fresh
confirming capture of each endpoint must be provided before it is wired into
`suno-http-client-core`.
