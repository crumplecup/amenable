//! Real Creusot proof content for `jiff::civil::Time`'s round-trip
//! property (`ext::jiff::civil_time` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits) — the same claim
//! `amenable_kani::ext::jiff::civil_time`'s own doc comment checks by
//! symbolic execution.
//!
//! `jiff::civil::Time` is uncontracted everywhere — not
//! `creusot-std`, not `elicitation`. Needs opaque logic accessors for
//! `hour()`/`minute()`/`second()`/`subsec_nanosecond()`, the same
//! shape `civil_date.rs`'s own accessors use — an ordinary method
//! call can't appear inside an `#[ensures(..)]`/`#[logic]` clause.
//!
//! Unlike `civil_date.rs`/`civil_iso_week_date.rs`, `Time::new`'s
//! validity is a fully rectangular, unconditional domain with no
//! interdependency between fields at all (confirmed the same way
//! `amenable_kani::ext::jiff::civil_time` is), so this states jiff's
//! FULL documented validity condition, not a narrowed sufficient
//! sub-range.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, requires};
}
#[cfg(creusot)]
use creusot_std::macros::{logic, trusted};
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, requires};

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_hour_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_minute_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_second_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_subsec_nanosecond_value(_t: &jiff::civil::Time) -> i32 {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::civil::Time {
        #[check(ghost)]
        #[ensures(match result {
            Ok(ref t) => civil_time_hour_value(t) == hour
                && civil_time_minute_value(t) == minute
                && civil_time_second_value(t) == second
                && civil_time_subsec_nanosecond_value(t) == subsec_nanosecond,
            Err(_) => hour < 0i8 || hour > 23i8
                || minute < 0i8 || minute > 59i8
                || second < 0i8 || second > 59i8
                || subsec_nanosecond < 0i32 || subsec_nanosecond > 999_999_999i32,
        })]
        fn new(hour: i8, minute: i8, second: i8, subsec_nanosecond: i32) -> Result<jiff::civil::Time, jiff::Error>;

        #[check(ghost)]
        #[ensures(result == civil_time_hour_value(&self))]
        fn hour(self) -> i8;

        #[check(ghost)]
        #[ensures(result == civil_time_minute_value(&self))]
        fn minute(self) -> i8;

        #[check(ghost)]
        #[ensures(result == civil_time_second_value(&self))]
        fn second(self) -> i8;

        #[check(ghost)]
        #[ensures(result == civil_time_subsec_nanosecond_value(&self))]
        fn subsec_nanosecond(self) -> i32;
    }
}

amenable_derive::harness! {
    creusot, CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::Time>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it. Exhaustive, not a narrowed
        /// sub-range: jiff's full documented validity condition,
        /// confirmed to have no interdependency between fields.
        #[logic(open)]
        fn civil_time_new_hour_minute_second_subsec_round_trips(
            hour: i8,
            minute: i8,
            second: i8,
            subsec_nanosecond: i32,
            round_trip: Result<(i8, i8, i8, i32), ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got.0 == hour && got.1 == minute && got.2 == second && got.3 == subsec_nanosecond,
                    Err(_) => hour < 0i8 || hour > 23i8
                        || minute < 0i8 || minute > 59i8
                        || second < 0i8 || second > 59i8
                        || subsec_nanosecond < 0i32 || subsec_nanosecond > 999_999_999i32,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_time::civil_time_new_hour_minute_second_subsec_round_trips",
        "creusot",
        "ensures",
        || CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_TIME_NEW_HOUR_MINUTE_SECOND_SUBSEC_ROUND_TRIPS_SRC, {
        /// `Time::new(hour, minute, second, subsec_nanosecond)`,
        /// whenever it succeeds, always returns a `Time` whose own
        /// `hour()`/`minute()`/`second()`/`subsec_nanosecond()` are
        /// exactly the inputs back — the same claim
        /// `amenable_kani::ext::jiff::civil_time::
        /// verify_civil_time_new_hour_minute_second_subsec_round_trips`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition resting on the `extern_spec!` above.
        #[requires(true)]
        #[ensures(civil_time_new_hour_minute_second_subsec_round_trips(hour, minute, second, subsec_nanosecond, result))]
        fn verify_civil_time_new_hour_minute_second_subsec_round_trips(
            hour: i8,
            minute: i8,
            second: i8,
            subsec_nanosecond: i32,
        ) -> Result<(i8, i8, i8, i32), ()> {
            match jiff::civil::Time::new(hour, minute, second, subsec_nanosecond) {
                Ok(t) => Ok((t.hour(), t.minute(), t.second(), t.subsec_nanosecond())),
                Err(_) => Err(()),
            }
        }
    }
}
