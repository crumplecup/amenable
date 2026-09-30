//! Trusted `jiff::civil::*` builder/option types (the `*Arithmetic`/
//! `*Difference`/`*Round`/`*With` families) -- each has a setter-only
//! public surface with no getters to check.

//! `jiff::civil::Date` gets a real checked property too (see
//! `civil_date.rs`), the first `civil::*` type: unlike `civil::
//! DateTime`/`Zoned`, `Date` is a pure calendar value with no time
//! zone involved at all, so a real `extern_spec!` (not an
//! accommodation model) round-trips `Date::new`/`year`/`month`/`day`
//! over jiff's real API — checked directly, not assumed safe by
//! resemblance to the already-trusted composite types.
//!
//! `jiff::civil::DateArithmetic` stays trusted, the identical shape
//! to `TimestampArithmetic`/`ZonedArithmetic`: it has no public
//! methods of its own at all, and its one field is private with no
//! getter.
//!
//! `jiff::civil::DateDifference` stays trusted for the same
//! builder-only reason as `TimestampDifference`/`ZonedDifference`:
//! its five public methods are all plain setters, and both fields
//! are private with no getters.
//!
//! `jiff::civil::DateSeries` gets a real checked property too (see
//! `date_series.rs`), an accommodation model — checked here despite
//! being trusted for Kani specifically, for the same real reason
//! `TimestampSeries` is (a `jiff::Error` Drop-glue wall), confirmed
//! distinct from `ZonedSeries`'s `TimeZone::Repr` wall since `Date`
//! has no time zone at all. Unlike `ZonedSeries`'s model, this one
//! needs no `TimeZone::UTC` scoping caveat: `Date` has no DST-repeat
//! retry loop to begin with.
//!
//! `jiff::civil::DateTimeArithmetic` stays trusted, the identical
//! shape to `DateArithmetic`/`TimestampArithmetic`/`ZonedArithmetic`:
//! it has no public methods of its own at all, and its one field is
//! private with no getter.
//!
//! `jiff::civil::DateTimeDifference` stays trusted for the same
//! builder-only reason as `DateDifference`/`TimestampDifference`/
//! `ZonedDifference`: its five public methods are all plain setters,
//! and both fields are private with no getters.
//!
//! `jiff::civil::DateTimeRound` stays trusted for the same
//! builder-only reason as `TimestampRound`/`ZonedRound`: its four
//! public methods are all plain setters, and all three fields are
//! private with no getters.
//!
//! `jiff::civil::DateTimeSeries` gets a real checked property too
//! (see `date_time_series.rs`), an accommodation model — checked
//! here despite being trusted for Kani specifically, for the
//! identical real reason `DateSeries` is (a `jiff::Error` Drop-glue
//! wall, confirmed distinct from `ZonedSeries`'s `TimeZone::Repr`
//! wall since `civil::DateTime` has no time zone at all). Identical
//! in shape to `date_series.rs`'s own model: no `TimeZone::UTC`
//! scoping caveat needed at all.
//!
//! `jiff::civil::DateTimeWith` stays trusted, for the identical real
//! reason as `ZonedWith` — its type can't distinguish "no overrides"
//! from "some override" any more than `ZonedWith`'s can, so the one
//! real law jiff documents can't be stated soundly as an
//! unconditional `#[ensures(..)]`, and an accommodation model of just
//! the "no override" case reduces to a bare identity function. See
//! `ZonedWith`'s own doc comment above for the full reasoning.
//!
//! `jiff::civil::DateWith` stays trusted too, the third confirmed
//! instance: its fields (private `Option`-wrapped overrides) are
//! just as invisible to a real `extern_spec!` as `ZonedWith`'s/
//! `DateTimeWith`'s, so the same soundness obstacle applies.
//!
//! `jiff::civil::Era` gets a real checked property too (see
//! `civil_era.rs`): a genuine `extern_spec!` on `Date::era_year`
//! checking jiff's own documented BCE/CE classification law, not
//! assumed trusted just because `Era` is fieldless. A real, distinct
//! Creusot finding from `Unit`'s own `DeepModel` wall: `Era` has no
//! `Ord` at all, so that specific wall never comes up, but a bare
//! equality comparison (`==`) is still an ordinary method call that
//! can't appear inside `#[ensures(..)]`/`#[logic]` — worked around
//! the same way as `civil_date.rs`'s multi-field accessors: an opaque
//! `era_discriminant` axiom stands in for "which variant this is,"
//! stated without ever calling `Era::eq` directly.
//!
//! `jiff::civil::ISOWeekDate` gets a real checked property too (see
//! `civil_iso_week_date.rs`): a real `extern_spec!` over `new`/
//! `year`/`week`/`weekday` plus `Weekday::to_monday_one_offset`,
//! scoped to a narrowed year range (`-9990..=9990`) confirmed
//! necessary by a real Kani failure — see `amenable_kani::ext::jiff`'s
//! own doc comment for the full finding.
//!
//! `jiff::civil::Time` also gets a real checked property (see
//! `civil_time.rs`) — a fully rectangular, unconditional validity
//! domain with no interdependency between fields, so this states
//! jiff's FULL documented validity condition, not a narrowed
//! sufficient sub-range.
//!
//! `jiff::civil::TimeArithmetic` stays trusted, the identical shape
//! to `DateArithmetic`/`DateTimeArithmetic`/`TimestampArithmetic`/
//! `ZonedArithmetic`: it has no public methods of its own at all,
//! and its one field is private with no getter.
//!
//! `jiff::civil::TimeDifference` stays trusted for the same
//! builder-only reason as `DateDifference`/`DateTimeDifference`/
//! `TimestampDifference`/`ZonedDifference`: its five public methods
//! are all plain setters, and both fields are private with no
//! getters.
//!
//! `jiff::civil::TimeRound` stays trusted for the same builder-only
//! reason as `DateTimeRound`/`TimestampRound`/`ZonedRound`: its four
//! public methods are all plain setters, and all three fields are
//! private with no getters.
//!
//! `jiff::civil::TimeSeries` gets a real checked property too (see
//! `civil_time_series.rs`), an accommodation model — checked here
//! despite being trusted for Kani specifically, for the identical
//! real reason `DateSeries`/`DateTimeSeries` are (a `jiff::Error`
//! Drop-glue wall, confirmed distinct from `ZonedSeries`'s
//! `TimeZone::Repr` wall since `civil::Time` has no time zone at
//! all). Modeled as a signed nanosecond count rather than jiff's
//! real within-a-day wraparound semantics, scoped to a comfortably
//! safe range that never approaches wraparound.
//!
//! `jiff::civil::TimeWith` stays trusted too, the fourth confirmed
//! instance of the `ZonedWith`/`DateTimeWith`/`DateWith` pattern:
//! its private `Option`-wrapped override fields are just as
//! invisible to a model boundary as the earlier three's.
//!
//! `jiff::civil::Weekday` gets a real checked property too (see
//! `civil_weekday.rs`) — checked directly, not assumed trusted just
//! because it's a fieldless enum. Reuses `civil_iso_week_date.rs`'s
//! existing `extern_spec!` for `Weekday::
//! from_monday_one_offset`/`to_monday_one_offset` directly, since
//! Creusot allows only one `extern_spec!` per real function
//! crate-wide.
//!
//! `jiff::civil::WeekdaysForward` gets a real checked property too
//! (see `civil_weekdays_forward.rs`), an accommodation model —
//! `WeekdaysForward::next()` has no `jiff::Error` anywhere in its
//! call chain at all (confirmed by reading jiff's real source), so
//! this is checked for real here rather than trusted like the
//! `*Series` family's Kani side; modeled purely in terms of the
//! plain `i8` Monday-one offset, matching `civil_weekday.rs`'s own
//! choice.
//!
//! `jiff::civil::WeekdaysReverse` gets a real checked property too
//! (see `civil_weekdays_reverse.rs`), the identical accommodation
//! model shape — re-verified, not assumed from `WeekdaysForward`'s
//! own confirmed no-`jiff::Error` shape.

use crate::CreusotWitness;
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

use super::bridge::{bridge_creusot_witness, impl_creusot_witness_trusted_ext};

impl_creusot_witness_trusted_ext!(
    jiff::civil::DateTime,
    jiff::civil::DateArithmetic,
    jiff::civil::DateDifference,
    jiff::civil::DateTimeArithmetic,
    jiff::civil::DateTimeDifference,
    jiff::civil::DateTimeRound,
    jiff::civil::DateTimeWith,
    jiff::civil::DateWith,
    jiff::civil::TimeArithmetic,
    jiff::civil::TimeDifference,
    jiff::civil::TimeRound,
    jiff::civil::TimeWith,
);
