#![cfg(creusot)]
//! `jiff::tz::TimeZone`'s own `unknown`/`is_unknown`/`to_fixed_offset`
//! trusted logic axiom and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was four separately `#[cfg(creusot)]`-gated items in the parent
//! file down to zero there, cordial's CFG-SCATTER finding.
//!
//! `TimeZone::fixed`/`to_ambiguous_timestamp`/`to_ambiguous_zoned`/
//! `into_ambiguous_zoned` are ALREADY extern-spec'd in
//! `tz_ambiguous_timestamp.rs`/`tz_ambiguous_zoned.rs` (only one
//! `extern_spec!` per real function crate-wide) — this file adds
//! `unknown`/`is_unknown`/`to_fixed_offset`, the first real
//! TimeZone-only claims not already covered by a sibling file's own
//! `AmbiguousTimestamp`/`AmbiguousZoned`/`OffsetConflict` proof.
//! `tz_is_unknown_value` itself now lives in `ext_jiff::shared_
//! trusted_accessors` so `tz_ambiguous_timestamp.rs`'s own `fixed`
//! extern_spec can extend its ensures with an extra conjunct
//! (`fixed`-constructed values are never unknown) instead of
//! redeclaring `fixed` itself.
//!
//! `to_fixed_offset`'s own real body ALSO dispatches through
//! `repr::each!` (confirmed by reading jiff's real source directly),
//! but that's irrelevant to Creusot — `extern_spec!` never executes
//! the real body, it's a trusted axiom either way. Scoped the same
//! way every sibling `TimeZone`-touching file is: only the
//! `TimeZone::fixed`-constructed case is exercised, reusing the
//! shared `tz_fixed_seconds_value`.

use crate::ext_jiff::shared_trusted_accessors::{
    offset_seconds_value, tz_fixed_seconds_value, tz_is_unknown_value,
};
use creusot_std::macros::{check, ensures, extern_spec};

extern_spec! {
    impl jiff::tz::TimeZone {
        #[check(ghost)]
        #[ensures(tz_is_unknown_value(&result) == true)]
        fn unknown() -> jiff::tz::TimeZone;

        #[check(ghost)]
        #[ensures(result == tz_is_unknown_value(&self))]
        fn is_unknown(&self) -> bool;

        #[check(ghost)]
        #[ensures(match result {
            Ok(ref o) => offset_seconds_value(o) == tz_fixed_seconds_value(&self),
            Err(_) => false,
        })]
        fn to_fixed_offset(&self) -> Result<jiff::tz::Offset, jiff::Error>;
    }
}
