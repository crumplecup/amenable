//! `VerusWitness` impls for Rust standard-library carriers.
//!
//! One block per concrete type: a Verus-checkable property doesn't
//! generalize across types the way provenance does, so there is no
//! blanket impl here — each type gets exactly the spec that's actually
//! true of it. The bridge to `Witness<VerusVerifier>` is mechanical
//! (delegates straight to `VerusWitness`), so it's generated per type by a
//! macro rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Verus to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! spec content to add. `char` and `String` do carry a genuine constraint,
//! so their `proof()` also names the Verus spec function that checks it,
//! alongside the same chain-derived provenance.
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
//! A "checked" carrier's [`CheckedProof::claim`] is the spec's own
//! verbatim source (`ensures` clause included), captured via
//! [`amenable_derive::harness!`] so it can never drift from the real spec
//! — auditing reads the actual `ensures` clause, not a hand-maintained
//! paraphrase of it.

use amenable_core::{Evidence, Provenance, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard};

use crate::{VerusVerifier, VerusWitness};

macro_rules! bridge_verus_witness {
    ($ty:ty) => {
        impl Witness<VerusVerifier> for $ty {
            type SupportingEvidence = <$ty as VerusWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as VerusWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as VerusWitness>::proof()
            }
        }
    };
}

macro_rules! impl_verus_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl VerusWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_verus_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "verus",
                    describe: || <RustStdStandard<$ty> as VerusWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_verus_witness_trusted!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// Proof artifact for a carrier with a real, machine-checked Verus spec:
/// names the spec function, carries its verbatim source as `claim`, and
/// still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Verus spec function that checks this carrier's invariant.
    pub harness: &'static str,
    /// The spec's own source — what it actually ensures, verbatim.
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

impl VerusWitness for RustStdStandard<char> {
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

bridge_verus_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "verus",
        describe: || <RustStdStandard<char> as VerusWitness>::proof().to_string(),
    }
}

// Gated on a `#[cfg(verus)]` convention matching this workspace's Kani
// harnesses; unlike Kani, Verus's actual toolchain-gating convention
// hasn't been verified against a real Verus install.
amenable_derive::harness! {
    verus, VERIFY_CHAR_ROUNDTRIP_SRC, {
        // `char` round-trips through itself as an identity — Verus's spec
        // surface for scalar carriers.
        verus! {
            fn verify_char_roundtrip(c: char) -> (result: char)
                ensures result == c,
            {
                c
            }
        }
    }
}

impl VerusWitness for RustStdStandard<String> {
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

bridge_verus_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "verus",
        describe: || <RustStdStandard<String> as VerusWitness>::proof().to_string(),
    }
}

// `String` round-trips through itself as an identity — Verus's spec
// surface for owned UTF-8 carriers.
amenable_derive::harness! {
    verus, VERIFY_STRING_ROUNDTRIP_SRC, {
        verus! {
            fn verify_string_roundtrip(s: String) -> (result: String)
                ensures result == s,
            {
                s
            }
        }
    }
}
