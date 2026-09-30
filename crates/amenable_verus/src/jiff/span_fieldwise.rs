//! Verus accommodation model for `jiff::SpanFieldwise`'s negation
//! property.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism for
//! Verus to reach jiff's actual code, full stop. The established
//! response (see `span.rs`'s own doc comment for the precedent) is a
//! hand-verified Verus-native model reproducing the type's documented
//! behavior. This proof is conditional on that reproduction being
//! faithful — which `amenable_kani::ext::jiff::span_fieldwise`'s and
//! `amenable_creusot::ext_jiff::span_fieldwise`'s own harnesses for the
//! identical claim, checked directly against jiff's real
//! `Span::fieldwise`/`SpanFieldwise`'s `Neg` impl/`Span::get_years`/…,
//! independently confirm. jiff's own valid ranges are all symmetric
//! (`-max..=max`), so negating an in-range value always stays in
//! range — the model below relies on that (confirmed against
//! `span.rs`'s own range constants, not assumed).

use super::span::SpanUnitFields;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The negation law this model establishes: within range for every
/// field, negating the modeled fieldwise wrapper negates every one of
/// its ten unit getters simultaneously; out of range for any field, it
/// reports none — the same claim `amenable_kani::ext::jiff::
/// span_fieldwise::verify_span_fieldwise_negation_negates_every_unit_getter`
/// and `amenable_creusot::ext_jiff::span_fieldwise::
/// verify_span_fieldwise_negation_negates_every_unit_getter` check
/// against jiff's real API.
pub open spec fn span_fieldwise_negation_model_holds(
    fields: SpanUnitFields,
    result: Option<SpanUnitFields>,
) -> bool {
    if super::span::span_years_in_range(fields.years as int)
        && super::span::span_months_in_range(fields.months as int)
        && super::span::span_weeks_in_range(fields.weeks as int)
        && super::span::span_days_in_range(fields.days as int)
        && super::span::span_hours_in_range(fields.hours as int)
        && super::span::span_minutes_in_range(fields.minutes as int)
        && super::span::span_seconds_in_range(fields.seconds as int)
        && super::span::span_milliseconds_in_range(fields.milliseconds as int)
        && super::span::span_microseconds_in_range(fields.microseconds as int)
        && super::span::span_nanoseconds_in_range(fields.nanoseconds as int) {
        match result {
            Some(negated) => negated.years == -fields.years && negated.months == -fields.months
                && negated.weeks == -fields.weeks && negated.days == -fields.days
                && negated.hours == -fields.hours && negated.minutes == -fields.minutes
                && negated.seconds == -fields.seconds
                && negated.milliseconds == -fields.milliseconds
                && negated.microseconds == -fields.microseconds
                && negated.nanoseconds == -fields.nanoseconds,
            None => false,
        }
    } else {
        result is None
    }
}

/// A model of negating each of `SpanFieldwise`'s ten unit getters
/// simultaneously: in range for every field, every getter negates
/// exactly; out of range for any field, `None`.
pub fn verify_span_fieldwise_negation_model_negates_every_unit_getter(
    fields: SpanUnitFields,
) -> (result: Option<SpanUnitFields>)
    ensures
        span_fieldwise_negation_model_holds(fields, result),
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
        Some(SpanUnitFields {
            years: -fields.years,
            months: -fields.months,
            weeks: -fields.weeks,
            days: -fields.days,
            hours: -fields.hours,
            minutes: -fields.minutes,
            seconds: -fields.seconds,
            milliseconds: -fields.milliseconds,
            microseconds: -fields.microseconds,
            nanoseconds: -fields.nanoseconds,
        })
    } else {
        None
    }
}

} // verus!
