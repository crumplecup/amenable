//! Verus accommodation model for `jiff::fmt::temporal::
//! TimeZoneAnnotation<'static>`'s `From<&str>`/`From<Offset>`
//! constructors.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation`'s and `amenable_creusot::
//! ext_jiff::fmt_temporal_time_zone_annotation`'s own harnesses for
//! the identical claim, checked directly against jiff's real API,
//! independently confirm.
//!
//! Models `TimeZoneAnnotation` as a plain struct (`{ is_named: bool,
//! offset_seconds: i32 }`), at the same NARROWER scope
//! `fmt_temporal_time_zone_annotation.rs`'s own Creusot doc comment
//! documents: the name's exact string content is out of scope here
//! too (this model carries no string payload at all), so it checks
//! only which variant a construction reports (and, for `Offset`, its
//! numeric payload) with `is_critical` always `false`. Self-contained
//! rather than cross-importing a sibling model, matching every other
//! model in this directory's own choice.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::temporal::TimeZoneAnnotation<'static>`,
/// scoped to what this module's own doc comment documents (variant +
/// numeric payload only, no string content, `critical` always
/// `false` since only the `From` constructors are modeled).
pub struct TimeZoneAnnotationModel {
    /// Models the `Named` variant when `true`, `Offset` when `false`.
    pub is_named: bool,
    /// Models the wrapped `Offset`'s own seconds, meaningful only
    /// when `is_named` is `false`.
    pub offset_seconds: i32,
}

/// `from_name`'s postcondition: always the `Named` variant.
pub open spec fn tz_annotation_from_name_holds(result: TimeZoneAnnotationModel) -> bool {
    result.is_named
}

/// `from_offset_seconds`'s postcondition: always the `Offset` variant,
/// carrying `secs` as its own offset seconds.
pub open spec fn tz_annotation_from_offset_seconds_holds(
    secs: i32,
    result: TimeZoneAnnotationModel,
) -> bool {
    !result.is_named && result.offset_seconds == secs
}

impl TimeZoneAnnotationModel {
    /// Models `TimeZoneAnnotation::from(name)`: always the `Named`
    /// variant.
    pub fn from_name() -> (result: TimeZoneAnnotationModel)
        ensures
            tz_annotation_from_name_holds(result),
    {
        TimeZoneAnnotationModel { is_named: true, offset_seconds: 0 }
    }

    /// Models `TimeZoneAnnotation::from(offset)`: always the
    /// `Offset` variant, carrying `offset`'s own seconds.
    pub fn from_offset_seconds(secs: i32) -> (result: TimeZoneAnnotationModel)
        ensures
            tz_annotation_from_offset_seconds_holds(secs, result),
    {
        TimeZoneAnnotationModel { is_named: false, offset_seconds: secs }
    }
}

/// Exercises `from_name`/`from_offset_seconds` — the same claim
/// `amenable_kani::ext::jiff::fmt_temporal_time_zone_annotation` and
/// `amenable_creusot::ext_jiff::fmt_temporal_time_zone_annotation`
/// both check against jiff's real API, at the narrower scope this
/// module's own doc comment documents.
pub fn verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset_model(
    secs: i32,
) -> (result: bool)
    ensures
        result,
{
    let named_ok = TimeZoneAnnotationModel::from_name().is_named;
    let offset_model = TimeZoneAnnotationModel::from_offset_seconds(secs);
    let offset_ok = !offset_model.is_named && offset_model.offset_seconds == secs;

    named_ok && offset_ok
}

} // verus!
