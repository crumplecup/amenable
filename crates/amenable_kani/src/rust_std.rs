//! `KaniWitness` impls for Rust standard-library carriers.
//!
//! One block per concrete type: a Kani-checkable property doesn't
//! generalize across types the way provenance does, so there is no
//! blanket impl here — each type gets exactly the harness that's actually
//! true of it. The bridge to `Witness<KaniVerifier>` is mechanical
//! (delegates straight to `KaniWitness`), so it's generated per type by a
//! macro rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Kani to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! harness content to add. `char` and `String` do carry a genuine
//! constraint (Unicode scalar validity; UTF-8 validity), so their
//! `proof()` also names the Kani harness that checks it, alongside the
//! same chain-derived provenance.
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
//! A "checked" carrier's [`CheckedProof::claim`] is the harness's own
//! verbatim source, captured via [`amenable_derive::harness!`] so it can
//! never drift from what `cargo kani` would actually run — auditing reads
//! the real assertions, not a hand-maintained paraphrase of them.

use amenable_core::{Evidence, Provenance, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard};

use crate::{KaniVerifier, KaniWitness};

macro_rules! bridge_kani_witness {
    ($ty:ty) => {
        impl Witness<KaniVerifier> for $ty {
            type SupportingEvidence = <$ty as KaniWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as KaniWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as KaniWitness>::proof()
            }
        }
    };
}

macro_rules! impl_kani_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl KaniWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_kani_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "kani",
                    describe: || <RustStdStandard<$ty> as KaniWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_kani_witness_trusted!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

/// Proof artifact for a carrier with a real, machine-checked Kani harness:
/// names the harness, carries its verbatim source as `claim`, and still
/// rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Kani harness that checks this carrier's invariant.
    pub harness: &'static str,
    /// The harness's own source — what it actually asserts, verbatim.
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

impl KaniWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_unicode_scalar",
            claim: VERIFY_CHAR_UNICODE_SCALAR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "kani",
        describe: || <RustStdStandard<char> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHAR_UNICODE_SCALAR_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through `u32`.
        #[kani::proof]
        fn verify_char_unicode_scalar() {
            let c: char = kani::any();
            let u = c as u32;

            assert!(
                u <= 0xD7FF || (0xE000..=0x10FFFF).contains(&u),
                "char is a valid Unicode scalar value"
            );

            let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
            assert!(c == c2, "char round-trips through u32");
        }
    }
}

impl KaniWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_utf8_valid",
            claim: VERIFY_STRING_UTF8_VALID_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "kani",
        describe: || <RustStdStandard<String> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_STRING_UTF8_VALID_SRC, {
        /// `String` is always valid UTF-8, and its length/emptiness are
        /// consistent. Kani cannot model heap allocation symbolically, so
        /// this checks the invariant on concrete representative values.
        #[kani::proof]
        fn verify_string_utf8_valid() {
            let s = String::from("hello");
            assert!(
                std::str::from_utf8(s.as_bytes()).is_ok(),
                "String is valid UTF-8"
            );
            assert!(!s.is_empty(), "non-empty string has positive length");

            let empty = String::new();
            assert!(empty.is_empty(), "empty string has zero length");
            assert!(
                std::str::from_utf8(empty.as_bytes()).is_ok(),
                "empty String is valid UTF-8"
            );
        }
    }
}
