# design-tokens-theme-definitions

**Purpose:** Catppuccin/Monokai token structs, no UI code

**Layer:** foundation

**Phase:** Phase 0 — see `docs/product/04-phase-roadmap.md`

**Public API:** `DesignTokens`, `Rgba`, `all_themes()`, `default_theme()`,
`theme_by_name(name)`, plus per-theme constructors (`catppuccin_latte()`,
`catppuccin_frappe()`, `catppuccin_macchiato()`, `catppuccin_mocha()`,
`monokai_classic()`)

**Key dependencies:** `serde`, `catppuccin` (+ `toml`/`serde_json` as
dev-dependencies for round-trip tests)

**Depended on by:** `ui-shared-widget-library`, `ui-app-shell-and-navigation`,
`station-app`

## Behavior

- Ships the five day-one themes from doc 08 §2: Catppuccin Latte, Frappé,
  Macchiato, Mocha (the default), and Monokai.
- Catppuccin palette values are sourced at compile time from the official
  [`catppuccin`](https://crates.io/crates/catppuccin) crate per the
  prior-art mandate; Monokai values are embedded constants citing their
  published source.
- `DesignTokens` is plain serde data (doc 08 §3's full payload: name,
  dark flag, colors, radii, spacing unit, blur, shadow opacity). This crate
  contains no rendering or widget code — applying tokens to egui lives in
  `ui-shared-widget-library`.
- Matching `.toml` token sources live in `assets/themes/`; tests verify the
  registry themes round-trip through TOML/JSON.

## Known debt

- The `assets/themes/*.toml` files are not yet automatically equality-tested
  against the code-built themes (see "Phase 0 known debt" in
  `docs/product/99-ideas-backlog.md`). The flat ~20-field struct is a
  documented deliberate exception to doc 18 §2.4's 10–12 field heuristic.
