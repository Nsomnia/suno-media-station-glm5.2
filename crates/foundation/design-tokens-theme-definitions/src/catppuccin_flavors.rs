//! The four Catppuccin themes, built from the official `catppuccin` crate.
//!
//! Prior-art mandate (constitution §3): the hex/RGB values below come
//! straight from [`catppuccin::PALETTE`] — the maintained official Rust
//! palette crate (MIT) — rather than hand-transcribed constants. Only the
//! *semantic mapping* (which Catppuccin role plays which design-system role)
//! is decided here.

use catppuccin::{Flavor, FlavorName};

use crate::design_tokens::DesignTokens;
use crate::rgba::Rgba;

/// Converts one Catppuccin palette entry into our normalized `Rgba`.
fn rgba(color: &catppuccin::Color) -> Rgba {
    Rgba::from_rgb8(color.rgb.r, color.rgb.g, color.rgb.b)
}

/// Maps one flavor onto the semantic token roles of doc 08 §3.
///
/// Mapping rationale (inferred where doc 08 does not pin a value):
/// - `base` → background, `surface0` → elevated fill (one step above base in
///   both light and dark flavors), `overlay0` → subtle border.
/// - `text` / `subtext0` / `overlay1` → primary/secondary/disabled text,
///   giving three clearly separated contrast levels.
/// - `mauve` → primary accent (Catppuccin's signature purple), `blue` →
///   secondary accent; `green`/`yellow`/`red` map to success/warning/danger.
/// - Glass alpha, blur, and shadow strength are tuned per flavor: lighter
///   themes need more panel opacity (and less shadow) to stay legible.
fn tokens_from_flavor(flavor: &Flavor) -> DesignTokens {
    let c = &flavor.colors;
    let name = match flavor.name {
        FlavorName::Latte => "Catppuccin Latte",
        FlavorName::Frappe => "Catppuccin Frappé",
        FlavorName::Macchiato => "Catppuccin Macchiato",
        FlavorName::Mocha => "Catppuccin Mocha",
    };

    // (glass_alpha, blur_radius, shadow_opacity) per flavor. Latte is light
    // and needs the most opaque panels; darker flavors can afford more
    // translucency and stronger depth shadows.
    let (glass_alpha, blur_radius, shadow_opacity) = match flavor.name {
        FlavorName::Latte => (0.80, 12.0, 0.15),
        FlavorName::Frappe => (0.74, 16.0, 0.35),
        FlavorName::Macchiato => (0.72, 18.0, 0.40),
        FlavorName::Mocha => (0.70, 20.0, 0.45),
    };

    DesignTokens {
        name: name.to_string(),
        is_dark: flavor.dark,
        color_background_base: rgba(&c.base),
        color_background_elevated: rgba(&c.surface0),
        color_surface_glass_alpha: glass_alpha,
        color_border_subtle: rgba(&c.overlay0),
        color_text_primary: rgba(&c.text),
        color_text_secondary: rgba(&c.subtext0),
        color_text_disabled: rgba(&c.overlay1),
        color_accent_primary: rgba(&c.mauve),
        color_accent_secondary: rgba(&c.blue),
        color_success: rgba(&c.green),
        color_warning: rgba(&c.yellow),
        color_danger: rgba(&c.red),
        radius_small: 6.0,
        radius_medium: 12.0,
        radius_large: 20.0,
        spacing_unit: 4.0,
        blur_radius_glass_panel: blur_radius,
        elevation_shadow_opacity: shadow_opacity,
    }
}

/// **Catppuccin Latte** — the light theme (doc 08 §2).
#[must_use]
pub fn catppuccin_latte() -> DesignTokens {
    tokens_from_flavor(&catppuccin::PALETTE.latte)
}

/// **Catppuccin Frappé** — dark, muted (doc 08 §2).
#[must_use]
pub fn catppuccin_frappe() -> DesignTokens {
    tokens_from_flavor(&catppuccin::PALETTE.frappe)
}

/// **Catppuccin Macchiato** — dark, richer (doc 08 §2).
#[must_use]
pub fn catppuccin_macchiato() -> DesignTokens {
    tokens_from_flavor(&catppuccin::PALETTE.macchiato)
}

/// **Catppuccin Mocha** — dark; this is the app's default theme (doc 08 §2).
#[must_use]
pub fn catppuccin_mocha() -> DesignTokens {
    tokens_from_flavor(&catppuccin::PALETTE.mocha)
}
