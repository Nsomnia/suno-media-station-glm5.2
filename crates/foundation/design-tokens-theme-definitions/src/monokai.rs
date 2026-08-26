//! The Monokai theme.
//!
//! Prior-art source (constitution §3.4): the palette constants below are the
//! classic **Monokai** color scheme by Wimer Hazenberg
//! (https://www.monokai.nl/blog/2006/07/15/monokai/), as popularized by
//! TextMate/Sublime Text and reproduced across countless editor ports — not
//! invented here. Roles doc 08 does not pin (elevated surface, border,
//! disabled text) reuse adjacent canonical entries from the same scheme and
//! are marked `// inferred` inline.

use crate::design_tokens::DesignTokens;
use crate::rgba::Rgba;

/// Classic Monokai background (`#272822`).
const BACKGROUND: Rgba = Rgba::new(
    0x27 as f32 / 255.0,
    0x28 as f32 / 255.0,
    0x22 as f32 / 255.0,
    1.0,
);
/// Line-highlight tone used for the elevated/glass fill (`#3e3d32`). // inferred role
const ELEVATED: Rgba = Rgba::new(
    0x3e as f32 / 255.0,
    0x3d as f32 / 255.0,
    0x32 as f32 / 255.0,
    1.0,
);
/// Selection tone used for the subtle border (`#49483e`). // inferred role
const BORDER: Rgba = Rgba::new(
    0x49 as f32 / 255.0,
    0x48 as f32 / 255.0,
    0x3e as f32 / 255.0,
    1.0,
);
/// Classic foreground white (`#f8f8f2`).
const TEXT_PRIMARY: Rgba = Rgba::new(
    0xf8 as f32 / 255.0,
    0xf8 as f32 / 255.0,
    0xf2 as f32 / 255.0,
    1.0,
);
/// Comment gray used for secondary text (`#75715e`).
const TEXT_SECONDARY: Rgba = Rgba::new(
    0x75 as f32 / 255.0,
    0x71 as f32 / 255.0,
    0x5e as f32 / 255.0,
    1.0,
);
/// Darker step between comment gray and the border tone, for disabled text
/// (`#5c5b52`). // inferred value
const TEXT_DISABLED: Rgba = Rgba::new(
    0x5c as f32 / 255.0,
    0x5b as f32 / 255.0,
    0x52 as f32 / 255.0,
    1.0,
);
/// Monokai pink/magenta (`#f92672`) — primary accent.
const ACCENT_PRIMARY: Rgba = Rgba::new(
    0xf9 as f32 / 255.0,
    0x26 as f32 / 255.0,
    0x72 as f32 / 255.0,
    1.0,
);
/// Monokai cyan (`#66d9ef`) — secondary accent.
const ACCENT_SECONDARY: Rgba = Rgba::new(
    0x66 as f32 / 255.0,
    0xd9 as f32 / 255.0,
    0xef as f32 / 255.0,
    1.0,
);
/// Monokai green (`#a6e22e`).
const SUCCESS: Rgba = Rgba::new(
    0xa6 as f32 / 255.0,
    0xe2 as f32 / 255.0,
    0x2e as f32 / 255.0,
    1.0,
);
/// Monokai orange (`#fd971f`) — reads better than string-yellow as a warning on this bg.
const WARNING: Rgba = Rgba::new(
    0xfd as f32 / 255.0,
    0x97 as f32 / 255.0,
    0x1f as f32 / 255.0,
    1.0,
);

/// **Monokai** — dark, high-contrast "classic dev tool" theme (doc 08 §2).
///
/// Glass/shadow values follow the darker end of the Catppuccin dark-flavor
/// scale since Monokai's background is similarly deep; radius and spacing
/// scales are shared with every other theme by design (doc 08 §3).
#[must_use]
pub fn monokai_classic() -> DesignTokens {
    DesignTokens {
        name: "Monokai".to_string(),
        is_dark: true,
        color_background_base: BACKGROUND,
        color_background_elevated: ELEVATED,
        // Slightly more translucent than Mocha: Monokai is the high-contrast,
        // punchy option, so lean into the show-through glass effect. // inferred
        color_surface_glass_alpha: 0.68,
        color_border_subtle: BORDER,
        color_text_primary: TEXT_PRIMARY,
        color_text_secondary: TEXT_SECONDARY,
        color_text_disabled: TEXT_DISABLED,
        color_accent_primary: ACCENT_PRIMARY,
        color_accent_secondary: ACCENT_SECONDARY,
        color_success: SUCCESS,
        color_warning: WARNING,
        color_danger: ACCENT_PRIMARY,
        radius_small: 6.0,
        radius_medium: 12.0,
        radius_large: 20.0,
        spacing_unit: 4.0,
        blur_radius_glass_panel: 20.0,
        elevation_shadow_opacity: 0.50,
    }
}
