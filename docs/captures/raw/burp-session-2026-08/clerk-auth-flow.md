# Clerk Auth Flow — auth.suno.com (T1 capture, 2026-08-25)

> Sanitized from Burp export `auth.suno.com` (18 items, 13 non-static).
> All cookie/JWT/identifier values redacted — names and shapes only.

## Common request headers (all calls)

- `Cookie: <redacted>` — Clerk cookies observed by NAME: `__client`,
  `__client_uat`, plus suffixed variants `__client_uat_Jnxw-muT`,
  `__session_Jnxw-muT` (suffix appears to be an instance/instance-key marker);
  analytics cookies (`_ga`, `_fbp`, `_ttp`, `__stripe_mid/sid`, `ajs_anonymous_id`, …)
  also present but functionally irrelevant.
- `Origin: https://suno.com`, `Referer: https://suno.com/`
- Browser `User-Agent`, standard `sec-ch-ua*`
- `Content-Type: application/x-www-form-urlencoded` on POSTs (Clerk API is
  form-encoded, NOT JSON)

## Observed sequence (Google OAuth sign-in → session bootstrap)

1. **`POST /v1/client/sign_ins`** → 200
   - Form body: `strategy=oauth_google&redirect_url=https://suno.com/sso-callback?auth_mode=sign-in&action_complete_redirect_url=<post-login url with embedded __clerk_handshake JWT>&…`
   - Response JSON: `{"response": {"object": "sign_in_attempt", "status":
     "needs_first_factor", "first_factor_verification": {"object":
     "verification_oauth", "strategy": "oauth_google",
     "external_verification_redirect_url":
     "https://auth.suno.com/social/login/google-oauth2/?…&__client=<redacted-jwt>",
     …}, …}, "client": {…}}` (client object mirrors the attempt; also a
     parallel `sign_up` object with `status: complete|missing_requirements`)
   - 4 `Set-Cookie` headers returned (values redacted).

2. **`GET /social/login/google-oauth2/?next=…&__client=<redacted-jwt>`** → 302
   to Google (`accounts.google.com`). Note the `__client` query value here is
   itself a JWT whose payload decodes to claims structure:
   `{"suno.com/claims/client_id": "<redacted>", "suno.com/claims/token_type":
   "refresh", "iss": "https://auth.suno.com", "exp": <~1 year out>, "secret":
   "<redacted>"}` — i.e. the Clerk *client token* doubles as a refresh token.

3. Google consent round-trip → **`GET /social/complete/google-oauth2/?state=…&code=<redacted>&scope=email+profile+openid…&prompt=none`**
   → 302 back to suno.com.

4. **`GET /v1/client/handshake?redirect_url=…&__clerk_api_version=2025-04-10&suffixed_cookies=true&__clerk_hs_reason=client-uat-but-no-session-token&format=nonce`**
   → 302, `Location: https://suno.com/<path>?__clerk_handshake=<redacted-jwt>`
   (handshake JWT payload structure: `token_type: "handshake"` + array of
   Set-Cookie strings to replay). This bridges the new session cookies onto
   suno.com. Observed twice.

5. **`GET /v1/client?__clerk_api_version=2025-11-10&_clerk_js_version=5.117.0`**
   → 200
   - Response: `{"response": {"object": "client", "id":
     "<redacted-client-id>", "sessions": [{…}], "sign_in": {…"status":
     "complete"}, "last_active_session_id": "<redacted>", …}}`
   - Each `session` entry contains: `"status": "active"`, `expire_at`
     (~1 year), `factor_verification_age: [0, -1]`, and crucially
     **`last_active_token: {"object": "token", "jwt": "<redacted-jwt>"}`** —
     the Suno API bearer token is delivered right here.
   - Embedded `user` object structure: `id ("user_…")`, `username` (email),
     `email_addresses[]`, `image_url`, flags (`two_factor_enabled`,
     `banned`, `locked`, …), timestamps. All values redacted.
   - Also returns `cookie_expires_at` (~15 min rolling) and `captcha_bypass`.

6. **`POST /v1/client/sessions/{sid}/touch?__clerk_api_version=2025-11-10&_clerk_js_version=5.117.0`**
   → 200, empty body, `Content-Length: 0`
   - Response: same `{response: <session>, client: <client>}` envelope as
     `/v1/client`; the session's `last_active_token.jwt` carries a fresh
     bearer. This is the **observed refresh mechanism**.

