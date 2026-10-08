//! JSON-serializable shapes for `dump-registry`'s output. Not CLI code —
//! [`crate::cli`]'s `run_dump_registry` is the only caller; this module
//! just shapes the real `inventory`-registered types into an owned,
//! serializable snapshot. Split by concern: [`evidence`] and [`proof`]
//! each hold a small, independent record pair; [`witness_artifact`] holds
//! the recursive artifact tree; [`kani_proof`] holds the one remaining
//! leaf shape; [`collect`] holds the orchestrating [`RegistryDump`]
//! itself and its registry walk.

mod collect;
mod evidence;
mod kani_proof;
mod proof;
mod witness_artifact;

pub(crate) use collect::RegistryDump;
