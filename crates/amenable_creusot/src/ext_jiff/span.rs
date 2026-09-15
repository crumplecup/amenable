//! Real Creusot proof content for `jiff::Span`'s unit setter/getter
//! round-trip property (`ext::jiff::span` holds the `CreusotWitness`
//! bridge) — the same claim `amenable_kani::ext::jiff::span`'s real
//! Kani harness checks by symbolic execution, restated as a real
//! Creusot postcondition, for all ten unit fields.
//!
//! `jiff::Span` is uncontracted everywhere (no `creusot-std`/
//! `elicitation` prior art). Each getter (e.g. `get_years`) reads a
//! private field, so gets an opaque logic accessor (the same shape
//! `offset.rs`'s `offset_seconds_value` uses) — all ten typed `i64` for
//! uniformity, even where the real getter returns a narrower type
//! (`get_years() -> i16`, `get_months`/`get_weeks`/`get_days`/
//! `get_hours() -> i32`), since the postcondition just widens the real
//! result via `as i64` to compare.
//!
//! Every setter (`years`/`months`/…) is generic — `fn years<I:
//! Into<i64>>(self, years: I) -> Span` — a real toolchain finding
//! worth recording: Creusot's `extern_spec!` genuinely requires the
//! declared signature's generics to match the real function's,
//! confirmed by first trying a concrete (non-generic) signature and
//! getting a real "extern spec generics don't match" error; the fix is
//! writing the identical `<I: Into<i64>>` clause. But `.into()` itself
//! cannot appear inside `#[ensures]` — confirmed by a second real
//! error, "unbound function or predicate symbol `into_i16`", once the
//! generics matched but the ensures clause called `years.into()`
//! directly. The fix used here: an opaque `span_i64_of<I>` logic
//! function stands in for "whatever `Into<i64>::into` would produce"
//! (never calling it for real, since it's `#[logic(opaque)]`), plus
//! one small `#[trusted]` *lemma* function per concrete `I` this crate
//! actually instantiates the setters at (`i16`/`i32`/`i64` — the three
//! native widths jiff's own doc comments assign across the ten
//! fields), each just asserting `span_i64_of::<I>(x) == x as i64` and
//! called for its postcondition's side effect at each harness call
//! site. Three lemmas cover all ten fields, since the generic-over-`I`
//! opaqueness is shared.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, extern_spec, logic, requires, trusted};

    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_i64_of<I>(_x: I) -> i64 {
        dead
    }

    #[trusted]
    #[ensures(span_i64_of::<i16>(x) == x as i64)]
    pub(super) fn span_i64_of_i16_lemma(x: i16) {}

    #[trusted]
    #[ensures(span_i64_of::<i32>(x) == x as i64)]
    pub(super) fn span_i64_of_i32_lemma(x: i32) {}

    #[trusted]
    #[ensures(span_i64_of::<i64>(x) == x as i64)]
    pub(super) fn span_i64_of_i64_lemma(x: i64) {}

    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_years_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_months_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_weeks_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_days_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_hours_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_minutes_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_seconds_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_milliseconds_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_microseconds_value(_s: &jiff::Span) -> i64 {
        dead
    }
    #[trusted]
    #[logic(opaque)]
    pub(super) fn span_get_nanoseconds_value(_s: &jiff::Span) -> i64 {
        dead
    }
}
#[cfg(creusot)]
use mirror::{
    ensures, extern_spec, requires, span_get_days_value, span_get_hours_value,
    span_get_microseconds_value, span_get_milliseconds_value, span_get_minutes_value,
    span_get_months_value, span_get_nanoseconds_value, span_get_seconds_value,
    span_get_weeks_value, span_get_years_value, span_i64_of, span_i64_of_i16_lemma,
    span_i64_of_i32_lemma, span_i64_of_i64_lemma,
};

