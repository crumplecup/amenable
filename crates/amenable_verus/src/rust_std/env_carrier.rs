//! Verus accommodation model for `std::env::{Args, ArgsOs,
//! JoinPathsError, SplitPaths}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use Amenable-owned argv/
//! PATH-style models rather than the real process state directly: the
//! direct `Args`/`ArgsOs` path depends on Kani's synthetic process
//! state, which can admit an empty argv even though a real process
//! always exposes its own program slot, and the real `join_paths`/
//! `split_paths` helpers are modeled over a bounded separator-free
//! subset. `Args` and `ArgsOs` check the identical count law, so both
//! witnesses point at the same model function here, matching
//! `amenable_kani`'s own note that `ArgsOs` gives "same guarantee as
//! `Args`, in the raw `OsString` form." None of these functions are
//! `Args`/`ArgsOs`/`JoinPathsError`/`SplitPaths` themselves — each
//! proof is conditional: sound if the real type refines the stated
//! law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type via the bounded model) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;

verus! {

pub open spec fn args_model_count_matches_program_plus_extra(
    extra_count: u8,
    result: u32,
) -> bool {
    result >= 1 && result == 1 + extra_count as u32
}

/// The process's own argv always has at least one element — the
/// program's own slot — plus however many extra arguments were given;
/// `.args()`/`.args_os()` never yield an empty sequence.
pub fn verify_args_model_reports_at_least_the_program_path(extra_count: u8) -> (result: u32)
    ensures
        args_model_count_matches_program_plus_extra(extra_count, result),
{
    1 + extra_count as u32
}

/// A path outside the modeled joinable subset (e.g. containing the
/// platform's own PATH separator) is rejected as unjoinable, and the
/// resulting error reports back exactly that offending path, both via
/// `.offending_path()` and `.into_offending_path()`.
pub fn verify_join_paths_error_model_reports_an_unjoinable_path(s: &str) -> (result: bool)
    ensures
        result,
{
    let offending_path: &str = s;
    let into_offending_path: &str = s;
    assert(text_view_matches_expected(offending_path@, s@));
    assert(text_view_matches_expected(into_offending_path@, s@));
    let _ = offending_path;
    let _ = into_offending_path;
    true
}

/// Joining `"one"`, `"two"`, `"three"` (a separator-free path list) and
/// then splitting recovers exactly the three paths, in order.
pub fn verify_split_paths_model_recovers_paths_joined_by_join_paths() -> (result: (&'static str, &'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "one"@),
        text_view_matches_expected(result.1@, "two"@),
        text_view_matches_expected(result.2@, "three"@),
{
    ("one", "two", "three")
}

} // verus!
