//! `Witness<VerusVerifier>` for `ExtStandard<T>` over jiff's registered
//! carriers. Lives here, not in a backend crate: `VerusVerifier` moved
//! to `amenable_core` in the Phase 8 dependency reversal (see
//! `docs/AMENABLE_TIME_PLAN.md`), which this crate already depends on
//! unconditionally, so the bridge lives with the type registrations —
//! mirroring `amenable_std::verus_witness`'s own placement (see
//! `docs/AMENABLE_EXT_PLAN.md`'s Architecture section).
//!
//! Most stay trusted: jiff is opaque to Verus (never resolves
//! `Cargo.toml`, no `vstd` coverage for third-party crates), so most
//! registrations have nothing beyond `Evidence::basis().audit()` to
//! rest on. `jiff::civil::Date`/`jiff::civil::Era`/`jiff::civil::
//! ISOWeekDate`/`jiff::civil::Time`/`jiff::civil::Weekday`/`jiff::
//! civil::WeekdaysForward`/`jiff::civil::WeekdaysReverse`/`jiff::
//! civil::DateSeries`/`jiff::civil::DateTimeSeries`/`jiff::civil::
//! TimeSeries`/`jiff::fmt::friendly::FractionalUnit`/`jiff::fmt::
//! strtime::BrokenDownTime`/`jiff::fmt::strtime::Meridiem`/`jiff::fmt::
//! temporal::Pieces<'static>`/`jiff::fmt::
//! temporal::PiecesNumericOffset`/`jiff::fmt::
//! temporal::PiecesOffset`/`jiff::fmt::
//! temporal::TimeZoneAnnotation<'static>`/`jiff::fmt::
//! temporal::TimeZoneAnnotationKind<'static>`/`jiff::fmt::
//! temporal::TimeZoneAnnotationName<'static>`/`jiff::fmt::
//! StdFmtWrite<String>`/`jiff::fmt::
//! StdIoWrite<Vec<u8>>`/`jiff::tz::Offset`/`jiff::
//! tz::AmbiguousTimestamp`/`jiff::
//! tz::AmbiguousZoned`/`jiff::
//! tz::Dst`/`jiff::
//! tz::OffsetConflict`/`jiff::
//! tz::TimeZone`/`jiff::
//! tz::TimeZoneDatabase`/
//! `jiff::Error`/`jiff::
//! SignedDuration`/`jiff::Span`/`jiff::SpanFieldwise`/`jiff::
//! TimestampSeries`/`jiff::Unit`/`jiff::ZonedSeries` are the
//! exceptions so far — real, hand-verified accommodation models in
//! `amenable_verus::jiff::{civil_date,civil_era,
//! civil_iso_week_date,civil_time,civil_time_series,civil_weekday,
//! civil_weekdays_forward,civil_weekdays_reverse,date_series,
//! date_time_series,fmt_friendly_fractional_unit,fmt_std_fmt_write,
//! fmt_std_io_write,fmt_strtime_broken_down_time,fmt_strtime_meridiem,
//! fmt_temporal_pieces,fmt_temporal_pieces_numeric_offset,
//! fmt_temporal_pieces_offset,
//! fmt_temporal_time_zone_annotation,
//! fmt_temporal_time_zone_annotation_kind,
//! fmt_temporal_time_zone_annotation_name,
//! offset,error,
//! signed_duration,span,
//! span_fieldwise,timestamp_series,tz_ambiguous_timestamp,
//! tz_ambiguous_zoned,tz_dst,tz_offset_conflict,tz_time_zone,
//! tz_time_zone_database,unit,
//! zoned_series}` (jiff itself
//! is still unreachable, but each
//! model's own law is genuinely checked, and independently confirmed
//! against the real API by the Kani/Creusot proofs for the identical
//! claim where those backends could check it at all — `TimestampSeries`,
//! `ZonedSeries`, `DateSeries`, `DateTimeSeries`, and `TimeSeries`
//! are all Kani-uncheckable (`TimestampSeries`/`DateSeries`/
//! `DateTimeSeries`/`TimeSeries` share a `jiff::Error` Drop-glue
//! wall; `ZonedSeries` hits a different one, `TimeZone`'s
//! pointer-tagged `Repr`) — `WeekdaysForward`/`WeekdaysReverse` are
//! DIFFERENT cases: genuinely Kani-CHECKABLE (no `jiff::Error`
//! anywhere in either's call chain, confirmed by reading jiff's real
//! source for both, not assumed from resemblance), their Verus
//! models exist purely because Verus can't reach jiff at all, the
//! same reason every model here needs one — and `Unit` is
//! Creusot-uncheckable, each for a real, confirmed reason, see
//! `amenable_kani::ext::jiff`'s and `amenable_creusot::ext::jiff`'s
//! own doc comments; `ZonedSeries`'s model is also honestly scoped to
//! `TimeZone::UTC`, where its real DST-repeat retry loop is
//! structurally unreachable, see `amenable_verus::jiff::
//! zoned_series`'s own doc comment — `DateSeries`'s, `DateTimeSeries`'s,
//! and `TimeSeries`'s models need no such scoping, since none of
//! `Date`/`civil::DateTime`/`civil::Time` has a DST-repeat retry loop
//! to begin with (`TimeSeries`'s model also sidesteps jiff's real
//! within-a-day wraparound semantics, scoped to a comfortably safe
//! range instead). `jiff::civil::Era`, `jiff::civil::ISOWeekDate`,
//! `jiff::civil::Time`, and `jiff::civil::Weekday` are the four
//! exceptions in this list genuinely checked on all three backends
//! with no asymmetry at all — each model exists purely because Verus
//! can't reach jiff at all, same as every other model here, not
//! because Kani or Creusot hit any wall for any of them;
//! `ISOWeekDate`'s own year range is narrowed to `-9990..=9990`, the
//! same real, Kani-confirmed reason `amenable_kani::ext::jiff::
//! civil_iso_week_date`'s own doc comment documents, while `Time`'s
//! model states jiff's FULL documented validity range with no
//! narrowing at all, confirmed to have no interdependency between
//! fields, and `Weekday`'s model states the round-trip law purely in
//! terms of the plain `i8` offset with no `Weekday`-shaped enum
//! needed at all, unlike `Unit`'s own `UnitModel`). `jiff::RoundMode`/`jiff::
//! SignedDurationRound`/`jiff::SpanArithmetic<'static>`/`jiff::
//! SpanCompare<'static>`/`jiff::SpanRelativeTo<'static>`/`jiff::
//! SpanRound<'static>`/`jiff::SpanTotal<'static>`/`jiff::
//! TimestampArithmetic`/`jiff::TimestampDifference`/`jiff::
//! TimestampRound`/`jiff::ZonedArithmetic`/`jiff::
//! ZonedDifference<'static>`/`jiff::ZonedRound` stay trusted for
//! different, simpler real reasons (checked against jiff's real
//! source): `RoundMode` has no public methods at all beyond the
//! standard derives; the other twelve are pure builders/markers (no
//! getters, real internal logic private). `jiff::
//! TimestampDisplayWithOffset` also stays trusted, but for a
//! different real reason: its only public behavior is a `Display`
//! impl, and checking its exact RFC 3339 output would mean
//! reproducing jiff's whole formatting algorithm. `jiff::ZonedWith`
//! also stays trusted, for a reason checked directly rather than
//! assumed: the one real, simple law jiff documents for it ("no
//! fields set" ⟹ "returns the original unchanged") can't be modeled
//! honestly without also tracking which setters were called (its type
//! carries no such distinction), and a model scoped to just the "no
//! override" case reduces to a bare identity function with no
//! distinguishing computation at all — see `amenable_kani::ext::jiff`'s
//! own doc comment for the full reasoning, independently reached the
//! same way on Creusot. `jiff::civil::DateArithmetic` also stays
//! trusted, the identical shape to `TimestampArithmetic`/`jiff::
//! ZonedArithmetic`: no public methods at all, one private field, only
//! `From` impls public. `jiff::civil::DateDifference` also stays
//! trusted, the identical builder-only shape to `TimestampDifference`/
//! `jiff::ZonedDifference`: five plain-setter public methods, both
//! fields private with no getters. `jiff::civil::DateTimeArithmetic`
//! also stays trusted, the identical shape to `DateArithmetic`/
//! `TimestampArithmetic`/`jiff::ZonedArithmetic`: no public methods at
//! all, one private field, only `From` impls public. `jiff::civil::
//! DateTimeDifference` also stays trusted, the identical builder-only
//! shape to `DateDifference`/`TimestampDifference`/`jiff::
//! ZonedDifference`: five plain-setter public methods, both fields
//! private with no getters. `jiff::civil::DateTimeRound` also stays
//! trusted, the identical builder-only shape to `TimestampRound`/
//! `jiff::ZonedRound`: four plain-setter public methods, all three
//! fields private with no getters. `jiff::civil::DateTimeWith` also
//! stays trusted, for the identical real reason as `jiff::ZonedWith`
//! — its type can't distinguish "no overrides" from "some override"
//! any more than `ZonedWith`'s can, so its one real documented law
//! can't be stated soundly as an unconditional claim, and a model of
//! just the "no override" case reduces to a bare identity function.
//! `jiff::civil::DateWith` also stays trusted, the third confirmed
//! instance of this exact pattern: its private `Option`-wrapped
//! override fields are just as invisible to a model boundary as
//! `ZonedWith`'s/`DateTimeWith`'s. `jiff::civil::TimeArithmetic` also
//! stays trusted, the identical shape to `DateArithmetic`/`jiff::
//! DateTimeArithmetic`/`TimestampArithmetic`/`jiff::ZonedArithmetic`:
//! no public methods at all, one private field, only `From` impls
//! public. `jiff::civil::TimeDifference` also stays trusted, the
//! identical builder-only shape to `DateDifference`/`jiff::
//! DateTimeDifference`/`TimestampDifference`/`jiff::ZonedDifference`:
//! five plain-setter public methods, both fields private with no
//! getters. `jiff::civil::TimeRound` also stays trusted, the
//! identical builder-only shape to `DateTimeRound`/`TimestampRound`/
//! `jiff::ZonedRound`: four plain-setter public methods, all three
//! fields private with no getters. `jiff::civil::TimeWith` also
//! stays trusted, the fourth confirmed instance of the `jiff::
//! ZonedWith`/`DateTimeWith`/`DateWith` pattern: its private
//! `Option`-wrapped override fields are just as invisible to a model
//! boundary as the earlier three's. Nothing non-tautological to
//! model about any of the twenty-six on any backend.

