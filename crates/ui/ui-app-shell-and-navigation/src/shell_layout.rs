//! Frame layout of the app chrome: left nav rail + central content area.
//!
//! The nav rail hosts both the destination list (top-aligned) and the
//! theme-switcher control (pinned to the panel's bottom edge).

use egui::{CentralPanel, Label, Panel, RichText, Ui};

use crate::nav_destinations::NavDestination;
use crate::shell_state::ShellState;
use crate::theme_switcher;

/// Draws the persistent shell for one frame.
pub fn draw_shell(ui: &mut Ui, state: &mut ShellState) {
    draw_nav_rail(ui, state);
    draw_content_area(ui, state);
}

/// Left navigation rail (doc 08 §5): one entry per top-level destination,
/// carrying selected-state only in Phase 0, plus the theme-switcher control.
fn draw_nav_rail(ui: &mut Ui, state: &mut ShellState) {
    Panel::left("navigation_rail")
        .resizable(false)
        .exact_size(220.0)
        .show_inside(ui, |ui| {
            ui.add_space(12.0);
            ui.heading("Suno Media Station");
            ui.add_space(8.0);

            for destination in NavDestination::ALL {
                let is_selected = state.selected_destination() == destination;
                if ui
                    .selectable_label(is_selected, RichText::new(destination.label()))
                    .clicked()
                {
                    state.select_destination(destination);
                }
            }

            // Fill the rest of the panel from the bottom up so the switcher
            // sits at the rail's lower edge rather than trailing the list.
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                theme_switcher::theme_switcher_control(ui, state);
            });
        });
}

/// Central content area: displays the currently-selected placeholder screen.
fn draw_content_area(ui: &mut Ui, state: &mut ShellState) {
    CentralPanel::default().show_inside(ui, |ui| {
        let destination = state.selected_destination();
        ui.add_space(24.0);
        ui.heading(destination.label());
        ui.add_space(8.0);
        ui.add(
            Label::new(RichText::new(format!(
                "Placeholder — {}",
                destination.placeholder_description()
            )))
            .wrap(),
        );
        ui.add_space(12.0);
        // QA recipe (doc 08 §7 manual pass) wants at least one accent-filled
        // control visible in every screen's placeholder, so the token→widget
        // pipeline can be eyeballed per theme.
        ui_shared_widget_library::themed_accent_button(ui, state.active_theme(), "Primary Action");
    });
}