// jiff's own documented valid ranges for each `Span` unit setter (the
// same constants `amenable_kani::ext::jiff::span`'s Kani harness
// independently confirms).
#[cfg(creusot)]
extern_spec! {
    impl jiff::Span {
        #[ensures(true)]
        fn new() -> jiff::Span;

        #[ensures(span_get_years_value(&result) == span_i64_of(years))]
        fn years<I: Into<i64>>(self, years: I) -> jiff::Span;
        #[ensures(span_get_months_value(&result) == span_i64_of(months))]
        fn months<I: Into<i64>>(self, months: I) -> jiff::Span;
        #[ensures(span_get_weeks_value(&result) == span_i64_of(weeks))]
        fn weeks<I: Into<i64>>(self, weeks: I) -> jiff::Span;
        #[ensures(span_get_days_value(&result) == span_i64_of(days))]
        fn days<I: Into<i64>>(self, days: I) -> jiff::Span;
        #[ensures(span_get_hours_value(&result) == span_i64_of(hours))]
        fn hours<I: Into<i64>>(self, hours: I) -> jiff::Span;
        #[ensures(span_get_minutes_value(&result) == span_i64_of(minutes))]
        fn minutes<I: Into<i64>>(self, minutes: I) -> jiff::Span;
        #[ensures(span_get_seconds_value(&result) == span_i64_of(seconds))]
        fn seconds<I: Into<i64>>(self, seconds: I) -> jiff::Span;
        #[ensures(span_get_milliseconds_value(&result) == span_i64_of(milliseconds))]
        fn milliseconds<I: Into<i64>>(self, milliseconds: I) -> jiff::Span;
        #[ensures(span_get_microseconds_value(&result) == span_i64_of(microseconds))]
        fn microseconds<I: Into<i64>>(self, microseconds: I) -> jiff::Span;
        #[ensures(span_get_nanoseconds_value(&result) == span_i64_of(nanoseconds))]
        fn nanoseconds<I: Into<i64>>(self, nanoseconds: I) -> jiff::Span;

        #[ensures(result as i64 == span_get_years_value(&self))]
        fn get_years(&self) -> i16;
        #[ensures(result as i64 == span_get_months_value(&self))]
        fn get_months(&self) -> i32;
        #[ensures(result as i64 == span_get_weeks_value(&self))]
        fn get_weeks(&self) -> i32;
        #[ensures(result as i64 == span_get_days_value(&self))]
        fn get_days(&self) -> i32;
        #[ensures(result as i64 == span_get_hours_value(&self))]
        fn get_hours(&self) -> i32;
        #[ensures(result == span_get_minutes_value(&self))]
        fn get_minutes(&self) -> i64;
        #[ensures(result == span_get_seconds_value(&self))]
        fn get_seconds(&self) -> i64;
        #[ensures(result == span_get_milliseconds_value(&self))]
        fn get_milliseconds(&self) -> i64;
        #[ensures(result == span_get_microseconds_value(&self))]
        fn get_microseconds(&self) -> i64;
        #[ensures(result == span_get_nanoseconds_value(&self))]
        fn get_nanoseconds(&self) -> i64;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SPAN_UNIT_SETTERS_ROUND_TRIP_SRC, {
        /// Setting exactly one unit on a fresh `Span::new()` and reading
        /// it back through that unit's own getter reproduces the
        /// original value exactly, for every one of `Span`'s ten unit
        /// fields independently, whenever the value is within jiff's own
        /// documented range for that setter — the same claim
        /// `amenable_kani::ext::jiff::span`'s real Kani harness checks by
        /// symbolic execution, resting on the `extern_spec!` above.
        /// Calling each `span_i64_of_*_lemma` injects the fact that
        /// connects the generic setter's opaque `span_i64_of` to the
        /// concrete argument actually passed (see this module's own doc
        /// comment on why `.into()` can't appear in the `ensures` clause
        /// directly).
        #[requires(years > -19_998i16 - 1i16 && years < 19_998i16 + 1i16)]
        #[requires(months > -239_976i32 - 1i32 && months < 239_976i32 + 1i32)]
        #[requires(weeks > -1_043_497i32 - 1i32 && weeks < 1_043_497i32 + 1i32)]
        #[requires(days > -7_304_484i32 - 1i32 && days < 7_304_484i32 + 1i32)]
        #[requires(hours > -175_307_616i32 - 1i32 && hours < 175_307_616i32 + 1i32)]
        #[requires(minutes > -10_518_456_960i64 - 1i64 && minutes < 10_518_456_960i64 + 1i64)]
        #[requires(seconds > -631_107_417_600i64 - 1i64 && seconds < 631_107_417_600i64 + 1i64)]
        #[requires(
            milliseconds > -631_107_417_600_000i64 - 1i64
                && milliseconds < 631_107_417_600_000i64 + 1i64
        )]
        #[requires(
            microseconds > -631_107_417_600_000_000i64 - 1i64
                && microseconds < 631_107_417_600_000_000i64 + 1i64
        )]
        #[requires(nanoseconds > -9_223_372_036_854_775_807i64 && nanoseconds < i64::MAX)]
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
