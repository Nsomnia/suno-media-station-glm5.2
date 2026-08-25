//! Purpose: cargo-xtask style dev tooling for this workspace (layering checks,
//! file-size caps, future codegen and fixture-capture helpers).
//!
//! This crate does NOT contain any shipped-app functionality — it is build/
//! CI tooling only, and stays dependency-free (std only).
//!
//! Status: scaffolded in Phase 0; grows per docs/product/04-phase-roadmap.md as
//! automation needs appear.

mod commands;

use std::process::ExitCode;

const USAGE: &str = "usage: cargo xtask <command>

commands:
  check-layering    fail if any workspace crate depends on a higher layer
  check-file-caps   fail on any .rs file over the 300-line hard cap (>200 warns)";

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    let result = match arg.as_deref() {
        Some("check-layering") => commands::check_layering::run(),
        Some("check-file-caps") => commands::check_file_caps::run(),
        Some(other) => {
            eprintln!("error: unknown command `{other}`\n\n{USAGE}");
            return ExitCode::from(2);
        }
        None => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
