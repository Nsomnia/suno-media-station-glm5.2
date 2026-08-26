//! The eframe window wrapper around [`ShellState`] and its native options.
//!
//! Renderer backend is glow per ADR-013; the wgpu backend is deliberately
//! not compiled into the workspace's eframe dependency at all.

use eframe::egui;
use ui_app_shell_and_navigation::ShellState;

/// Opens the main window and runs the shell until the user closes it.
///
/// # Errors
///
/// Propagates eframe's native-window creation / event-loop errors.
pub fn run_native_window(shell_state: ShellState) -> Result<(), eframe::Error> {
    eframe::run_native(
        "Suno Media Station",
        native_options(),
        Box::new(|_creation_context| Ok(Box::new(StationApp { shell_state }))),
    )
}

/// Window defaults for Phase 0: glow renderer + a comfortable desktop size.
fn native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_title("Suno Media Station")
            .with_inner_size([1280.0, 800.0]),
        ..eframe::NativeOptions::default()
    }
}

struct StationApp {
    shell_state: ShellState,
}

impl eframe::App for StationApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.shell_state.shell_ui(ui);
    }
}
