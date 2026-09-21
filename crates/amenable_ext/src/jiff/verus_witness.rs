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
//! StdFmtWrite<String>`/`jiff::fmt::
//! StdIoWrite<Vec<u8>>`/`jiff::tz::Offset`/
//! `jiff::Error`/`jiff::
//! SignedDuration`/`jiff::Span`/`jiff::SpanFieldwise`/`jiff::
//! TimestampSeries`/`jiff::Unit`/`jiff::ZonedSeries` are the
//! exceptions so far — real, hand-verified accommodation models in
//! `amenable_verus::ext::jiff::{civil_date,civil_era,
//! civil_iso_week_date,civil_time,civil_time_series,civil_weekday,
//! civil_weekdays_forward,civil_weekdays_reverse,date_series,
//! date_time_series,fmt_friendly_fractional_unit,fmt_std_fmt_write,
//! fmt_std_io_write,fmt_strtime_broken_down_time,fmt_strtime_meridiem,
//! fmt_temporal_pieces,fmt_temporal_pieces_numeric_offset,
//! fmt_temporal_pieces_offset,
//! fmt_temporal_time_zone_annotation,
//! offset,error,
//! signed_duration,span,
//! span_fieldwise,timestamp_series,unit,zoned_series}` (jiff itself
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
//! structurally unreachable, see `amenable_verus::ext::jiff::
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
//! `jiff::fmt::StdFmtWrite<String>`'s model differs from every other
//! model in this list: it's not a from-scratch hand reproduction, but
//! uses `vstd`'s own already-contracted `String::append` directly
//! (real `vstd` coverage confirmed present, unlike jiff itself), and
//! states the FULL round-trip claim (`amenable_kani::ext::jiff::
//! fmt_std_fmt_write`'s own stronger claim) rather than
//! `amenable_creusot::ext_jiff::fmt_std_fmt_write`'s narrower
//! never-fails-only one — Verus never extern-specs jiff's actual
//! generic trait impl the way Creusot tried and hit a real "extern
//! spec generics don't match" wall on, so it has no reason to scope
//! down to match.
//!
//! `jiff::fmt::StdIoWrite<Vec<u8>>`'s model is the identical shape,
//! using `vstd`'s own already-contracted `Vec::extend_from_slice`
//! instead of `String::append` — also states the FULL round-trip
//! claim, unlike `amenable_creusot::ext_jiff::fmt_std_io_write`'s
//! narrower never-fails-only one.
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
//! `jiff::fmt::friendly::FractionalUnit`'s model reuses `unit.rs`'s
//! existing `UnitModel` directly rather than modeling `Unit`'s ten
//! variants a second time — the same claim `amenable_kani::ext::jiff::
//! fmt_friendly_fractional_unit` and `amenable_creusot::ext_jiff::
//! fmt_friendly_fractional_unit` check against jiff's real
//! `From<FractionalUnit> for Unit` conversion.
//!
//! `jiff::fmt::strtime::BrokenDownTime`'s model is the first `&mut
//! self`-setter model in this checklist: unlike every prior model
//! (pure functions building a fresh value), it uses Verus's own
//! `old(self)`/final-`self` convention directly — no `mem::forget`/
//! `ManuallyDrop` workaround needed at all, since Verus has no
//! CBMC-style Drop-glue cost (confirmed: this model verifies
//! instantly, unlike `amenable_kani::ext::jiff::
//! fmt_strtime_broken_down_time`'s own real CBMC wall for the
//! identical claim).
//!
//! `jiff::fmt::strtime::Meridiem`'s model reproduces its documented
//! `hour < 12` threshold directly with its own small `MeridiemModel`
//! enum (`Am`/`Pm`) — the same claim `amenable_kani::ext::jiff::
//! fmt_strtime_meridiem` and `amenable_creusot::ext_jiff::
//! fmt_strtime_meridiem` check against jiff's real `From<civil::Time>
//! for Meridiem` conversion.
//!
//! `jiff::fmt::temporal::Pieces<'static>`'s model reproduces
//! `date`/`time` as plain tuples (`(i16, i8, i8)`/`Option<(i8, i8,
//! i8, i32)>`) rather than reusing `civil_date.rs`'s/`civil_time.rs`'s
//! own model types, exercising `with_date`/`with_time` independently
//! on their own fresh instance each — the same claim
//! `amenable_kani::ext::jiff::fmt_temporal_pieces` and
//! `amenable_creusot::ext_jiff::fmt_temporal_pieces` both check
//! against jiff's real API, scoped to `date`/`time` only (see either
//! module's own doc comment for why `offset`/`time_zone_annotation`
//! are deliberately excluded).
//!
//! `jiff::fmt::temporal::PiecesNumericOffset`'s model reproduces it as
//! `{ offset_seconds: i32, is_negative: bool }`, self-contained rather
//! than cross-importing `offset.rs`'s own range spec fn — the same
//! claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_numeric_offset` and `amenable_creusot::
//! ext_jiff::fmt_temporal_pieces_numeric_offset` both check against
//! jiff's real `From<Offset>`/`with_negative_zero` API.
//!
//! `jiff::fmt::temporal::PiecesOffset`'s model reproduces it as a
//! plain struct (`{ is_zulu: bool, numeric_seconds: i32 }`), not a
//! data-carrying `enum` (a first attempt tripped a real `missing_docs`
//! warning on a Verus-synthesized method — see that model's own doc
//! comment), self-contained rather than cross-importing
//! `fmt_temporal_pieces_numeric_offset.rs`'s own model — the same
//! claim `amenable_kani::ext::jiff::
//! fmt_temporal_pieces_offset` and `amenable_creusot::ext_jiff::
//! fmt_temporal_pieces_offset` both check against jiff's real
//! `Zulu`/`From<Offset>`/`to_numeric_offset` API.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotation<'static>`'s model
//! reproduces it as a plain struct (`{ is_named: bool,
//! offset_seconds: i32 }`), at the same NARROWER scope
//! `fmt_temporal_time_zone_annotation.rs`'s own Creusot doc comment
//! documents (no string content modeled at all) — the same claim
//! `amenable_kani::ext::jiff::fmt_temporal_time_zone_annotation` and
//! `amenable_creusot::ext_jiff::fmt_temporal_time_zone_annotation`
//! both check against jiff's real `From<&str>`/`From<Offset>` API.
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