7. **`POST /v1/client/verify?__clerk_api_version=2025-11-10&_clerk_js_version=5.117.0`**
   → 204 No Content (observed ×5)
   - Form body: `captcha_token=<redacted turnstile token>&captcha_widget_type=invisible&captcha_action=heartbeat`
     — a Cloudflare Turnstile heartbeat, distinct from Suno's own captcha gate
     (see generation-v2-web.md).

8. **`GET /v1/environment?__clerk_api_version=2025-11-10&_clerk_js_version=5.117.0`**
   → 200
   - Instance config: `auth_config` (enabled identification strategies —
     oauth_apple/discord/facebook/google/microsoft/oauth_token_apple,
     phone_number + phone_code; password on; email_address attribute OFF),
     `display_config` (app name, URLs, Turnstile `captcha_public_key` shapes),
     `user_settings` (full social-provider matrix, password settings, attack
     protection). Static-ish config — cacheable.

## Verdict vs prototype recon ([recon-from-chadvis/auth.md](../recon-from-chadvis/auth.md))

| Recon claim | Capture verdict |
|---|---|
| `POST /v1/client/sessions/{sid}/tokens` is the primary session→JWT exchange | **NOT OBSERVED** in this capture. The bearer arrives via `GET /v1/client` → `sessions[].last_active_token.jwt` and is refreshed via `POST /v1/client/sessions/{sid}/touch` (whose response embeds a fresh `last_active_token.jwt`). The `/tokens` path may still exist server-side but was never exercised — do not build against it. |
| `POST /v1/client/sessions/{sid}/tokens/api` alternate exchange | Not exercised; treat as unconfirmed lead only. |
| `POST /v1/client/sessions/{sid}/touch` = keep-alive heartbeat | CONFIRMED as endpoint, but its role is bigger than "heartbeat": it returns the refreshed JWT. This is the refresh path. |
| `GET /v1/client` returns client state incl. `last_active_session_id` | CONFIRMED (query params actually `__clerk_api_version` + `_clerk_js_version`; `_is_native=true` not used by web client). |
| `GET /v1/client/sync`, `GET/POST /v1/event`, `GET /v1/logs`, `POST /v1/tickets/accept`, `GET /v1/client?_method=PATCH` | Not observed this session — remain leads. |
| `POST /v1/client/verify` | CONFIRMED, but semantics corrected: it is a **captcha heartbeat** (Turnstile token POST, 204 empty), not a generic preflight validity check. |
| `POST /v1/verify` | Not observed (the observed verify is under `/v1/client/verify`). |
| New, not in recon | `GET /v1/client/handshake` (302 handshake bridge), `GET /v1/environment` (instance config), `POST /v1/client/sign_ins` form flow, `/social/login|complete/{provider}` redirect pair. |

## Residual uncertainty

- The capture shows the happy path while a session was already valid
  (handshake reason: `client-uat-but-no-session-token`). What happens when the
  short-lived bearer expires mid-use — silent `touch` refresh vs forced
  re-handshake/re-auth — is **not definitively answered** by this capture.
  The ~15-minute `cookie_expires_at` vs ~1-year session expiry asymmetry
  suggests periodic touch suffices, but plan the client to handle a 401 from
  studio-api by attempting touch, then falling back to re-auth.

## Prototype implementation evidence (added 2026-08-25)

Source: working C++/Qt predecessor `~/Documents/chadvis-projectm-qt`
(read-only, outside repo) — `src/suno/SunoClient.cpp`,
`src/suno/SunoEndpoints.hpp`, `src/suno/SunoAuthManager.{hpp,cpp}`,
`src/suno/SunoAuthFailure.hpp`. This code shipped against production Suno,
so it is behavioral ground truth from the **prototype era** (months before
this capture); the live surface may have moved since.

### Token acquisition paths (three, in priority order)

1. **System-browser auth** (`SunoAuthManager.cpp:47-61`): login callback hands
   over a token string; accepted only if it starts with `eyJ` (i.e. a JWT),
   then persisted via config.
2. **Cookie-jar parse** (`SunoClient.cpp:68-112`, `setCookie`): splits the
   pasted/persisted cookie string into a map, then selects the session cookie
   by precedence: exact `__session` → exact **`__session_Jnxw-muT`**
   (suffixed variant HARDCODED — matches this capture's observation of
   instance-suffixed cookies) → any name starting with `__session`. Value must
   start with `eyJ`; it becomes the bearer directly.
