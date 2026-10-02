//! Clap types and dispatch. The `amenable` binary's `main` only parses
//! and calls [`Cli::act`]; every subcommand's own logic and any further
//! nested dispatch live here in the library, never in the binary.

mod commands;
mod hook;
mod parser;
mod run;

#[cfg(feature = "cli")]
pub use hook::install_hook;
pub use parser::Cli;
