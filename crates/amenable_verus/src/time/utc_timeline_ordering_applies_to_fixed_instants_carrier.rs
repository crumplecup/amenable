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

/// The exec body's result matches the predicate, named so the
/// exec-to-spec link is a citable fact.
pub open spec fn utc_timeline_ordering_result_matches(earlier: i32, later: i32, result: bool) -> bool {
    result == utc_timeline_ordering_applies_to_fixed_instants_holds(earlier, later)
}

/// The relation is total: any two instants are ordered one way or the
/// other — a genuinely different, independently-named property.
pub open spec fn utc_timeline_ordering_is_total(a: i32, b: i32) -> bool {
    utc_timeline_ordering_applies_to_fixed_instants_holds(a, b)
        || utc_timeline_ordering_applies_to_fixed_instants_holds(b, a)
}

/// The relation is antisymmetric: mutual ordering forces equality — a
/// genuinely different, independently-named property.
pub open spec fn utc_timeline_ordering_is_antisymmetric(a: i32, b: i32) -> bool {
    (utc_timeline_ordering_applies_to_fixed_instants_holds(a, b)
        && utc_timeline_ordering_applies_to_fixed_instants_holds(b, a)) ==> a == b
}

/// The relation is transitive over three instants — a genuinely
/// different, independently-named property.
pub open spec fn utc_timeline_ordering_is_transitive(a: i32, b: i32, c: i32) -> bool {
    (utc_timeline_ordering_applies_to_fixed_instants_holds(a, b)
        && utc_timeline_ordering_applies_to_fixed_instants_holds(b, c))
        ==> utc_timeline_ordering_applies_to_fixed_instants_holds(a, c)
}

/// `earlier <= later` (exec body) matches the predicate, and the relation is
/// reflexive, total, antisymmetric, and transitive over three instants.
pub fn verify_utc_timeline_ordering_applies_to_fixed_instants(a: i32, b: i32, c: i32) -> (result: bool)
    ensures
        utc_timeline_ordering_result_matches(a, b, result),
        utc_timeline_ordering_applies_to_fixed_instants_holds(a, a),
        utc_timeline_ordering_is_total(a, b),
        utc_timeline_ordering_is_antisymmetric(a, b),
        utc_timeline_ordering_is_transitive(a, b, c),
{
    // `c` is a third symbolic instant the `ensures` transitivity clause
    // needs; `verus! {}` erases that clause under plain rustc, so name it
    // used here (same idiom as `rust_std::misc::discriminant_carrier`).
    let _ = c;
    a <= b
}

} // verus!
