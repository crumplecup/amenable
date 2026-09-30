//! Trusted `jiff::civil::*` builder/option/series types for Kani --
//! each has a setter-only public surface, or hits a real Kani-specific
//! wall (see each paragraph below).

//! `jiff::civil::Date` gets a real checked property too (see
//! `civil_date.rs`), the first `civil::*` type assessed: unlike
//! `civil::DateTime`/`Zoned` (opaque composites needing `TimeZone`'s
//! `Repr`), `Date` is a pure calendar value with no time zone
//! involved at all — checked directly by reading jiff's real source,
//! not assumed safe just because it's a "calendar type." A real
//! `jiff::Error` Drop-glue wall on `Date::new`'s `Err` arm (the same
//! class this module's own opening paragraphs document for `Offset`)
//! is fixed the identical way: an independent, Drop-free bounds check
//! gates the call.
//!
//! `jiff::civil::DateArithmetic` stays trusted, the identical shape
//! to `TimestampArithmetic`/`ZonedArithmetic`: checked directly
//! against jiff's real source (`src/civil/date.rs`), it has *no*
//! public methods of its own at all — `checked_add`/`checked_neg`/
//! `is_negative` are all private; only the `From` impls (construction,
//! not inspection) are public. Its one field (`duration`) is private
//! with no getter.
//!
//! `jiff::civil::DateDifference` stays trusted for the same
//! builder-only reason as `TimestampDifference`/`ZonedDifference`:
//! checked directly against jiff's real source, its five public
//! methods (`new`/`smallest`/`largest`/`mode`/`increment`) are all
//! plain setters; `rounding_may_change_span`/`since_with_largest_unit`
//! are private, and both fields (`date`/`round`) are private with no
//! getters.
//!
//! `jiff::civil::DateSeries` also stays trusted for Kani specifically,
//! for the SAME real reason as `TimestampSeries` — not `ZonedSeries`'s
//! `TimeZone::Repr` wall (`Date` has no time zone at all): confirmed
//! empirically (not assumed from either resemblance) that a single
//! `.series(period).next()` call times out even for a tiny assumed
//! range, and even with the result immediately `mem::forget`-ed, while
//! bare `Date::new` alone passes instantly — isolating the cost to
//! `DateSeries::next`'s own `checked_mul`/`checked_add` calls, both
//! `Result<_, jiff::Error>`-returning and `.ok()`-converted on every
//! step, the identical recursive-Arc Drop-glue wall this module's own
//! opening paragraphs document for `Offset`/`Error`/`TimestampSeries`.
//! See `gallery::jiff_error_drop_cost`'s own doc comment for the full
//! isolation. Checked on Creusot and Verus instead (`ext_jiff::
//! date_series`/`ext::jiff::date_series`), neither of which shares
//! this Rust-Drop-glue mechanism.
//!
//! `jiff::civil::DateTimeArithmetic` stays trusted, the identical
//! shape to `DateArithmetic`/`TimestampArithmetic`/`ZonedArithmetic`:
//! checked directly against jiff's real source
//! (`src/civil/datetime.rs`), it has *no* public methods of its own
//! at all — `checked_add`/`checked_neg`/`is_negative` are all
//! private; only the `From` impls (construction, not inspection) are
//! public. Its one field (`duration`) is private with no getter.
//!
//! `jiff::civil::DateTimeDifference` stays trusted for the same
//! builder-only reason as `DateDifference`/`TimestampDifference`/
//! `ZonedDifference`: checked directly against jiff's real source,
//! its five public methods (`new`/`smallest`/`largest`/`mode`/
//! `increment`) are all plain setters; `rounding_may_change_span` is
//! private, and both fields (`datetime`/`round`) are private with no
//! getters.
//!
//! `jiff::civil::DateTimeRound` stays trusted for the same
//! builder-only reason as `TimestampRound`/`ZonedRound`: checked
//! directly against jiff's real source, its four public methods
//! (`new`/`smallest`/`mode`/`increment`) are all plain setters; the
//! actual rounding logic (`round`) is private, and all three fields
//! (`smallest`/`mode`/`increment`) are private with no getters.
//!
//! `jiff::civil::DateTimeSeries` also stays trusted for Kani
//! specifically, for the identical real reason as `DateSeries` —
//! `civil::DateTime`, like `civil::Date`, is a pure calendar+clock
//! value with no `TimeZone` at all, so `ZonedSeries`'s `Repr` wall
//! cannot apply here either; `DateTimeSeries::next()` has the exact
//! same `checked_mul`/`checked_add` shape as `DateSeries::next()`,
//! confirmed via `gallery::jiff_error_drop_cost`'s own doc comment to
//! hit the identical `jiff::Error` Drop-glue wall. Checked on Creusot
//! and Verus instead (`ext_jiff::date_time_series`/`ext::jiff::
//! date_time_series`), neither of which shares this Rust-Drop-glue
//! mechanism.
//!
//! `jiff::civil::DateTimeWith` stays trusted on all three backends,
//! for the identical real reason as `ZonedWith` — checked directly,
//! not assumed from resemblance: its type (`{date_with: DateWith,
//! time_with: TimeWith}`) can't distinguish "fresh from `.with()`"
//! from "modified by a setter" any more than `ZonedWith`'s can, so
//! the one real law jiff documents ("no fields set" ⟹ "`build()`
//! returns the original unchanged") can't be stated soundly as an
//! unconditional `#[ensures(..)]`, and an accommodation model of just
//! that case reduces to a bare identity function. See `ZonedWith`'s
//! own doc comment above for the full reasoning.
//!
//! `jiff::civil::DateWith` stays trusted too, the third confirmed
//! instance of this exact pattern: its fields (`original: Date`,
//! `year: Option<DateWithYear>`, `month: Option<i8>`, `day:
//! Option<DateWithDay>`) are private either way, so the extern_spec/
//! model boundary still can't distinguish "no override" from "some
//! override" from the outside — the `Option` wrapping doesn't change
//! that, since private fields are invisible to a real `extern_spec!`
//! regardless of their own internal shape.
//!
//! `jiff::civil::Era` gets a real checked property too (see
//! `civil_era.rs`): though `Era` is a plain fieldless 2-variant enum
//! with no constructor of its own, its only real production path
//! (`Date::era_year() -> (i16, Era)`) has a precisely documented,
//! genuinely checkable law (`year >= 1` ⟺ `(year, Era::CE)`; `year <=
//! 0` ⟺ `(-year + 1, Era::BCE)`) — checked directly, not assumed
//! trusted just because the type itself is fieldless (that would be
//! the wrong lesson from `RoundMode`'s own trusted reasoning, which
//! rested on having literally no public methods, not on having no
//! fields).
//!
//! `jiff::civil::ISOWeekDate` gets a real checked property too (see
//! `civil_iso_week_date.rs`): a real value type with real getters
//! (`year`/`week`/`weekday`), the same shape as `civil::Date` —
//! checked directly by reading jiff's real source. `ISOWeekDate::
//! new`'s validity is more complex than `Date::new`'s (week `53` is
//! only valid for years containing a "leap week"), so this scopes to
//! week `1..=52`, jiff's own documented always-valid sub-range,
//! matching `civil_date.rs`'s own day-range simplification.
//!
//! `jiff::civil::Time` also gets a real checked property (see
//! `civil_time.rs`) — but unlike `Date`/`ISOWeekDate`, its validity
//! is a fully rectangular, unconditional domain with no
//! interdependency between fields at all (checked directly:
//! `Time::MIN`/`MAX` exactly match the same independent per-field
//! bounds, no derived-elsewhere restriction), so this witness states
//! jiff's FULL documented validity condition, not a narrowed
//! sufficient sub-range.
//!
//! `jiff::civil::TimeArithmetic` stays trusted, the identical shape
//! to `DateArithmetic`/`DateTimeArithmetic`/`TimestampArithmetic`/
//! `ZonedArithmetic`: checked directly against jiff's real source
//! (`src/civil/time.rs`), it has *no* public methods of its own at
//! all — `wrapping_add`/`wrapping_sub`/`checked_add`/`checked_neg`/
//! `is_negative` are all private; only the `From` impls (construction,
//! not inspection) are public. Its one field (`duration`) is private
//! with no getter.
//!
//! `jiff::civil::TimeDifference` stays trusted for the same
//! builder-only reason as `DateDifference`/`DateTimeDifference`/
//! `TimestampDifference`/`ZonedDifference`: checked directly against
//! jiff's real source, its five public methods (`new`/`smallest`/
//! `largest`/`mode`/`increment`) are all plain setters;
//! `rounding_may_change_span`/`until_with_largest_unit` are private,
//! and both fields (`time`/`round`) are private with no getters.
//!
//! `jiff::civil::TimeRound` stays trusted for the same builder-only
//! reason as `DateTimeRound`/`TimestampRound`/`ZonedRound`: checked
//! directly against jiff's real source, its four public methods
//! (`new`/`smallest`/`mode`/`increment`) are all plain setters; the
//! actual rounding logic (`round`) is private, and all three fields
//! (`smallest`/`mode`/`increment`) are private with no getters.
//!
//! `jiff::civil::TimeSeries` also stays trusted for Kani specifically,
//! for the identical real reason as `DateSeries`/`DateTimeSeries` —
//! `civil::Time`, like `civil::Date`/`civil::DateTime`, is a pure
//! clock value with no `TimeZone` at all, so `ZonedSeries`'s `Repr`
//! wall cannot apply here either; `TimeSeries::next()` has the exact
//! same `checked_mul`/`checked_add` shape as `DateSeries::next()`/
//! `DateTimeSeries::next()`, confirmed via `gallery::
//! jiff_error_drop_cost`'s own doc comment to hit the identical
//! `jiff::Error` Drop-glue wall. Checked on Creusot and Verus instead
//! (`ext_jiff::civil_time_series`/`ext::jiff::civil_time_series`),
//! neither of which shares this Rust-Drop-glue mechanism.
//!
//! `jiff::civil::TimeWith` stays trusted too, the fourth confirmed
//! instance of the `ZonedWith`/`DateTimeWith`/`DateWith` pattern:
//! its fields (`original: Time`, `hour: Option<i8>`, `minute:
//! Option<i8>`, `second: Option<i8>`, `millisecond: Option<i16>`,
//! `microsecond: Option<i16>`, `nanosecond: Option<i16>`,
//! `subsec_nanosecond: Option<i32>`) are private either way, so the
//! extern_spec/model boundary still can't distinguish "no override"
//! from "some override" from the outside — the identical soundness
//! obstacle, checked directly against jiff's real source rather than
//! assumed from the earlier three.
//!
//! `jiff::civil::Weekday` gets a real checked property too (see
//! `civil_weekday.rs`) — checked directly, not assumed trusted just
//! because it's a fieldless enum (the same lesson `Era` already
//! established): a real round-trip law over
//! `from_monday_one_offset`/`to_monday_one_offset`, the identical
//! claim already extern-spec'd once for `ISOWeekDate`'s own witness.
//!
//! `jiff::civil::WeekdaysForward` gets a real checked property too
//! (see `civil_weekdays_forward.rs`) — checked directly on Kani,
//! genuinely different from the `*Series` family: its `Iterator::
//! next()` has no `Result`/`jiff::Error` anywhere in its call chain
//! at all (confirmed by reading jiff's real source — it delegates to
//! an infallible `jcore` cycle plus a plain `const fn` match), so
//! the recursive-Arc Drop-glue wall that makes `TimestampSeries`/
//! `DateSeries`/`DateTimeSeries`/`TimeSeries` Kani-uncheckable never
//! applies here — not assumed trusted-and-why by resemblance to that
//! family just because it's also an "iterator over a jiff type."
//!
//! `jiff::civil::WeekdaysReverse` gets a real checked property too
//! (see `civil_weekdays_reverse.rs`) — re-verified, not assumed from
//! `WeekdaysForward`'s own confirmed shape: checked directly against
//! jiff's real source, `WeekdaysReverse::next()` has the identical
//! infallible `jcore`-delegation shape, no `Result`/`jiff::Error`
//! anywhere.

use crate::jiff::macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(
    jiff::civil::DateTime,
    jiff::civil::DateArithmetic,
    jiff::civil::DateDifference,
    jiff::civil::DateSeries,
    jiff::civil::DateTimeArithmetic,
    jiff::civil::DateTimeDifference,
    jiff::civil::DateTimeRound,
    jiff::civil::DateTimeSeries,
    jiff::civil::DateTimeWith,
    jiff::civil::DateWith,
    jiff::civil::TimeArithmetic,
    jiff::civil::TimeDifference,
    jiff::civil::TimeRound,
    jiff::civil::TimeSeries,
    jiff::civil::TimeWith,
);
