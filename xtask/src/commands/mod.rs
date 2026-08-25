//! xtask subcommands. Each command lives in its own module and exposes a
//! single `fn run() -> Result<(), ()>` — process exit codes are derived from
//! that result in `main`.

pub mod check_file_caps;
pub mod check_layering;
