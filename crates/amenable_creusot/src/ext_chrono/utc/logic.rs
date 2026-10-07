#![cfg(creusot)]
//! `chrono::Utc`'s `extern_spec!` bridge: its `TimeZone` and `Offset` trait impls.
//!
//! Unlike `FixedOffset`'s inherent methods, `Utc`'s real behavior lives entirely in
//! two trait impls (`TimeZone for Utc`, `Offset for Utc`) — `extern_spec!` targets
//! a trait impl block the same way it targets an inherent one (confirmed precedent:
//! `ext_jiff::fmt_friendly_fractional_unit`'s `impl From<FractionalUnit> for Unit`).
//! `offset_from_local_date`'s postcondition only needs to match the `Single`
//! variant's tag, not compare its payload: `Utc` is zero-sized, so there is only
//! one possible value of that payload, and matching the tag already says
//! everything there is to say about it.

use crate::ext_chrono::shared_trusted_accessors::fixed_offset_local_minus_utc_value;
use creusot_std::macros::{check, ensures, extern_spec};

extern_spec! {
    impl chrono::TimeZone for chrono::Utc {
        #[check(ghost)]
        #[ensures(match result {
            chrono::MappedLocalTime::Single(_) => true,
            _ => false,
        })]
        fn offset_from_local_date(
            &self,
            local: &chrono::NaiveDate,
        ) -> chrono::MappedLocalTime<chrono::Utc>;
    }

    impl chrono::Offset for chrono::Utc {
        #[check(ghost)]
        #[ensures(fixed_offset_local_minus_utc_value(&result) == 0i32)]
        fn fix(&self) -> chrono::FixedOffset;
    }
}
