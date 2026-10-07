#![cfg(creusot)]
//! `chrono::NaiveTime`'s `extern_spec!` bridge: its inherent
//! `from_hms_nano_opt` constructor and its `Timelike` trait impl's four
//! accessors.
//!
//! Self-gated via this file's own `#![cfg(creusot)]`, same as every sibling
//! `ext_chrono` module.

use crate::ext_chrono::shared_trusted_accessors::{
    naive_time_hour_value, naive_time_minute_value, naive_time_nanosecond_value,
    naive_time_second_value,
};
use creusot_std::macros::{check, ensures, extern_spec};

// chrono's own documented valid range for `from_hms_nano_opt`'s four fields:
// `hour < 24`, `minute < 60`, `second < 60`, and `nano < 2_000_000_000` -- but a
// `nano` of `1_000_000_000` or more (the leap-second range) is only valid when
// `second == 59` -- the same bound the Kani harness for this type independently
// confirms.
extern_spec! {
    impl chrono::NaiveTime {
        #[check(ghost)]
        #[ensures(match result {
            Some(ref t) => hour < 24u32 && minute < 60u32 && second < 60u32 && nano < 2_000_000_000u32
                && (nano < 1_000_000_000u32 || second == 59u32)
                && naive_time_hour_value(t) == hour
                && naive_time_minute_value(t) == minute
                && naive_time_second_value(t) == second
                && naive_time_nanosecond_value(t) == nano,
            None => !(hour < 24u32 && minute < 60u32 && second < 60u32 && nano < 2_000_000_000u32
                && (nano < 1_000_000_000u32 || second == 59u32)),
        })]
        fn from_hms_nano_opt(hour: u32, minute: u32, second: u32, nano: u32) -> Option<chrono::NaiveTime>;
    }

    impl chrono::Timelike for chrono::NaiveTime {
        #[check(ghost)]
        #[ensures(result == naive_time_hour_value(self))]
        fn hour(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_time_minute_value(self))]
        fn minute(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_time_second_value(self))]
        fn second(&self) -> u32;

        #[check(ghost)]
        #[ensures(result == naive_time_nanosecond_value(self))]
        fn nanosecond(&self) -> u32;
    }
}
