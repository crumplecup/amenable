//! Verus spec for `amenable_time::UtcTimelineOrderingAppliesToFixedInstants`.
//!
//! RFC 3339, 5.1 — two fixed instants are totally ordered by their position on the UTC timeline. `<=` on `i32` timeline positions is proven reflexive, antisymmetric,
//! total, and transitive — a total order,
//! for every `i32` pair — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `UtcTimelineOrderingAppliesToFixedInstants`: `earlier` is at or before `later`.
pub open spec fn utc_timeline_ordering_applies_to_fixed_instants_holds(earlier: i32, later: i32) -> bool {
    earlier <= later
}

/// `earlier <= later` (exec body) matches the predicate, and the relation is
/// reflexive, total, antisymmetric, and transitive over three instants.
pub fn verify_utc_timeline_ordering_applies_to_fixed_instants(a: i32, b: i32, c: i32) -> (result: bool)
    ensures
        result == utc_timeline_ordering_applies_to_fixed_instants_holds(a, b),
        utc_timeline_ordering_applies_to_fixed_instants_holds(a, a),
        utc_timeline_ordering_applies_to_fixed_instants_holds(a, b)
            || utc_timeline_ordering_applies_to_fixed_instants_holds(b, a),
        (utc_timeline_ordering_applies_to_fixed_instants_holds(a, b)
            && utc_timeline_ordering_applies_to_fixed_instants_holds(b, a)) ==> a == b,
        (utc_timeline_ordering_applies_to_fixed_instants_holds(a, b)
            && utc_timeline_ordering_applies_to_fixed_instants_holds(b, c))
            ==> utc_timeline_ordering_applies_to_fixed_instants_holds(a, c),
{
    // `c` is a third symbolic instant the `ensures` transitivity clause
    // needs; `verus! {}` erases that clause under plain rustc, so name it
    // used here (same idiom as `rust_std::misc::discriminant_carrier`).
    let _ = c;
    a <= b
}

} // verus!
