//! The [`DesignTokens`] struct: the single theme payload defined by
//! `docs/specs/ui-ux/08-ui-ux-design-system.md` §3.
//!
//! One `DesignTokens` value fully describes a theme. It is plain data with
//! `serde` derives so themes round-trip through TOML/JSON (see
//! `assets/themes/*.toml`) and can be consumed by any UI crate without this
//! crate knowing about UI frameworks.

use serde::{Deserialize, Serialize};

use crate::rgba::Rgba;

/// The complete token set for one theme (doc 08 §3).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignTokens {
    /// Human-facing theme name, e.g. `"Catppuccin Mocha"`, `"Monokai"`.
    ///
    /// Inference vs doc 08 §3 (which sketches `&'static str`): stored as
    /// `String` so the struct can derive `Deserialize` and round-trip through
    /// TOML/JSON; constructors still pass `&'static str` names, so nothing is
    /// lost in practice.
    pub name: String,
    /// Whether this is a dark theme (drives default chrome decisions).
    pub is_dark: bool,

    /// Flat app background color.
    pub color_background_base: Rgba,
    /// Glass-panel fill color (pre-alpha; alpha applied via the next token).
    pub color_background_elevated: Rgba,
    /// Translucency amount for glass panels (`0.0..=1.0`).
    pub color_surface_glass_alpha: f32,
    /// 1px hairline border color for glass panels.
    pub color_border_subtle: Rgba,
    /// Primary text color.
    pub color_text_primary: Rgba,
    /// De-emphasized text color (captions, metadata).
    pub color_text_secondary: Rgba,
    /// Disabled-control text color.
    pub color_text_disabled: Rgba,
    /// Primary interactive accent (buttons, selection, focus).
    pub color_accent_primary: Rgba,
    /// Secondary interactive accent (links, alternate highlights).
    pub color_accent_secondary: Rgba,
    /// Positive-state color.
    pub color_success: Rgba,
    /// Caution-state color.
    pub color_warning: Rgba,
    /// Error/destructive-state color.
    pub color_danger: Rgba,

    /// Corner radius for inputs/buttons/chips.
    pub radius_small: f32,
    /// Corner radius for cards/panels.
    pub radius_medium: f32,
    /// Corner radius for modals/major containers.
    pub radius_large: f32,

    /// Base spacing unit; all other spacing is an integer multiple of this.
    pub spacing_unit: f32,

    /// Backdrop blur amount for glass panels (Tier A only — see doc 08 §3).
    pub blur_radius_glass_panel: f32,
    /// Opacity of the drop shadow applied to elevated/glass surfaces
    /// (`0.0..=1.0`).
    pub elevation_shadow_opacity: f32,
}
