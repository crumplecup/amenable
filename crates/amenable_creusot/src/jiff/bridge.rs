//! `ExtCheckedProof` -- the proof artifact for an `ExtStandard<T>` carrier
//! with a real, machine-checked Creusot contract -- plus the
//! `bridge_creusot_witness!`/`impl_creusot_witness_trusted_ext!` macros
//! shared by every `trusted_*` sibling module in this directory.

use amenable_core::Metadata;
use amenable_ext::ExtProvenance;

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
pub(crate) use bridge_creusot_witness;

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
pub(crate) use impl_creusot_witness_trusted_ext;

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
