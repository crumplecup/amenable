#![cfg(creusot)]
//! `jiff::tz::TimeZoneOffsetInfo`'s trusted logic axioms and the
//! `extern_spec!` bridge for `TimeZone::to_offset_info` and
//! `TimeZoneOffsetInfo::offset`/`dst`.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was five separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.

use crate::ext_jiff::shared_trusted_accessors::{offset_seconds_value, tz_fixed_seconds_value};
use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_offset_info_offset_seconds_value(_info: &jiff::tz::TimeZoneOffsetInfo<'_>) -> i32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_offset_info_is_dst_value(_info: &jiff::tz::TimeZoneOffsetInfo<'_>) -> bool {
    dead
}

extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(
            tz_offset_info_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
            && tz_offset_info_is_dst_value(&result) == false
        )]
        fn to_offset_info<'t>(&'t self, timestamp: jiff::Timestamp) -> jiff::tz::TimeZoneOffsetInfo<'t>;
    }

    impl<'t> jiff::tz::TimeZoneOffsetInfo<'t> {
        #[check(ghost)]
        #[ensures(offset_seconds_value(&result) == tz_offset_info_offset_seconds_value(&self))]
        fn offset(&self) -> jiff::tz::Offset;

        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::Dst::No => tz_offset_info_is_dst_value(&self) == false,
            jiff::tz::Dst::Yes => tz_offset_info_is_dst_value(&self) == true,
        })]
        fn dst(&self) -> jiff::tz::Dst;
    }
}
