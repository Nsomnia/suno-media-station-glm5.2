//! Conversion helpers: design-token [`Rgba`] → egui [`Color32`].
//!
//! Kept separate from style assembly so color math has one small, testable
//! home. Token colors are straight-alpha normalized floats; egui's glow
//! painter blends with premultiplied alpha (`ONE, ONE_MINUS_SRC_ALPHA`), so
//! every conversion here premultiplies — otherwise translucent glass fills
//! would render with fringed/brightened edges.

use design_tokens_theme_definitions::Rgba;
use egui::Color32;

/// Converts a token color into an egui `Color32` with premultiplied alpha.
///
/// For fully opaque colors this is a plain channel rescale; for translucent
/// colors (glass-panel fills) premultiplication matches how the glow painter
/// actually blends, per doc 08 §3's Tier-B translucency recipe.
#[must_use]
pub fn to_egui_premultiplied(color: Rgba) -> Color32 {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * color.a * 255.0).round() as u8;
    Color32::from_rgba_premultiplied(
        channel(color.r),
        channel(color.g),
        channel(color.b),
        (color.a.clamp(0.0, 1.0) * 255.0).round() as u8,
    )
}

/// Picks black or white text for maximal contrast on top of `background`.
///
/// Used for text/icons placed on accent fills (buttons), so accent colors can
/// change freely between themes without hand-picking a foreground per theme.
#[must_use]
pub fn contrast_text_color(background: Rgba) -> Color32 {
    // Rec.601-style relative luminance; good enough for a binary choice.
    let luminance = 0.299 * background.r + 0.587 * background.g + 0.114 * background.b;
    if luminance > 0.6 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_colors_rescale_channels() {
        let color = to_egui_premultiplied(Rgba::from_rgb8(255, 128, 0));
        assert_eq!(color, Color32::from_rgb(255, 128, 0));
    }

    #[test]
    fn translucent_colors_are_premultiplied() {
        let half_white = Rgba::new(1.0, 1.0, 1.0, 0.5);
        let color = to_egui_premultiplied(half_white);
        assert_eq!(color, Color32::from_rgba_premultiplied(128, 128, 128, 128));
    }

    #[test]
    fn contrast_text_flips_on_light_and_dark_backgrounds() {
        assert_eq!(
            contrast_text_color(Rgba::from_rgb8(20, 20, 20)),
            Color32::WHITE
        );
        assert_eq!(
            contrast_text_color(Rgba::from_rgb8(240, 240, 240)),
            Color32::BLACK
        );
    }
}
