# Multi-Account & Download Manager — Detail

> **Last Updated:** 2026-08-25 · **Status:** Active
>
> **Status: Superseded (2026-08-25)** — content folded into doc 05 (§7 account lifecycle states) and doc 07 (§3 sync cursors, §4 download policy). Retained for history.

## 1. Scope

Expands on doc 05 (auth mechanics) and doc 07 §§2-4 (schema) with the
operational/UX detail needed to implement Phase 1's account-switching and
download-manager features concretely.

## 2. Account Lifecycle States

An account (`accounts` row, doc 07 §2) moves through:

```
added (credential captured, profile not yet fetched)
   → active_valid (profile fetched, API calls succeeding)
   → needs_reauth (401s that refresh couldn't resolve, per doc 05 §4)
   → removed (soft-deleted, deleted_at set)
```

The UI must clearly distinguish `active_valid` from `needs_reauth` in the
account switcher (doc 08 §5) — e.g. a small warning badge on that
account's entry — since a multi-account power user needs to know at a
glance which of their several accounts requires attention, without it
blocking use of their other, still-valid accounts.

## 3. Adding an Account — Flow

1. User initiates "Add Account" from the account management screen
   (`ui-screen-account-management`).
2. User picks a method: **Paste Token** (always available), **Suno Login**
   (embedded webview), or **Sign in with Google/Facebook** (system browser
   + loopback).
3. On successful credential capture (any method), the app immediately
   makes one lightweight authenticated call (the account/profile endpoint,
   once captured per doc 06 §2.6) to (a) verify the credential actually
   works before committing it, and (b) populate `display_name`/
   `avatar_url`/`suno_user_id` — never save an account row with unverified
   credentials.
4. New account is added to `account-profile-store`; user chooses whether
   to make it active immediately or just add it alongside existing
   accounts.
5. First library sync for a newly added account happens automatically in
   the background (via `suno-library-sync-service`) so switching to it
   later doesn't present an empty library while a first sync churns.

## 4. Switching Accounts — UX Detail (expands doc 05 §5)

- Switching is a single click/selection in the always-visible account
  switcher (doc 08 §5) — no re-navigation, no loading-screen interstitial
  if that account's library cache already exists (instant swap); a brief,
  clearly-indicated background refresh may still occur to catch up on
  remote changes since last sync, but the UI should render the last-known
  cached state immediately rather than blanking while that refresh
  completes.
- Any in-progress operation scoped to the *previous* active account (e.g.
  a download queued from that account) continues running in the
  background — switching active account for browsing purposes must not
  cancel unrelated in-flight operations tied to another account.

## 5. Download Manager — Operational Detail

- Downloads are queued (not immediately fired) with the concurrency model
  matching what doc 13 §4 established for automation renders — a
  configurable max-concurrent-downloads setting (default modest, e.g. 3),
  since simultaneous large downloads can saturate bandwidth or hit
  server-side rate limits (watch for this in Phase 1's real usage and
  document observed behavior in doc 06 §2.7 if Suno's CDN pushes back).
- **Retry policy:** failed downloads (`status = 'failed'`) get an
  exponential-backoff automatic retry up to a small cap (e.g. 3 attempts,
  tracked via `attempt_count`), then surface as needing manual user
  action (a visible "retry" button) rather than retrying forever silently.
- **Resumability:** partial downloads (`bytes_downloaded` < `bytes_total`)
  should resume via HTTP range requests if the audio CDN supports them
  (confirm via capture/testing in Phase 1 — if not supported, downloads
  restart from zero on retry, which is acceptable but should be noted in
  doc 06 as an observed CDN limitation rather than assumed).
- **Storage location:** a user-configurable local library root folder
  (settings screen), with a sane per-platform default (e.g. platform
  "Music" folder or an app-specific subfolder — let the user decide which
  convention they prefer at setup, don't force one silently).
- **File naming:** a configurable template (mirroring doc 13 §3's export
  path template mechanism — reuse the same tiny templating logic if
  practical, per doc 18 §2.2's duplicate-logic guardrail, rather than
  writing two separate path-templating implementations for downloads vs.
  pipeline exports).

## 6. Bulk Library Operations — Scope for Phase 1

`suno-bulk-library-operations-service` initially supports whatever the
first real doc 06 capture reveals is *actually* bulk-capable server-side
(e.g. bulk tag, bulk delete) — if the real API only supports single-item
operations, this service still provides a "bulk" UX by looping
client-side with clear per-item progress/failure reporting (same
per-item-isolation principle as doc 13 §4's pipeline runs — don't let one
item's failure abort a 200-item bulk tag operation). Document explicitly
in doc 06 §2.5 which reality applies once known, rather than assuming
native bulk-endpoint support upfront.

## 7. Data Ownership Clarification (ties to doc 07 §3-4)

- `remote_tracks` rows are account-scoped cache entries — the same
  underlying Suno track visible to two different local accounts (unlikely
  but possible, e.g. a track shared/duplicated across accounts) gets two
  separate cache rows, not a deduplicated shared row — simplicity over
  premature cross-account deduplication (which would require confidently
  identifying "same track" across accounts, itself non-trivial and not a
  named product requirement).
- `downloads` rows reference a specific `remote_tracks` row (thus
  implicitly a specific account), even though the resulting
  `local_file_path` lives in one shared local library folder — if the
  same audio is downloaded once per account by coincidence, that's two
  local files for now (a future dedup-by-content-hash optimization is a
  reasonable backlog item, not a v1 concern).
