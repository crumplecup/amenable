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

// ── naive_date ───────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_year_value(_d: &chrono::NaiveDate) -> i32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_month_value(_d: &chrono::NaiveDate) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_day_value(_d: &chrono::NaiveDate) -> u32 {
    dead
}

// ── naive_date_time ──────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_year_value(_dt: &chrono::NaiveDateTime) -> i32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_month_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_day_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_hour_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_minute_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_second_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn naive_date_time_nanosecond_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

// ── chrono_tz::Tz ────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_name_logic(_t: &chrono_tz::Tz) -> &'static str {
    dead
}

// ── chrono_tz::TzOffset ──────────────────────────────────────────────
//
// `TzOffset` wraps two private `i32` fields (`FixedTimespan { utc_offset,
// dst_offset, .. }`, confirmed in source), unreachable here the same way
// `fixed_offset_local_minus_utc_value` above stands in for a value this
// crate cannot read directly. `fix()`/`base_utc_offset()`/`dst_offset()`
// are all real, independent public methods over that same private state
// -- each gets its own opaque accessor below, exactly the two-pillar
// shape `tz_name_logic` already uses for `Tz` itself.

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_offset_utc_offset_seconds_logic(_o: &chrono_tz::TzOffset) -> i64 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_offset_dst_offset_seconds_logic(_o: &chrono_tz::TzOffset) -> i64 {
    dead
}

// `chrono::Duration::num_seconds()` -- no existing `extern_spec!` for it
// anywhere in this crate yet, needed here since `base_utc_offset()`/
// `dst_offset()` both return a `Duration`, not a bare `i64`.
#[trusted]
#[logic(opaque)]
pub(crate) fn duration_num_seconds_value(_d: &chrono::Duration) -> i64 {
    dead
}
