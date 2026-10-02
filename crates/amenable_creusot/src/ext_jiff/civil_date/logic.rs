#![cfg(creusot)]
//! `jiff::civil::Date`'s `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` rather than a
//! `#[cfg(creusot)]` on its `mod` declaration in the parent (cordial's
//! CFG-SCATTER finding; same fix as every sibling `ext_jiff` module).
//! The opaque `date_*_value` accessors themselves now live in
//! `ext_jiff::shared_trusted_accessors` (reused by `civil_era.rs` and
//! `fmt_temporal_pieces.rs`, so a module too thin on its own to clear
//! cordial's `VIS-MOD-THIN-001` floor would have had to exist here
//! otherwise — see that module's own doc comment).

use crate::ext_jiff::shared_trusted_accessors::{
    date_day_value, date_month_value, date_year_value,
};
use creusot_std::macros::{check, ensures, extern_spec};

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
