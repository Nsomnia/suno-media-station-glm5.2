//! Crate-level unit tests: theme validity, serde round-trips, and the
//! default-theme contract.

use crate::{DesignTokens, all_themes, default_theme, theme_by_name};

/// Every shipped theme must be numerically sane: finite channels, alphas
/// within range, positive radii, and the shared spacing/radius scales.
#[test]
fn all_constructors_produce_valid_tokens() {
    let themes = all_themes();
    assert_eq!(themes.len(), 5, "doc 08 §2 ships exactly five themes");

    for theme in &themes {
        assert!(!theme.name.is_empty(), "{}: empty name", theme.name);

        for (role, c) in [
            ("background_base", theme.color_background_base),
            ("background_elevated", theme.color_background_elevated),
            ("border_subtle", theme.color_border_subtle),
            ("text_primary", theme.color_text_primary),
            ("text_secondary", theme.color_text_secondary),
            ("text_disabled", theme.color_text_disabled),
            ("accent_primary", theme.color_accent_primary),
            ("accent_secondary", theme.color_accent_secondary),
            ("success", theme.color_success),
            ("warning", theme.color_warning),
            ("danger", theme.color_danger),
        ] {
            for (channel, v) in [("r", c.r), ("g", c.g), ("b", c.b), ("a", c.a)] {
                assert!(
                    v.is_finite(),
                    "{}.{}.{channel} not finite",
                    theme.name,
                    role
                );
                assert!(
                    (0.0..=1.0).contains(&v),
                    "{}.{}.{channel} out of 0..=1: {v}",
                    theme.name,
                    role
                );
            }
        }

        let name = &theme.name;
        for (token, v) in [
            ("color_surface_glass_alpha", theme.color_surface_glass_alpha),
            ("elevation_shadow_opacity", theme.elevation_shadow_opacity),
        ] {
            assert!(v.is_finite(), "{name}.{token} not finite");
            assert!((0.0..=1.0).contains(&v), "{name}.{token} out of 0..=1: {v}");
        }

        for (token, v) in [
            ("radius_small", theme.radius_small),
            ("radius_medium", theme.radius_medium),
            ("radius_large", theme.radius_large),
            ("blur_radius_glass_panel", theme.blur_radius_glass_panel),
        ] {
            assert!(
                v.is_finite() && v > 0.0,
                "{name}.{token} must be > 0, got {v}"
            );
        }

        assert_eq!(
            theme.spacing_unit, 4.0,
            "doc 08 §3 pins spacing_unit at 4.0"
        );

        // Radius scale must stay strictly ordered.
        assert!(theme.radius_small < theme.radius_medium);
        assert!(theme.radius_medium < theme.radius_large);
    }
}

/// Every theme must survive a JSON and a TOML round-trip unchanged.
#[test]
fn serde_round_trip_preserves_values_through_json_and_toml() {
    for theme in all_themes() {
        let json = serde_json::to_string(&theme).expect("serialize to JSON");
        let from_json: DesignTokens = serde_json::from_str(&json).expect("parse from JSON");
        assert_eq!(
            from_json, theme,
            "JSON round-trip diverged for {}",
            theme.name
        );

        let toml = toml::to_string(&theme).expect("serialize to TOML");
        let from_toml: DesignTokens = toml::from_str(&toml).expect("parse from TOML");
        assert_eq!(
            from_toml, theme,
            "TOML round-trip diverged for {}",
            theme.name
        );
    }
}

/// The default-theme accessor must return Catppuccin Mocha (doc 08 §2).
#[test]
fn default_theme_is_catppuccin_mocha() {
    let default = default_theme();
    assert_eq!(default.name, "Catppuccin Mocha");
    assert!(default.is_dark);
    // And it must be the same value the registry lists under that name.
    assert_eq!(theme_by_name("Catppuccin Mocha").as_ref(), Some(&default));
}
