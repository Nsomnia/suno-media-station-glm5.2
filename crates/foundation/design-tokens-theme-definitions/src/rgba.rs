//! A UI-framework-agnostic RGBA color type.
//!
//! The design-token model in `docs/specs/ui-ux/08-ui-ux-design-system.md` §3
//! refers to an `Rgba` type, but deliberately does not pick a color type from
//! any particular UI stack (egui today, something else possibly later).
//! Defining our own tiny normalized-channel struct keeps this crate pure data
//! and lets each consumer convert to whatever its renderer wants.

use serde::{Deserialize, Serialize};

/// A color stored as four normalized `f32` channels (`0.0..=1.0`).
///
/// Channels are `f32` rather than `u8` so renderers that work in linear or
/// normalized space can consume tokens without per-use conversions; the
/// constructors take care of the common `u8`/hex input case.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgba {
    /// Red channel, normalized `0.0..=1.0`.
    pub r: f32,
    /// Green channel, normalized `0.0..=1.0`.
    pub g: f32,
    /// Blue channel, normalized `0.0..=1.0`.
    pub b: f32,
    /// Alpha (opacity) channel, normalized `0.0..=1.0`.
    pub a: f32,
}

impl Rgba {
    /// Builds an [`Rgba`] from explicit normalized channels.
    ///
    /// Callers are expected to keep channels within `0.0..=1.0`; see the
    /// crate tests for the validity guarantees themes are held to.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Builds a fully opaque [`Rgba`] from 8-bit-per-channel components.
    #[must_use]
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
            1.0,
        )
    }
}
