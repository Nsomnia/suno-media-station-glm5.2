# Security Policy

## Scope

This application handles **Suno.com account credentials** (session cookies,
JWTs) and stores them locally. Reports involving any of the following are in
scope:

- Credential leakage (logs, crash dumps, plaintext persistence — the
  keyring-storage requirement in `docs/specs/suno-integration/05-auth-and-multi-account.md`
  exists precisely to prevent this).
- The embedded-browser / loopback OAuth flows (`suno-auth-*` crates),
  including CSRF state validation.
- Path traversal or unsafe file handling in download/export/playlist-import.
- Unsafe FFI boundaries in the external-bridges layer.

Out of scope: Suno's own server-side security.

## Reporting

Open a [security advisory](https://github.com/Nsomnia/suno-media-station-glm5.2/security/advisories/new)
rather than a public issue. Expect an initial response within a few days —
this is a solo-maintained project; all code is AI-authored and human-reviewed.

## Notes for Agents

Anything touching credentials/secrets requires the dedicated security review
lens per `docs/process/03-agent-constitution.md` §14 — never fold it silently
into general review. Never log tokens/cookies, even redacted, even in tests.
