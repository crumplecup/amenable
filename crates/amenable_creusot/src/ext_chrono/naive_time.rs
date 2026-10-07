//! Real Creusot proof content for `chrono::NaiveTime`'s round-trip property
//! (`chrono::naive_time` holds the `CreusotWitness` bridge that references the
//! `_SRC` constant this file's `harness!` call emits).
//!
//! `chrono::NaiveTime` has no Creusot-std or elicitation prior coverage. A real
//! `extern_spec!` needs trusted logic accessors for the four values `hour()`/
//! `minute()`/`second()`/`nanosecond()` return -- ordinary method calls can't
//! appear inside a Pearlite `#[ensures(..)]`/`#[logic]` clause -- so
//! `from_hms_nano_opt`'s postcondition and each accessor's own postcondition all
//! reference the same four opaque accessors instead of calling each other. Those
//! accessors live in `ext_chrono::shared_trusted_accessors`, alongside
//! `FixedOffset`'s own.

mod logic;
#[cfg(creusot)]
use chrono::Timelike as _;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, NAIVE_TIME_FROM_HMS_NANO_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<chrono::NaiveTime>` postcondition --
        /// real, callable Pearlite content, not just descriptive text alongside
        /// it. Restates chrono's documented valid range, including the
        /// leap-second exception, rather than reproducing its bounds-check
        /// arithmetic.
        #[logic(open)]
        fn naive_time_from_hms_nano_round_trips(
            hour: u32,
            minute: u32,
            second: u32,
            nano: u32,
            round_trip: Result<(u32, u32, u32, u32), ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got == (hour, minute, second, nano),
                    Err(_) => !(hour < 24u32 && minute < 60u32 && second < 60u32 && nano < 2_000_000_000u32
                        && (nano < 1_000_000_000u32 || second == 59u32)),
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::naive_time::naive_time_from_hms_nano_round_trips",
        "creusot",
        "ensures",
        || NAIVE_TIME_FROM_HMS_NANO_ROUND_TRIPS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_NAIVE_TIME_FROM_HMS_NANO_ROUND_TRIPS_SRC, {
        /// `NaiveTime::from_hms_nano_opt(hour, minute, second, nano)`, whenever
        /// it succeeds, always returns a `NaiveTime` whose own four accessors
        /// report back the same four values -- the same claim
        /// `amenable_kani::chrono::civil_naive_time::
        /// verify_naive_time_from_hms_nano_matches_time_of_day_validity` checks
        /// by symbolic execution, restated as a real Creusot postcondition
        /// resting on the `extern_spec!` above.
        #[requires(true)]
        #[ensures(naive_time_from_hms_nano_round_trips(hour, minute, second, nano, result))]
        fn verify_naive_time_from_hms_nano_round_trips(
            hour: u32,
            minute: u32,
            second: u32,
            nano: u32,
        ) -> Result<(u32, u32, u32, u32), ()> {
            match chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano) {
                Some(t) => Ok((t.hour(), t.minute(), t.second(), t.nanosecond())),
                None => Err(()),
            }
        }
    }
}
