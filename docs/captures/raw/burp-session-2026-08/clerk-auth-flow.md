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
