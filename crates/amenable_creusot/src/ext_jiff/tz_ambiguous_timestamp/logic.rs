#![cfg(creusot)]
//! `jiff::tz::AmbiguousTimestamp`'s trusted logic axioms and the
//! `extern_spec!` bridge for `TimeZone::fixed`/`to_ambiguous_timestamp`
//! and `AmbiguousTimestamp::offset`/`is_ambiguous`.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was six separately `#[cfg(creusot)]`-gated items in the parent file
//! down to zero there, cordial's CFG-SCATTER finding.
//!
//! `AmbiguousTimestamp`'s own constructor is `pub(crate)`, only
//! reachable via `TimeZone::to_ambiguous_timestamp(dt)`. Deliberately
//! NARROWER than Kani's own (now-abandoned) attempt at the full
//! claim: doesn't check `.datetime()` round-trips `dt` at all —
//! `civil::DateTime` is itself trusted/opaque in this crate (no
//! decomposition accessors exist for it, by deliberate Phase 1
//! design), so `dt` is passed through this proof entirely as an
//! opaque black-box value, never inspected. Checks only `.offset()`/
//! `.is_ambiguous()`, decomposed via fresh opaque accessors.
//!
//! `fixed`'s own `extern_spec!` lives here (the first file to need
//! it), so `tz_time_zone.rs`'s own `unknown`/`is_unknown` claim
//! (`fixed`-constructed values are never unknown) extends THIS
//! ensures clause with an extra conjunct rather than redeclaring
//! `fixed` itself — only one `extern_spec!` per real function
//! crate-wide. `tz_fixed_seconds_value` stays `pub(crate)`, declared
//! directly at this module's own top level: reused by `tz_time_zone.
//! rs`, `tz_time_zone_offset_info.rs`, `tz_offset_conflict.rs`, and
//! `tz_ambiguous_zoned.rs`.

use crate::ext_jiff::offset::logic::offset_seconds_value;
use crate::ext_jiff::tz_time_zone::logic::tz_is_unknown_value;
use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_fixed_seconds_value(_tz: &jiff::tz::TimeZone) -> i32 {
    dead
}

#[trusted]
#[logic(opaque)]
fn amb_ts_offset_is_unambiguous_value(_t: &jiff::tz::AmbiguousTimestamp) -> bool {
    dead
}

#[trusted]
#[logic(opaque)]
fn amb_ts_offset_seconds_value(_t: &jiff::tz::AmbiguousTimestamp) -> i32 {
    dead
}

extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(
            tz_fixed_seconds_value(&result) == offset_seconds_value(&offset)
            && tz_is_unknown_value(&result) == false
        )]
        fn fixed(offset: jiff::tz::Offset) -> jiff::tz::TimeZone;

        #[check(ghost)]
        #[ensures(
            amb_ts_offset_is_unambiguous_value(&result) == true
            && amb_ts_offset_seconds_value(&result) == tz_fixed_seconds_value(&self)
        )]
        fn to_ambiguous_timestamp(&self, dt: jiff::civil::DateTime) -> jiff::tz::AmbiguousTimestamp;
    }

    impl jiff::tz::AmbiguousTimestamp {
        #[check(ghost)]
        #[ensures(match result {
            jiff::tz::AmbiguousOffset::Unambiguous { offset } => {
                amb_ts_offset_is_unambiguous_value(&self) == true
                    && offset_seconds_value(&offset) == amb_ts_offset_seconds_value(&self)
            }
            _ => amb_ts_offset_is_unambiguous_value(&self) == false,
        })]
        fn offset(&self) -> jiff::tz::AmbiguousOffset;

        #[check(ghost)]
        #[ensures(result == (amb_ts_offset_is_unambiguous_value(&self) == false))]
        fn is_ambiguous(&self) -> bool;
    }
}
