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
//! rest on. `jiff::tz::Offset`/`jiff::Error`/`jiff::SignedDuration`/
//! `jiff::Span`/`jiff::SpanFieldwise`/`jiff::TimestampSeries`/`jiff::
//! Unit` are the exceptions so far — real, hand-verified
//! accommodation models in `amenable_verus::ext::jiff::{offset,error,
//! signed_duration,span,span_fieldwise,timestamp_series,unit}` (jiff
//! itself is still unreachable, but each model's own law is genuinely
//! checked, and independently confirmed against the real API by the
//! Kani/Creusot proofs for the identical claim where those backends
//! could check it at all — `TimestampSeries` is Kani-uncheckable and
//! `Unit` is Creusot-uncheckable, each for a real, confirmed reason,
//! see `amenable_kani::ext::jiff`'s and `amenable_creusot::ext::jiff`'s
//! own doc comments). `jiff::RoundMode`/`jiff::
//! SignedDurationRound`/`jiff::SpanArithmetic<'static>`/`jiff::
//! SpanCompare<'static>`/`jiff::SpanRelativeTo<'static>`/`jiff::
//! SpanRound<'static>`/`jiff::SpanTotal<'static>`/`jiff::
//! TimestampArithmetic`/`jiff::TimestampDifference`/`jiff::
//! TimestampRound`/`jiff::ZonedArithmetic` stay trusted for different,
//! simpler real reasons (checked against jiff's real source):
//! `RoundMode` has no public methods at all beyond the standard
//! derives; the other ten are pure builders/markers (no getters, real
//! internal logic private). `jiff::TimestampDisplayWithOffset` also
//! stays trusted, but for a different real reason: its only public
//! behavior is a `Display` impl, and checking its exact RFC 3339
//! output would mean reproducing jiff's whole formatting algorithm.
//! Nothing non-tautological to model about any of the twelve on any
//! backend.

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
    jiff::ZonedArithmetic
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
