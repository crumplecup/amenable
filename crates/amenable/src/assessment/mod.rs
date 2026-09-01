//! Structured, append-only assessments of executable proof harnesses.
//!
//! Split by role: `vocabulary` holds the fixed scoring/recommendation
//! terms a reviewer chooses from; `catalog` enumerates the proofs a
//! reviewer could assess; `record` is the persisted assessment shape and
//! its JSON Lines I/O; `cli` is the `clap` argument surface; `commands`
//! wires an invocation to its effect. Only [`AssessArgs`] and [`load`] are
//! visible outside this module -- everything else is `pub(super)`,
//! confined to this subtree the same way it was implicitly confined to
//! one file before the split.

mod catalog;
mod cli;
mod commands;
mod record;
mod vocabulary;

pub use cli::AssessArgs;
pub use record::load;