use amenable_core::{
    ClassifiedWitness, Evidence, Metadata, VerusVerifier, Witness, WitnessSupportSummary,
};
use derive_getters::Getters;
use derive_new::new;

use crate::{ExtProvenance, ExtStandard};

macro_rules! impl_verus_witness_trusted_ext {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Witness<VerusVerifier> for ExtStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = ExtProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }

                fn support() -> WitnessSupportSummary {
                    WitnessSupportSummary::trusted_leaf()
                }
            }

            impl ClassifiedWitness<VerusVerifier> for ExtStandard<$ty> {}

            ::inventory::submit! {
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_ext::ExtStandard<", stringify!($ty), ">"),
                    "verus",
                    || <ExtStandard<$ty> as Witness<VerusVerifier>>::proof().to_string(),
                )
            }
        )*
    };
}

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
    jiff::fmt::temporal::SpanPrinter
);

/// Proof artifact for an `ExtStandard<T>` carrier with a real,
/// hand-verified Verus accommodation model — the Verus counterpart of
/// `amenable_kani`/`amenable_creusot`'s own `ExtCheckedProof`, holding
/// the model's source instead of a real per-type harness on jiff
/// itself (which Verus cannot reach at all).
#[derive(Debug, Clone, PartialEq, Eq, Getters, new)]
pub struct ExtCheckedProof {
    /// The Verus proof function that checks the model's invariant.
    harness: String,
    /// The model's own source — what it actually asserts, verbatim.
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

/// Registers a real, hand-verified Verus accommodation model as an
/// `ExtStandard<$ty>` witness — the checked counterpart of
/// [`impl_verus_witness_trusted_ext`]. `$src_path` is the model's own
/// source file (embedded verbatim as the witness's `claim`), relative
/// to this file.
macro_rules! impl_verus_witness_checked_ext {
    ($ty:ty, $harness:literal, $src_path:literal) => {
        impl Witness<VerusVerifier> for ExtStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = ExtCheckedProof;

            fn proof() -> Self::ProofArtifact {
                ExtCheckedProof::new(
                    $harness.to_owned(),
                    include_str!($src_path).to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }

            fn support() -> WitnessSupportSummary {
                WitnessSupportSummary::checked_leaf()
            }
        }

        impl ClassifiedWitness<VerusVerifier> for ExtStandard<$ty> {}

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_ext::ExtStandard<", stringify!($ty), ">"),
                "verus",
                || <ExtStandard<$ty> as Witness<VerusVerifier>>::proof().to_string(),
            )
        }
    };
}

