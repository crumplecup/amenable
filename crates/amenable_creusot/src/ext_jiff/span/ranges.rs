//! jiff's own documented valid input range for each `Span` unit
//! setter — the same constants `amenable_kani::ext::jiff::span`'s Kani
//! harness independently confirms.
//!
//! Each of the ten span-unit range predicates below is written out
//! literally, not macro-generated: `amenable_derive::harness!` captures
//! its `{ .. }` block's own *source span*, and a `macro_rules!` wrapper
//! around it captures the macro *definition's* unsubstituted `$name`/
//! `$ty` text at that span instead of each real instantiation -- a real
//! toolchain finding confirmed by reading the actual registered
//! fragment text cordial dumped, not assumed.

#[cfg(creusot)]
use creusot_std::macros::logic;

amenable_derive::harness! {
    creusot, SPAN_YEARS_IN_JIFF_RANGE_HOLDS_SRC, {
        /// jiff's own documented valid range for `Span::years` --
        /// `pub(crate)` so `span_fieldwise.rs` can reuse it.
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
