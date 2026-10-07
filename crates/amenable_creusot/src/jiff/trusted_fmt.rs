//! Trusted `jiff::fmt::*` types -- formatter/parser builders and
//! configuration markers with no inspectable state or output not
//! already covered by a real checked sibling.

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

use crate::CreusotWitness;
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

use crate::ext_bridge::{bridge_creusot_witness, impl_creusot_witness_trusted_ext};

impl_creusot_witness_trusted_ext!(
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
