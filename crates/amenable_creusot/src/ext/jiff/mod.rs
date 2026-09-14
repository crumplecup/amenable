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
//! `jiff::RoundMode` also stays trusted, for the same real reason named
//! in `amenable_kani::ext::jiff`'s own doc comment: checked directly
//! against jiff's real source, it has no public methods at all beyond
//! the standard derives — nothing non-tautological to state on any
//! backend.

mod error;
mod offset;

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
    jiff::RoundMode
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
