//! Verus accommodation model for `jiff::fmt::temporal::
//! TimeZoneAnnotationKind<'static>`'s `From<&str>`/`From<Offset>`
//! constructors.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_temporal_time_zone_annotation_kind`'s and `amenable_creusot::
//! ext_jiff::fmt_temporal_time_zone_annotation_kind`'s own harnesses
//! for the identical claim, checked directly against jiff's real
//! API, independently confirm.
//!
//! Models `TimeZoneAnnotationKind` as a plain struct (`{ is_named:
//! bool, offset_seconds: i32 }`), the same shape
//! `fmt_temporal_time_zone_annotation.rs`'s own model uses (no
//! `critical` field here — this enum itself has none) — no string
//! payload modeled, self-contained rather than cross-importing a
//! sibling model, matching every other model in this directory's
//! own choice.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::temporal::TimeZoneAnnotationKind<'static>`.
pub struct TimeZoneAnnotationKindModel {
    /// Models the `Named` variant when `true`, `Offset` when `false`.
    pub is_named: bool,
    /// Models the wrapped `Offset`'s own seconds, meaningful only
    /// when `is_named` is `false`.
    pub offset_seconds: i32,
}

impl TimeZoneAnnotationKindModel {
    /// Models `TimeZoneAnnotationKind::from(name)`: always the
    /// `Named` variant.
    pub fn from_name() -> (result: TimeZoneAnnotationKindModel)
        ensures
            result.is_named,
    {
        TimeZoneAnnotationKindModel { is_named: true, offset_seconds: 0 }
    }

    /// Models `TimeZoneAnnotationKind::from(offset)`: always the
    /// `Offset` variant, carrying `offset`'s own seconds.
    pub fn from_offset_seconds(secs: i32) -> (result: TimeZoneAnnotationKindModel)
        ensures
            !result.is_named,
            result.offset_seconds == secs,
    {
        TimeZoneAnnotationKindModel { is_named: false, offset_seconds: secs }
    }
}

/// Exercises `from_name`/`from_offset_seconds` — the same claim
/// `amenable_kani::ext::jiff::fmt_temporal_time_zone_annotation_kind`
/// and `amenable_creusot::ext_jiff::
/// fmt_temporal_time_zone_annotation_kind` both check against jiff's
/// real API.
pub fn verify_fmt_temporal_time_zone_annotation_kind_from_name_and_from_offset_model(
    secs: i32,
) -> (result: bool)
    ensures
        result,
{
    let named_ok = TimeZoneAnnotationKindModel::from_name().is_named;
    let offset_model = TimeZoneAnnotationKindModel::from_offset_seconds(secs);
    let offset_ok = !offset_model.is_named && offset_model.offset_seconds == secs;

    named_ok && offset_ok
}

} // verus!
