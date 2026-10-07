#![cfg(creusot)]
//! `chrono::NaiveDateTime`'s `extern_spec!` bridge: `NaiveDate::from_ymd_opt`
//! (needed to build a valid date to pass to `NaiveDateTime::new`, and not
//! extern-spec'd anywhere else -- `civil_naive_date.rs`'s own witness for this
//! type is model-based, a separate mechanism from this crate-wide-unique
//! `extern_spec!`), `NaiveDateTime::new` itself, and `NaiveDateTime`'s own
//! `Datelike`/`Timelike` trait impls.
//!
//! Self-gated via this file's own `#![cfg(creusot)]`, same as every sibling
//! `ext_chrono` module.

use crate::ext_chrono::civil_naive_date::valid_gregorian_spec;
use crate::ext_chrono::shared_trusted_accessors::{
    naive_date_day_value, naive_date_month_value, naive_date_time_day_value,
    naive_date_time_hour_value, naive_date_time_minute_value, naive_date_time_month_value,
    naive_date_time_nanosecond_value, naive_date_time_second_value, naive_date_time_year_value,
    naive_date_year_value, naive_time_hour_value, naive_time_minute_value,
    naive_time_nanosecond_value, naive_time_second_value,
};
use creusot_std::macros::{check, ensures, extern_spec};

extern_spec! {
    impl chrono::NaiveDate {
        #[check(ghost)]
        #[ensures(match result {
            Some(ref d) => valid_gregorian_spec(year, month, day)
                && naive_date_year_value(d) == year
                && naive_date_month_value(d) == month
                && naive_date_day_value(d) == day,
            None => !valid_gregorian_spec(year, month, day),
        })]
        fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<chrono::NaiveDate>;
    }

    impl chrono::NaiveDateTime {
        // `date`/`time` are already-valid constructed values (not raw
        // integers), so `new` is unconditional -- no `requires` beyond
        // `true` is needed, matching chrono's own real signature having no
        // `Option`/`Result` at all.
        #[check(ghost)]
        #[ensures(
            naive_date_time_year_value(&result) == naive_date_year_value(&date)
                && naive_date_time_month_value(&result) == naive_date_month_value(&date)
                && naive_date_time_day_value(&result) == naive_date_day_value(&date)
                && naive_date_time_hour_value(&result) == naive_time_hour_value(&time)
                && naive_date_time_minute_value(&result) == naive_time_minute_value(&time)
                && naive_date_time_second_value(&result) == naive_time_second_value(&time)
                && naive_date_time_nanosecond_value(&result) == naive_time_nanosecond_value(&time)
        )]
        fn new(date: chrono::NaiveDate, time: chrono::NaiveTime) -> chrono::NaiveDateTime;
    }

    impl chrono::Datelike for chrono::NaiveDateTime {
        #[check(ghost)]
        #[ensures(result == naive_date_time_year_value(self))]
        fn year(&self) -> i32;

        #[check(ghost)]
        #[ensures(result == naive_date_time_month_value(self))]
        fn month(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_date_time_day_value(self))]
        fn day(&self) -> u32;
    }

    impl chrono::Timelike for chrono::NaiveDateTime {
        #[check(ghost)]
        #[ensures(result == naive_date_time_hour_value(self))]
        fn hour(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_date_time_minute_value(self))]
        fn minute(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_date_time_second_value(self))]
        fn second(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_date_time_nanosecond_value(self))]
        fn nanosecond(&self) -> u32;
    }
}
