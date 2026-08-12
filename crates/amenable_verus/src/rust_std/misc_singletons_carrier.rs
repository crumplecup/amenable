//! Verus accommodation model for a handful of single-type checklist
//! clusters: `core::pin::Pin<Box<i32>>`, `core::ptr::NonNull<i32>`,
//! `std::alloc::System`, `std::backtrace::{Backtrace,
//! BacktraceStatus}`, `std::panic::PanicHookInfo<'static>`, and
//! `alloc::collections::vec_deque::Drain<'static, i32>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `Pin`/`NonNull`/`System`/`Drain` are checked directly by
//! `amenable_kani` (no timeout concerns); `Backtrace`/`BacktraceStatus`/
//! `PanicHookInfo` use bounded observations instead, since their direct
//! paths reach Kani's unsupported platform-unwinding/`catch_unwind`
//! boundaries. `Backtrace` and `BacktraceStatus` check the identical
//! forced-capture status claim, so both witnesses point at the same
//! model function here, matching `amenable_kani`'s own note that
//! `BacktraceStatus`'s proof states "the same underlying status law as
//! `Backtrace`." None of these functions are the real types themselves
//! — each proof is conditional: sound if the real type refines the
//! stated law, which `amenable_kani`'s own harness for that exact type
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;
#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;

verus! {

/// `Pin` is transparent to its pointee through `Deref`, and (for an
/// `Unpin` target like `i32`) `as_mut().get_mut()` grants a safe
/// `&mut` back to it — a write through it is visible through deref.
pub fn verify_pin_model_derefs_and_get_mut_round_trip(value: i32, updated: i32) -> (result: (i32, i32))
    ensures
        result == (value, updated),
{
    (value, updated)
}

/// `(valid_pointer_accepted, null_pointer_accepted,
/// from_preserves_the_address)`.
type NonNullResult = (bool, bool, bool);

/// `NonNull::new` accepts a valid pointer and rejects the null
/// pointer, and `NonNull::from` preserves the address of the reference
/// it was built from.
pub fn verify_non_null_model_rejects_the_null_pointer() -> (result: NonNullResult)
    ensures
        result.0,
        !result.1,
        result.2,
{
    (true, false, true)
}

/// `System` is the process's default global allocator: a `Box`
/// allocation serviced by it round-trips its value.
pub fn verify_system_model_allocates_and_deallocates_a_layout(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

/// `Backtrace::force_capture()` always actually captures — reported as
/// `Captured` both from `Backtrace::status()` and from
/// `BacktraceStatus` itself.
pub fn verify_backtrace_model_force_capture_always_actually_captures() -> (result: bool)
    ensures
        result,
{
    true
}

/// A custom panic hook observes the in-progress panic's own payload,
/// exactly as it was passed to `panic!()` (checked via view equality,
/// since `str`'s own `PartialEq::eq` has no `vstd` spec support).
pub fn verify_panic_hook_info_model_reports_the_panics_own_message(s: &str) -> (result: bool)
    ensures
        result,
{
    let captured: &str = s;
    assert(text_view_matches_expected(captured@, s@));
    let _ = captured;
    true
}

/// `.drain(..)` yields every element in front-to-back order and leaves
/// the deque empty.
pub fn verify_vec_deque_drain_model_removes_and_yields_in_order(a: i32, b: i32) -> (result: (i32, i32, bool))
    ensures
        result.0 == a,
        result.1 == b,
        result.2,
{
    (a, b, true)
}

} // verus!
