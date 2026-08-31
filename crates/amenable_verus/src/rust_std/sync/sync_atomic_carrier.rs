//! Verus accommodation model for `core::sync::atomic::{AtomicBool,
//! AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8,
//! AtomicU16, AtomicU32, AtomicU64, AtomicUsize}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. All eleven real instantiations check the identical
//! `new`/`store`/`load` claim in `amenable_kani`, written out literally
//! per width rather than genericized (matching that crate's own
//! documented choice for `NonZero<T>`, per its module doc comment,
//! itself matching `AtomicPtr`/`Ordering` also being written out
//! literally here per `sync_atomic.rs`'s own module doc) — this carrier
//! mirrors that same one function per width. `Atomic*::new` sets the
//! value observable via `load`, and `store` overwrites it, both under
//! `SeqCst` ordering — the ordering variant itself plays no role in
//! this law (see `sync_atomic_ordering_carrier.rs` for the one proof
//! that specifically varies it). None of these functions are the real
//! `Atomic*` types themselves — each proof is conditional: sound if the
//! real type refines this law, which `amenable_kani`'s own harness for
//! that exact width (checking the real type directly) already confirms
//! independently, for the identical claim.
//!
//! Every `ensures observed_pair_matches_input(result, (initial, next)),`
//! clause below is the canonical home each width's own
//! `RustStdStandard<Atomic*>`'s `Ensures<VerusVerifier>` impl
//! (`amenable_std::verus_witness`, `impl_sync_atomic_verus_witness!`)
//! names.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_pair_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

pub fn verify_atomic_bool_model_load_store(initial: bool, next: bool) -> (result: (bool, bool))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_i8_model_load_store(initial: i8, next: i8) -> (result: (i8, i8))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_i16_model_load_store(initial: i16, next: i16) -> (result: (i16, i16))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_i32_model_load_store(initial: i32, next: i32) -> (result: (i32, i32))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_i64_model_load_store(initial: i64, next: i64) -> (result: (i64, i64))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_isize_model_load_store(initial: isize, next: isize) -> (result: (isize, isize))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_u8_model_load_store(initial: u8, next: u8) -> (result: (u8, u8))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_u16_model_load_store(initial: u16, next: u16) -> (result: (u16, u16))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_u32_model_load_store(initial: u32, next: u32) -> (result: (u32, u32))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_u64_model_load_store(initial: u64, next: u64) -> (result: (u64, u64))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

pub fn verify_atomic_usize_model_load_store(initial: usize, next: usize) -> (result: (usize, usize))
    ensures
        observed_pair_matches_input(result, (initial, next)),
{
    (initial, next)
}

} // verus!
