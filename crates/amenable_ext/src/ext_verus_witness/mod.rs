//! `Witness<VerusVerifier>`/`ClassifiedWitness<VerusVerifier>` bridge
//! macros and the `ExtCheckedProof` proof artifact shared by every
//! `trusted`/`checked_*` sibling module in this directory. The real
//! content lives in [`bridge`] -- see its own doc comment for the
//! trusted/checked split rationale.

mod bridge;

pub use bridge::ExtCheckedProof;
pub(crate) use bridge::{impl_verus_witness_checked_ext, impl_verus_witness_trusted_ext};
