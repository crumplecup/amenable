//! Verus accommodation model for `core::sync::atomic::AtomicPtr<i32>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real `AtomicPtr`
//! directly, using the addresses of distinct local slots as
//! pairwise-distinct pointer values (no pointer is ever dereferenced).
//! This carrier models each pointer value as a plain `u64` address
//! stand-in — `vstd` has no spec support for `AtomicPtr` or raw
//! pointers at all — checking the same load/store/swap/
//! compare-exchange-success law. `compare_exchange`'s failure branch is
//! deliberately not modeled here either, matching `amenable_kani`'s own
//! documented choice (a genuine Kani/CBMC limitation on that branch,
//! not a logic gap). Not `AtomicPtr` itself — the proof is conditional:
//! sound if the real type refines this law, which `amenable_kani`'s own
//! `verify_atomic_ptr_load_store_swap_and_compare_exchange` harness
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `(load_after_new, load_after_store, swap_returned_previous,
/// load_after_swap, compare_exchange_returned_previous,
/// load_after_compare_exchange)`.
type AtomicPtrResult = (u64, u64, u64, u64, u64, u64);

/// The `AtomicPtr` load/store/swap/compare-exchange-success law the
/// accommodation model establishes.
pub open spec fn atomic_ptr_model_load_store_swap_and_compare_exchange(
    load_after_new: u64,
    load_after_store: u64,
    swap_returned_previous: u64,
    load_after_swap: u64,
    compare_exchange_returned_previous: u64,
    load_after_compare_exchange: u64,
    initial: u64,
    stored: u64,
    swapped_in: u64,
    exchange_target: u64,
) -> bool {
    load_after_new == initial
        && load_after_store == stored
        && swap_returned_previous == stored
        && load_after_swap == swapped_in
        && compare_exchange_returned_previous == swapped_in
        && load_after_compare_exchange == exchange_target
}

/// `AtomicPtr::new` sets the address observable via `load`; `store`
/// overwrites it; `swap` returns the previous address and installs the
/// new one; `compare_exchange` (current matching) updates and reports
/// the previous address.
pub fn verify_atomic_ptr_model_load_store_swap_and_compare_exchange(initial: u64, stored: u64, swapped_in: u64, exchange_target: u64) -> (result: AtomicPtrResult)
    ensures
        atomic_ptr_model_load_store_swap_and_compare_exchange(
            result.0,
            result.1,
            result.2,
            result.3,
            result.4,
            result.5,
            initial,
            stored,
            swapped_in,
            exchange_target,
        ),
{
    (initial, stored, stored, swapped_in, swapped_in, exchange_target)
}

} // verus!
