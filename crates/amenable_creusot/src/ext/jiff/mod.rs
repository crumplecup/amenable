//! `CreusotWitness` impls for `amenable_ext::ExtStandard<T>` over jiff's
//! registered carriers, split one file per real jiff area that has
//! earned a checked contract, plus the trusted carriers that haven't
//! (below).
//!
//! `Timestamp`/`Zoned`/`civil::DateTime` stay trusted here: they're
//! opaque, high-level composite types with no simple property statable
//! without a much larger `extern_spec!` surface than jiff has earned
//! yet — see `docs/AMENABLE_EXT_PLAN.md`'s Phase 1. Simpler value types
//! get a real per-type assessment as they're added (see `offset.rs` for
//! the first checked example, mirroring `amenable_kani::ext::jiff::offset`).
//!
//! `bridge_creusot_witness!`/`ExtCheckedProof` are shared between this
//! file and its `jiff::` siblings only — not promoted to a crate-wide
//! module, matching `rust_std_witness`'s own convention of never sharing
//! this bridge across unrelated files.
//!
//! `jiff::RoundMode`/`jiff::SignedDurationRound`/`jiff::
//! SpanArithmetic<'static>`/`jiff::SpanCompare<'static>`/`jiff::
//! SpanRelativeTo<'static>`/`jiff::SpanRound<'static>`/`jiff::
//! SpanTotal<'static>`/`jiff::TimestampArithmetic`/`jiff::
//! TimestampDifference`/`jiff::TimestampRound` also stay trusted, for
//! the same real reasons named in `amenable_kani::ext::jiff`'s own
//! doc comment: `RoundMode` has no public methods at all beyond the
//! standard derives; the other nine are pure builders/markers (no
//! getters, their real logic is private). `jiff::
//! TimestampDisplayWithOffset` also stays trusted, but for a
//! different real reason: its only public behavior is a `Display`
//! impl, and checking its exact RFC 3339 output would mean
//! reproducing jiff's whole formatting algorithm. Nothing
//! non-tautological to state about any of the eleven on any backend.
//!
//! `jiff::TimestampSeries` gets a real checked property (see
//! `timestamp_series.rs`): an accommodation model (not a real
//! `extern_spec!` against jiff's actual `Iterator` impl, matching
//! this crate's own established precedent for iterator types lacking
//! real contract coverage), checking the same periodicity law
//! `amenable_kani::ext::jiff` documents as unavoidably too costly for
//! Kani specifically (a real `jiff::Error` Drop-glue wall inside
//! `Timestamp::series`'s own private implementation).
//!
//! `jiff::Unit` also stays trusted here, but for a real reason
//! specific to Creusot, checked (not assumed): an `extern_spec!` for
//! `Ord::cmp` on `jiff::Unit` compiles fine as a bare declaration, but
//! actually calling it inside a harness fails with "the trait bound
//! `jiff::Unit: creusot_std::model::DeepModel` is not satisfied" —
//! the same class of constraint this crate's own `rust_std::
//! cmp_carriers` already documents for `Reverse<T>: OrdLogic` (a
//! foreign type's comparison machinery needs a model Creusot can't
//! derive for third-party types), reached directly on a concrete enum
//! this time rather than through a generic wrapper's blanket impl.
//! Checked for real on Kani instead (`amenable_kani::ext::jiff::unit`,
//! exhaustive enumeration of all 100 ordered pairs among the ten real
//! variants).
//!
//! `jiff::ZonedArithmetic` also stays trusted, the identical shape to
//! `TimestampArithmetic`: it has no public methods of its own at all,
//! and its one field is private with no getter.
//!
//! `jiff::ZonedDifference<'static>` also stays trusted, the identical
//! builder-only shape to `TimestampDifference`: its five public
//! methods are all plain setters, and both fields are private with no
//! getters.
//!
//! `jiff::ZonedRound` also stays trusted, the identical builder-only
//! shape to `TimestampRound`: its four public methods are all plain
//! setters, and its one field is private with no getter.
//!
//! `jiff::ZonedSeries` gets a real checked property too (see
//! `zoned_series.rs`), an accommodation model scoped to `TimeZone::
//! UTC` (see `ext_jiff::zoned_series`'s own doc comment for the real,
//! confirmed reason the scoping is honest, not a shortcut). Checked
//! here despite being trusted for Kani specifically, for a real reason
//! genuinely different from `TimestampSeries`'s: not `jiff::Error`
//! Drop glue, but `TimeZone`'s own hand-rolled pointer-tagged `Repr`,
//! confirmed via `amenable_kani::gallery::jiff_error_drop_cost` to time
//! out CBMC even for a single, fully concrete `TimeZone::UTC.
//! to_offset(..)` call.
//!
//! `jiff::ZonedWith` also stays trusted, but not for a Kani-shaped
//! reason this time — checked directly, not assumed, that a real
//! `extern_spec!` here would be UNSOUND if scoped narrowly to jiff's
//! own documented "no fields set ⟹ returns the original unchanged"
//! law: `ZonedWith`'s type alone can't distinguish "fresh from
//! `.with()`" from "modified by a setter" (both the same type, private
//! fields), so an `#[ensures(..)]` on `build()` stating that law
//! unconditionally would be FALSE for the general case (setting
//! `.date(..)` deliberately changes the result) — correctly scoping it
//! would mean extern-speccing every setter against a tracked ghost
//! override-state, plus `civil::DateTimeWith`'s own calendar-field
//! setters underneath, disproportionate to one type and reopening
//! `Zoned`/`civil::DateTime`'s deliberately-opaque surface. An
//! accommodation model of just the "no override" case would reduce to
//! a bare identity function with no distinguishing computation at all
//! — genuinely tautological, the case this codebase's own
//! tautological-model policy says to accept trusted for rather than
//! build a thin model for its own sake.
//!
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
//!
//! `jiff::fmt::StdFmtWrite<String>` gets a real checked property too
//! (see `fmt_std_fmt_write.rs`): a real `extern_spec!` for jiff's own
//! `fmt::Write::write_str`, checking a genuinely non-tautological
//! infallibility claim — narrower than `amenable_kani::ext::jiff::
//! fmt_std_fmt_write`'s own stronger round-trip claim, for a real
//! design reason (no honest way found to connect an opaque `&str`
//! content accessor to the harness's concrete single-character
//! string without a string-content model this crate has no local
//! precedent for), documented in that module's own doc comment.
//!
//! `jiff::fmt::StdIoWrite<Vec<u8>>` gets a real checked property too
//! (see `fmt_std_io_write.rs`): the identical generics-matching wall
//! as `StdFmtWrite<String>`'s own, but `Vec<u8>`'s own `std::io::
//! Write` impl is ALSO generic (`impl<A: Allocator> Write for
//! Vec<u8, A>`, confirmed against std's real source), so the fix here
//! extern-specs that generic impl directly (matching `creusot-std`'s
//! own established pattern for `Vec<T, A>::push`/`len`), rather than
//! `StdFmtWrite<String>`'s non-generic-impl workaround.
//!
//! `jiff::fmt::friendly::Designator` stays trusted, checked directly
//! against jiff's real source (`src/fmt/friendly/printer.rs`): a
//! fieldless, `#[non_exhaustive]` config-marker enum deriving only
//! `Clone`/`Copy`/`Debug`, no `impl Designator` block at all — only
//! consumed by `SpanPrinter::designator`. The same "no public methods
//! beyond standard derives" shape as `RoundMode`, not the "fieldless
//! enum" shortcut `Era`/`Weekday` already showed is the wrong lesson.
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
//! `jiff::fmt::strtime::BrokenDownTime` gets a real checked property
//! too (see `fmt_strtime_broken_down_time.rs`), the first `&mut self`
//! setter extern-spec'd anywhere in this checklist: 12 numeric
//! setter/getter pairs, each via an opaque per-field accessor plus
//! Creusot's `^self` ("final") prophecy operator, the same mechanism
//! `creusot-std`'s own `Vec::push` extern_spec uses for its `View`.
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
//! `jiff::fmt::strtime::Meridiem` gets a real checked property too
//! (see `fmt_strtime_meridiem.rs`): a real `extern_spec!` for jiff's
//! own `From<civil::Time> for Meridiem`, reusing `civil_time.rs`'s
//! existing `civil_time_hour_value` accessor directly. `Meridiem` is
//! NOT `#[non_exhaustive]`, so matching its two variants needs no
//! wildcard arm — the same pure pattern-matching technique
//! `fmt_friendly_fractional_unit.rs` already established.
//!
//! `jiff::fmt::friendly::FractionalUnit` gets a real checked property
//! too (see `fmt_friendly_fractional_unit.rs`): a real `extern_spec!`
//! for jiff's own `From<FractionalUnit> for Unit`, checking the same
//! documented per-variant mapping `amenable_kani::ext::jiff::
//! fmt_friendly_fractional_unit` checks by exhaustive enumeration —
//! both the `extern_spec!` ensures and the harness body need a
//! wildcard arm (`FractionalUnit` is `#[non_exhaustive]`).
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
//! `jiff::fmt::temporal::Pieces<'static>` gets a real checked property
//! too (see `fmt_temporal_pieces.rs`), the first real data-carrying
//! `fmt::*` type in this checklist: real `extern_spec!`s for
//! `date`/`time`/`with_date`/`with_time` using Pieces-specific
//! DECOMPOSED opaque accessors (`i16`/`i8`/`Option<i8>`/etc.) rather
//! than the `Date`/`Time` type directly, since comparing two `Date`/
//! `Time` values via `==` hits the same `DeepModel` wall `unit.rs`
//! already documents — reuses `civil_date.rs`'s/`civil_time.rs`'s
//! existing accessors to decompose both sides.
//!
//! `jiff::fmt::temporal::PiecesNumericOffset` gets a real checked
//! property too (see `fmt_temporal_pieces_numeric_offset.rs`): a real
//! data-carrying type wrapping a real `jiff::tz::Offset`, reusing
//! `offset.rs`'s own `offset_seconds_value` opaque accessor (hoisted
//! to `pub(crate)` at that module's top level for this purpose)
//! rather than a disconnected copy.
//!
//! `jiff::fmt::temporal::PiecesOffset` gets a real checked property
//! too (see `fmt_temporal_pieces_offset.rs`): a real, `#[non_
//! exhaustive]` two-variant enum, matched directly on its own
//! variants inside the `extern_spec!`'s `#[ensures(..)]` clause —
//! the same `fmt_friendly_fractional_unit.rs`-established technique
//! for sidestepping a foreign enum's `DeepModel` wall — reusing
//! `fmt_temporal_pieces_numeric_offset.rs`'s own
//! `pno_offset_seconds_value` opaque accessor.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotation<'static>` gets a real
//! checked property too (see `fmt_temporal_time_zone_annotation.rs`),
//! at a NARROWER scope than its own Kani harness: checks which
//! variant `kind()` reports (and, for `Offset`, its numeric payload)
//! with `is_critical() == false`, but not the exact `Named` string
//! content — no existing file in this crate compares `&str` content
//! inside a Pearlite `#[ensures(..)]` clause, and there's no sibling
//! accessor to decompose through yet (`TimeZoneAnnotationKind`/
//! `TimeZoneAnnotationName` are later, not-yet-assessed checklist
//! entries).
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationKind<'static>` gets a real
//! checked property too (see
//! `fmt_temporal_time_zone_annotation_kind.rs`): unlike
//! `TimeZoneAnnotation<'static>`, this `#[non_exhaustive]` enum's own
//! variants are the public API — `result` in each constructor's own
//! `extern_spec!` ensures IS the value being constructed, so this
//! matches directly on it, needing NO opaque accessor at all for the
//! discriminant. Reuses `offset.rs`'s own `offset_seconds_value` for
//! the `Offset` variant's payload.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationName<'static>` gets a real
//! checked property too (see
//! `fmt_temporal_time_zone_annotation_name.rs`), at the FULL scope
//! this time (no enum-matching escape hatch for this struct): `&str`/
//! `str` themselves ARE std/core types with real `creusot-std`
//! coverage (unlike jiff's own foreign types), so `==` between two
//! `&str` values inside a Pearlite `#[ensures(..)]` clause works —
//! confirmed by this being the first file in this crate to try it.
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
//!
//! `jiff::tz::AmbiguousTimestamp` gets a real checked property too
//! (see `tz_ambiguous_timestamp.rs`), checked here despite being
//! trusted for Kani specifically — see `amenable_kani::ext::jiff`'s
//! own doc comment for the real, confirmed `TimeZone` `Repr`-dispatch
//! CBMC wall. Deliberately NARROWER than a full claim would be:
//! `civil::DateTime` stays opaque (trusted, no decomposition
//! accessors) by deliberate Phase 1 design, so `dt` passes through
//! this proof entirely unexamined; only `.offset()`/`.is_ambiguous()`
//! are checked, decomposed via fresh opaque accessors tied to a
//! `TimeZone::fixed`-specific one (`tz_fixed_seconds_value`, hoisted
//! `pub(crate)` for `TimeZone`'s own future checklist entry to reuse).
//!
//! `jiff::tz::AmbiguousZoned` gets a real checked property too (see
//! `tz_ambiguous_zoned.rs`), the same shape and scope as
//! `AmbiguousTimestamp`: reuses that file's own `tz_fixed_seconds_
//! value` accessor. Trusted for Kani too, without a redundant fresh
//! confirmation run — real jiff source confirms `to_ambiguous_zoned`
//! calls `to_ambiguous_timestamp` directly, the SAME real function
//! already empirically confirmed to time out under CBMC.
//!
//! `jiff::tz::Disambiguation` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`/`AmbiguousOffset`:
//! there is no `impl Disambiguation` block at all — a real,
//! `#[non_exhaustive]` four-variant configuration marker enum
//! consumed only by OTHER types' methods, never producing or
//! inspecting anything of its own beyond the standard derives.
//!
//! `jiff::tz::Dst` gets a real checked property too (see
//! `tz_dst.rs`): a real, plain (NOT `#[non_exhaustive]`) two-variant
//! enum — matches exhaustively on `self`/`result` directly, needing
//! no opaque accessor and no wildcard arm.
//!
//! `jiff::tz::OffsetArithmetic` also stays trusted, the identical
//! shape to `TimestampArithmetic`/`ZonedArithmetic`: it has no public
//! methods of its own at all, and its one field is private with no
//! getter.
//!
//! `jiff::tz::OffsetConflict` gets a real checked property too (see
//! `tz_offset_conflict.rs`), checked despite being trusted for Kani
//! specifically (real jiff source shows `resolve_with`'s
//! `AlwaysTimeZone` branch calls `TimeZone::into_ambiguous_zoned`
//! directly, the same already-confirmed-timing-out function).
//! Scoped to `AlwaysOffset`/`AlwaysTimeZone` only — `PreferOffset`/
//! `Reject` delegate to private helpers with their own separately
//! nontrivial logic, disproportionate to add here.
//!
//! `jiff::tz::OffsetRound` also stays trusted, the identical
//! builder-only shape to `TimestampRound`/`ZonedRound`: its four
//! public methods are all plain setters, and all three fields are
//! private with no getters.
//!
//! `jiff::tz::TimeZone` gets a real checked property too (see
//! `tz_time_zone.rs`), checked despite being trusted for Kani
//! specifically (the type underlying every `Repr`-dispatch CBMC wall
//! already confirmed this session): `unknown().is_unknown()` is
//! always `true`, `fixed(offset)` is never unknown, and its own
//! `to_fixed_offset()` always round-trips `offset`'s own seconds.
//! `fixed`/`to_ambiguous_timestamp`/`to_ambiguous_zoned`/
//! `into_ambiguous_zoned` are already extern-spec'd in `tz_ambiguous_
//! timestamp.rs`/`tz_ambiguous_zoned.rs`; this file adds `unknown`/
//! `is_unknown`/`to_fixed_offset`, extending `fixed`'s own ensures
//! clause with an extra conjunct there rather than redeclaring it.
//!
//! `jiff::tz::TimeZoneDatabase` gets a real checked property too
//! (see `tz_time_zone_database.rs`): `none()`/`is_definitively_
//! empty()` are the checked subset — see `amenable_kani::ext::jiff::
//! tz_time_zone_database`'s own doc comment for the real reason
//! `get()`/`bundled()`/etc. are out of scope.
//!
//! `jiff::tz::TimeZoneFollowingTransitions<'static>` stays trusted
//! here too, for a genuinely different, confirmed reason: a first
//! attempt to `extern_spec!` its `Iterator::next` directly hit TWO
//! real toolchain walls in succession — (1) a real E0716 "temporary
//! value dropped while borrowed" for `TimeZone::UTC.following(..)`
//! (fixed via `Box::leak` for a genuine `&'static TimeZone`, itself
//! blocked by a second, real "unsupported constant value" error when
//! tried via a local `static` instead), then (2) a real "the trait
//! bound `TimeZoneFollowingTransitions<'_>: creusot_std::prelude::
//! IteratorSpec` is not satisfied" error — `extern_spec!` doesn't
//! support third-party `Iterator` trait impls at all. The fallback
//! accommodation-model pattern `timestamp_series.rs` established for
//! iterator types (compute the expected value directly, never
//! calling the real `next()`) would be genuinely content-free here
//! (`next()` is unconditionally `None`, no formula to check
//! consistency of) — the case this codebase's own tautological-model
//! policy says to accept trusted for rather than build a thin model
//! for its own sake.
//!
//! `jiff::tz::TimeZoneName<'static>` also stays trusted, checked
//! directly against jiff's real source: its own constructor (`fn
//! new`) is completely private — no way to build one from outside
//! jiff at all, only reachable via real IANA tzdb data through
//! `TimeZoneNameIter`.
//!
//! `jiff::tz::TimeZoneNameIter<'static>` also stays trusted here, for
//! a DIFFERENT real reason from `TimeZoneName`: checked on Kani
//! instead (`amenable_kani::ext::jiff::tz_time_zone_name_iter`, real
//! and passing — `TimeZoneDatabase::none().available().next() ==
//! None` is genuinely cheap/dispatch-free, no `TimeZone::Repr`
//! involved) — trusted here only because `extern_spec!` doesn't
//! support third-party `Iterator` trait impls at all (the same real
//! "`IteratorSpec` is not satisfied" error already confirmed for
//! `TimeZoneFollowingTransitions`), and the fallback accommodation-
//! model pattern would be genuinely content-free for this
//! unconditional-constant claim.

