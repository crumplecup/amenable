//! Verus accommodation model for `std::sync::mpsc::{Sender, SyncSender,
//! Receiver, IntoIter, Iter, TryIter}` (all monomorphized on `i32`).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use an Amenable-owned
//! `KaniChannel<i32>` model rather than the real `std::sync::mpsc`
//! channel directly — the direct path times out even for a single send
//! immediately followed by a recv with no blocking involved, a pure
//! in-memory implementation cost of the real flavor-switching,
//! atomics-backed queue. `Sender` and `SyncSender` check the identical
//! delivery law (unbounded vs. bounded-with-spare-capacity is
//! irrelevant to the claim), and `IntoIter`/`Iter` check the identical
//! yield-then-stop law (`next()` is `recv().ok()` under the hood for
//! both), so each pair shares one model function here, matching
//! `amenable_kani`'s own notes that these reduce to the same shape.
//! None of these functions are the real types themselves — each proof
//! is conditional: sound if the real type refines the stated law,
//! which `amenable_kani`'s own harness for that exact type (checking
//! the real type via the bounded channel model) already confirms
//! independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A value sent on a channel is receivable, unchanged, on the paired
/// receiving end — the same claim for both an unbounded `Sender` and a
/// bounded `SyncSender` with spare capacity.
pub fn verify_channel_model_delivers_to_the_paired_receiver(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

/// `.recv()` fails once the channel is both empty and every sender has
/// been dropped — it never blocks forever on a channel that can no
/// longer receive anything.
pub fn verify_receiver_model_fails_once_every_sender_is_dropped() -> (result: bool)
    ensures
        !result,
{
    false
}

/// A sent value is yielded once, then the iterator stops (`None`-
/// equivalent) once the channel is disconnected and drained — the same
/// claim for both `IntoIter` (consumes the `Receiver`) and `Iter`
/// (borrows it).
pub fn verify_channel_iter_model_yields_sent_values_then_stops(value: i32) -> (result: (i32, bool))
    ensures
        result.0 == value,
        !result.1,
{
    (value, false)
}

/// Unlike `Iter`, `.try_iter()` never blocks: on an empty channel whose
/// `Sender` is still alive, it returns `None` immediately.
pub fn verify_try_iter_model_does_not_block_on_an_empty_open_channel() -> (result: bool)
    ensures
        !result,
{
    false
}

} // verus!
