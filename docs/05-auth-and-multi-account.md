**Authentication & Multi-Account Design**

**1. Why Auth Is Split Three Ways**

Suno's login surface covers native email/password AND federated login
(Google/Facebook). These cannot share one capture mechanism:

- **Native login** happens entirely on suno.com's own pages — an embedded
  webview can render this and intercept the resulting network traffic/cookies
  without issue, since Suno itself isn't blocking embedded rendering (Suno's
  page, not Google's).
- **Federated login (Google/Facebook)** hits the identity provider's own
  consent screens, which actively detect and block known embedded-webview
  user-agents (Google's `disallowed_useragent` error is the classic symptom —
  the same failure Electron and Qt `QWebEngineView`-based tools hit). The only
  reliable path is the **system's real default browser**, with the OAuth
  `redirect_uri` pointed at a temporary localhost server the app spins up to
  catch the callback ("installed application" OAuth pattern).
- **Manual paste** (bearer JWT + refresh cookie captured via a browser JS
  snippet, or from a Burp Suite capture) is always available as a fallback
  and is the **primary bootstrap path** during early development, since it
  requires no auth-flow code at all to start integrating against real Suno
  data.

**2. Crate Responsibilities**

- `suno-auth-manual-token-paste` — parses a pasted blob (JWT + cookie string,
  format TBD from first real capture) into a normalized `SunoCredential`
  struct. No network activity of its own.
- `suno-auth-embedded-webview-login` — opens a `wry`-based webview at Suno's
  login URL, watches outgoing requests/response `Set-Cookie` headers (or a
  post-login redirect target) for the bearer token + refresh cookie, then
  closes the webview and hands back a `SunoCredential`.
- `suno-auth-oauth-loopback-google-fb` — starts a short-lived
  `127.0.0.1:{ephemeral-or-fixed-port}` HTTP listener, opens the federated
  provider's OAuth URL in the **system default browser** via the `open` (or
  `webbrowser`) crate, receives the callback (auth code and/or token
  depending on Suno's flow shape — TBD from capture), exchanges/relays as
  needed, and hands back a `SunoCredential`. Shuts the listener down
  immediately after success or timeout.
- All three produce the same normalized output type so
  `suno-http-client-core` and `account-profile-store` never need to know
  which method produced a given account's credential.

```rust
// illustrative shape only — finalize once first real capture is in hand
pub struct SunoCredential {
    pub bearer_token: String,
    pub refresh_cookie: Option<String>,
    pub obtained_via: AuthMethod, // ManualPaste | EmbeddedWebview | OAuthLoopback
    pub captured_at: chrono::DateTime<chrono::Utc>,
}
```

**3. Storage & Security**

- `SunoCredential` values are never stored in SQLite in plaintext. They go
  into the OS keyring via `os-keyring-secret-storage`, keyed by an opaque
  account-id; `account-profile-store` (SQLite) holds only the account-id,
  display name, auth method, and non-sensitive cached profile metadata
  (username/avatar/etc, for display purposes).
- Encrypted-file fallback (for platforms/environments without a usable OS
  keyring, e.g. some Linux headless setups) uses a locally-generated key
  file with restrictive filesystem permissions — documented as a reduced-
  security mode in the settings UI, not silently equivalent to keyring
  storage.
- Logs must never contain full tokens/cookies. `structured-logging-and-
  tracing` setup should include a redaction layer/newtype
  (`SecretString`-style, e.g. via the `secrecy` crate) so accidental
  `{:?}`-formatting of a credential struct doesn't leak it into logs.

**4. Token Refresh Strategy**

- On any Suno API call returning 401, `suno-http-client-core` attempts a
  refresh using the stored refresh cookie (mechanism TBD — confirm from
  capture whether this is a distinct refresh endpoint or simply re-hitting
  a session endpoint with the cookie attached).
- If refresh succeeds, the new bearer token replaces the stored one
  transparently and the original request is retried once.
- If refresh fails or no refresh cookie is available for that account, the
  UI surfaces a clear "please re-authenticate this account" prompt scoped to
  that specific account (not a global logout) — critical in a multi-account
  app where other accounts may still have valid sessions.
- **This entire section is provisional until doc 06 has a real capture of
  Suno's refresh behavior.** Do not implement speculative refresh-endpoint
  code before that capture exists (per doc 03 §7).

**5. Multi-Account Switching Mechanics**

- `account-profile-store` tracks all known accounts + which one is "active"
  for the current app session.
- Switching accounts:
  1. Swap the active credential handle used to construct/parameterize the
     `suno-http-client-core` instance (the client is per-account-scoped, not
     a single global singleton with mutable auth state).
  2. Re-key/refresh the views backed by `suno-remote-library-cache-store`
     (which is itself keyed by account-id, so cached data for inactive
     accounts isn't discarded — switching back is instant/cheap, not a
     re-fetch).
  3. Local downloads (`local-download-manager-store`) are also account-
     scoped in their metadata (a downloaded track remembers which account it
     came from) even though the files themselves live in one shared local
     library folder — this matters for re-sync/dedup logic later.
- UI: an account switcher control (avatar/name dropdown or similar, exact
  design deferred to doc 08) always visible in the app shell, not buried in
  settings — this is a named power-user feature, not an edge case.

**6. Bootstrap Workflow During Development**

Until the embedded-webview and OAuth-loopback flows are built and verified,
all Suno integration work proceeds against credentials obtained via:

1. Human performs a real login in their own browser (or via Burp Suite as a
   proxy) and captures the resulting requests.
2. Human provides the **sanitized** capture (see doc 06 §1 for the
   redaction rule) to the agent as the ground truth for both the API
   contract AND to manually test `suno-auth-manual-token-paste` end to end.
3. `suno-auth-manual-token-paste`'s UI (a simple paste box in the account
   management screen) is therefore effectively a **Phase 1 priority-one**
   deliverable, even though it's the "least glamorous" of the three auth
   methods — everything else depends on it existing first for testing.
