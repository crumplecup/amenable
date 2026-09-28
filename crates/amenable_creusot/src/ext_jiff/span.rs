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
}
#[cfg(creusot)]
use creusot_std::macros::{logic, trusted};
#[cfg(creusot)]
use mirror::{ensures, extern_spec, requires, span_i64_of};

amenable_derive::harness! {
    creusot, SPAN_I64_OF_I16_MATCHES_CAST_HOLDS_SRC, {
        /// The opaque `span_i64_of::<i16>` stand-in agrees with the
        /// real `as i64` cast it stands in for — named so `span_i64_of_
        /// i16_lemma`'s own postcondition points at a real predicate
        /// instead of restating the comparison inline.
        #[logic(open)]
        fn span_i64_of_i16_matches_cast(x: i16) -> bool {
            pearlite! { span_i64_of::<i16>(x) == x as i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_i64_of_i16_matches_cast",
        "creusot",
        "ensures",
        || SPAN_I64_OF_I16_MATCHES_CAST_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_I64_OF_I32_MATCHES_CAST_HOLDS_SRC, {
        /// The opaque `span_i64_of::<i32>` stand-in agrees with the
        /// real `as i64` cast it stands in for.
        #[logic(open)]
        fn span_i64_of_i32_matches_cast(x: i32) -> bool {
            pearlite! { span_i64_of::<i32>(x) == x as i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_i64_of_i32_matches_cast",
        "creusot",
        "ensures",
        || SPAN_I64_OF_I32_MATCHES_CAST_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_I64_OF_I64_MATCHES_CAST_HOLDS_SRC, {
        /// The opaque `span_i64_of::<i64>` stand-in agrees with the
        /// real `as i64` cast it stands in for (the identity cast, but
        /// checked the same way as the narrower widths for uniformity).
        #[logic(open)]
        fn span_i64_of_i64_matches_cast(x: i64) -> bool {
            pearlite! { span_i64_of::<i64>(x) == x as i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_i64_of_i64_matches_cast",
        "creusot",
        "ensures",
        || SPAN_I64_OF_I64_MATCHES_CAST_HOLDS_SRC,
    )
}

// `pub(crate)`, declared directly here (not nested in `mirror`, for the
// same real reason the getter accessors below are: `span_fieldwise.rs`
// needs to call these too, and a two-hop re-export through a private
// nested module is real toolchain territory Creusot's own visibility
// check rejects even though plain rustc accepts it.
#[cfg(creusot)]
#[trusted]
#[ensures(span_i64_of_i16_matches_cast(x))]
pub(crate) fn span_i64_of_i16_lemma(x: i16) {}

#[cfg(creusot)]
#[trusted]
#[ensures(span_i64_of_i32_matches_cast(x))]
pub(crate) fn span_i64_of_i32_lemma(x: i32) {}

#[cfg(creusot)]
#[trusted]
#[ensures(span_i64_of_i64_matches_cast(x))]
pub(crate) fn span_i64_of_i64_lemma(x: i64) {}

// `pub(crate)`, declared directly here (not nested in `mirror`):
// reused by `span_fieldwise.rs`'s own extern_spec, which needs the SAME
// opaque accessors the extern_spec below already ties them to --
// Creusot only allows one extern_spec per real function crate-wide, so
// redeclaring a second one for the same method isn't an option, and a
// two-hop re-export through a private nested module (`mirror::foo` ->
// `pub(crate) use mirror::foo`) is real toolchain territory Creusot's
// own visibility check rejects even though plain rustc accepts it --
// confirmed by a real "function import ... is private" error from
// `cargo creusot` (not from `cargo check`, which never compiles this
// `#[cfg(creusot)]`-gated code at all) when these lived inside `mirror`.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_years_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_months_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_weeks_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_days_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_hours_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_minutes_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_seconds_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_milliseconds_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_microseconds_value(_s: &jiff::Span) -> i64 {
    dead
}
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_nanoseconds_value(_s: &jiff::Span) -> i64 {
    dead
}

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

// Each of the ten span-unit range predicates below is written out
// literally, not macro-generated: `amenable_derive::harness!` captures
// its `{ .. }` block's own *source span*, and a `macro_rules!` wrapper
// around it captures the macro *definition's* unsubstituted `$name`/
// `$ty` text at that span instead of each real instantiation -- a real
// toolchain finding confirmed by reading the actual registered
// fragment text cordial dumped, not assumed.
amenable_derive::harness! {
    creusot, SPAN_YEARS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::years` --
        /// `pub(crate)` (not nested in `mirror`) so `span_fieldwise.rs`
        /// can reuse it.
        #[logic(open)]
        pub(crate) fn span_years_in_jiff_range(years: i16) -> bool {
            pearlite! { years > -19_998i16 - 1i16 && years < 19_998i16 + 1i16 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_years_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_YEARS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_MONTHS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::months`.
        #[logic(open)]
        pub(crate) fn span_months_in_jiff_range(months: i32) -> bool {
            pearlite! { months > -239_976i32 - 1i32 && months < 239_976i32 + 1i32 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_months_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_MONTHS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_WEEKS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::weeks`.
        #[logic(open)]
        pub(crate) fn span_weeks_in_jiff_range(weeks: i32) -> bool {
            pearlite! { weeks > -1_043_497i32 - 1i32 && weeks < 1_043_497i32 + 1i32 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_weeks_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_WEEKS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_DAYS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::days`.
        #[logic(open)]
        pub(crate) fn span_days_in_jiff_range(days: i32) -> bool {
            pearlite! { days > -7_304_484i32 - 1i32 && days < 7_304_484i32 + 1i32 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_days_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_DAYS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_HOURS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::hours`.
        #[logic(open)]
        pub(crate) fn span_hours_in_jiff_range(hours: i32) -> bool {
            pearlite! { hours > -175_307_616i32 - 1i32 && hours < 175_307_616i32 + 1i32 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_hours_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_HOURS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_MINUTES_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::minutes`.
        #[logic(open)]
        pub(crate) fn span_minutes_in_jiff_range(minutes: i64) -> bool {
            pearlite! { minutes > -10_518_456_960i64 - 1i64 && minutes < 10_518_456_960i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_minutes_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_MINUTES_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_SECONDS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::seconds`.
        #[logic(open)]
        pub(crate) fn span_seconds_in_jiff_range(seconds: i64) -> bool {
            pearlite! { seconds > -631_107_417_600i64 - 1i64 && seconds < 631_107_417_600i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_seconds_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_SECONDS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_MILLISECONDS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::milliseconds`.
        #[logic(open)]
        pub(crate) fn span_milliseconds_in_jiff_range(milliseconds: i64) -> bool {
            pearlite! {
                milliseconds > -631_107_417_600_000i64 - 1i64
                    && milliseconds < 631_107_417_600_000i64 + 1i64
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_milliseconds_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_MILLISECONDS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_MICROSECONDS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::microseconds`.
        #[logic(open)]
        pub(crate) fn span_microseconds_in_jiff_range(microseconds: i64) -> bool {
            pearlite! {
                microseconds > -631_107_417_600_000_000i64 - 1i64
                    && microseconds < 631_107_417_600_000_000i64 + 1i64
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_microseconds_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_MICROSECONDS_IN_JIFF_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_NANOSECONDS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::nanoseconds`.
        #[logic(open)]
        pub(crate) fn span_nanoseconds_in_jiff_range(nanoseconds: i64) -> bool {
            pearlite! { nanoseconds > -9_223_372_036_854_775_807i64 && nanoseconds < i64::MAX }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_nanoseconds_in_jiff_range",
        "creusot",
        "requires",
        || SPAN_NANOSECONDS_IN_JIFF_RANGE_HOLDS_SRC,
    )
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
