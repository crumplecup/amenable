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

// All seven of this section's accessors are `pub`, not `pub(crate)`
// like this file's other accessors: called from
// `ext_chrono::gap_info::logic`'s `#[logic(open)]`
// `naive_date_time_before` (a transparent helper, not an opaque
// extern_spec `#[ensures]` clause) -- confirmed by real `cargo creusot`
// output that an open logic function's callees must be fully `pub`, not
// `pub(crate)`, or translation fails with "would call a less-visible
// item".

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_year_value(_dt: &chrono::NaiveDateTime) -> i32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_month_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_day_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_hour_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_minute_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_second_value(_dt: &chrono::NaiveDateTime) -> u32 {
    dead
}

#[trusted]
#[logic(opaque)]
pub fn naive_date_time_nanosecond_value(_dt: &chrono::NaiveDateTime) -> u32 {
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

// ── chrono::DateTime<chrono_tz::Tz> ─────────────────────────────────
//
// `DateTime<Tz>` stores its `NaiveDateTime`/`Tz::Offset` fields privately
// (confirmed in chrono's own source: `naive_local()` is
// `self.datetime.checked_add_offset(self.offset.fix())`, both fields
// unreachable here). Only `naive_local()`'s own result matters to
// `GapInfo::new`'s bracketing claim, so this is the one accessor needed,
// not the full `checked_add_offset`/`offset()` chain.

#[trusted]
#[logic(opaque)]
pub fn date_time_tz_naive_local_value<T: chrono::TimeZone>(
    _dt: &chrono::DateTime<T>,
) -> chrono::NaiveDateTime {
    dead
}

// Shared between `Tz::offset_from_local_datetime`'s and `GapInfo::new`'s
// own axioms: both real functions run the *same* binary search over
// `tz.timespans()` (confirmed by reading chrono-tz 0.10.4's
// `timezone_impl.rs` side by side -- both call `binary_search(0,
// timespans.len(), |i| timespans.local_span(i).cmp(timestamp))` and
// treat `Err(_)` as "no timespan contains this instant"), so tying both
// axioms to this one opaque predicate is a real structural fact, not a
// convenient assumption, and lets a real-execution wrapper cross-check
// their agreement without ever touching `GapInfo`'s own fields.
#[trusted]
#[logic(opaque)]
pub fn is_local_gap_logic(_tz: &chrono_tz::Tz, _local: &chrono::NaiveDateTime) -> bool {
    dead
}
