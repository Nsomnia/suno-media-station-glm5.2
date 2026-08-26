# structured-logging-and-tracing

**Purpose:** tracing subscriber setup, log file rotation

**Layer:** foundation

**Phase:** Phase 0 — see `docs/product/04-phase-roadmap.md`

## Public API

- `initialize_structured_logging(&LoggingSettings) -> Result<LoggingGuard, LoggingSetupError>` —
  installs the global `tracing` subscriber (pretty stdout layer + JSON rotating
  file layer) and returns a guard that must be kept alive for the process
  lifetime (flush-on-drop semantics of the non-blocking writer).
- `initialize_structured_logging_with_base_data_directory(...)` — same, with an
  injected base data directory for tests / embedded hosts.
- `LoggingSettings` (serde-friendly) — default filter directive (`RUST_LOG`
  fallback), log-directory override, rotation period, stdout toggle.
- `LogRotationPeriod` — `daily` | `hourly` | `minutely` | `never`.
- `LoggingSetupError` (thiserror).

This crate does not emit application log events; it only emits its own setup
diagnostics. Verbosity is controlled by `RUST_LOG` (default `"info"`).

**Key dependencies:** `tracing`, `tracing-subscriber` (`env-filter`, `json`),
`tracing-appender`, `dirs`, `serde`, `thiserror`

**Depended on by (planned):** `app/station-app` composition root; other crates
log via the installed global subscriber without depending on this crate.