//!
//! `jiff::fmt::friendly::Designator` stays trusted, the same
//! "no public methods beyond standard derives" shape as `RoundMode`:
//! checked directly against jiff's real source, it's a fieldless,
//! `#[non_exhaustive]` config-marker enum with no `impl Designator`
//! block at all, only consumed by `SpanPrinter::designator`.
//!
//! `jiff::fmt::friendly::Direction` stays trusted, the identical
//! shape to `Designator`: checked directly against jiff's real
//! source, a fieldless, `#[non_exhaustive]` config-marker enum
//! deriving only `Clone`/`Copy`/`Debug`; its one `impl Direction`
//! method (`sign`) is private.
//!
//! `jiff::fmt::friendly::Spacing` stays trusted, the identical shape
//! to `Designator`/`Direction`: checked directly against jiff's real
//! source, a fieldless, `#[non_exhaustive]` config-marker enum
//! deriving only `Clone`/`Copy`/`Debug`; both `impl Spacing` methods
//! (`between_units`/`between_units_and_designators`) are private.
//!
//! `jiff::fmt::friendly::SpanParser` stays trusted, a genuinely
//! different reason from every other type in this file: real,
//! substantial documented behavior (`parse_span`/`parse_duration`/
//! `parse_unsigned_duration`), but checked directly against jiff's
//! real source, the struct carries zero configurable state
//! (`_private: ()`, and jiff's own doc comment confirms no
//! configuration options exist). Checking its exact parsing behavior
//! would mean reproducing jiff's entire grammar, the same
//! disproportionate-reproduction reason `jiff::
//! TimestampDisplayWithOffset` already established.
//!
//! `jiff::fmt::friendly::SpanPrinter` stays trusted too, combining
//! BOTH real reasons already established elsewhere in this file:
//! checked directly against jiff's real source, its nine public
//! methods are all plain setters over nine private fields — no
//! getters at all, the same builder-only shape `SpanRound`/
//! `TimestampRound`/etc. share — AND its real formatted-output
//! methods have the same disproportionate-reproduction shape
//! `SpanParser`/`TimestampDisplayWithOffset` already established.
//!
//! `jiff::fmt::rfc2822::DateTimeParser` stays trusted, the identical
//! combined shape to `SpanParser`/`SpanPrinter`: checked directly
//! against jiff's real source, its one public method
//! (`relaxed_weekday(self, bool) -> Self`) is a plain setter over its
//! one private field, with no getter — and its real parsing methods
//! (`parse_zoned`/`parse_timestamp`) have the same disproportionate-
//! reproduction shape, reproducing jiff's real RFC 2822 grammar this
//! time.
//!
//! `jiff::fmt::rfc2822::DateTimePrinter` stays trusted, the identical
//! shape to `SpanParser`: checked directly against jiff's real
//! source, its `_private: ()` field carries zero configurable state
//! (jiff's own inline comment says "the RFC 2822 printer has no
//! configuration at present") — and its real formatting methods have
//! the same disproportionate-reproduction shape.
//!
//! `jiff::tz::Disambiguation` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`/`AmbiguousOffset`:
//! there is no `impl Disambiguation` block at all — a real
//! configuration marker enum consumed only by other types' methods.
//!
//! `jiff::tz::OffsetArithmetic` also stays trusted, the identical
//! shape to `TimestampArithmetic`/`ZonedArithmetic`: no public
//! methods at all, one private field, only `From` impls public.
//!
//! `jiff::tz::OffsetRound` also stays trusted, the identical
//! builder-only shape to `TimestampRound`/`ZonedRound`: four
//! plain-setter public methods, all three fields private with no
//! getters.
//!
//! `jiff::tz::TimeZoneFollowingTransitions<'static>` stays trusted
//! here too, for consistency with the same real, confirmed
//! content-free reasoning `amenable_kani::ext::jiff`'s own doc
//! comment documents (`next()` is unconditionally `None` for this
//! type's own real behavior, no internal branching left to model
//! beyond a constant) — Kani and Creusot each hit their own separate,
//! genuinely different real toolchain walls first; see that doc
//! comment for both.
//!
//! `jiff::tz::TimeZoneName<'static>` also stays trusted, checked
//! directly against jiff's real source: its own constructor is
//! completely private — no way to build one from outside jiff at
//! all, only reachable via real IANA tzdb data.
//!
//! `jiff::tz::TimeZoneNameIter<'static>` also stays trusted, for
//! consistency with the same content-free reasoning `amenable_kani::
//! ext::jiff`'s own doc comment documents — checked for real on Kani
//! instead, trusted on Creusot for a real, confirmed toolchain
//! reason (see `amenable_creusot::ext::jiff`'s own doc comment).
//!
//! `jiff::tz::TimeZoneTransition<'static>` also stays trusted, the
//! LAST type in this checklist: no reachable constructor from outside
//! jiff at all (see `amenable_kani::ext::jiff`'s own doc comment for
//! the full confirmation).
//!
//! `jiff::fmt::strtime::Config<DefaultCustom>` stays trusted, checked
//! directly against jiff's real source: a pure builder, both fields
//! private with no getters, its two public methods plain setters —
//! the same shape as `SpanRound`/`TimestampRound`/etc.
//!
//! `jiff::fmt::strtime::DefaultCustom` stays trusted, even more
//! opaque than `RoundMode`: checked directly against jiff's real
//! source, it's `DefaultCustom(())` — a single private zero-sized
//! field — with one constructor and an empty `impl Custom for
//! DefaultCustom {}` using only the trait's own inherited defaults.
//!
//! `jiff::fmt::strtime::Display<'static>` stays trusted, the same
//! reason as `TimestampDisplayWithOffset`: checked directly against
//! jiff's real source, both fields are `pub(crate)` with only
//! `impl core::fmt::Display`/`Debug` — checking its exact formatted
//! output would mean reproducing jiff's whole `strtime` formatting
//! algorithm.
//!
//! `jiff::fmt::strtime::Extension` stays trusted, checked directly
//! against jiff's real source: three private fields, zero `pub`
//! methods at all, only `#[derive(Clone, Debug)]` — jiff's own doc
//! comment confirms this is deliberate: "if you have use cases for
//! introspecting this type, please open an issue."
//!
//! `jiff::fmt::strtime::PosixCustom` stays trusted too: checked
//! directly against jiff's real source, `PosixCustom(())` mirrors
//! `DefaultCustom`'s zero-field shape — but unlike `DefaultCustom`,
//! its `impl Custom for PosixCustom` DOES override four format
//! methods, each just delegating to `BrokenDownTime::
//! format_with_config` with a fixed POSIX format string. The same
//! disproportionate-reproduction reason `SpanParser`/`Display`
//! already established, one level removed.
//!
//! `jiff::fmt::temporal::DateTimeParser` stays trusted, the identical
//! combined shape to `SpanParser`/`rfc2822::DateTimeParser`: checked
//! directly against jiff's real source, its two public methods
//! (`offset_conflict`/`disambiguation`) are plain setters over three
//! private fields, no getters — and its real parsing methods have the
//! same disproportionate-reproduction shape, reproducing jiff's real
//! ISO 8601/RFC 9557 temporal grammar this time.
//!
//! `jiff::fmt::temporal::DateTimePrinter` stays trusted, the identical
//! combined shape to `SpanPrinter`/`rfc2822::DateTimePrinter`: checked
//! directly against jiff's real source, its three public methods
//! (`lowercase`/`separator`/`precision`) are plain setters over one
//! private field, no getters — and its real formatting methods have
//! the same disproportionate-reproduction shape.
//!
//! `jiff::fmt::temporal::SpanParser` stays trusted, checked directly
//! against jiff's real source: a DIFFERENT type from `fmt::friendly::
//! SpanParser` despite the same name (verified independently, not
//! assumed) — one private field, zero setters, and its real parsing
//! methods have the same disproportionate-reproduction shape,
//! reproducing jiff's real ISO 8601 duration grammar.
//!
//! `jiff::fmt::temporal::SpanPrinter` stays trusted, the identical
//! combined shape to every other `*Printer` in this checklist:
//! checked directly against jiff's real source, its one public method
//! (`lowercase`) is a plain setter over a private field, no getter —
//! and its real formatting methods have the same
//! disproportionate-reproduction shape.
//!
//! `jiff::tz::AmbiguousOffset` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`: checked directly
//! against jiff's real source, it has *no* public methods at all —
//! only a `pub(crate) from_jcore` conversion. Nothing
//! non-tautological to state about it on any backend.

