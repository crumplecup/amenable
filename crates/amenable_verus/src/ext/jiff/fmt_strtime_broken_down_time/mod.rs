//! Verus accommodation model for `jiff::fmt::strtime::
//! BrokenDownTime`'s 12 numeric setter/getter round trips.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_strtime_broken_down_time`'s own harness for the identical
//! claim, checked directly against jiff's real API, independently
//! confirms (see that module's own doc comment for the real bounds
//! and the scoping rationale: exactly these 12 numeric fields, not
//! the 5 reference-type ones).
//!
//! Genuinely new territory for this checklist: every prior Verus
//! model in `ext::jiff` is a pure function building a fresh value;
//! this one models a real `&mut self` setter, using Verus's own
//! `old(self)`/final-`self` convention (no `mem::forget`/`ManuallyDrop`
//! workaround needed at all — Verus has no CBMC-style Drop-glue cost,
//! confirmed by this model verifying instantly).
//!
//! The model itself lives in `model.rs`, the range/post-state
//! predicates each setter names live in `predicates.rs`, and the 12
//! setters live in `setters.rs` — split three ways so this file stays
//! just `mod`/`pub use`.

mod model;
mod predicates;
mod setters;

pub use model::{
    BrokenDownTimeModel, BrokenDownTimeNumericFieldsModel,
    verify_fmt_strtime_broken_down_time_numeric_setters_round_trip_model,
};

#[cfg(verus_keep_ghost)]
pub use predicates::{
    broken_down_time_day_in_range, broken_down_time_day_of_year_in_range,
    broken_down_time_day_of_year_set_or_preserved, broken_down_time_day_set_or_preserved,
    broken_down_time_hour_in_range, broken_down_time_hour_set_or_preserved,
    broken_down_time_iso_week_in_range, broken_down_time_iso_week_set_or_preserved,
    broken_down_time_iso_week_year_set_or_preserved, broken_down_time_minute_or_second_in_range,
    broken_down_time_minute_set_or_preserved, broken_down_time_month_in_range,
    broken_down_time_month_set_or_preserved, broken_down_time_new_matches_new_spec,
    broken_down_time_second_set_or_preserved, broken_down_time_subsec_nanosecond_in_range,
    broken_down_time_subsec_nanosecond_set_or_preserved,
    broken_down_time_week_mon_set_or_preserved, broken_down_time_week_number_in_range,
    broken_down_time_week_sun_set_or_preserved, broken_down_time_year_in_range,
    broken_down_time_year_set_or_preserved,
};
