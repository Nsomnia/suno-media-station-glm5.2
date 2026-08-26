//! Purpose: Catppuccin/Monokai token structs, no UI code.
//!
//! This crate does NOT contain any rendering or widget code — it is pure data definitions.
//!
//! Status: Implemented (Phase 0). Ships the five day-one themes from
//! `docs/specs/ui-ux/08-ui-ux-design-system.md` §2 as [`DesignTokens`]
//! values: Catppuccin Latte, Frappé, Macchiato, Mocha (default), and
//! Monokai. Catppuccin palette values are sourced at compile time from the
//! official [`catppuccin`](https://crates.io/crates/catppuccin) crate per the
//! prior-art mandate; Monokai values are embedded constants citing their
//! published source. Every theme serializes via `serde`, and matching
//! `.toml` token sources live in `assets/themes/`.

mod catppuccin_flavors;
mod design_tokens;
mod monokai;
mod registry;
mod rgba;

pub use catppuccin_flavors::{
    catppuccin_frappe, catppuccin_latte, catppuccin_macchiato, catppuccin_mocha,
};
pub use design_tokens::DesignTokens;
pub use monokai::monokai_classic;
pub use registry::{all_themes, default_theme, theme_by_name};
pub use rgba::Rgba;

#[cfg(test)]
mod tests;
