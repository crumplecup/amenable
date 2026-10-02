#![cfg(creusot)]
//! `jiff::civil::ISOWeekDate`'s trusted logic axioms and `extern_spec!`
//! bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was six separately `#[cfg(creusot)]`-gated items in the parent file
//! down to zero there, cordial's CFG-SCATTER finding.

use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub fn iso_week_date_year_value(_d: &jiff::civil::ISOWeekDate) -> i16 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn iso_week_date_week_value(_d: &jiff::civil::ISOWeekDate) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn iso_week_date_weekday_offset_value(_d: &jiff::civil::ISOWeekDate) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn weekday_monday_one_offset_value(_w: &jiff::civil::Weekday) -> i8 {
    dead
}

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
