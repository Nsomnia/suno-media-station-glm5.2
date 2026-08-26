//! Purpose: buttons/cards/glass-panel primitives, themed.
//!
//! # Single token→egui mapping point
//!
//! This crate is the **only** place in the codebase allowed to translate
//! [`DesignTokens`](design_tokens_theme_definitions::DesignTokens) into
//! concrete egui types ([`egui::Color32`], [`egui::Visuals`], [`egui::Style`]).
//! Screen crates must consume the themed primitives from here and never
//! hardcode colors, fonts, or spacing values themselves (doc 08 §1/§9,
//! `crates/ui/AGENTS.md`).
//!
//! This crate does NOT own theme data — themes are pure data produced by
//! `design-tokens-theme-definitions`; this crate only knows how to *apply*
//! them to an egui context.

pub mod theme_style_application;
pub mod themed_primary_button;
pub mod token_color_conversion;

pub use theme_style_application::{apply_theme, build_style_from_tokens};
pub use themed_primary_button::themed_accent_button;
pub use token_color_conversion::{contrast_text_color, to_egui_premultiplied};
