#![cfg(creusot)]
//! `jiff::tz::AmbiguousZoned`'s trusted logic axioms and the
//! `extern_spec!` bridge for `TimeZone::to_ambiguous_zoned`/
//! `into_ambiguous_zoned` and `AmbiguousZoned::offset`/`is_ambiguous`.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was five separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.
//!
//! `AmbiguousZoned` wraps two private fields (`ts: AmbiguousTimestamp`,
//! `tz: TimeZone`), with a completely PRIVATE (not even `pub(crate)`)
//! constructor — only reachable via `TimeZone::to_ambiguous_zoned(dt)`/
//! `into_ambiguous_zoned(dt)`. Real jiff source confirms
//! `to_ambiguous_zoned` calls `self.clone().into_ambiguous_zoned(dt)`,
//! which calls `self.to_ambiguous_timestamp(dt)` directly — the SAME
//! real function `amenable_kani::ext::jiff`'s own doc comment already
//! confirmed times out under CBMC even for a fully concrete call, so
//! this type is marked trusted for Kani too without a redundant fresh
//! 3-minute confirmation run.
//!
//! `amb_zoned_offset_is_unambiguous_value`/`amb_zoned_offset_seconds_
//! value` themselves now live in `ext_jiff::shared_trusted_accessors`:
//! `tz_offset_conflict.rs` needs to reuse them (its own `OffsetConflict::
//! resolve` extern_spec needs to relate to the real `AmbiguousZoned`
//! it returns). Also extern-specs `TimeZone::into_ambiguous_zoned`
//! (the by-value sibling of `to_ambiguous_zoned`, a DIFFERENT real
//! function, not yet contracted anywhere) for the same reason —
//! `OffsetConflict::resolve_with`'s real `AlwaysTimeZone` branch calls
//! it directly.

use crate::ext_jiff::shared_trusted_accessors::{
    amb_zoned_offset_is_unambiguous_value, amb_zoned_offset_seconds_value, offset_seconds_value,
    tz_fixed_seconds_value,
};
use creusot_std::macros::{check, ensures, extern_spec};

extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(
            amb_zoned_offset_is_unambiguous_value(&result) == true
            && amb_zoned_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
        )]
        fn to_ambiguous_zoned(&self, dt: jiff::civil::DateTime) -> jiff::tz::AmbiguousZoned;

        #[check(ghost)]
        #[ensures(
            amb_zoned_offset_is_unambiguous_value(&result) == true
            && amb_zoned_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
        )]
        fn into_ambiguous_zoned(self, dt: jiff::civil::DateTime) -> jiff::tz::AmbiguousZoned;
    }

    impl jiff::tz::AmbiguousZoned {
        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::AmbiguousOffset::Unambiguous { offset } => {
                amb_zoned_offset_is_unambiguous_value(&self) == true
                    && offset_seconds_value(&offset) == amb_zoned_offset_seconds_value(&self)
            }
            _ => amb_zoned_offset_is_unambiguous_value(&self) == false,
        })]
        fn offset(&self) -> jiff::tz::AmbiguousOffset;

        #[check(ghost)]
        #[ensures(result == (amb_zoned_offset_is_unambiguous_value(&self) == false))]
        fn is_ambiguous(&self) -> bool;
    }
}
