//! Verus accommodation model for `jiff::fmt::temporal::
//! PiecesNumericOffset`'s `From<Offset>`/`with_negative_zero` round
//! trips.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_numeric_offset`'s and `amenable_creusot::
//! ext_jiff::fmt_temporal_pieces_numeric_offset`'s own harnesses for
//! the identical claim, checked directly against jiff's real API,
//! independently confirm.
//!
//! Models `PiecesNumericOffset` as `{ offset_seconds: i32, is_negative:
//! bool }` — self-contained, not cross-importing `offset.rs`'s own
//! `offset_seconds_in_range` spec fn, matching every other model in
//! this directory (none of them cross-import a sibling's spec fn; each
//! restates what it needs).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own documented valid range for `Offset::from_seconds` — the
/// same constant `offset.rs`'s own model independently confirms.
pub open spec fn pno_offset_seconds_in_range(secs: int) -> bool {
    secs >= -93599 && secs <= 93599
}

/// A model of `jiff::fmt::temporal::PiecesNumericOffset`.
pub struct PiecesNumericOffsetModel {
    /// Models `PiecesNumericOffset::offset().seconds()`.
    pub offset_seconds: i32,
    /// Models `PiecesNumericOffset::is_negative()`.
    pub is_negative: bool,
}

/// `from_offset_seconds`'s postcondition: the wrapped offset
/// round-trips exactly, and `is_negative` matches the sign of `secs`.
pub open spec fn pno_from_offset_seconds_holds(secs: i32, result: PiecesNumericOffsetModel) -> bool {
    result.offset_seconds == secs && result.is_negative == (secs < 0)
}

/// `with_negative_zero`'s postcondition: the wrapped offset is
/// preserved from the receiver, `is_negative` is always forced `true`.
pub open spec fn pno_with_negative_zero_holds(
    before: PiecesNumericOffsetModel,
    result: PiecesNumericOffsetModel,
) -> bool {
    result.offset_seconds == before.offset_seconds && result.is_negative == true
}

impl PiecesNumericOffsetModel {
    /// Models `PiecesNumericOffset::from(Offset::from_seconds(secs)
    /// .unwrap())`: within jiff's documented valid range, the wrapped
    /// offset round-trips exactly and `is_negative` is set to whether
    /// `secs` is negative.
    pub fn from_offset_seconds(secs: i32) -> (result: PiecesNumericOffsetModel)
        requires
            pno_offset_seconds_in_range(secs as int),
        ensures
            pno_from_offset_seconds_holds(secs, result),
    {
        PiecesNumericOffsetModel { offset_seconds: secs, is_negative: secs < 0 }
    }

    /// Models `PiecesNumericOffset::with_negative_zero`: preserves the
    /// wrapped offset exactly, always forces `is_negative` to `true`.
    pub fn with_negative_zero(self) -> (result: PiecesNumericOffsetModel)
        ensures
            pno_with_negative_zero_holds(self, result),
    {
        PiecesNumericOffsetModel { offset_seconds: self.offset_seconds, is_negative: true }
    }
}

/// Exercises `from_offset_seconds`/`with_negative_zero` — the same
/// claim `amenable_kani::ext::jiff::fmt_temporal_pieces_numeric_offset`
/// and `amenable_creusot::ext_jiff::fmt_temporal_pieces_numeric_offset`
/// both check against jiff's real API.
pub fn verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_model(
    secs: i32,
) -> (result: bool)
    ensures
        result,
{
    if !(-93599..=93599).contains(&secs) {
        return true;
    }

    let pno = PiecesNumericOffsetModel::from_offset_seconds(secs);
    let from_ok = pno.offset_seconds == secs && pno.is_negative == (secs < 0);

    let pno_zeroed = PiecesNumericOffsetModel::from_offset_seconds(secs).with_negative_zero();
    let with_negative_zero_ok = pno_zeroed.offset_seconds == secs && pno_zeroed.is_negative;

    from_ok && with_negative_zero_ok
}

} // verus!
