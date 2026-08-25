# Suno Integration Specs

How the app talks to Suno.com.

| Doc | What it covers |
|---|---|
| [05 — Auth & Multi-Account](05-auth-and-multi-account.md) | Suno auth flows & multi-account design |
| [06 — Suno API Integration Contract](06-suno-api-integration-contract.md) | Capture-driven Suno API endpoint contract |

Doc 06 is capture-driven: never invent an endpoint shape — request a capture
(see `captures/`) if one is missing.

Back to the [wiki hub](../../README.md).
