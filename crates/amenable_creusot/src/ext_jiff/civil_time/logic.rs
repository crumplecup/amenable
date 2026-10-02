#![cfg(creusot)]
//! `jiff::civil::Time`'s trusted logic axioms and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was six separately `#[cfg(creusot)]`-gated items in the parent file
//! (three `use`, four `fn`, counting only "fn"/"use" kinds per
//! cordial's own classification) down to zero there, since the whole
//! file's inclusion is already conditional on its own first line
//! (cordial's CFG-SCATTER finding). `civil_time_*_value` stay
//! `pub(crate)`, declared directly at this module's own top level:
//! reused by `fmt_temporal_pieces.rs` and `fmt_strtime_meridiem.rs`.

use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_hour_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_minute_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_second_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_subsec_nanosecond_value(_t: &jiff::civil::Time) -> i32 {
    dead
}

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
