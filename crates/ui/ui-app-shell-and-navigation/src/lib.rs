//! Purpose: window, top nav/routing, layout skeleton.
//!
//! This crate owns the persistent app chrome: the left navigation rail, the
//! central content area, and the theme-switcher control. It does NOT own any
//! screen content — each top-level destination is a placeholder until its
//! `ui-screen-*` crate's phase begins.
//!
//! Screens eventually depend only on application-services handles; Phase 0
//! placeholders hold no store/bridge dependencies at all (layering, doc 01 §3).
//! All theming flows through `ui-shared-widget-library`'s token application —
//! no hardcoded colors here.

mod nav_destinations;
mod shell_layout;
mod shell_state;
mod theme_switcher;

pub use nav_destinations::NavDestination;
pub use shell_state::ShellState;
