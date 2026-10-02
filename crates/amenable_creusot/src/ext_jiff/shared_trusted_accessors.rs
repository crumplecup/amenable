#![cfg(creusot)]
//! Opaque trusted logic accessors reused across more than one sibling
//! file's own `extern_spec!`.
//!
//! Self-gated via this file's own `#![cfg(creusot)]`, same as every
//! other `ext_jiff` proof file. Exists because Creusot allows only one
//! `extern_spec!` per real function crate-wide: whichever file first
//! extern-specs a type becomes the forced "owner" of the opaque
//! accessor its postcondition rests on, and every OTHER file whose own
//! `extern_spec!` needs to relate to that same type has no choice but
//! to import the existing accessor rather than redeclare it.
//!
//! These 17 names used to live scattered across eight separate
//! per-type `logic` submodules (`civil_date::logic`, `civil_time::
//! logic`, `offset::logic`, `tz_ambiguous_timestamp::logic`,
//! `tz_time_zone::logic`, `tz_ambiguous_zoned::logic`,
//! `fmt_temporal_pieces_numeric_offset::logic`, `span::lemmas::
//! logic`), each too thin on its own (1-4 names) to clear cordial's
//! `VIS-MOD-THIN-001` ten-leaf-name floor for a `pub(crate)` module.
//! Collecting them here — the real shared concern they always were,
//! not eight unrelated accidents of which file happened to extern-spec
//! first — clears that floor honestly instead of exempting each one.
//! Each origin type's own `extern_spec!`-holding `logic.rs` now
//! imports its accessor(s) from here instead of defining them locally.

use creusot_std::macros::{ensures, logic, trusted};

// ── civil_date ───────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn date_year_value(_d: &jiff::civil::Date) -> i16 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn date_month_value(_d: &jiff::civil::Date) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn date_day_value(_d: &jiff::civil::Date) -> i8 {
    dead
}

// ── civil_time ───────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_hour_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_minute_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_second_value(_t: &jiff::civil::Time) -> i8 {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn civil_time_subsec_nanosecond_value(_t: &jiff::civil::Time) -> i32 {
    dead
}

// ── offset ───────────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn offset_seconds_value(_o: &jiff::tz::Offset) -> i32 {
    dead
}

// ── tz_ambiguous_timestamp ───────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_fixed_seconds_value(_tz: &jiff::tz::TimeZone) -> i32 {
    dead
}

// ── tz_time_zone ─────────────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn tz_is_unknown_value(_tz: &jiff::tz::TimeZone) -> bool {
    dead
}

// ── tz_ambiguous_zoned ───────────────────────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn amb_zoned_offset_is_unambiguous_value(_z: &jiff::tz::AmbiguousZoned) -> bool {
    dead
}

#[trusted]
#[logic(opaque)]
pub(crate) fn amb_zoned_offset_seconds_value(_z: &jiff::tz::AmbiguousZoned) -> i32 {
    dead
}

// ── fmt_temporal_pieces_numeric_offset ───────────────────────────────

#[trusted]
#[logic(opaque)]
pub(crate) fn pno_offset_seconds_value(_p: &jiff::fmt::temporal::PiecesNumericOffset) -> i32 {
    dead
}

// ── span::lemmas ─────────────────────────────────────────────────────
//
// The three lemmas' `#[ensures(..)]` clauses reference
// `span::lemmas`'s own `span_i64_of_*_matches_cast` logic predicates
// (each `harness!`-generated there), imported by name — a sibling
// module calling into another sibling is ordinary Rust name
// resolution, not a cycle: both sides only exist together, under the
// same `creusot` compilation.

use crate::ext_jiff::span::{
    span_i64_of_i16_matches_cast, span_i64_of_i32_matches_cast, span_i64_of_i64_matches_cast,
};

#[trusted]
#[logic(opaque)]
pub(crate) fn span_i64_of<I>(_x: I) -> i64 {
    dead
}

#[trusted]
#[ensures(span_i64_of_i16_matches_cast(x))]
pub(crate) fn span_i64_of_i16_lemma(x: i16) {}

#[trusted]
#[ensures(span_i64_of_i32_matches_cast(x))]
pub(crate) fn span_i64_of_i32_lemma(x: i32) {}

#[trusted]
#[ensures(span_i64_of_i64_matches_cast(x))]
pub(crate) fn span_i64_of_i64_lemma(x: i64) {}
