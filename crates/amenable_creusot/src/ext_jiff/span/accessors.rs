#![cfg(creusot)]
//! `jiff::Span`'s real API contract: an opaque logic accessor per unit
//! getter, and the `extern_spec!` tying all ten setters/getters to
//! jiff's real methods.
//!
//! `jiff::Span` is uncontracted everywhere (no `creusot-std`/
//! `elicitation` prior art). Each getter (e.g. `get_years`) reads a
//! private field, so gets an opaque logic accessor (the same shape
//! `offset.rs`'s `offset_seconds_value` uses) — all ten typed `i64` for
//! uniformity, even where the real getter returns a narrower type
//! (`get_years() -> i16`, `get_months`/`get_weeks`/`get_days`/
//! `get_hours() -> i32`), since the postcondition just widens the real
//! result via `as i64` to compare.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` rather than a
//! `#[cfg(creusot)]` on every item here — collapses thirteen separately
//! gated items (ten `fn`s, three `use`s) into zero, since the whole
//! file's inclusion is already conditional on its own first line
//! (cordial's CFG-SCATTER finding).
//!
//! `span_get_*_value` are `pub(crate)`, declared directly at this
//! module's own top level: reused by `span_fieldwise.rs`'s own
//! extern_spec, which needs the SAME opaque accessors the extern_spec
//! below already ties them to — Creusot only allows one extern_spec
//! per real function crate-wide, so redeclaring a second one for the
//! same method isn't an option, and a two-hop re-export through a
//! private nested module is real toolchain territory Creusot's own
//! visibility check rejects even though plain rustc accepts it —
//! confirmed by a real "function import ... is private" error from
//! `cargo creusot` (not from `cargo check`, which never compiles this
//! creusot-only code at all) when these lived behind that indirection.

use super::lemmas::logic::span_i64_of;
use creusot_std::macros::{extern_spec, logic, trusted};

#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_years_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_months_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_weeks_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_days_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_hours_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_minutes_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_seconds_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_milliseconds_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_microseconds_value(_s: &jiff::Span) -> i64 {
    dead
}
#[trusted]
#[logic(opaque)]
pub(crate) fn span_get_nanoseconds_value(_s: &jiff::Span) -> i64 {
    dead
}

// jiff's own documented valid ranges for each `Span` unit setter (the
// same constants `amenable_kani::ext::jiff::span`'s Kani harness
// independently confirms).
extern_spec! {
    impl jiff::Span {
        #[ensures(true)]
        fn new() -> jiff::Span;

        #[ensures(span_get_years_value(&result) == span_i64_of(years))]
        fn years<I: Into<i64>>(self, years: I) -> jiff::Span;
        #[ensures(span_get_months_value(&result) == span_i64_of(months))]
        fn months<I: Into<i64>>(self, months: I) -> jiff::Span;
        #[ensures(span_get_weeks_value(&result) == span_i64_of(weeks))]
        fn weeks<I: Into<i64>>(self, weeks: I) -> jiff::Span;
        #[ensures(span_get_days_value(&result) == span_i64_of(days))]
        fn days<I: Into<i64>>(self, days: I) -> jiff::Span;
        #[ensures(span_get_hours_value(&result) == span_i64_of(hours))]
        fn hours<I: Into<i64>>(self, hours: I) -> jiff::Span;
        #[ensures(span_get_minutes_value(&result) == span_i64_of(minutes))]
        fn minutes<I: Into<i64>>(self, minutes: I) -> jiff::Span;
        #[ensures(span_get_seconds_value(&result) == span_i64_of(seconds))]
        fn seconds<I: Into<i64>>(self, seconds: I) -> jiff::Span;
        #[ensures(span_get_milliseconds_value(&result) == span_i64_of(milliseconds))]
        fn milliseconds<I: Into<i64>>(self, milliseconds: I) -> jiff::Span;
        #[ensures(span_get_microseconds_value(&result) == span_i64_of(microseconds))]
        fn microseconds<I: Into<i64>>(self, microseconds: I) -> jiff::Span;
        #[ensures(span_get_nanoseconds_value(&result) == span_i64_of(nanoseconds))]
        fn nanoseconds<I: Into<i64>>(self, nanoseconds: I) -> jiff::Span;

        #[ensures(result as i64 == span_get_years_value(&self))]
        fn get_years(&self) -> i16;
        #[ensures(result as i64 == span_get_months_value(&self))]
        fn get_months(&self) -> i32;
        #[ensures(result as i64 == span_get_weeks_value(&self))]
        fn get_weeks(&self) -> i32;
        #[ensures(result as i64 == span_get_days_value(&self))]
        fn get_days(&self) -> i32;
        #[ensures(result as i64 == span_get_hours_value(&self))]
        fn get_hours(&self) -> i32;
        #[ensures(result == span_get_minutes_value(&self))]
        fn get_minutes(&self) -> i64;
        #[ensures(result == span_get_seconds_value(&self))]
        fn get_seconds(&self) -> i64;
        #[ensures(result == span_get_milliseconds_value(&self))]
        fn get_milliseconds(&self) -> i64;
        #[ensures(result == span_get_microseconds_value(&self))]
        fn get_microseconds(&self) -> i64;
        #[ensures(result == span_get_nanoseconds_value(&self))]
        fn get_nanoseconds(&self) -> i64;
    }
}
