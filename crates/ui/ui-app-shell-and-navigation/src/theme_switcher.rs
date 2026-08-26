//! The theme-switcher dropdown, docked at the bottom of the navigation rail.
//!
//! Lists every shipped theme name from the registry; switching routes
//! through [`ShellState::switch_theme`] so the composition root's
//! persistence handler fires (direct save — debounce deliberately deferred
//! per Phase 0 scope).

use egui::{ComboBox, Ui};

use crate::shell_state::ShellState;

/// Draws the theme-switcher control (called from the nav rail's bottom-up
/// region, so this appends upward from the rail's bottom edge).
pub fn theme_switcher_control(ui: &mut Ui, state: &mut ShellState) {
    ui.add_space(12.0);
    let theme_names = state.available_theme_names().to_vec();
    ComboBox::from_label("Theme")
        .selected_text(state.active_theme().name.as_str())
        .show_ui(ui, |ui| {
            for name in theme_names {
                let is_active = name == state.active_theme().name;
                if ui.selectable_label(is_active, name.clone()).clicked() {
                    state.switch_theme(&name);
                }
            }
        });
}
