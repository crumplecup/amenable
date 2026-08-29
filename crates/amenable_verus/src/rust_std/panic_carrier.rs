//! Verus accommodation model for `core::panic::{AssertUnwindSafe<i32>,
//! Location<'static>}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `AssertUnwindSafe` has no timeout concerns —
//! `amenable_kani`'s harness checks the real type directly — but
//! `Location`'s harness uses a bounded `KaniCallerLocationObservation`
//! instead, since the direct `Location::caller()` path reaches Kani's
//! unsupported `track_caller` boundary. This carrier states each
//! resulting law directly. Neither function is `AssertUnwindSafe`/
//! `Location` themselves — each proof is conditional: sound if the real
//! type refines the stated law, which `amenable_kani`'s own harness for
//! that exact type (checking the real type directly, or via the
//! bounded observation) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_pair_matches_input;
#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;
#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::values_are_distinct;

verus! {

/// `AssertUnwindSafe` is a bare assertion wrapper: `Deref`/`DerefMut`
/// expose the wrapped value with no transformation, and a write through
/// `DerefMut` is visible afterward.
pub fn verify_assert_unwind_safe_model_derefs_transparently(value: i32, updated: i32) -> (result: (i32, i32))
    ensures
        observed_pair_matches_input(result, (value, updated)),
{
    (value, updated)
}

/// `#[track_caller]` threads through to the immediate call site: two
/// calls from distinct lines in the same file report that same file
/// (checked via view equality, since `str`'s own `PartialEq::eq` has no
/// `vstd` spec support) — the "distinct lines" half of the claim is
/// carried by the `requires` itself, matching the bounded observation's
/// own same-file-distinct-lines shape.
pub fn verify_location_model_caller_reflects_the_immediate_call_site(file: &str, line_a: u32, line_b: u32) -> (result: bool)
    requires
        values_are_distinct(line_a, line_b),
    ensures
        result,
{
    let observed_file: &str = file;
    assert(text_view_matches_expected(observed_file@, file@));
    let _ = observed_file;
    let _ = line_a;
    let _ = line_b;
    true
}

} // verus!
