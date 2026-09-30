//! Real Creusot proof content for `jiff::Span`'s unit setter/getter
//! round-trip property (`ext::jiff::span` holds the `CreusotWitness`
//! bridge) — the same claim `amenable_kani::ext::jiff::span`'s real
//! Kani harness checks by symbolic execution, restated as a real
//! Creusot postcondition, for all ten unit fields.
//!
//! Split by concern: `lemmas` (the `Into<i64>`-stand-in machinery),
//! `accessors` (jiff's real `Span` API contract), and `ranges` (each
//! unit's valid input domain) are the three pieces `round_trip`'s
//! final harness draws on.

mod accessors;
mod lemmas;
mod ranges;
mod round_trip;

// The harness's own source constant is defined unconditionally by
// `amenable_derive::harness!` (only the contained fn item itself is
// `#[cfg(creusot)]`-gated), and `lib.rs` re-exports it from here.
pub use round_trip::VERIFY_SPAN_UNIT_SETTERS_ROUND_TRIP_SRC;

// Re-exported at this module's own level so `span_fieldwise.rs`'s
// existing `use super::span::{...}`/`use crate::ext_jiff::span::{...}`
// keep resolving unchanged.
#[cfg(creusot)]
pub(crate) use accessors::{
    span_get_days_value, span_get_hours_value, span_get_microseconds_value,
    span_get_milliseconds_value, span_get_minutes_value, span_get_months_value,
    span_get_nanoseconds_value, span_get_seconds_value, span_get_weeks_value, span_get_years_value,
};
#[cfg(creusot)]
pub(crate) use lemmas::{span_i64_of_i16_lemma, span_i64_of_i32_lemma, span_i64_of_i64_lemma};
#[cfg(creusot)]
pub(crate) use ranges::{
    span_days_in_jiff_range, span_hours_in_jiff_range, span_microseconds_in_jiff_range,
    span_milliseconds_in_jiff_range, span_minutes_in_jiff_range, span_months_in_jiff_range,
    span_nanoseconds_in_jiff_range, span_seconds_in_jiff_range, span_weeks_in_jiff_range,
    span_years_in_jiff_range,
};
