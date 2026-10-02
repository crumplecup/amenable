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
//!
//! `date_year_value`/`date_month_value`/`date_day_value` live in the
//! `logic` submodule (self-gated via its own `#![cfg(creusot)]`,
//! collapsing what was four separately `#[cfg(creusot)]`-gated items
//! here into the single macro-import `use` below — cordial's
//! CFG-SCATTER finding), `pub(crate)` there and reached by
//! `civil_era.rs` via the full `super::civil_date::logic::
//! date_year_value` path: its own `Date::era_year` extern_spec needs
//! `date_year_value` to state anything about `self`, and Creusot only
//! allows one `extern_spec!` per real function crate-wide — confirmed
//! via a genuine "duplicate extern specification for
//! jiff::civil::Date::new" compiler error from a first attempt that
//! (wrongly) redeclared `Date::new`'s own contract in `civil_era.rs`
//! instead of reusing this one.

pub(crate) mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

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

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_date::civil_date_new_year_month_day_round_trips",
        "creusot",
        "ensures",
        || CIVIL_DATE_NEW_YEAR_MONTH_DAY_ROUND_TRIPS_HOLDS_SRC,
    )
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
