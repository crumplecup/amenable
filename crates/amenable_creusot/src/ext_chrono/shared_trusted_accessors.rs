#![cfg(creusot)]
//! Opaque trusted logic accessors reused across more than one sibling file's own
//! `extern_spec!`.
//!
//! Self-gated via this file's own `#![cfg(creusot)]`, the same reason every
//! `ext_jiff` module is. chrono's `ext_jiff::shared_trusted_accessors` counterpart
//! only reached this shape after eight separate per-type `logic` submodules each
//! turned out too thin on their own to clear cordial's `VIS-MOD-THIN-001` floor;
//! chrono's own accessors start collected here from the first one, instead of
//! repeating that rework.

use creusot_std::macros::{logic, trusted};

// ── fixed_offset ─────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn fixed_offset_local_minus_utc_value(_o: &chrono::FixedOffset) -> i32 {
    dead
}

// ── naive_time ───────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_time_hour_value(_t: &chrono::NaiveTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_time_minute_value(_t: &chrono::NaiveTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_time_second_value(_t: &chrono::NaiveTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_time_nanosecond_value(_t: &chrono::NaiveTime) -> u32 {
    dead
}
