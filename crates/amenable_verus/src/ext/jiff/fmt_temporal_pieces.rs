//! Verus accommodation model for `jiff::fmt::temporal::
//! Pieces<'static>`'s `with_date`/`with_time` round trips.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_temporal_pieces`'s own harness for the identical claim,
//! checked directly against jiff's real API, independently confirms
//! (see that module's own doc comment for the scoping rationale:
//! `date`/`time` only, not `offset`/`time_zone_annotation`).
//!
//! Models `date`/`time` as plain tuples (`(i16, i8, i8)`/
//! `Option<(i8, i8, i8, i32)>`) rather than reusing `civil_date.rs`'s/
//! `civil_time.rs`'s own model types — `Pieces` itself has no
//! interdependency between its `date`/`time` fields (confirmed by
//! reading jiff's real `with_date`/`with_time` bodies: each is a
//! trivial `Pieces { date, ..self }`/`Pieces { time: Some(time),
//! ..self }`), so this model only needs to state that each setter
//! preserves what it doesn't touch.
//!
//! **A real, confirmed finding**: comparing tuples directly via `==`
//! in ordinary (exec) code hit a real "`core::tuple::impl&::eq` is
//! not supported" error — `vstd` doesn't cover 3-tuple/4-tuple
//! `PartialEq` out of the box, unlike the primitive fields
//! themselves. Fixed by comparing each tuple field individually
//! (`.0`/`.1`/`.2`/etc.) instead — real `i16`/`i8`/`i32` comparisons,
//! not a new `assume_specification` axiom.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::temporal::Pieces<'static>`, scoped to its
/// `date`/`time` fields (see this module's own doc comment for why
/// `offset`/`time_zone_annotation` are deliberately excluded).
pub struct PiecesModel {
    /// Models `Pieces::date()` as `(year, month, day)`.
    pub date: (i16, i8, i8),
    /// Models `Pieces::time()` as `Some((hour, minute, second,
    /// subsec_nanosecond))`, or `None` if unset.
    pub time: Option<(i8, i8, i8, i32)>,
}

/// `from_date`'s postcondition: the given date is stored, and no time
/// is set.
pub open spec fn pieces_from_date_holds(date: (i16, i8, i8), result: PiecesModel) -> bool {
    result.date == date && result.time is None
}

/// `with_date`'s postcondition: `date` is overwritten, `time` is
/// preserved exactly from the receiver.
pub open spec fn pieces_with_date_holds(
    before: PiecesModel,
    date: (i16, i8, i8),
    result: PiecesModel,
) -> bool {
    result.date == date && result.time == before.time
}

/// `with_time`'s postcondition: `time` is overwritten to `Some(time)`,
/// `date` is preserved exactly from the receiver.
pub open spec fn pieces_with_time_holds(
    before: PiecesModel,
    time: (i8, i8, i8, i32),
    result: PiecesModel,
) -> bool {
    result.time == Some(time) && result.date == before.date
}

impl PiecesModel {
    /// Models `Pieces::from(date)`: a fresh instance with the given
    /// date and no time set.
    pub fn from_date(date: (i16, i8, i8)) -> (result: PiecesModel)
        ensures
            pieces_from_date_holds(date, result),
    {
        PiecesModel { date, time: None }
    }

    /// Models `Pieces::with_date`: overwrites `date`, preserves
    /// `time` exactly.
    pub fn with_date(self, date: (i16, i8, i8)) -> (result: PiecesModel)
        ensures
            pieces_with_date_holds(self, date, result),
    {
        PiecesModel { date, time: self.time }
    }

    /// Models `Pieces::with_time`: overwrites `time` to `Some(time)`,
    /// preserves `date` exactly.
    pub fn with_time(self, time: (i8, i8, i8, i32)) -> (result: PiecesModel)
        ensures
            pieces_with_time_holds(self, time, result),
    {
        PiecesModel { date: self.date, time: Some(time) }
    }
}

/// Exercises `with_date`/`with_time` independently on their own fresh
/// instance each — the same claim `amenable_kani::ext::jiff::
/// fmt_temporal_pieces` and `amenable_creusot::ext_jiff::
/// fmt_temporal_pieces` both check against jiff's real API.
pub fn verify_fmt_temporal_pieces_with_date_with_time_round_trip_model(
    initial_date: (i16, i8, i8),
    date: (i16, i8, i8),
    time: (i8, i8, i8, i32),
) -> (result: bool)
    ensures
        result,
{
    let pieces_date = PiecesModel::from_date(initial_date).with_date(date);
    let date_ok = pieces_date.date.0 == date.0
        && pieces_date.date.1 == date.1
        && pieces_date.date.2 == date.2;

    let pieces_time = PiecesModel::from_date(initial_date).with_time(time);
    let time_ok = match pieces_time.time {
        Some(t) => t.0 == time.0 && t.1 == time.1 && t.2 == time.2 && t.3 == time.3,
        None => false,
    };

    date_ok && time_ok
}

} // verus!
