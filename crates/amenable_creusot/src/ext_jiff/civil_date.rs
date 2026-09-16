//! Real Creusot proof content for `jiff::civil::Date`'s round-trip
//! property (`ext::jiff::civil_date` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits) — the same claim
//! `amenable_kani::ext::jiff::civil_date`'s own doc comment checks by
//! symbolic execution.
//!
//! `jiff::civil::Date` is uncontracted everywhere — not `creusot-std`,
//! not `elicitation`. A real `extern_spec!` needs trusted logic
//! accessors for the values `Date::year()`/`Date::month()`/
//! `Date::day()` return, the same shape `offset.rs`'s own
//! `offset_seconds_value` axiom uses: an ordinary method call can't
//! appear inside an `#[ensures(..)]`/`#[logic]` clause, so `new`'s
//! postcondition and each accessor's postcondition all reference the
//! same three opaque axioms instead of calling each other.
//!
//! Scoped the same way `amenable_kani::ext::jiff::civil_date` is,
//! for the same real reason: day `1..=28` is valid for every month in
//! every year (even February in a non-leap year), so this avoids
//! needing to model days-in-month for a symbolic year/month
//! combination while still checking the full year/month range.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};

    #[trusted]
    #[logic(opaque)]
    pub(super) fn date_year_value(_d: &jiff::civil::Date) -> i16 {
        dead
    }

    #[trusted]
    #[logic(opaque)]
    pub(super) fn date_month_value(_d: &jiff::civil::Date) -> i8 {
        dead
    }

    #[trusted]
    #[logic(opaque)]
    pub(super) fn date_day_value(_d: &jiff::civil::Date) -> i8 {
        dead
    }
}
#[cfg(creusot)]
use mirror::{
    check, date_day_value, date_month_value, date_year_value, ensures, extern_spec, logic, requires,
};

#[cfg(creusot)]
extern_spec! {
    impl jiff::civil::Date {
        #[check(ghost)]
        #[ensures(match result {
            Ok(ref d) => date_year_value(d) == year
                && date_month_value(d) == month
                && date_day_value(d) == day,
            Err(_) => year < -9999i16 || year > 9999i16
                || month < 1i8 || month > 12i8
                || day < 1i8 || day > 28i8,
        })]
        fn new(year: i16, month: i8, day: i8) -> Result<jiff::civil::Date, jiff::Error>;

        #[check(ghost)]
        #[ensures(result == date_year_value(&self))]
        fn year(self) -> i16;

        #[check(ghost)]
        #[ensures(result == date_month_value(&self))]
        fn month(self) -> i8;

        #[check(ghost)]
        #[ensures(result == date_day_value(&self))]
        fn day(self) -> i8;
    }
}

amenable_derive::harness! {
    creusot, CIVIL_DATE_NEW_YEAR_MONTH_DAY_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::Date>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it. Sufficient, not exhaustive
        /// on the error side (states the same always-valid `1..=28`
        /// day sub-range `amenable_kani::ext::jiff::civil_date` uses,
        /// rather than reproducing jiff's real days-in-month logic).
        #[logic(open)]
        fn civil_date_new_year_month_day_round_trips(
            year: i16,
            month: i8,
            day: i8,
            round_trip: Result<(i16, i8, i8), ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got.0 == year && got.1 == month && got.2 == day,
                    Err(_) => year < -9999i16 || year > 9999i16
                        || month < 1i8 || month > 12i8
                        || day < 1i8 || day > 28i8,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_DATE_NEW_YEAR_MONTH_DAY_ROUND_TRIPS_SRC, {
        /// `Date::new(year, month, day)`, whenever it succeeds within
        /// the always-valid `1..=28` day sub-range, always returns a
        /// `Date` whose own `year()`/`month()`/`day()` are exactly
        /// `year`/`month`/`day` back — the same claim
        /// `amenable_kani::ext::jiff::civil_date::
        /// verify_civil_date_new_year_month_day_round_trips` checks by
        /// symbolic execution, restated as a real Creusot
        /// postcondition resting on the `extern_spec!` above.
        #[requires(true)]
        #[ensures(civil_date_new_year_month_day_round_trips(year, month, day, result))]
        fn verify_civil_date_new_year_month_day_round_trips(
            year: i16,
            month: i8,
            day: i8,
        ) -> Result<(i16, i8, i8), ()> {
            match jiff::civil::Date::new(year, month, day) {
                Ok(d) => Ok((d.year(), d.month(), d.day())),
                Err(_) => Err(()),
            }
        }
    }
}
