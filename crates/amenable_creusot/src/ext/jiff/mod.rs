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

mod error;
mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;
mod timestamp_series;

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
    jiff::ZonedDifference<'static>
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
