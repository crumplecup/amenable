//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::SpanFieldwise>` —
//! a real, checked property over jiff's actual public API
//! (`SpanFieldwise`'s own `Neg` impl, composed with `Span`'s unit
//! getters), not a trusted stub.
//!
//! `SpanFieldwise` is a `#[repr(transparent)]` newtype over `Span`
//! (`pub Span` field) that swaps in field-by-field `Eq`/`Hash` for a
//! type that otherwise has none — its own added value is exactly
//! `Neg`/`Eq`/`Hash`, not anything `Span` doesn't already offer.
//! Confirmed by reading jiff's real source (`src/span.rs`):
//! `Neg::neg` is `SpanFieldwise(self.0.negate())`, and `Span::negate`
//! is `Span { sign: -self.sign, ..self }` — flips the one shared sign
//! field, magnitudes untouched. Since every getter is `sign *
//! magnitude`, negating a `SpanFieldwise` negates every one of its ten
//! unit getters simultaneously — the real, checked property here,
//! reusing `span.rs`'s own "one setter call on a fresh `Span::new()`"
//! construction discipline and range constants. jiff's own valid
//! ranges (see `span.rs`'s own doc comment) are each one narrower than
//! their storage type's true minimum specifically so negation never
//! overflows (e.g. nanoseconds' documented minimum is `i64::MIN + 1`,
//! not `i64::MIN`) — confirmed here empirically by Kani, not merely
//! assumed from that reading.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use super::span::{
    SPAN_DAYS_MAX, SPAN_DAYS_MIN, SPAN_HOURS_MAX, SPAN_HOURS_MIN, SPAN_MICROSECONDS_MAX,
    SPAN_MICROSECONDS_MIN, SPAN_MILLISECONDS_MAX, SPAN_MILLISECONDS_MIN, SPAN_MINUTES_MAX,
    SPAN_MINUTES_MIN, SPAN_MONTHS_MAX, SPAN_MONTHS_MIN, SPAN_NANOSECONDS_MAX, SPAN_NANOSECONDS_MIN,
    SPAN_SECONDS_MAX, SPAN_SECONDS_MIN, SPAN_WEEKS_MAX, SPAN_WEEKS_MIN, SPAN_YEARS_MAX,
    SPAN_YEARS_MIN, SpanUnitFields,
};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::SpanFieldwise> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_span_fieldwise_negation_negates_every_unit_getter".to_owned(),
            VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::SpanFieldwise>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::SpanFieldwise>",
        "kani",
        || <ExtStandard<jiff::SpanFieldwise> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::SpanFieldwise>,
    "amenable_ext::ExtStandard<jiff::SpanFieldwise>",
    SpanUnitFields,
    |(
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
        milliseconds,
        microseconds,
        nanoseconds,
    )| {
        let years_ok = !(SPAN_YEARS_MIN..=SPAN_YEARS_MAX).contains(&years)
            || (-jiff::Span::new().years(years).fieldwise()).0.get_years() == -years;
        let months_ok = !(SPAN_MONTHS_MIN..=SPAN_MONTHS_MAX).contains(&months)
            || (-jiff::Span::new().months(months).fieldwise())
                .0
                .get_months()
                == -months;
        let weeks_ok = !(SPAN_WEEKS_MIN..=SPAN_WEEKS_MAX).contains(&weeks)
            || (-jiff::Span::new().weeks(weeks).fieldwise()).0.get_weeks() == -weeks;
        let days_ok = !(SPAN_DAYS_MIN..=SPAN_DAYS_MAX).contains(&days)
            || (-jiff::Span::new().days(days).fieldwise()).0.get_days() == -days;
        let hours_ok = !(SPAN_HOURS_MIN..=SPAN_HOURS_MAX).contains(&hours)
            || (-jiff::Span::new().hours(hours).fieldwise()).0.get_hours() == -hours;
        let minutes_ok = !(SPAN_MINUTES_MIN..=SPAN_MINUTES_MAX).contains(&minutes)
            || (-jiff::Span::new().minutes(minutes).fieldwise())
                .0
                .get_minutes()
                == -minutes;
        let seconds_ok = !(SPAN_SECONDS_MIN..=SPAN_SECONDS_MAX).contains(&seconds)
            || (-jiff::Span::new().seconds(seconds).fieldwise())
                .0
                .get_seconds()
                == -seconds;
        let milliseconds_ok = !(SPAN_MILLISECONDS_MIN..=SPAN_MILLISECONDS_MAX)
            .contains(&milliseconds)
            || (-jiff::Span::new().milliseconds(milliseconds).fieldwise())
                .0
                .get_milliseconds()
                == -milliseconds;
        let microseconds_ok = !(SPAN_MICROSECONDS_MIN..=SPAN_MICROSECONDS_MAX)
            .contains(&microseconds)
            || (-jiff::Span::new().microseconds(microseconds).fieldwise())
                .0
                .get_microseconds()
                == -microseconds;
        let nanoseconds_ok = !(SPAN_NANOSECONDS_MIN..=SPAN_NANOSECONDS_MAX).contains(&nanoseconds)
            || (-jiff::Span::new().nanoseconds(nanoseconds).fieldwise())
                .0
                .get_nanoseconds()
                == -nanoseconds;
        years_ok
            && months_ok
            && weeks_ok
            && days_ok
            && hours_ok
            && minutes_ok
            && seconds_ok
            && milliseconds_ok
            && microseconds_ok
            && nanoseconds_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_SPAN_FIELDWISE_NEGATION_NEGATES_EVERY_UNIT_GETTER_SRC, {
        /// Negating a `SpanFieldwise` (via its own `Neg` impl) negates
        /// every one of `Span`'s ten unit getters, for every one of
        /// `Span`'s ten unit fields independently, whenever the value is
        /// within jiff's own documented range for that setter. Checked
        /// for every value in each field's own integer width, not an
        /// assumed slice.
        #[kani::proof]
        fn verify_span_fieldwise_negation_negates_every_unit_getter() {
            let years: i16 = kani::any();
            let months: i32 = kani::any();
            let weeks: i32 = kani::any();
            let days: i32 = kani::any();
            let hours: i32 = kani::any();
            let minutes: i64 = kani::any();
            let seconds: i64 = kani::any();
            let milliseconds: i64 = kani::any();
            let microseconds: i64 = kani::any();
            let nanoseconds: i64 = kani::any();
            assert!(
                ExtStandard::<jiff::SpanFieldwise>::ensures((
                    years,
                    months,
                    weeks,
                    days,
                    hours,
                    minutes,
                    seconds,
                    milliseconds,
                    microseconds,
                    nanoseconds,
                )),
                "negating a SpanFieldwise must negate every one of Span's ten unit getters"
            );
        }
    }
}