3. **Clerk SID extraction** (`SunoClient.cpp:114-125`): base64url-decodes the
   JWT payload and reads claim **`sid`** → cached as `clerkSid_`. (Note: the
   2026-08-25 capture's studio-api bearer claims did not obviously show `sid`;
   this may be a Clerk session-token vs Suno access-token distinction.)

### Refresh flow (`SunoClient.cpp:131-196`, `refreshAuthToken`)

- **Trigger:** lazy only — `withValidToken` (lines 198-207) refreshes solely
  when `token_` is empty (first authenticated call after restore, or after a
  401 cleared the token). **No proactive TTL/expiry check exists**; the JWT
  `iat`/`exp` are never parsed for timing.
- **Step 1 — discover SID** (when missing): `GET https://clerk.suno.com/v1/client?_is_native=true&_clerk_js_version=5.117.0`
  with raw `Cookie` header and `User-Agent: Mozilla/5.0`; parses
  `response.last_active_session_id`, falling back to `response.sessions[0].id`;
  recurses into step 2 on success.
- **Step 2 — exchange:** `POST` (empty body) to
  `https://clerk.suno.com/v1/client/sessions/{sid}/client?_is_native=true&_clerk_js_version=5.117.0`
  — URL assembled from `CLERK_SESSION = "/client/sessions/"` + sid +
  `CLERK_CLIENT = "/client?_is_native=true&_clerk_js_version="`
  (`SunoEndpoints.hpp:32-33`, `SunoClient.cpp:170-179`). Bearer read from
  response JSON path `jwt`, falling back to `response.jwt` (line 184-185).
- **Host note:** the prototype targets **`clerk.suno.com`**, not
  `auth.suno.com`. The 2026-08-25 capture shows the current web client
  exclusively on `auth.suno.com` (same Cloudflare IP). The Burp export bucket
  named `clerk.suno.com` contains **zero items**, so whether the old
  host+path still works today is unverified.
- **Error paths:** 401/"Unauthorized" classified by `isAuthFailure`
  (`SunoAuthFailure.hpp:13-16`); `handleNetworkError` (`SunoClient.cpp:326-335`)
  clears the token and emits an error — **no automatic retry/re-auth loop**;
  the user must re-authenticate. A `isRefreshingToken_` guard flag is
  declared (`SunoAuthManager.hpp:35`) but never used.

### Reconciliation verdict vs this capture

**Partially superseded — three distinct mechanisms across two eras:**

| Mechanism | Prototype era (code, pre-capture) | Live capture 2026-08-25 |
|---|---|---|
| Host | `clerk.suno.com/v1` | `auth.suno.com/v1` |
| Initial bearer | `__session*` cookie value used AS the JWT | `GET /v1/client` → `sessions[].last_active_token.jwt` |
| Refresh | `POST /v1/client/sessions/{sid}/client?_is_native=true&…` (worked then; unverified now) | `POST /v1/client/sessions/{sid}/touch` |
| `_is_native=true` | sent (native client) | absent (browser client sends `__clerk_api_version` instead) |

Notably, the earlier recon doc (`recon-from-chadvis/auth.md`) claimed the
prototype used `sessions/{sid}/tokens` — **the prototype source contradicts
that**: no `/tokens` path appears anywhere in the code. The recon doc was
inaccurate even about its own corpus. Neither era's mechanism confirms the
other; both `/tokens` and the prototype's `/client`-suffix exchange remain
unverified against today's API.

The prototype also does **not** resolve the silent-refresh question: its
strategy was lazy-refresh-on-empty-token plus manual re-auth after 401 —
there is no evidence of background periodic refresh surviving long sessions
(either way).

### Recommended implementation strategy for suno-http-client-core

1. Primary: `GET https://auth.suno.com/v1/client` (capture-proven) — extract
   `last_active_session_id` and `sessions[].last_active_token.jwt`.
2. Refresh: `POST …/v1/client/sessions/{sid}/touch` (capture-proven), reading
   the fresh JWT from `response.last_active_token.jwt`.
3. Documented-but-unverified fallbacks, in order: Clerk-standard
   `POST …/sessions/{sid}/tokens`, then the prototype-era
   `POST …/sessions/{sid}/client?_is_native=true…`. Log clearly when a
   fallback fires so a fresh capture can be requested.
4. On studio-api 401: clear bearer → one refresh attempt via (2) → prompt
   interactive re-auth if still failing. Add proactive expiry tracking
   (~55 min into the ~1 h JWT life) which the prototype lacked.
5. Keep `_is_native=true` in the toolbox for native flows, but match the
   captured web-client query params (`__clerk_api_version`,
   `_clerk_js_version`) as the default.

