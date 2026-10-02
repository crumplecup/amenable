//! Clap subcommands. `Commands` implements the top-level dispatch, while
//! each command family keeps its own clap types and nested dispatch in a
//! sibling module.

// `creusot`/`verus` are whole feature-gated subcommand families: the
// `#[cfg]` sits once on each private `mod` declaration (the shape the
// cfg-scatter lint recommends), and the only other gated sites are the
// exempt enum variant and its one dispatch arm below. Each family's arg
// types are named through the module path (`creusot::CreusotArgs`) and its
// leaf executors live in that same module, so nothing crosses a boundary
// and there is no gated re-export to scatter the predicate onto.
#[cfg(feature = "creusot")]
mod creusot;
mod dispatch;
mod inspection;
mod verify;
#[cfg(feature = "verus")]
mod verus;

pub(in crate::cli) use inspection::{AuditArgs, DumpRegistryArgs};
pub(in crate::cli) use verify::VerifyArgs;

pub(super) use dispatch::Commands;