mod civil_date;
mod civil_era;
mod civil_iso_week_date;
mod civil_time;
mod civil_time_series;
mod civil_weekday;
mod civil_weekdays_forward;
mod civil_weekdays_reverse;
mod date_series;
mod date_time_series;
mod error;
mod fmt_friendly_fractional_unit;
mod fmt_std_fmt_write;
mod fmt_std_io_write;
mod fmt_strtime_broken_down_time;
mod fmt_strtime_meridiem;
mod fmt_temporal_pieces;
mod fmt_temporal_pieces_numeric_offset;
mod fmt_temporal_pieces_offset;
mod fmt_temporal_time_zone_annotation;
mod fmt_temporal_time_zone_annotation_kind;
mod fmt_temporal_time_zone_annotation_name;
mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;
mod timestamp_series;
mod tz_ambiguous_timestamp;
mod tz_ambiguous_zoned;
mod tz_dst;
mod tz_offset_conflict;
mod tz_time_zone;
mod tz_time_zone_database;
mod zoned_series;

use crate::CreusotWitness;
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl amenable_core::Witness<crate::CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as crate::CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as crate::CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as crate::CreusotWitness>::proof()
            }
        }
    };
}
pub(super) use bridge_creusot_witness;

macro_rules! impl_creusot_witness_trusted_ext {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CreusotWitness for ExtStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = ExtProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_creusot_witness!(ExtStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_ext::ExtStandard<", stringify!($ty), ">"),
                    "creusot",
                    || <ExtStandard<$ty> as CreusotWitness>::proof().report().to_string(),
                )
            }
        )*
    };
}

impl_creusot_witness_trusted_ext!(
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
    jiff::Unit,
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
    jiff::tz::TimeZoneNameIter<'static>
);

/// Proof artifact for an `ExtStandard<T>` carrier with a real,
/// machine-checked Creusot contract — the third-party-crate counterpart
/// of `rust_std_witness::CheckedProof`, holding an `ExtProvenance`
/// instead of a `RustStdProvenance`.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct ExtCheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    harness: String,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    claim: String,
    /// The chain-derived provenance this claim still rests on.
    provenance: ExtProvenance,
}

impl std::fmt::Display for ExtCheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}
