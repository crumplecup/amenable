#![cfg(creusot)]
//! `chrono_tz::TzOffset`'s `extern_spec!` bridge: `fix()`,
//! `base_utc_offset()`, and `dst_offset()`, each stated as its own
//! independent axiom (not the combined relationship stated wholesale
//! as one fact) -- the wrapper in the parent module's own `#[ensures]`
//! is what checks these three axioms are consistent with each other.
//! The same two-pillar shape `ext_chrono::tz::logic` already uses for
//! `Tz::name()`/`FromStr`, just three pillars here.
//!
//! Grounded in chrono-tz's own real source (`TzOffset { tz, offset:
//! FixedTimespan { utc_offset, dst_offset, .. } }`,
//! `fix()` computing `FixedOffset::east_opt(utc_offset + dst_offset)`,
//! `base_utc_offset()`/`dst_offset()` returning `utc_offset`/
//! `dst_offset` directly) and by chrono-tz's own public doctest on
//! `OffsetComponents` itself, which asserts exactly this relationship:
//! `total_offset.num_seconds() == london_time.offset().fix().
//! local_minus_utc() as i64`.

use crate::ext_chrono::shared_trusted_accessors::{
    duration_num_seconds_value, fixed_offset_local_minus_utc_value,
    tz_offset_dst_offset_seconds_logic, tz_offset_utc_offset_seconds_logic,
};
use creusot_std::macros::{check, ensures, extern_spec};

extern_spec! {
    impl chrono::Duration {
        #[check(ghost)]
        #[ensures(result == duration_num_seconds_value(&self))]
        fn num_seconds(&self) -> i64;
    }

    impl chrono_tz::OffsetComponents for chrono_tz::TzOffset {
        #[check(ghost)]
        #[ensures(duration_num_seconds_value(&result)@ == tz_offset_utc_offset_seconds_logic(&self)@)]
        fn base_utc_offset(&self) -> chrono::Duration;

        #[check(ghost)]
        #[ensures(duration_num_seconds_value(&result)@ == tz_offset_dst_offset_seconds_logic(&self)@)]
        fn dst_offset(&self) -> chrono::Duration;
    }

    impl chrono::Offset for chrono_tz::TzOffset {
        #[check(ghost)]
        #[ensures(
            fixed_offset_local_minus_utc_value(&result)@
                == tz_offset_utc_offset_seconds_logic(&self)@ + tz_offset_dst_offset_seconds_logic(&self)@
        )]
        fn fix(&self) -> chrono::FixedOffset;
    }
}
