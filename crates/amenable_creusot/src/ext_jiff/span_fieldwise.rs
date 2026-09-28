//! Real Creusot proof content for `jiff::SpanFieldwise`'s negation
//! property (`ext::jiff::span_fieldwise` holds the `CreusotWitness`
//! bridge) — the same claim `amenable_kani::ext::jiff::span_fieldwise`'s
//! real Kani harness checks by symbolic execution, restated as a real
//! Creusot postcondition.
//!
//! `SpanFieldwise`'s own added value over `Span` is `Neg`/`Eq`/`Hash`;
//! `Span::fieldwise` is a pure wrap (`SpanFieldwise(self)`, confirmed by
//! reading jiff's real source), and `Neg::neg` is `SpanFieldwise(self.0
//! .negate())` — `negate` flips `Span`'s one shared sign field, so
//! every one of its ten unit getters (each `sign * magnitude`) negates
//! simultaneously. Both extern_specs below restate that fact via
//! `span.rs`'s already-established opaque getter accessors
//! (`span_get_years_value`/etc, `pub(crate)`-reused rather than
//! redeclared — Creusot allows only one extern_spec per real function
//! crate-wide, and those are already tied to the extern_specs on
//! `Span::get_years`/etc in `span.rs`), never `Span`-to-`Span`
//! structural equality directly (`Span` has no `PartialEq`/`DeepModel`
//! of its own to compare against — it's a foreign, opaque type from
//! Creusot's perspective).
//!
//! A real debugging lesson worth recording, not just the destination:
//! an early version of this file's harness omitted the
//! `span_i64_of_*_lemma` calls `span.rs`'s own harness needs (see that
//! file's doc comment) — assumed unnecessary while simplifying a
//! failing proof down to a minimal repro, since the *getter* chain
//! looked self-contained. It reliably failed at the exact same ratio
//! (20/30 sub-goals) with or without `--no-cache`, with or without a
//! 10x larger `--time` budget, and with or without the `fieldwise`/
//! `Neg` extern_specs even present — ruling out a timeout and ruling
//! out cross-file extern_spec interference. `why3find prove -X` on the
//! failing goal showed the real gap directly: the setter's postcondition
//! only ties the opaque `span_i64_of_i16(years)` to the result, and
//! nothing connects that opaque value back to the concrete `years`
//! input without the lemma call actually being made in the harness
//! body first — the missing lemma calls, not a toolchain limitation.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, extern_spec, requires};
}
#[cfg(creusot)]
use super::span::{
    span_days_in_jiff_range, span_hours_in_jiff_range, span_microseconds_in_jiff_range,
    span_milliseconds_in_jiff_range, span_minutes_in_jiff_range, span_months_in_jiff_range,
    span_nanoseconds_in_jiff_range, span_seconds_in_jiff_range, span_weeks_in_jiff_range,
    span_years_in_jiff_range,
};
#[cfg(creusot)]
use mirror::{ensures, extern_spec, requires};

#[cfg(creusot)]
use crate::ext_jiff::span::{
    span_get_days_value, span_get_hours_value, span_get_microseconds_value,
    span_get_milliseconds_value, span_get_minutes_value, span_get_months_value,
    span_get_nanoseconds_value, span_get_seconds_value, span_get_weeks_value, span_get_years_value,
    span_i64_of_i16_lemma, span_i64_of_i32_lemma, span_i64_of_i64_lemma,
};

#[cfg(creusot)]
extern_spec! {
    impl jiff::Span {
        #[ensures(
            span_get_years_value(&result.0) == span_get_years_value(&self)
                && span_get_months_value(&result.0) == span_get_months_value(&self)
                && span_get_weeks_value(&result.0) == span_get_weeks_value(&self)
                && span_get_days_value(&result.0) == span_get_days_value(&self)
                && span_get_hours_value(&result.0) == span_get_hours_value(&self)
                && span_get_minutes_value(&result.0) == span_get_minutes_value(&self)
                && span_get_seconds_value(&result.0) == span_get_seconds_value(&self)
                && span_get_milliseconds_value(&result.0) == span_get_milliseconds_value(&self)
                && span_get_microseconds_value(&result.0) == span_get_microseconds_value(&self)
                && span_get_nanoseconds_value(&result.0) == span_get_nanoseconds_value(&self)
        )]
        fn fieldwise(self) -> jiff::SpanFieldwise;
    }

    impl core::ops::Neg for jiff::SpanFieldwise {
        #[ensures(
            span_get_years_value(&result.0) == -span_get_years_value(&self.0)
                && span_get_months_value(&result.0) == -span_get_months_value(&self.0)
                && span_get_weeks_value(&result.0) == -span_get_weeks_value(&self.0)
                && span_get_days_value(&result.0) == -span_get_days_value(&self.0)
                && span_get_hours_value(&result.0) == -span_get_hours_value(&self.0)
                && span_get_minutes_value(&result.0) == -span_get_minutes_value(&self.0)
                && span_get_seconds_value(&result.0) == -span_get_seconds_value(&self.0)
                && span_get_milliseconds_value(&result.0) == -span_get_milliseconds_value(&self.0)
                && span_get_microseconds_value(&result.0) == -span_get_microseconds_value(&self.0)
                && span_get_nanoseconds_value(&result.0) == -span_get_nanoseconds_value(&self.0)
        )]
        fn neg(self) -> jiff::SpanFieldwise;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC, {
        /// Negating a `SpanFieldwise` negates every one of `Span`'s ten
        /// unit getters, for every one of `Span`'s ten unit fields
        /// independently, whenever the value is within jiff's own
        /// documented range for that setter — the same claim
        /// `amenable_kani::ext::jiff::span_fieldwise`'s real Kani
        /// harness checks by symbolic execution, resting on the
        /// `extern_spec!` above. Calling each `span_i64_of_*_lemma`
        /// injects the fact that connects the generic setter's opaque
        /// `span_i64_of` to the concrete argument actually passed (see
        /// `span.rs`'s own doc comment, and this file's own doc comment
        /// on the real debugging path that found this was missing).
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
        fn verify_span_fieldwise_negation_negates_every_unit_getter(
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
            (-jiff::Span::new().years(years).fieldwise()).0.get_years() == -years
                && (-jiff::Span::new().months(months).fieldwise()).0.get_months() == -months
                && (-jiff::Span::new().weeks(weeks).fieldwise()).0.get_weeks() == -weeks
                && (-jiff::Span::new().days(days).fieldwise()).0.get_days() == -days
                && (-jiff::Span::new().hours(hours).fieldwise()).0.get_hours() == -hours
                && (-jiff::Span::new().minutes(minutes).fieldwise()).0.get_minutes() == -minutes
                && (-jiff::Span::new().seconds(seconds).fieldwise()).0.get_seconds() == -seconds
                && (-jiff::Span::new().milliseconds(milliseconds).fieldwise())
                    .0
                    .get_milliseconds()
                    == -milliseconds
                && (-jiff::Span::new().microseconds(microseconds).fieldwise())
                    .0
                    .get_microseconds()
                    == -microseconds
                && (-jiff::Span::new().nanoseconds(nanoseconds).fieldwise())
                    .0
                    .get_nanoseconds()
                    == -nanoseconds
        }
    }
}
