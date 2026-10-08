//! `Witness<VerusVerifier>`/`ClassifiedWitness<VerusVerifier>` bridge
//! macros and the `ExtCheckedProof` proof artifact shared by every
//! `trusted`/`checked_*` sibling module in this directory -- see
//! `trusted.rs`'s own doc comment for the trusted/checked split
//! rationale.

use amenable_core::Metadata;
use derive_getters::Getters;
use derive_new::new;

use crate::ExtProvenance;

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
pub(crate) use impl_verus_witness_trusted_ext;

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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

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
pub(crate) use impl_verus_witness_checked_ext;
