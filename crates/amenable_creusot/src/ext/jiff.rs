//! `CreusotWitness` impls for `amenable_ext::ExtStandard<T>` over jiff's
//! registered carriers (`jiff::{Timestamp, Zoned, civil::DateTime}` — see
//! `docs/AMENABLE_EXT_PLAN.md`'s Phase 1 for why that set and not a
//! wider one). All trusted for now: jiff is opaque to Creusot, the same
//! way most `RustStdStandard<T>` registrations are in
//! `rust_std_witness::trusted_leaf_types` — `checked` gets added per
//! type only where a real contract adds value over trusting jiff's own
//! correctness.
//!
//! `bridge_creusot_witness!` is defined locally here, not shared from a
//! crate-wide module — matching every other file in `rust_std_witness`
//! (`trusted_leaf_types` included), none of which import it from
//! elsewhere either.

use crate::{CreusotVerifier, CreusotWitness};
use amenable_core::{Evidence, Metadata, Witness};

use amenable_ext::{ExtProvenance, ExtStandard};

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}

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

impl_creusot_witness_trusted_ext!(jiff::Timestamp, jiff::Zoned, jiff::civil::DateTime);
