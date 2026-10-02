#![cfg(creusot)]
//! `jiff::tz::Offset`'s trusted logic axiom and `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` — collapses what
//! was two separately `#[cfg(creusot)]`-gated items in the parent file
//! down to zero there, cordial's CFG-SCATTER finding.
//!
//! `offset_seconds_value` stays `pub(crate)`, declared directly at
//! this module's own top level: reused by `fmt_temporal_pieces_
//! numeric_offset.rs`, `tz_time_zone.rs`, `tz_ambiguous_timestamp.rs`,
//! `tz_ambiguous_zoned.rs`, `tz_time_zone_offset_info.rs`, and
//! `fmt_temporal_time_zone_annotation.rs` — Creusot only allows one
//! `extern_spec!` per real function crate-wide, so `Offset::seconds()`'s
//! own contract can't be redeclared at any of those call sites.

use creusot_std::macros::{check, ensures, extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn offset_seconds_value(_o: &jiff::tz::Offset) -> i32 {
    dead
}

// jiff's own documented valid range for `Offset::from_seconds`
// (`-25:59:59..=25:59:59`, in seconds) — the same constant the Kani
// harness for this type independently confirms, restated here as the
// trusted axiom's own failure-side claim.
extern_spec! {
    impl jiff::tz::Offset {
        #[check(ghost)]
        #[ensures(match result {
            Ok(ref offset) => offset_seconds_value(offset) == seconds,
            Err(_) => seconds < -93_599i32 || seconds > 93_599i32,
        })]
        fn from_seconds(seconds: i32) -> Result<jiff::tz::Offset, jiff::Error>;

        #[check(ghost)]
        #[ensures(result == offset_seconds_value(&self))]
        fn seconds(self) -> i32;
    }
}
