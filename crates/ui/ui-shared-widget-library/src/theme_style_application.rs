//! Assembles an egui [`Style`] from [`DesignTokens`] and applies it to a
//! context. This is the write-side of the "single mapping point" contract in
//! the crate docs: screen code calls [`apply_theme`] (or draws themed
//! widgets from this crate) and never touches raw colors itself.

use design_tokens_theme_definitions::DesignTokens;
use egui::{Context, CornerRadius, Margin, Style, Vec2, Visuals, style::WidgetVisuals};

use crate::token_color_conversion::to_egui_premultiplied;

/// Builds a complete egui style (visuals + spacing) from theme tokens.
#[must_use]
pub fn build_style_from_tokens(tokens: &DesignTokens) -> Style {
    let mut style = Style {
        visuals: build_visuals(tokens),
        ..Style::default()
    };
    apply_spacing_tokens(&mut style, tokens);
    style
}

/// Applies `tokens` to `ctx`, replacing its global style wholesale.
pub fn apply_theme(ctx: &Context, tokens: &DesignTokens) {
    ctx.set_global_style(build_style_from_tokens(tokens));
}

/// Maps the token palette onto egui's visuals (colors, rounding, dark mode).
fn build_visuals(tokens: &DesignTokens) -> Visuals {
    let mut visuals = if tokens.is_dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    visuals.dark_mode = tokens.is_dark;

    let background_base = to_egui_premultiplied(tokens.color_background_base);
    let elevated = to_egui_premultiplied(tokens.color_background_elevated);

    visuals.panel_fill = background_base;
    visuals.window_fill = elevated;
    visuals.extreme_bg_color = background_base;
    visuals.override_text_color = Some(to_egui_premultiplied(tokens.color_text_primary));
    visuals.hyperlink_color = to_egui_premultiplied(tokens.color_accent_secondary);

    visuals.selection.bg_fill = to_egui_premultiplied(tokens.color_accent_primary);
    visuals.selection.stroke.color = to_egui_premultiplied(tokens.color_accent_primary);

    let large_rounding = CornerRadius::same(tokens.radius_large.round() as u8);
    visuals.window_corner_radius = large_rounding;
    visuals.menu_corner_radius = large_rounding;

    for widget in [
        &mut visuals.widgets.noninteractive,
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.open,
    ] {
        *widget = themed_widget_visuals(*widget, tokens);
    }
    visuals
}

/// Re-colors/re-rounds one of egui's five widget-state visual slots while
/// keeping egui's own state-dependent bg/stroke strength choices.
fn themed_widget_visuals(mut widget: WidgetVisuals, tokens: &DesignTokens) -> WidgetVisuals {
    widget.corner_radius = CornerRadius::same(tokens.radius_small.round() as u8);
    widget.fg_stroke.color = to_egui_premultiplied(tokens.color_text_primary);
    widget
}

/// Derives egui's spacing metrics from the token spacing scale (doc 08 §3):
/// item gaps use the 2x multiple, buttons the 2x/1x pair.
fn apply_spacing_tokens(style: &mut Style, tokens: &DesignTokens) {
    let unit = tokens.spacing_unit;
    style.spacing.item_spacing = Vec2::splat(unit * 2.0);
    style.spacing.button_padding = Vec2::new(unit * 2.0, unit);
    // Margin channels are integer pixels in egui 0.34 (i8), so fractional
    // spacing units round to the nearest whole pixel here.
    style.spacing.menu_margin = Margin::same((unit * 2.0).round() as i8);
    style.spacing.indent = unit * 4.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use design_tokens_theme_definitions::all_themes;

    #[test]
    fn every_shipped_theme_maps_without_panic() {
        for tokens in all_themes() {
            let style = build_style_from_tokens(&tokens);
            assert_eq!(style.visuals.dark_mode, tokens.is_dark);
            assert!(style.spacing.item_spacing.x > 0.0);
        }
    }

    #[test]
    fn is_dark_token_drives_egui_dark_mode() {
        for tokens in all_themes() {
            let style = build_style_from_tokens(&tokens);
            assert_eq!(
                style.visuals.dark_mode, tokens.is_dark,
                "theme {} mismatched",
                tokens.name
            );
        }
    }

    #[test]
    fn panel_fill_follows_theme_background() {
        let tokens = design_tokens_theme_definitions::default_theme();
        let style = build_style_from_tokens(&tokens);
        assert_eq!(
            style.visuals.panel_fill,
            to_egui_premultiplied(tokens.color_background_base)
        );
    }
}
