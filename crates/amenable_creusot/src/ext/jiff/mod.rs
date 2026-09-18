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

mod civil_date;
mod date_series;
mod date_time_series;
mod error;
mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;
mod timestamp_series;
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
    jiff::civil::DateTimeWith
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
