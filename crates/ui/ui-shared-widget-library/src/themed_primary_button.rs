//! The first themed primitive: an accent-filled button.
//!
//! Exists to prove the token→widget pattern end to end; more primitives
//! (glass panels, cards, track rows — doc 08 §9 naming scheme) land here as
//! their phases begin.

use design_tokens_theme_definitions::DesignTokens;
use egui::{Button, Response, RichText, Ui};

use crate::token_color_conversion::{contrast_text_color, to_egui_premultiplied};

/// Draws a primary action button filled with the theme's accent color.
///
/// Text color is derived by contrast rather than stored per-theme, so any
/// accent swap stays readable automatically. Returns egui's standard
/// [`Response`] so callers handle interaction themselves.
pub fn themed_accent_button(ui: &mut Ui, tokens: &DesignTokens, label: &str) -> Response {
    let button =
        Button::new(RichText::new(label).color(contrast_text_color(tokens.color_accent_primary)))
            .fill(to_egui_premultiplied(tokens.color_accent_primary))
            .corner_radius(tokens.radius_small.round() as u8);
    ui.add(button)
}
