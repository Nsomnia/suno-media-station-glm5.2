**UI/UX Design System**

**1. Aesthetic Direction**

**"Modern glass."** Translucent, blurred-background panels with soft
borders and subtle depth (layered elevation via blur + slight shadow, not
skeuomorphic gloss), floating over content — well suited to a visualizer-
centric app where the visualizer itself is often the most visually "loud"
element and UI chrome should feel like it's sitting *above* the show, not
competing with it.

**2. Theme Inventory (ship-day-one set)**

- **Catppuccin Latte** (light)
- **Catppuccin Frappé** (dark, muted)
- **Catppuccin Macchiato** (dark, richer)
- **Catppuccin Mocha** (dark, default) — **this is the app's default theme**
- **Monokai** (dark, high-contrast accent colors — for users who want a more
  "classic dev tool" punch of color over Catppuccin's softness)

Each theme is a `DesignTokens` value (see §3) produced from the
`design-tokens-theme-definitions` crate; for Catppuccin, source the actual
published palette values from the official `catppuccin/catppuccin` palette
spec (via `gh`/crates.io — a `catppuccin` Rust crate already exists and
should be used rather than hand-transcribing hex codes) per doc 03 §3's
prior-art mandate. Monokai's palette should similarly be sourced from an
existing well-known reference rather than invented.

**3. Design Token Model**

`design-tokens-theme-definitions` is pure data — no UI framework
dependency — so it can be consumed by whatever the UI layer ends up being
(egui now, potentially something else for a bolted-on surface later).

```rust
pub struct DesignTokens {
    pub name: &'static str,             // "Catppuccin Mocha", "Monokai", ...
    pub is_dark: bool,

    pub color_background_base: Rgba,
    pub color_background_elevated: Rgba,   // glass-panel fill (pre-alpha; alpha applied separately)
    pub color_surface_glass_alpha: f32,    // translucency amount for glass panels
    pub color_border_subtle: Rgba,
    pub color_text_primary: Rgba,
    pub color_text_secondary: Rgba,
    pub color_text_disabled: Rgba,
    pub color_accent_primary: Rgba,        // primary interactive accent
    pub color_accent_secondary: Rgba,
    pub color_success: Rgba,
    pub color_warning: Rgba,
    pub color_danger: Rgba,

    pub radius_small: f32,
    pub radius_medium: f32,
    pub radius_large: f32,

    pub spacing_unit: f32,   // base spacing unit; other spacing = multiples of this

    pub blur_radius_glass_panel: f32,
    pub elevation_shadow_opacity: f32,
}
```

- **Spacing scale:** multiples of `spacing_unit` (default 4.0px-equivalent):
  1x, 2x, 3x, 4x, 6x, 8x, 12x, 16x — components reference the multiple, not a
  raw pixel value, so density/scaling adjustments are a one-token change.
- **Radius scale:** small (inputs/buttons/chips), medium (cards/panels),
  large (modals/major containers).
- **Glass panel recipe (applies to: side nav, floating toolbars, modal
  dialogs, the canvas editor's property inspector):** background =
  `color_background_elevated` at `color_surface_glass_alpha`, backdrop blur
  = `blur_radius_glass_panel`, 1px border at `color_border_subtle`, subtle
  drop shadow at `elevation_shadow_opacity`. Content directly over the live
  visualizer (in preview/canvas-editor screens) uses glass panels; content
  over a flat background (library browser, settings) may use a lighter/flat
  elevated surface instead — glass is a deliberate accent for
  visualizer-adjacent chrome, not applied blindly everywhere.

**4. Typography**

- One primary UI font (proportional, geometric/humanist sans — e.g. Inter
  or similar widely-available open font, bundled) for all UI chrome.
- One monospace font (for any code/JSON display, e.g. in settings/debug
  views) — e.g. JetBrains Mono or similar.
- Type scale (relative, tokens not raw pixels): `caption`, `body`,
  `body_emphasis`, `heading_small`, `heading_medium`, `heading_large`,
  `display` (used sparingly — e.g. a big now-playing title).
- Karaoke/lyric text on the canvas is a **separate concern** from UI
  typography — it's user-customizable per-scene (font, size, color, style)
  via the canvas editor, not tied to the app's own theme tokens.

**5. Layout Conventions**

- **App shell:** persistent left-side nav (collapsible) for top-level
  sections (Library, Player/Visualizer, Canvas Editor, Automation, Studio,
  Settings) + a top bar hosting the account switcher (always visible, per
  doc 05 §5) and global search.
- **Screen composition:** each `ui-screen-*` crate owns one top-level
  screen; screens compose widgets from `ui-shared-widget-library`, never
  duplicate a widget's implementation locally — if a screen needs a widget
  that doesn't exist yet, add it to the shared library crate, don't inline
  a one-off copy.
- **Responsive behavior:** desktop-first, but the app shell should tolerate
  reasonable window resizing gracefully (collapsing nav to icons-only below
  a width threshold, etc) — full mobile/tablet responsiveness is explicitly
  out of scope (this is a desktop app).
- **Modal usage:** sparing — prefer inline panels/drawers over modal dialogs
  where the task isn't truly blocking (e.g., "edit track tags" = inline
  popover, not a modal; "confirm delete" = modal, since it's a genuine
  interrupt).

**6. Motion / Animation Guidelines**

- UI chrome transitions (panel open/close, hover states, theme switch)
  should be quick and subtle — target ~120-200ms, ease-out — never
  block interaction waiting for an animation to finish.
- The canvas editor's own keyframe animation system (doc 04 Phase 5) is a
  **user-authored content feature**, entirely separate from this section's
  "UI chrome polish" concern — don't conflate the two when implementing.
- Respect a system-level "reduce motion" preference where the OS/platform
  exposes one; fall back to instant transitions for chrome (not for
  user-authored canvas content, which always plays as designed).

**7. Iconography**

- Use a single consistent open-source icon set (evaluate via prior-art
  search per doc 03 §3 — e.g. `lucide` or `phosphor` icon sets have Rust-
  friendly SVG/font distributions) rather than mixing icon sources.
- Icons follow the same accent/neutral color tokens as text — no
  hardcoded icon colors outside the token system.

**8. Accessibility Baseline (v1 minimum bar)**

- All interactive elements reachable via keyboard (tab order sane, no
  keyboard traps).
- Color tokens chosen/verified for reasonable contrast on both light
  (Latte) and dark (Mocha/Frappé/Macchiato/Monokai) themes — spot-check
  text-on-glass-panel contrast specifically, since translucency can degrade
  contrast versus the palette's "on solid background" intent.
- No motion-only signaling for critical state (e.g., download failure must
  also have a text/icon indicator, not just a color/animation change).
- Full screen-reader support is a stretch goal, not a v1 blocker, given
  egui's current maturity in that area — note this honestly rather than
  overclaiming.

**9. Component Naming Convention (for `ui-shared-widget-library`)**

Match the verbose-naming philosophy from doc 03: `glass_panel_container.rs`,
`primary_accent_button.rs`, `track_list_row_card.rs`,
`account_switcher_dropdown.rs`, `keyframe_timeline_ruler.rs`, etc — not
`panel.rs`, `button.rs`, `row.rs`.
