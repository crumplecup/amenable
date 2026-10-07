//! Creusot proof content for `chrono::NaiveDate`, over an Amenable-owned model
//! of the proleptic Gregorian calendar.
//!
//! The proof is about the model. It holds for chrono's `NaiveDate::from_ymd_opt`
//! only under the refinement premise, which is stated, not discharged here:
//! chrono's `from_ymd_opt` refines `gregorian_from_ymd_model` over chrono's
//! supported year range `-262143..=262142` (`chrono` 0.4.45's `MIN_YEAR` and
//! `MAX_YEAR`). Creusot cannot see chrono's body, so that premise is a trusted
//! statement and is labeled as one on the row.
//!
//! Nothing here is narrowed. The day ranges over every `u32`, the month over
//! every `u32`, and the year over the full supported range.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, NAIVE_DATE_IS_LEAP_YEAR_SRC, {
        /// Gregorian leap year: divisible by 4, except centuries not divisible by 400.
        #[logic(open)]
        pub fn is_leap_year(year: i32) -> bool {
            pearlite! { year@ % 4 == 0 && (year@ % 100 != 0 || year@ % 400 == 0) }
        }
    }
}

amenable_derive::harness! {
    creusot, NAIVE_DATE_DAYS_IN_MONTH_SPEC_SRC, {
        /// Days in `month` of `year` in the proleptic Gregorian calendar, or `0`
        /// for a month outside `1..=12`.
        #[logic(open)]
        pub fn days_in_month_spec(year: i32, month: u32) -> u32 {
            pearlite! {
                if month == 2u32 {
                    if is_leap_year(year) { 29u32 } else { 28u32 }
                } else if month == 4u32 || month == 6u32 || month == 9u32 || month == 11u32 {
                    30u32
                } else if month >= 1u32 && month <= 12u32 {
                    31u32
                } else {
                    0u32
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, NAIVE_DATE_VALID_GREGORIAN_SPEC_SRC, {
        /// The specification: `(year, month, day)` names a real proleptic
        /// Gregorian date inside chrono's supported year range.
        #[logic(open)]
        pub fn valid_gregorian_spec(year: i32, month: u32, day: u32) -> bool {
            pearlite! {
                -262143i32 <= year && year <= 262142i32
                    && month >= 1u32 && month <= 12u32
                    && day >= 1u32 && day <= days_in_month_spec(year, month)
            }
        }
    }
}

amenable_derive::harness! {
    creusot, NAIVE_DATE_FROM_YMD_MODEL_ROUND_TRIP_HOLDS_SRC, {
        /// The round-trip law: a `Some` result reports back the year, month, and
        /// day it was built from, and `None` exactly when the triple is not a
        /// valid Gregorian date. Named so every caller's `ensures` points at a
        /// real, registered contract fragment instead of a raw tuple equation.
        #[logic(open)]
        pub fn naive_date_from_ymd_model_round_trip_holds(
            year: i32,
            month: u32,
            day: u32,
            result: Option<(i32, u32, u32)>,
        ) -> bool {
            pearlite! {
                match result {
                    Some(date) => date == (year, month, day) && valid_gregorian_spec(year, month, day),
                    None => !valid_gregorian_spec(year, month, day),
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::civil_naive_date::naive_date_from_ymd_model_round_trip_holds",
        "creusot",
        "ensures",
        || NAIVE_DATE_FROM_YMD_MODEL_ROUND_TRIP_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, NAIVE_DATE_FROM_YMD_MODEL_SRC, {
        /// The model of `NaiveDate::from_ymd_opt`. Returns `Some` exactly when the
        /// triple is a valid Gregorian date, and the `Some` value is the triple.
        #[requires(true)]
        #[ensures(naive_date_from_ymd_model_round_trip_holds(year, month, day, result))]
        fn gregorian_from_ymd_model(year: i32, month: u32, day: u32) -> Option<(i32, u32, u32)> {
            if -262143i32 <= year
                && year <= 262142i32
                && 1u32 <= month
                && month <= 12u32
                && 1u32 <= day
                && day <= days_in_month_model(year, month)
            {
                Some((year, month, day))
            } else {
                None
            }
        }
    }
}

amenable_derive::harness! {
    creusot, NAIVE_DATE_DAYS_IN_MONTH_MODEL_SRC, {
        /// Executable days-in-month, proven equal to the specification.
        #[requires(true)]
        #[ensures(result == days_in_month_spec(year, month))]
        fn days_in_month_model(year: i32, month: u32) -> u32 {
            let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
            if month == 2u32 {
                if leap { 29u32 } else { 28u32 }
            } else if month == 4u32 || month == 6u32 || month == 9u32 || month == 11u32 {
                30u32
            } else if month >= 1u32 && month <= 12u32 {
                31u32
            } else {
                0u32
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_NAIVE_DATE_MODEL_ROUND_TRIPS_SRC, {
        /// The round-trip claim, proven against the model over every input: a
        /// `Some` result reports back the year, month, and day it was built from,
        /// and `None` exactly when the triple is not a valid Gregorian date.
        #[requires(true)]
        #[ensures(naive_date_from_ymd_model_round_trip_holds(year, month, day, result))]
        fn verify_naive_date_model_round_trips(year: i32, month: u32, day: u32) -> Option<(i32, u32, u32)> {
            gregorian_from_ymd_model(year, month, day)
        }
    }
}
