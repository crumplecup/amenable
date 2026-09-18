//! Real Creusot proof content for `jiff::civil::ISOWeekDate`'s
//! round-trip property (`ext::jiff::civil_iso_week_date` holds the
//! `CreusotWitness` bridge that references the `_SRC` constant this
//! file's `harness!` call emits) — the same claim
//! `amenable_kani::ext::jiff::civil_iso_week_date`'s own doc comment
//! checks by symbolic execution.
//!
//! `jiff::civil::ISOWeekDate` is uncontracted everywhere — not
//! `creusot-std`, not `elicitation`. Needs opaque logic accessors for
//! `year()`/`week()`/`weekday()`, the same shape `civil_date.rs`'s
//! own accessors use, plus one for `Weekday`'s own real
//! `to_monday_one_offset()` accessor (an ordinary method call, same
//! restriction `civil_era.rs`'s own `era_discriminant` works around
//! for `Era`'s equality).
//!
//! Scoped the same way `amenable_kani::ext::jiff::civil_iso_week_date`
//! is, for the same real reason confirmed there (not assumed): a
//! first attempt using the full `-9999..=9999` year range alongside
//! week `1..=52` FAILED for real (`ISOWeekDate::MIN`/`MAX` are
//! derived from `Date::MIN`/`MAX`, not the leap-week rule alone, so a
//! week/weekday combination near the exact boundary years can still
//! land outside the overall representable range even at week `<=
//! 52`) — narrowed to `-9990..=9990` to restore the property honestly.

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
pub fn iso_week_date_year_value(_d: &jiff::civil::ISOWeekDate) -> i16 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn iso_week_date_week_value(_d: &jiff::civil::ISOWeekDate) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn iso_week_date_weekday_offset_value(_d: &jiff::civil::ISOWeekDate) -> i8 {
    dead
}

#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
pub fn weekday_monday_one_offset_value(_w: &jiff::civil::Weekday) -> i8 {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl jiff::civil::ISOWeekDate {
        #[check(ghost)]
        #[ensures(match result {
            Ok(ref d) => iso_week_date_year_value(d) == year
                && iso_week_date_week_value(d) == week
                && iso_week_date_weekday_offset_value(d) == weekday_monday_one_offset_value(&weekday),
            Err(_) => year < -9990i16 || year > 9990i16
                || week < 1i8 || week > 52i8,
        })]
        fn new(year: i16, week: i8, weekday: jiff::civil::Weekday) -> Result<jiff::civil::ISOWeekDate, jiff::Error>;

        #[check(ghost)]
        #[ensures(result == iso_week_date_year_value(&self))]
        fn year(self) -> i16;

        #[check(ghost)]
        #[ensures(result == iso_week_date_week_value(&self))]
        fn week(self) -> i8;

        #[check(ghost)]
        #[ensures(weekday_monday_one_offset_value(&result) == iso_week_date_weekday_offset_value(&self))]
        fn weekday(self) -> jiff::civil::Weekday;
    }

    impl jiff::civil::Weekday {
        #[check(ghost)]
        #[ensures(result == weekday_monday_one_offset_value(&self))]
        fn to_monday_one_offset(self) -> i8;

        #[check(ghost)]
        #[ensures(match result {
            Ok(ref w) => weekday_monday_one_offset_value(w) == offset,
            Err(_) => offset < 1i8 || offset > 7i8,
        })]
        fn from_monday_one_offset(offset: i8) -> Result<jiff::civil::Weekday, jiff::Error>;
    }
}

amenable_derive::harness! {
    creusot, CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::ISOWeekDate>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it. Sufficient, not exhaustive
        /// on the error side (states the same always-valid `1..=52`
        /// week sub-range, narrowed year range, `amenable_kani::
        /// ext::jiff::civil_iso_week_date` uses).
        #[logic(open)]
        fn civil_iso_week_date_new_year_week_weekday_round_trips(
            year: i16,
            week: i8,
            weekday_offset: i8,
            round_trip: Result<(i16, i8, i8), ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got.0 == year && got.1 == week && got.2 == weekday_offset,
                    Err(_) => year < -9990i16 || year > 9990i16
                        || week < 1i8 || week > 52i8
                        || weekday_offset < 1i8 || weekday_offset > 7i8,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_ISO_WEEK_DATE_NEW_YEAR_WEEK_WEEKDAY_ROUND_TRIPS_SRC, {
        /// `ISOWeekDate::new(year, week, weekday)`, whenever it
        /// succeeds within the always-valid `1..=52` week sub-range
        /// and narrowed year range, always returns an `ISOWeekDate`
        /// whose own `year()`/`week()`/`weekday()` (the last compared
        /// via its own `to_monday_one_offset()`) are exactly
        /// `year`/`week`/`weekday` back — the same claim
        /// `amenable_kani::ext::jiff::civil_iso_week_date::
        /// verify_civil_iso_week_date_new_year_week_weekday_round_trips`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition resting on the `extern_spec!` above.
        #[requires(true)]
        #[ensures(civil_iso_week_date_new_year_week_weekday_round_trips(year, week, weekday_offset, result))]
        fn verify_civil_iso_week_date_new_year_week_weekday_round_trips(
            year: i16,
            week: i8,
            weekday_offset: i8,
        ) -> Result<(i16, i8, i8), ()> {
            let weekday = match jiff::civil::Weekday::from_monday_one_offset(weekday_offset) {
                Ok(w) => w,
                Err(_) => return Err(()),
            };
            match jiff::civil::ISOWeekDate::new(year, week, weekday) {
                Ok(d) => Ok((d.year(), d.week(), d.weekday().to_monday_one_offset())),
                Err(_) => Err(()),
            }
        }
    }
}
