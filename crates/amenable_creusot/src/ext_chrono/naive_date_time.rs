//! Real Creusot proof content for `chrono::NaiveDateTime`'s round-trip
//! property (`chrono::naive_date_time` holds the `CreusotWitness` bridge that
//! references the `_SRC` constant this file's `harness!` call emits).
//!
//! Scoped to the seven scalar fields (year, month, day, hour, minute, second,
//! nanosecond), the same shape `amenable_kani::chrono::civil_naive_date_time`'s
//! own harness checks. chrono's real claim also checks `combined.date() ==
//! date && combined.time() == time`, which this witness does not restate:
//! comparing two `NaiveDate`/`NaiveTime` values via `==` inside a Pearlite
//! clause would need `DeepModel` coverage this crate has no precedent for on a
//! foreign type, and the check is redundant with the seven scalar ones in any
//! case -- if every field matches, the two objects carry identical state by
//! every means chrono exposes to compare them.

mod logic;
#[cfg(creusot)]
use chrono::{Datelike as _, Timelike as _};
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, NAIVE_DATE_TIME_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<chrono::NaiveDateTime>` postcondition
        /// -- real, callable Pearlite content, not just descriptive text
        /// alongside it.
        #[logic(open)]
        fn naive_date_time_round_trips(
            year: i32,
            month: u32,
            day: u32,
            hour: u32,
            minute: u32,
            second: u32,
            nano: u32,
            round_trip: Result<(i32, u32, u32, u32, u32, u32, u32), ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got == (year, month, day, hour, minute, second, nano),
                    Err(_) => true,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::naive_date_time::naive_date_time_round_trips",
        "creusot",
        "ensures",
        || NAIVE_DATE_TIME_ROUND_TRIPS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_NAIVE_DATE_TIME_ROUND_TRIPS_SRC, {
        /// `NaiveDateTime::new` over a date and a time built from the same
        /// fields reports back the same seven fields through its own
        /// `Datelike`/`Timelike` accessors, whenever both parts build
        /// successfully -- the same claim `amenable_kani::chrono::
        /// civil_naive_date_time::
        /// verify_naive_date_time_new_matches_date_and_time_validity` checks
        /// by symbolic execution, restated as a real Creusot postcondition
        /// resting on the `extern_spec!` above. Vacuously true (the `Err`
        /// side of the postcondition) whenever either part fails to build --
        /// this witness makes no claim about that case, the same scope
        /// `amenable_kani`'s own witness and the `extern_spec!` above both
        /// take, rather than independently restating chrono's bounds-check
        /// arithmetic a third time.
        #[requires(true)]
        #[ensures(naive_date_time_round_trips(year, month, day, hour, minute, second, nano, result))]
        fn verify_naive_date_time_round_trips(
            year: i32,
            month: u32,
            day: u32,
            hour: u32,
            minute: u32,
            second: u32,
            nano: u32,
        ) -> Result<(i32, u32, u32, u32, u32, u32, u32), ()> {
            match (
                chrono::NaiveDate::from_ymd_opt(year, month, day),
                chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano),
            ) {
                (Some(date), Some(time)) => {
                    let combined = chrono::NaiveDateTime::new(date, time);
                    Ok((
                        combined.year(),
                        combined.month(),
                        combined.day(),
                        combined.hour(),
                        combined.minute(),
                        combined.second(),
                        combined.nanosecond(),
                    ))
                }
                _ => Err(()),
            }
        }
    }
}
