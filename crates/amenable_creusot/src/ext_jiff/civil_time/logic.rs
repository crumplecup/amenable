#![cfg(creusot)]
//! `jiff::civil::Time`'s `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` (cordial's
//! CFG-SCATTER finding; same fix as every sibling `ext_jiff` module).
//! The opaque `civil_time_*_value` accessors themselves now live in
//! `ext_jiff::shared_trusted_accessors` (reused by `fmt_temporal_
//! pieces.rs` and `fmt_strtime_meridiem.rs`, so a module too thin on
//! its own to clear cordial's `VIS-MOD-THIN-001` floor would have had
//! to exist here otherwise — see that module's own doc comment).

use crate::ext_jiff::shared_trusted_accessors::{
    civil_time_hour_value, civil_time_minute_value, civil_time_second_value,
    civil_time_subsec_nanosecond_value,
};
use creusot_std::macros::{check, ensures, extern_spec};

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
