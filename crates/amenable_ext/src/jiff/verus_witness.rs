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
//! rest on. `jiff::tz::Offset` is the first exception — a real,
//! hand-verified accommodation model in
//! `amenable_verus::ext::jiff::offset` (jiff itself is still
//! unreachable, but the model's own round-trip law is genuinely
//! checked, and independently confirmed against the real API by the
//! Kani/Creusot proofs for the identical claim).

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

impl_verus_witness_trusted_ext!(jiff::Timestamp, jiff::Zoned, jiff::civil::DateTime);

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

const VERIFY_OFFSET_FROM_SECONDS_MODEL_ROUND_TRIPS_SRC: &str =
    include_str!("../../../amenable_verus/src/ext/jiff/offset.rs");

impl Witness<VerusVerifier> for ExtStandard<jiff::tz::Offset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_offset_from_seconds_model_round_trips".to_owned(),
            VERIFY_OFFSET_FROM_SECONDS_MODEL_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for ExtStandard<jiff::tz::Offset> {}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::tz::Offset>",
        "verus",
        || <ExtStandard<jiff::tz::Offset> as Witness<VerusVerifier>>::proof().to_string(),
    )
}
