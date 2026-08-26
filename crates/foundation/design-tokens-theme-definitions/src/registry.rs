//! The theme registry: the day-one theme set and the default-theme choice.
//!
//! Doc 08 §2 fixes the ship-day-one inventory and names Catppuccin Mocha as
//! the app's default theme; this module is the single place that knowledge
//! lives in code.

use crate::catppuccin_flavors::{
    catppuccin_frappe, catppuccin_latte, catppuccin_macchiato, catppuccin_mocha,
};
use crate::design_tokens::DesignTokens;
use crate::monokai::monokai_classic;

/// Every theme that ships on day one (doc 08 §2), in display order.
#[must_use]
pub fn all_themes() -> Vec<DesignTokens> {
    vec![
        catppuccin_latte(),
        catppuccin_frappe(),
        catppuccin_macchiato(),
        catppuccin_mocha(),
        monokai_classic(),
    ]
}

/// The app's default theme: **Catppuccin Mocha** (doc 08 §2).
#[must_use]
pub fn default_theme() -> DesignTokens {
    catppuccin_mocha()
}

/// Looks a shipped theme up by its exact [`DesignTokens::name`].
#[must_use]
pub fn theme_by_name(name: &str) -> Option<DesignTokens> {
    all_themes().into_iter().find(|theme| theme.name == name)
}