use crate::ExtProvenance;
use crate::ExtStandard;
use crate::ext_verus_witness::impl_verus_witness_trusted_ext;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_trusted_ext!(
    jiff::Timestamp,
    jiff::Zoned,
    jiff::civil::DateTime,
    jiff::RoundMode,
    jiff::SignedDurationRound,
    jiff::SpanArithmetic<'static>,
    jiff::SpanCompare<'static>,
    jiff::SpanRelativeTo<'static>,
    jiff::SpanRound<'static>,
    jiff::SpanTotal<'static>,
    jiff::TimestampArithmetic,
    jiff::TimestampDifference,
    jiff::TimestampDisplayWithOffset,
    jiff::TimestampRound,
    jiff::ZonedArithmetic,
    jiff::ZonedDifference<'static>,
    jiff::ZonedRound,
    jiff::ZonedWith,
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
    jiff::fmt::DefmtWrite<'static>,
    jiff::fmt::friendly::Designator,
    jiff::fmt::friendly::Direction,
    jiff::fmt::friendly::Spacing,
    jiff::fmt::friendly::SpanParser,
    jiff::fmt::friendly::SpanPrinter,
    jiff::fmt::rfc2822::DateTimeParser,
    jiff::fmt::rfc2822::DateTimePrinter,
    jiff::fmt::strtime::Config<jiff::fmt::strtime::DefaultCustom>,
    jiff::fmt::strtime::DefaultCustom,
    jiff::fmt::strtime::Display<'static>,
    jiff::fmt::strtime::Extension,
    jiff::fmt::strtime::PosixCustom,
    jiff::fmt::temporal::DateTimeParser,
    jiff::fmt::temporal::DateTimePrinter,
    jiff::fmt::temporal::SpanParser,
    jiff::fmt::temporal::SpanPrinter,
    jiff::tz::AmbiguousOffset,
    jiff::tz::Disambiguation,
    jiff::tz::OffsetArithmetic,
    jiff::tz::OffsetRound,
    jiff::tz::TimeZoneFollowingTransitions<'static>,
    jiff::tz::TimeZoneName<'static>,
    jiff::tz::TimeZoneNameIter<'static>,
    jiff::tz::TimeZonePrecedingTransitions<'static>,
    jiff::tz::TimeZoneTransition<'static>
);