impl_verus_witness_checked_ext!(
    jiff::civil::Date,
    "verify_civil_date_new_year_month_day_round_trips_model",
    "../../../amenable_verus/src/ext/jiff/civil_date.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::Era,
    "verify_civil_era_year_classifies_bce_and_ce_correctly_model",
    "../../../amenable_verus/src/ext/jiff/civil_era.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::ISOWeekDate,
    "verify_civil_iso_week_date_new_year_week_weekday_round_trips_model",
    "../../../amenable_verus/src/ext/jiff/civil_iso_week_date.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::Time,
    "verify_civil_time_new_hour_minute_second_subsec_round_trips_model",
    "../../../amenable_verus/src/ext/jiff/civil_time.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::WeekdaysReverse,
    "verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor",
    "../../../amenable_verus/src/ext/jiff/civil_weekdays_reverse.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::WeekdaysForward,
    "verify_civil_weekdays_forward_next_yields_start_then_its_successor",
    "../../../amenable_verus/src/ext/jiff/civil_weekdays_forward.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::Weekday,
    "verify_civil_weekday_monday_one_offset_round_trips_model",
    "../../../amenable_verus/src/ext/jiff/civil_weekday.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::TimeSeries,
    "verify_civil_time_series_next_yields_start_then_advances_by_period",
    "../../../amenable_verus/src/ext/jiff/civil_time_series.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::DateSeries,
    "verify_date_series_next_yields_start_then_advances_by_period",
    "../../../amenable_verus/src/ext/jiff/date_series.rs"
);

impl_verus_witness_checked_ext!(
    jiff::civil::DateTimeSeries,
    "verify_date_time_series_next_yields_start_then_advances_by_period",
    "../../../amenable_verus/src/ext/jiff/date_time_series.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::friendly::FractionalUnit,
    "verify_fmt_friendly_fractional_unit_from_matches_documented_mapping_model",
    "../../../amenable_verus/src/ext/jiff/fmt_friendly_fractional_unit.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::strtime::BrokenDownTime,
    "verify_fmt_strtime_broken_down_time_numeric_setters_round_trip_model",
    "../../../amenable_verus/src/ext/jiff/fmt_strtime_broken_down_time.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::strtime::Meridiem,
    "verify_fmt_strtime_meridiem_from_time_matches_hour_threshold_model",
    "../../../amenable_verus/src/ext/jiff/fmt_strtime_meridiem.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::Pieces<'static>,
    "verify_fmt_temporal_pieces_with_date_with_time_round_trip_model",
    "../../../amenable_verus/src/ext/jiff/fmt_temporal_pieces.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::PiecesNumericOffset,
    "verify_fmt_temporal_pieces_numeric_offset_from_and_with_negative_zero_model",
    "../../../amenable_verus/src/ext/jiff/fmt_temporal_pieces_numeric_offset.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::PiecesOffset,
    "verify_fmt_temporal_pieces_offset_zulu_and_from_offset_round_trip_model",
    "../../../amenable_verus/src/ext/jiff/fmt_temporal_pieces_offset.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::temporal::TimeZoneAnnotation<'static>,
    "verify_fmt_temporal_time_zone_annotation_from_name_and_from_offset_model",
    "../../../amenable_verus/src/ext/jiff/fmt_temporal_time_zone_annotation.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::StdFmtWrite<String>,
    "verify_fmt_std_fmt_write_write_str_model",
    "../../../amenable_verus/src/ext/jiff/fmt_std_fmt_write.rs"
);

impl_verus_witness_checked_ext!(
    jiff::fmt::StdIoWrite<Vec<u8>>,
    "verify_fmt_std_io_write_write_str_model",
    "../../../amenable_verus/src/ext/jiff/fmt_std_io_write.rs"
);

impl_verus_witness_checked_ext!(
    jiff::tz::Offset,
    "verify_offset_from_seconds_model_round_trips",
    "../../../amenable_verus/src/ext/jiff/offset.rs"
);

impl_verus_witness_checked_ext!(
    jiff::Error,
    "verify_error_classification_predicates_are_mutually_exclusive",
    "../../../amenable_verus/src/ext/jiff/error.rs"
);

impl_verus_witness_checked_ext!(
    jiff::SignedDuration,
    "verify_signed_duration_new_model_normalizes_nanos_and_carries_into_secs",
    "../../../amenable_verus/src/ext/jiff/signed_duration.rs"
);

impl_verus_witness_checked_ext!(
    jiff::Span,
    "verify_span_unit_setters_model_round_trips",
    "../../../amenable_verus/src/ext/jiff/span.rs"
);

impl_verus_witness_checked_ext!(
    jiff::SpanFieldwise,
    "verify_span_fieldwise_negation_model_negates_every_unit_getter",
    "../../../amenable_verus/src/ext/jiff/span_fieldwise.rs"
);

impl_verus_witness_checked_ext!(
    jiff::TimestampSeries,
    "verify_timestamp_series_next_yields_start_then_advances_by_period",
    "../../../amenable_verus/src/ext/jiff/timestamp_series.rs"
);

impl_verus_witness_checked_ext!(
    jiff::Unit,
    "verify_unit_ordering_matches_discriminant_order_model",
    "../../../amenable_verus/src/ext/jiff/unit.rs"
);

impl_verus_witness_checked_ext!(
    jiff::ZonedSeries,
    "verify_zoned_series_next_yields_start_then_advances_by_period_under_utc",
    "../../../amenable_verus/src/ext/jiff/zoned_series.rs"
);
