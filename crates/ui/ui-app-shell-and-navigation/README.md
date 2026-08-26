# ui-app-shell-and-navigation

**Purpose:** window, top nav/routing, layout skeleton

**Layer:** ui

**Phase:** Phase 0 — see `docs/product/04-phase-roadmap.md`

**Public API:** `ShellState` (constructor, `selected_destination()`,
`select_destination()`, `switch_theme()`, `active_theme()`,
`available_theme_names()`, `set_theme_change_handler()`, `shell_ui()`),
`NavDestination`, `shell_layout::draw_shell`

**Key dependencies:** `egui`, `design-tokens-theme-definitions`,
`ui-shared-widget-library`

**Depended on by:** `station-app` (composition root hosts it in the eframe
window)

## Behavior

- Owns the persistent app chrome: left navigation rail (220 px), central
  content area, and a theme-switcher dropdown pinned to the rail's bottom
  edge (doc 08 §5 nav-first layout).
- `ShellState::shell_ui(ui)` draws one full frame: it re-applies the active
  theme via `ui-shared-widget-library` every frame (cheap relative to
  rendering; handles egui context resets and mid-session switches), then
  draws rail + content.
- The ten `NavDestination` entries mirror doc 02's `ui-screen-*` crates
  one-to-one so adding real screens later is a mechanical substitution.
  In Phase 0 they carry selected-state only; the content area shows the
  destination's heading, placeholder description, and an accent button
  (so the QA recipe can eyeball theming on every screen).
- No screen content and no store/bridge dependencies live here (layering,
  doc 01 §3). Theme persistence is delegated out through
  `set_theme_change_handler`, which the composition root wires to its
  config file — the shell itself never touches configuration.
