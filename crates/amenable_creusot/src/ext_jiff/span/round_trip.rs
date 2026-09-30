//! The final claim this whole module exists to prove, drawing on
//! `accessors` (jiff's real `Span` API contract), `ranges` (each
//! unit's valid input domain), and `lemmas` (the `Into<i64>`-stand-in
//! machinery).

#[cfg(creusot)]
use super::lemmas::{span_i64_of_i16_lemma, span_i64_of_i32_lemma, span_i64_of_i64_lemma};
#[cfg(creusot)]
use super::ranges::{
    span_days_in_jiff_range, span_hours_in_jiff_range, span_microseconds_in_jiff_range,
    span_milliseconds_in_jiff_range, span_minutes_in_jiff_range, span_months_in_jiff_range,
    span_nanoseconds_in_jiff_range, span_seconds_in_jiff_range, span_weeks_in_jiff_range,
    span_years_in_jiff_range,
};

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, requires};
}
#[cfg(creusot)]
use mirror::{ensures, requires};

amenable_derive::harness! {
    creusot, VERIFY_SPAN_UNIT_SETTERS_ROUND_TRIP_SRC, {
        /// Setting exactly one unit on a fresh `Span::new()` and reading
        /// it back through that unit's own getter reproduces the
        /// original value exactly, for every one of `Span`'s ten unit
        /// fields independently, whenever the value is within jiff's own
        /// documented range for that setter — the same claim
        /// `amenable_kani::ext::jiff::span`'s real Kani harness checks by
        /// symbolic execution, resting on `accessors.rs`'s `extern_spec!`.
        /// Calling each `span_i64_of_*_lemma` injects the fact that
        /// connects the generic setter's opaque `span_i64_of` to the
        /// concrete argument actually passed (see `lemmas.rs`'s own doc
        /// comment on why `.into()` can't appear in the `ensures` clause
        /// directly).
        #[requires(span_years_in_jiff_range(years))]
        #[requires(span_months_in_jiff_range(months))]
        #[requires(span_weeks_in_jiff_range(weeks))]
        #[requires(span_days_in_jiff_range(days))]
        #[requires(span_hours_in_jiff_range(hours))]
        #[requires(span_minutes_in_jiff_range(minutes))]
        #[requires(span_seconds_in_jiff_range(seconds))]
        #[requires(span_milliseconds_in_jiff_range(milliseconds))]
        #[requires(span_microseconds_in_jiff_range(microseconds))]
        #[requires(span_nanoseconds_in_jiff_range(nanoseconds))]
        #[ensures(result)]
        fn verify_span_unit_setters_round_trip(
            years: i16,
            months: i32,
            weeks: i32,
            days: i32,
            hours: i32,
            minutes: i64,
            seconds: i64,
            milliseconds: i64,
            microseconds: i64,
            nanoseconds: i64,
        ) -> bool {
            span_i64_of_i16_lemma(years);
            span_i64_of_i32_lemma(months);
            span_i64_of_i32_lemma(weeks);
            span_i64_of_i32_lemma(days);
            span_i64_of_i32_lemma(hours);
            span_i64_of_i64_lemma(minutes);
            span_i64_of_i64_lemma(seconds);
            span_i64_of_i64_lemma(milliseconds);
            span_i64_of_i64_lemma(microseconds);
            span_i64_of_i64_lemma(nanoseconds);
            jiff::Span::new().years(years).get_years() == years
                && jiff::Span::new().months(months).get_months() == months
                && jiff::Span::new().weeks(weeks).get_weeks() == weeks
                && jiff::Span::new().days(days).get_days() == days
                && jiff::Span::new().hours(hours).get_hours() == hours
                && jiff::Span::new().minutes(minutes).get_minutes() == minutes
                && jiff::Span::new().seconds(seconds).get_seconds() == seconds
                && jiff::Span::new().milliseconds(milliseconds).get_milliseconds() == milliseconds
                && jiff::Span::new().microseconds(microseconds).get_microseconds() == microseconds
                && jiff::Span::new().nanoseconds(nanoseconds).get_nanoseconds() == nanoseconds
        }
    }
}
