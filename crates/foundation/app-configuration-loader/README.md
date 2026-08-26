# app-configuration-loader

**Purpose:** reads/writes app config (TOML), env overrides

**Layer:** foundation

**Phase:** Phase 0 — see `docs/product/04-phase-roadmap.md`

**Public API:** `load_or_create()`, `load()`, `AppConfig::save()` /
`save_at(path)`, `ConfigurationError`

**Key dependencies:** `serde`, `toml`, `dirs`, `thiserror` (+ `tempfile`
as dev-dependency)

**Depended on by (planned):** station-app composition root first; later
service crates that need settings values.

## Behavior

- File location: `<OS config dir>/SunoMediaStation/config.toml`
  (macOS: `~/Library/Application Support/…`; Linux/XDG: `$XDG_CONFIG_HOME/…`).
- Precedence: built-in defaults < TOML file < `SMS_`-prefixed environment
  variables (`SMS_THEME_NAME`, `SMS_LOGGING_LEVEL`,
  `SMS_LOGGING_ENABLE_STDOUT`). CLI wiring and debounced live-writeback land
  in a later phase on top of these primitives.
- Versioned schema modules (termusic prior-art pattern): the file carries
  `version = 1`; breaking changes add `config/v2.rs` + a step in
  `migrate_from_previous` instead of mutating the frozen v1 schema.
- Unknown TOML fields are ignored (forward compatible).
- Saves are atomic (temp file + rename). This crate never holds secrets —
  credentials live in `os-keyring-secret-storage`.
