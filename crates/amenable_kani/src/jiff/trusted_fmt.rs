//! Trusted `jiff::fmt::*` types for Kani -- formatter/parser builders
//! and configuration markers with no inspectable state or output not
//! already covered by a real checked sibling.

//! `jiff::fmt::StdFmtWrite<String>` gets a real checked property too
//! (see `fmt_std_fmt_write.rs`): a real newtype wrapper adapting any
//! `core::fmt::Write` value to jiff's own public `jiff::fmt::Write`
//! trait — checked directly against jiff's real source. Scoped to
//! `W = String`, whose `core::fmt::Write` implementation never fails,
//! so the `Err` arm (the only path that would construct a real
//! `jiff::Error`) is never reached, avoiding the recursive-Arc
//! Drop-glue wall this module's own opening paragraphs document
//! rather than needing to work around it.
//!
//! `jiff::fmt::StdIoWrite<Vec<u8>>` gets a real checked property too
//! (see `fmt_std_io_write.rs`), the identical shape to `StdFmtWrite<
//! String>`'s own: a real newtype wrapper adapting any `std::io::
//! Write` value to jiff's `jiff::fmt::Write` trait, scoped to
//! `W = Vec<u8>`, whose `write_all` is documented to never fail (it
//! just extends the vector), so the `Err` arm is never reached.
//!
//! `jiff::fmt::DefmtWrite<'static>` stays trusted, and this one is a
//! must-trust case rather than a choice. Its `write_str` is visibly
//! `Ok` in source (`defmt::write!` then `Ok(())`), but the wrapped
//! `defmt::Formatter` has only a `pub(crate)` field and no public
//! constructor, so no proof can obtain a value to call it on. The
//! write path belongs to defmt's runtime, and that is what must be
//! trusted.
//!
//! `jiff::fmt::friendly::Designator` stays trusted, the first
//! `jiff::fmt::*` type to land here rather than get a checked
//! property: checked directly against jiff's real source
//! (`src/fmt/friendly/printer.rs`), it's a fieldless, `#[non_exhaustive]`
//! config-marker enum (`Verbose`/`Short`/`Compact`/`HumanTime`)
//! deriving only `Clone`/`Copy`/`Debug` — no `impl Designator` block
//! at all, only consumed by `SpanPrinter::designator(self, Designator)
//! -> SpanPrinter`. The identical "no public methods beyond the
//! standard derives" shape `RoundMode`'s own trusted reasoning rests
//! on, not the "fieldless enum" shortcut `Era`/`Weekday` already
//! showed is the wrong lesson to draw.
//!
//! `jiff::fmt::friendly::Direction` stays trusted, the identical
//! shape to `Designator`: checked directly against jiff's real
//! source, it's a fieldless, `#[non_exhaustive]` config-marker enum
//! (`Auto`/`Sign`/`ForceSign`/`Suffix`) deriving only `Clone`/`Copy`/
//! `Debug`; its one `impl Direction` method (`sign`) is private.
//!
//! `jiff::fmt::friendly::Spacing` stays trusted, the identical shape
//! to `Designator`/`Direction`: checked directly against jiff's real
//! source, it's a fieldless, `#[non_exhaustive]` config-marker enum
//! (`None`/`BetweenUnits`/`BetweenUnitsAndDesignators`) deriving only
//! `Clone`/`Copy`/`Debug`; both `impl Spacing` methods
//! (`between_units`/`between_units_and_designators`) are private.
//!
//! `jiff::fmt::friendly::SpanParser` stays trusted, a genuinely
//! different reason from every other type in this file: unlike the
//! builder/config-marker shapes above, it has real, substantial
//! documented behavior (`parse_span`/`parse_duration`/
//! `parse_unsigned_duration`, jiff's real friendly-duration parsing
//! grammar) — but checked directly against jiff's real source, the
//! struct itself carries zero configurable state (`_private: ()`,
//! and jiff's own doc comment says outright "there are no available
//! configuration options for this parser"). Checking its exact
//! parsing behavior would mean reproducing jiff's entire grammar,
//! the same disproportionate-reproduction reason `jiff::
//! TimestampDisplayWithOffset` already established for formatting
//! output.
//!
//! `jiff::fmt::friendly::SpanPrinter` stays trusted too, combining
//! BOTH real reasons already established elsewhere in this file:
//! checked directly against jiff's real source, its nine public
//! methods (`designator`/`spacing`/`direction`/`fractional`/
//! `comma_after_designator`/`hours_minutes_seconds`/`padding`/
//! `precision`/`zero_unit`) are all plain setters over its nine
//! private fields — no getters at all, the same builder-only shape
//! `SpanRound`/`TimestampRound`/etc. share — AND its real formatted-
//! output methods (`span_to_string`/`print_span`/etc.) have the same
//! disproportionate-reproduction shape `SpanParser`/
//! `TimestampDisplayWithOffset` already established.
//!
//! `jiff::fmt::rfc2822::DateTimeParser` stays trusted, the identical
//! combined shape to `SpanParser`/`SpanPrinter`: checked directly
//! against jiff's real source (`src/fmt/rfc2822.rs`), its one public
//! method (`relaxed_weekday(self, bool) -> Self`) is a plain setter
//! over its one private field, with no getter — and its real parsing
//! methods (`parse_zoned`/`parse_timestamp`) have the same
//! disproportionate-reproduction shape `SpanParser` already
//! established, reproducing jiff's real RFC 2822 grammar this time
//! rather than the friendly-duration one.
//!
//! `jiff::fmt::rfc2822::DateTimePrinter` stays trusted, the identical
//! shape to `SpanParser`: checked directly against jiff's real
//! source, its `_private: ()` field carries zero configurable state
//! (jiff's own inline comment says "the RFC 2822 printer has no
//! configuration at present") — and its real formatting methods
//! (`zoned_to_string`/`timestamp_to_string`/
//! `timestamp_to_rfc9110_string`/etc.) have the same disproportionate-
//! reproduction shape.
//!
//! `jiff::fmt::strtime::BrokenDownTime` gets a real checked property
//! too (see `fmt_strtime_broken_down_time.rs`), the biggest single
//! type assessed since `Span` itself: 12 numeric fields, each with a
//! real, independently-validated `Result`-returning setter/getter
//! pair, checked directly against jiff's real source and `jiff-core`'s
//! own bounds table (not assumed) — no cross-field interdependency at
//! all, confirmed by reading `set_day`'s own doc comment: "setting a
//! day to a value that is legal in any context is always valid, even
//! if it isn't valid for the year, month." Deliberately scoped to
//! these 12 numeric round trips, not the 5 reference-type fields
//! (`offset`/`weekday`/`meridiem`/`timestamp`/`iana_time_zone`),
//! whose setters are plain unconditional field assignments with no
//! validation logic at all.
//!
//! `jiff::fmt::strtime::Config<DefaultCustom>` stays trusted, checked
//! directly against jiff's real source: a pure builder — both fields
//! (`custom`/`lenient`) are private with no getters at all, and its
//! two public methods (`custom<U: Custom>`/`lenient(bool)`) are plain
//! setters, the same shape as `SpanRound`/`TimestampRound`/etc.
//!
//! `jiff::fmt::strtime::DefaultCustom` stays trusted, even more
//! opaque than `RoundMode`: checked directly against jiff's real
//! source, it's `DefaultCustom(())` — a single private zero-sized
//! field — with one constructor (`new()`) and an empty `impl Custom
//! for DefaultCustom {}` using only the trait's own inherited
//! defaults, no overrides at all. Nothing publicly inspectable to
//! state a property about.
//!
//! `jiff::fmt::strtime::Display<'static>` stays trusted, the same
//! reason as `TimestampDisplayWithOffset`: checked directly against
//! jiff's real source, both fields (`fmt`/`tm`) are `pub(crate)`
//! (invisible outside jiff), with only `impl core::fmt::Display`/
//! `Debug` — checking its exact formatted output would mean
//! reproducing jiff's whole `strtime` formatting algorithm, the same
//! disproportionate-reproduction reason already established.
//!
//! `jiff::fmt::strtime::Extension` stays trusted, checked directly
//! against jiff's real source: three private fields, zero `pub`
//! methods at all (`parse_flag`/`parse_width`/`parse_colons` are all
//! private), only `#[derive(Clone, Debug)]` — jiff's own doc comment
//! confirms this is deliberate: "if you have use cases for
//! introspecting this type, please open an issue."
//!
//! `jiff::fmt::strtime::PosixCustom` stays trusted too: checked
//! directly against jiff's real source, `PosixCustom(())` mirrors
//! `DefaultCustom`'s own zero-field shape (its own value type has no
//! methods beyond `new()`) — but unlike `DefaultCustom`, its `impl
//! Custom for PosixCustom` DOES override `format_datetime`/
//! `format_date`/`format_time`/`format_12hour_time`, each just
//! delegating to `BrokenDownTime::format_with_config` with a fixed
//! POSIX format string. Checking that exactly would mean reproducing
//! the same `strtime` formatting engine `SpanParser`/`Display`
//! already established as disproportionate, one level removed.
//!
//! `jiff::fmt::strtime::Meridiem` gets a real checked property too
//! (see `fmt_strtime_meridiem.rs`), the "small enums aren't
//! automatically trusted" lesson `civil::Era`/`civil::Weekday`
//! already established recurring here: checked directly against
//! jiff's real source, it has one real public conversion, `impl
//! From<civil::Time> for Meridiem`, a documented threshold (`AM` for
//! `hour < 12`, `PM` otherwise).
//!
//! `jiff::fmt::friendly::FractionalUnit` gets a real checked property
//! too (see `fmt_friendly_fractional_unit.rs`), genuinely different
//! from `Designator`/`Direction`: checked directly against jiff's
//! real source, it has no `impl FractionalUnit` block at all, but it
//! DOES have one real public conversion, `impl From<FractionalUnit>
//! for Unit`, a documented per-variant mapping — checked by exhaustive
//! enumeration of all 5 variants.
//!
//! `jiff::fmt::temporal::DateTimeParser` stays trusted, the identical
//! combined shape to `SpanParser`/`rfc2822::DateTimeParser`: checked
//! directly against jiff's real source, its two public methods
//! (`offset_conflict`/`disambiguation`) are plain setters over three
//! private fields, no getters — and its real parsing methods
//! (`parse_zoned`/`parse_timestamp`/`parse_datetime`/etc.) have the
//! same disproportionate-reproduction shape, reproducing jiff's real
//! ISO 8601/RFC 9557 temporal grammar this time.
//!
//! `jiff::fmt::temporal::DateTimePrinter` stays trusted, the identical
//! combined shape to `SpanPrinter`/`rfc2822::DateTimePrinter`: checked
//! directly against jiff's real source, its three public methods
//! (`lowercase`/`separator`/`precision`) are plain setters over one
//! private field, no getters — and its real formatting methods
//! (`zoned_to_string`/`timestamp_to_string`/`print_zoned`/etc.) have
//! the same disproportionate-reproduction shape.
//!
//! `jiff::fmt::temporal::Pieces<'static>` gets a real checked
//! property too (see `fmt_temporal_pieces.rs`), genuinely different
//! from every other `fmt::*` type assessed so far: a real,
//! substantial data-carrying type (four private fields), checked
//! directly against jiff's real source — `with_date`/`with_time` are
//! real, unconditional setters (`Pieces { date, ..self }`/`Pieces {
//! time: Some(time), ..self }`) round-tripping through their matching
//! `date()`/`time()` getters. Scoped to those two fields
//! deliberately, not `offset`/`time_zone_annotation` — their own
//! types are separate, not-yet-assessed checklist entries.
//!
//! `jiff::fmt::temporal::PiecesNumericOffset` gets a real checked
//! property too (see `fmt_temporal_pieces_numeric_offset.rs`): a real
//! data-carrying type (`offset: Offset`, `is_negative: bool`), not a
//! parser/printer — `From<Offset>` round-trips the wrapped offset's
//! seconds and sets `is_negative` to whether it's negative, and
//! `with_negative_zero` preserves the wrapped offset while forcing
//! `is_negative` to `true`.
//!
//! `jiff::fmt::temporal::PiecesOffset` gets a real checked property
//! too (see `fmt_temporal_pieces_offset.rs`): a real, `#[non_
//! exhaustive]` two-variant enum (`Zulu`, `Numeric(
//! PiecesNumericOffset)`) — `Zulu.to_numeric_offset()` is always
//! `Offset::UTC`, and `From<Offset>` always builds `Numeric`,
//! round-tripping through `to_numeric_offset()`.
//!
//! `jiff::fmt::temporal::SpanParser` stays trusted, checked directly
//! against jiff's real source: a DIFFERENT type from `fmt::friendly::
//! SpanParser` despite the same name (verified independently, not
//! assumed) — one private field (`p: parser::SpanParser`, the
//! internal engine), zero setters at all this time, and its real
//! parsing methods (`parse_span`/`parse_duration`/
//! `parse_unsigned_duration`) have the same disproportionate-
//! reproduction shape, reproducing jiff's real ISO 8601 duration
//! grammar.
//!
//! `jiff::fmt::temporal::SpanPrinter` stays trusted, the identical
//! combined shape to every other `*Printer` in this checklist:
//! checked directly against jiff's real source, its one public
//! method (`lowercase`) is a plain setter over a private field, no
//! getter — and its real formatting methods (`span_to_string`/
//! `duration_to_string`/`unsigned_duration_to_string`/`print_span`/
//! etc.) have the same disproportionate-reproduction shape.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotation<'static>` gets a real
//! checked property too (see `fmt_temporal_time_zone_annotation.rs`):
//! a real data-carrying type (`kind`, `critical`, both `pub(crate)`
//! to jiff) — `From<&str>` always builds `Named` with
//! `is_critical() == false`, `From<Offset>` always builds `Offset`
//! (carrying the wrapped offset's own seconds) with
//! `is_critical() == false`.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationKind<'static>` gets a real
//! checked property too (see
//! `fmt_temporal_time_zone_annotation_kind.rs`): unlike
//! `TimeZoneAnnotation<'static>`, this `#[non_exhaustive]` enum's own
//! variants are public (no private-field indirection), so
//! `From<&str>`'s exact name content is checked directly by
//! matching — `From<Offset>` always builds `Offset` carrying the
//! wrapped offset's own seconds.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationName<'static>` gets a real
//! checked property too (see
//! `fmt_temporal_time_zone_annotation_name.rs`): wraps one private
//! field, with a real, checkable round trip —
//! `TimeZoneAnnotationName::from(s).as_str() == s` for every `&str`.

use crate::jiff::macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(
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
);
