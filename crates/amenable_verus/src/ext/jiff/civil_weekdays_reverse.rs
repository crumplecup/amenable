//! Verus accommodation model for `jiff::civil::WeekdaysReverse`'s
//! periodicity property (`Weekday::cycle_reverse`/`Iterator::next`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! civil_weekdays_reverse`'s own harness for the identical claim,
//! checked directly against jiff's real API by symbolic execution,
//! and `amenable_creusot::ext_jiff::civil_weekdays_reverse`'s own
//! accommodation model, independently confirm.
//!
//! Modeled purely in terms of the plain `i8` Monday-one offset,
//! matching `civil_weekdays_forward.rs`'s own choice.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own real Monday-one offset numbering range (`1..=7`) — the
/// same constant `civil_weekdays_forward.rs`'s own model uses.
pub open spec fn civil_weekdays_reverse_start_offset_in_range(start_offset: int) -> bool {
    start_offset >= 1 && start_offset <= 7
}

/// The periodicity law this model establishes: the first `next()`
/// call returns the starting weekday's offset exactly, and the
/// second returns its predecessor, wrapping `1 -> 7` — the same claim
/// `amenable_kani::ext::jiff::civil_weekdays_reverse::
/// verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor`
/// checks against jiff's real API, and `amenable_creusot::ext_jiff::
/// civil_weekdays_reverse::
/// verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor`
/// checks as an accommodation model too.
pub open spec fn civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds(
    start_offset: i8,
    result: (i8, i8),
) -> bool {
    result.0 == start_offset && result.1 == if start_offset == 1 {
        7
    } else {
        start_offset - 1
    }
}

/// A model of `weekday.cycle_reverse()`'s first two `next()` calls:
/// the first yields `start_offset` exactly, the second its
/// predecessor (wrapping `1 -> 7`).
pub fn verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor(
    start_offset: i8,
) -> (result: (i8, i8))
    requires
        civil_weekdays_reverse_start_offset_in_range(start_offset as int),
    ensures
        civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds(start_offset, result),
{
    let previous_offset = if start_offset == 1 { 7 } else { start_offset - 1 };
    (start_offset, previous_offset)
}

} // verus!
