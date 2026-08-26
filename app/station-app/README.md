# station-app

**Purpose:** The binary crate and composition root for Suno Media Station.

**Layer:** app (composition root — may depend on any workspace layer)

**Phase:** Phase 0 — see `docs/product/04-phase-roadmap.md`

**Key dependencies:** `app-configuration-loader`,
`design-tokens-theme-definitions`, `structured-logging-and-tracing`,
`ui-app-shell-and-navigation`, `eframe` (glow renderer per ADR-013),
`tracing`

**Depended on by:** nothing — this is the top of the dependency graph

## Behavior

This crate implements no features; it only wires workspace crates together
at startup:

1. **Config** (`startup.rs`): loads `<OS config dir>/SunoMediaStation/
   config.toml` via `load_or_create()`. Any load failure degrades to
   defaults with a stderr warning — a desktop app should still open.
2. **Logging**: maps the config's v1 logging section onto
   `LoggingSettings` and installs the process-global tracing subscriber;
   the returned `LoggingGuard` is held for the process lifetime in
   `main.rs`.
3. **Initial theme**: resolves the configured theme name against the
   shipped registry, falling back to the default theme for unknown names.
4. **Window** (`bootstrapped_app.rs`): opens the eframe native window
   (glow renderer per ADR-013, 1280×800 default) hosting the UI app shell;
   `ShellState::shell_ui` draws every frame.
5. **Theme persistence**: theme switches inside the shell are persisted by
   rewriting the shared config atomically (direct save; debouncing deferred
   per Phase 0 scope).

## Failure policy

Config problems (including mutex poisoning, handled gracefully without
panics) fall back to defaults/recovered values; logging setup failure is
fatal because every later phase assumes it.
