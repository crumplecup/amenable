//! `CreusotWitness` impls for Rust standard-library carriers.
//!
//! One block per concrete type: a Creusot-checkable property doesn't
//! generalize across types the way provenance does, so there is no
//! blanket impl here — each type gets exactly the contract that's actually
//! true of it. The bridge to `Witness<CreusotVerifier>` is mechanical
//! (delegates straight to `CreusotWitness`), so it's generated per type by
//! a macro rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Creusot to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! contract content to add. `char` and `String` do carry a genuine
//! constraint, so their `proof()` also names the Creusot contract function
//! that checks it, alongside the same chain-derived provenance.
//!
//! Each type also registers a [`amenable_core::ProofRecord`] alongside its
//! `Witness` bridge, so `proof()`'s output is discoverable by name for
//! audit — see `amenable_core::chain::proof_chain`. The registered
//! `evidence` name is built from a hardcoded module-path literal, not
//! `module_path!()` (which would resolve to this crate, not
//! `amenable_std` where `RustStdStandard` is actually defined) — see the
//! matching registration in `amenable_std::rust_std` for why both sides
//! need to agree on the same literal convention rather than a computed one.
//!
//! A "checked" carrier's [`CheckedProof::claim`] is the contract's own
//! verbatim source (`#[requires]`/`#[ensures]` included), captured via
//! [`amenable_derive::harness!`] so it can never drift from the real
//! contract — auditing reads the actual `requires`/`ensures` clauses, not
//! a hand-maintained paraphrase of them.

use amenable_core::{Evidence, Provenance, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard};

use crate::{CreusotVerifier, CreusotWitness};

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

macro_rules! impl_creusot_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CreusotWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_creusot_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "creusot",
                    describe: || <RustStdStandard<$ty> as CreusotWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_creusot_witness_trusted!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// Proof artifact for a carrier with a real, machine-checked Creusot
/// contract: names the contract function, carries its verbatim source as
/// `claim`, and still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    pub harness: &'static str,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    pub claim: &'static str,
    /// The chain-derived provenance this claim still rests on.
    pub provenance: RustStdProvenance,
}

impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

impl CreusotWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_roundtrip",
            claim: VERIFY_CHAR_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        describe: || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHAR_ROUNDTRIP_SRC, {
        /// `char` round-trips through itself as an identity — Creusot's
        /// contract surface for scalar carriers.
        #[requires(true)]
        #[ensures(result == c)]
        fn verify_char_roundtrip(c: char) -> char {
            c
        }
    }
}

impl CreusotWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_roundtrip",
            claim: VERIFY_STRING_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "creusot",
        describe: || <RustStdStandard<String> as CreusotWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    creusot, VERIFY_STRING_ROUNDTRIP_SRC, {
        /// `String` round-trips through itself as an identity — Creusot's
        /// contract surface for owned UTF-8 carriers.
        #[requires(true)]
        #[ensures(result == s)]
        fn verify_string_roundtrip(s: String) -> String {
            s
        }
    }
}
