//! Verus accommodation model for `jiff::fmt::temporal::
//! TimeZoneAnnotationName<'static>`'s `From<&str>`/`as_str` round
//! trip.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation_name`'s and `amenable_creusot::
//! ext_jiff::fmt_temporal_time_zone_annotation_name`'s own harnesses
//! for the identical claim, checked directly against jiff's real
//! API, independently confirm.
//!
//! Models `TimeZoneAnnotationName` as a trivial `&str` wrapper — the
//! real type's own round trip is a store-and-return-back with no
//! transformation, so the model is the identity function over the
//! borrowed string itself, not a byte/`Seq<char>` reconstruction.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::temporal::TimeZoneAnnotationName<'static>`.
pub struct TimeZoneAnnotationNameModel<'a> {
    /// Models `TimeZoneAnnotationName::as_str()`'s own return value.
    pub name: &'a str,
}

impl<'a> TimeZoneAnnotationNameModel<'a> {
    /// Models `TimeZoneAnnotationName::from(s)`.
    pub fn from_name(s: &'a str) -> (result: TimeZoneAnnotationNameModel<'a>)
        ensures
            result.name == s,
    {
        TimeZoneAnnotationNameModel { name: s }
    }

    /// Models `TimeZoneAnnotationName::as_str`.
    pub fn as_str(&self) -> (result: &'a str)
        ensures
            result == self.name,
    {
        self.name
    }
}

/// Exercises `from_name`/`as_str` — the same claim
/// `amenable_kani::ext::jiff::fmt_temporal_time_zone_annotation_name`
/// and `amenable_creusot::ext_jiff::
/// fmt_temporal_time_zone_annotation_name` both check against jiff's
/// real API.
pub fn verify_fmt_temporal_time_zone_annotation_name_from_str_round_trips_model(
    name: &str,
) -> (result: bool)
    ensures
        result,
{
    // `_got == name` at the exec level invokes `&str`'s real
    // `PartialEq`, which Verus doesn't verify as content-equality
    // here (confirmed: a first attempt computing the returned `bool`
    // directly via exec `==` left a real "postcondition not
    // satisfied" error, even though `from_name`/`as_str`'s own
    // `ensures` clauses -- spec-level equality -- already establish
    // the identical fact). Asserting in spec context instead lets
    // Verus chain those same `ensures` clauses successfully. (Calling
    // `from_name`/`as_str` directly inside `assert(..)` isn't legal
    // either -- confirmed via a real "cannot call function ... with
    // mode exec" error -- so the exec calls stay outside it, bound
    // first.) `_`-prefixed since `verus! {}` erases the `assert(..)`
    // itself under a plain (non-Verus) rustc build, leaving these
    // reads genuinely absent there -- the real Verus toolchain still
    // resolves `_ann_name`/`_got` by their exact name.
    let _ann_name = TimeZoneAnnotationNameModel::from_name(name);
    let _got = _ann_name.as_str();
    assert(_got == name);
    true
}

} // verus!
