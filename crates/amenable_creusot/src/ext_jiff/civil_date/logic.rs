#![cfg(creusot)]
//! `jiff::civil::Date`'s trusted logic axioms and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` rather than a
//! `#[cfg(creusot)]` on its `mod` declaration in the parent — collapses
//! what was four separately `#[cfg(creusot)]`-gated items (one `use`,
//! three `fn`s) in the parent file into zero, since this file's
//! inclusion is already conditional on its own first line (cordial's
//! CFG-SCATTER finding; same fix as every sibling `ext_jiff` module).

use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn date_year_value(_d: &jiff::civil::Date) -> i16 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn date_month_value(_d: &jiff::civil::Date) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn date_day_value(_d: &jiff::civil::Date) -> i8 {
    dead
}

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
