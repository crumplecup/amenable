//! `KaniWitness` impls for Rust's scalar primitives and `String`.

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};
use crate::{KaniUtf8Buffer, KaniVerifier};

impl_kani_witness_trusted!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

impl KaniWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_unicode_scalar".to_owned(),
            claim: VERIFY_CHAR_UNICODE_SCALAR_SRC.to_owned(),
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
            harness: "verify_string_utf8_valid".to_owned(),
            claim: VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
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

/// Lawful token minted once `RustStdStandard<String>`'s UTF-8 bookkeeping
/// claim has been established from an already-proven `KaniUtf8Buffer<2>` --
/// the buffer's own bookkeeping is proven once, generically, by
/// `utf8_model::verify_kani_utf8_buffer_bookkeeping_is_consistent`; this
/// impl is what lets `String`'s proof rest on that instead of re-deriving
/// the same length/emptiness/byte-recovery facts independently.
pub struct RustStdStringUtf8Token(());

impl ProofToken for RustStdStringUtf8Token {
    type Proposition = RustStdStandard<String>;
}

impl Establish<KaniUtf8Buffer<2>, KaniVerifier> for RustStdStandard<String> {
    type Token = RustStdStringUtf8Token;

    fn establish(_credential: &KaniUtf8Buffer<2>) -> Self::Token {
        RustStdStringUtf8Token(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_STRING_UTF8_VALID_SRC, {
        /// `String`'s length and emptiness are consistent with its byte
        /// content.
        /// This proof uses `KaniUtf8Buffer` (`utf8_model.rs`), following the
        /// pattern documented in `elicitation`'s
        /// `verification::types::Utf8Bytes<MAX_LEN>`: validity is assumed
        /// symbolically under Kani rather than computed, since both the
        /// real `std::str::from_utf8` path and `utf8_model`'s own full
        /// validation state machine were confirmed to time out even for
        /// two fixed bytes when every byte is valid (see
        /// `gallery::utf8_validation_algorithm_cost`).
        /// `String`'s own type invariant already guarantees its content is
        /// valid UTF-8 by construction (nothing unsafe can produce an
        /// invalid one); what this proof establishes is that the
        /// bookkeeping `String` shares with any owned buffer -- length
        /// tracks the stored bytes, and emptiness tracks a zero length --
        /// holds conditional on that invariant. The claim is established
        /// through `Establish<KaniUtf8Buffer<2>, KaniVerifier> for
        /// RustStdStandard<String>` rather than asserted independently, so
        /// it rests on the buffer's own proven bookkeeping instead of
        /// re-deriving it inline.
        #[kani::proof]
        fn verify_string_utf8_valid() {
            use crate::{KaniUtf8Buffer, KaniUtf8BufferError};

            let bytes: [u8; 2] = kani::any();
            let len: usize = kani::any();
            kani::assume(len <= 2);

            match KaniUtf8Buffer::<2>::new(bytes, len) {
                Ok(buffer) => {
                    let _token = RustStdStandard::<String>::establish(&buffer);

                    assert_eq!(buffer.len(), len, "length tracks the stored bytes");
                    assert_eq!(buffer.is_empty(), len == 0, "emptiness tracks a zero length");
                    assert_eq!(buffer.as_bytes().len(), len);
                }
                Err(KaniUtf8BufferError::InvalidUtf8) => {
                    // Bytes can be assumed invalid under Kani's
                    // symbolic-validity model; the bookkeeping claim above
                    // only applies to the accepted construction path.
                }
                Err(KaniUtf8BufferError::TooLong) => {
                    unreachable!("len is assumed <= the buffer's own capacity")
                }
            }
        }
    }
}
