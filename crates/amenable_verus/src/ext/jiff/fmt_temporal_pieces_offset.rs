//! Verus accommodation model for `jiff::fmt::temporal::
//! PiecesOffset`'s `Zulu`/`From<Offset>` mapping into
//! `to_numeric_offset`.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_offset`'s and `amenable_creusot::ext_jiff::
//! fmt_temporal_pieces_offset`'s own harnesses for the identical
//! claim, checked directly against jiff's real API, independently
//! confirm.
//!
//! Models `PiecesOffset` as a plain struct (`{ is_zulu: bool,
//! numeric_seconds: i32 }`), not a data-carrying `enum`: a first
//! attempt at `enum { Zulu, Numeric(i32) }` compiled but tripped a
//! real `missing_docs` warning on a Verus-synthesized method tied to
//! the data-carrying variant's own span (no doc comment can target
//! it) — confirmed to be specific to a *data-carrying* variant, since
//! every field-less enum model already in this directory (`unit.rs`,
//! `error.rs`, `civil_era.rs`, `fmt_friendly_fractional_unit.rs`,
//! `fmt_strtime_meridiem.rs`) verifies clean. The `numeric_seconds`
//! field models `PiecesNumericOffset::offset().seconds()` only
//! (`is_negative` doesn't affect `to_numeric_offset`'s real behavior,
//! so it's out of scope here), self-contained rather than
//! cross-importing `fmt_temporal_pieces_numeric_offset.rs`'s own
//! model, matching every other model in this directory's choice not
//! to cross-import a sibling's spec fn.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::temporal::PiecesOffset`, scoped to what
/// `to_numeric_offset` needs (see this module's own doc comment for
/// why `is_negative` and the data-carrying `enum` shape are both out
/// of scope).
pub struct PiecesOffsetModel {
    /// Models the `PiecesOffset::Zulu` variant when `true`.
    pub is_zulu: bool,
    /// Models `PiecesOffset::Numeric(noffset)`'s
    /// `noffset.offset().seconds()`, meaningful only when `is_zulu`
    /// is `false`.
    pub numeric_seconds: i32,
}

/// `zulu`'s postcondition: the result is always the `Zulu` variant.
pub open spec fn pieces_offset_zulu_holds(result: PiecesOffsetModel) -> bool {
    result.is_zulu
}

/// `from_offset_seconds`'s postcondition: always the `Numeric`
/// variant, carrying `secs` as its own numeric seconds.
pub open spec fn pieces_offset_from_offset_seconds_holds(
    secs: i32,
    result: PiecesOffsetModel,
) -> bool {
    !result.is_zulu && result.numeric_seconds == secs
}

/// `to_numeric_offset_seconds`'s postcondition: `Zulu` maps to zero
/// seconds, `Numeric` unwraps to its own seconds.
pub open spec fn pieces_offset_to_numeric_seconds_holds(
    before: PiecesOffsetModel,
    result: i32,
) -> bool {
    if before.is_zulu {
        result == 0
    } else {
        result == before.numeric_seconds
    }
}

impl PiecesOffsetModel {
    /// Models `PiecesOffset::Zulu`.
    pub fn zulu() -> (result: PiecesOffsetModel)
        ensures
            pieces_offset_zulu_holds(result),
    {
        PiecesOffsetModel { is_zulu: true, numeric_seconds: 0 }
    }

    /// Models `PiecesOffset::from(offset)`: always the `Numeric`
    /// variant, carrying `offset`'s own seconds.
    pub fn from_offset_seconds(secs: i32) -> (result: PiecesOffsetModel)
        ensures
            pieces_offset_from_offset_seconds_holds(secs, result),
    {
        PiecesOffsetModel { is_zulu: false, numeric_seconds: secs }
    }

    /// Models `PiecesOffset::to_numeric_offset`: `Zulu` maps to zero
    /// seconds (`Offset::UTC`), `Numeric` unwraps to its own seconds.
    pub fn to_numeric_offset_seconds(&self) -> (result: i32)
        ensures
            pieces_offset_to_numeric_seconds_holds(*self, result),
    {
        if self.is_zulu { 0 } else { self.numeric_seconds }
    }
}

/// Exercises `zulu`/`from_offset_seconds`/`to_numeric_offset_seconds`
/// — the same claim `amenable_kani::ext::jiff::
/// fmt_temporal_pieces_offset` and `amenable_creusot::ext_jiff::
/// fmt_temporal_pieces_offset` both check against jiff's real API.
pub fn verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_model(
    secs: i32,
) -> (result: bool)
    ensures
        result,
{
    let zulu_ok = PiecesOffsetModel::zulu().to_numeric_offset_seconds() == 0;
    let from_ok = PiecesOffsetModel::from_offset_seconds(secs).to_numeric_offset_seconds() == secs;

    zulu_ok && from_ok
}

} // verus!
