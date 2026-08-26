//! Purpose: the binary crate and composition root for Suno Media Station.
//!
//! This crate does NOT implement features — it only wires concrete
//! implementations of workspace crates together at startup: config loading,
//! logging setup, initial theme resolution, and the eframe (glow, per
//! ADR-013) window hosting the UI app shell.

mod bootstrapped_app;
mod startup;

fn main() -> Result<(), eframe::Error> {
    let bootstrapped = startup::Bootstrapped::run();
    // The logging guard must outlive everything below it: dropping it shuts
    // down the non-blocking log writer.
    let _logging_guard = bootstrapped.logging_guard;

    tracing::info!(
        theme = %bootstrapped.shell_state.active_theme().name,
        "launching station-app window"
    );

    bootstrapped_app::run_native_window(bootstrapped.shell_state)
}
