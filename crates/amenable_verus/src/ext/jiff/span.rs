//! Verus accommodation model for `jiff::Span`'s ten independent unit
//! setter/getter round-trip pairs (`years`/`get_years`, `months`/
//! `get_months`, …).
//!
//! `jiff` has zero `vstd` coverage (a third-party crate, not `std`) and
//! Verus never resolves `Cargo.toml` at all — unlike Kani/Creusot,
//! there is no mechanism for Verus to reach jiff's actual code, full
//! stop. The established response (see `offset.rs`'s own doc comment
//! for the precedent) is a hand-verified Verus-native model
//! reproducing the type's documented behavior. This proof is
//! conditional on that reproduction being faithful — which
//! `amenable_kani::ext::jiff::span`'s and `amenable_creusot::
//! ext_jiff::span`'s own harnesses for the identical claim, checked
//! directly against jiff's real `Span::years`/`get_years`/…,
//! independently confirm. The model deliberately only covers the
//! in-range case (jiff's real setters panic out of range rather than
//! returning an error value — the same "sufficient, not exhaustive on
//! the failure side" scope the Kani/Creusot proofs for this type also
//! chose, guarding the call rather than modeling the panic).
//!
//! Mirrors `offset.rs`'s own `Option`-returning shape (in range:
//! `Some` of the value back out; out of range: `None`), generalized
//! from one field to all ten via one tuple, so the model actually
//! carries each value through rather than only restating "in range" as
//! a boolean.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own documented valid ranges for each `Span` unit setter — the
/// same constants the Kani and Creusot proofs for this type
/// independently confirm.
pub open spec fn span_years_in_range(years: int) -> bool {
    years >= -19998 && years <= 19998
}

/// jiff's own documented valid range for `Span::months`.
pub open spec fn span_months_in_range(months: int) -> bool {
    months >= -239976 && months <= 239976
}

/// jiff's own documented valid range for `Span::weeks`.
pub open spec fn span_weeks_in_range(weeks: int) -> bool {
    weeks >= -1043497 && weeks <= 1043497
}

/// jiff's own documented valid range for `Span::days`.
pub open spec fn span_days_in_range(days: int) -> bool {
    days >= -7304484 && days <= 7304484
}

/// jiff's own documented valid range for `Span::hours`.
pub open spec fn span_hours_in_range(hours: int) -> bool {
    hours >= -175307616 && hours <= 175307616
}

/// jiff's own documented valid range for `Span::minutes`.
pub open spec fn span_minutes_in_range(minutes: int) -> bool {
    minutes >= -10518456960 && minutes <= 10518456960
}

/// jiff's own documented valid range for `Span::seconds`.
pub open spec fn span_seconds_in_range(seconds: int) -> bool {
    seconds >= -631107417600 && seconds <= 631107417600
}

/// jiff's own documented valid range for `Span::milliseconds`.
pub open spec fn span_milliseconds_in_range(milliseconds: int) -> bool {
    milliseconds >= -631107417600000 && milliseconds <= 631107417600000
}

/// jiff's own documented valid range for `Span::microseconds`.
pub open spec fn span_microseconds_in_range(microseconds: int) -> bool {
    microseconds >= -631107417600000000 && microseconds <= 631107417600000000
}

/// jiff's own documented valid range for `Span::nanoseconds`.
pub open spec fn span_nanoseconds_in_range(nanoseconds: int) -> bool {
    nanoseconds >= -9223372036854775807 && nanoseconds <= 9223372036854775807
}

/// All ten unit fields, in the same order the real `Span` setters take
/// them (`years`, `months`, `weeks`, `days`, `hours`, `minutes`,
/// `seconds`, `milliseconds`, `microseconds`, `nanoseconds`).
pub struct SpanUnitFields {
    /// The `years` unit.
    pub years: i16,
    /// The `months` unit.
    pub months: i32,
    /// The `weeks` unit.
    pub weeks: i32,
    /// The `days` unit.
    pub days: i32,
    /// The `hours` unit.
    pub hours: i32,
    /// The `minutes` unit.
    pub minutes: i64,
    /// The `seconds` unit.
    pub seconds: i64,
    /// The `milliseconds` unit.
    pub milliseconds: i64,
    /// The `microseconds` unit.
    pub microseconds: i64,
    /// The `nanoseconds` unit.
    pub nanoseconds: i64,
}

/// The round-trip law this model establishes: within range for every
/// field, the modeled setter/getter round trip carries every field's
/// value back out exactly; out of range for any field, it reports
/// none — the same claim `amenable_kani::ext::jiff::span::
/// verify_span_unit_setters_round_trip` and `amenable_creusot::
/// ext_jiff::span::verify_span_unit_setters_round_trip` check against
/// jiff's real API.
pub open spec fn span_unit_setters_model_round_trip_holds(
    fields: SpanUnitFields,
    result: Option<SpanUnitFields>,
) -> bool {
    if span_years_in_range(fields.years as int) && span_months_in_range(fields.months as int)
        && span_weeks_in_range(fields.weeks as int) && span_days_in_range(fields.days as int)
        && span_hours_in_range(fields.hours as int) && span_minutes_in_range(fields.minutes as int)
        && span_seconds_in_range(fields.seconds as int)
        && span_milliseconds_in_range(fields.milliseconds as int)
        && span_microseconds_in_range(fields.microseconds as int)
        && span_nanoseconds_in_range(fields.nanoseconds as int) {
        match result {
            Some(got) => got == fields,
            None => false,
        }
    } else {
        result is None
    }
}

/// A model of setting each of `Span`'s ten units independently on a
/// fresh zero span and reading each back through its own getter: in
/// range for every field, every value round-trips exactly; out of
/// range for any field, `None`.
pub fn verify_span_unit_setters_model_round_trips(fields: SpanUnitFields) -> (result: Option<
    SpanUnitFields,
>)
    ensures
        span_unit_setters_model_round_trip_holds(fields, result),
{
    if (-19998..=19998).contains(&fields.years) && (-239976..=239976).contains(&fields.months)
        && (-1043497..=1043497).contains(&fields.weeks)
        && (-7304484..=7304484).contains(&fields.days)
        && (-175307616..=175307616).contains(&fields.hours)
        && (-10518456960..=10518456960).contains(&fields.minutes)
        && (-631107417600..=631107417600).contains(&fields.seconds)
        && (-631107417600000..=631107417600000).contains(&fields.milliseconds)
        && (-631107417600000000..=631107417600000000).contains(&fields.microseconds)
        && (-9223372036854775807..=9223372036854775807).contains(&fields.nanoseconds) {
        Some(fields)
    } else {
        None
    }
}

} // verus!
