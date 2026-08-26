# ui-shared-widget-library

**Purpose:** buttons/cards/glass-panel primitives, themed

**Layer:** ui

**Phase:** Phase 0 — see `docs/product/04-phase-roadmap.md`

**Public API:** `apply_theme(ctx, &tokens)`, `build_style_from_tokens(&tokens)`,
`themed_accent_button(ui, &tokens, label)`, plus color helpers
(`to_egui_premultiplied`, `contrast_text_color`) in `token_color_conversion`

**Key dependencies:** `egui`, `design-tokens-theme-definitions`

**Depended on by:** `ui-app-shell-and-navigation` (and later every
`ui-screen-*` crate)

## Behavior

- **Single token→egui mapping point:** this is the only crate allowed to
  translate `DesignTokens` into concrete egui types (`Color32`, `Visuals`,
  `Style`). Screen crates must consume the themed primitives from here and
  never hardcode colors/fonts/spacing (doc 08 §1/§9, `crates/ui/AGENTS.md`).
- `apply_theme` re-applies a theme to an egui context; callers re-run it
  each frame so context resets and mid-session switches both end up correct.
- `themed_accent_button` is the first themed primitive — it proves the
  token→widget pattern end to end (accent fill + contrast-derived text
  color + themed corner radius). Cards, glass panels, and track rows land
  here in later phases (doc 08 §9 naming scheme).
- Text-on-accent colors are derived by contrast at draw time rather than
  stored per-theme, so any accent swap stays readable automatically.

## Not yet implemented

Glass-panel/backdrop-blur widgets await ADR-013's `backdrop-blur-egui`
integration (Phase 5); when it lands, it is pinned exactly and isolated
behind this crate.
