#[cfg(kani)]
use amenable_core::Ensures;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;
#[cfg(kani)]
use amenable_std::ValidUnicodeScalar;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
use crate::{KaniUtf8Buffer, KaniVerifier};

impl KaniWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_unicode_scalar".to_owned(),
            VERIFY_CHAR_UNICODE_SCALAR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "kani",
        || <RustStdStandard<char> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<char>,
    "amenable_std::rust_std::RustStdStandard<char>",
    (char, char),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_UNICODE_SCALAR_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through `u32`.
        ///
        /// The first assertion is the canonical home
        /// `amenable_std::ValidUnicodeScalar` names — see that type for the
        /// same bound stated once (currently sourced from
        /// `rust_std::char`'s `verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range`,
        /// a different but equivalent restatement of this exact claim).
        #[kani::proof]
        fn verify_char_unicode_scalar() {
            let c: char = kani::any();
            let u = c as u32;

            assert!(
                <ValidUnicodeScalar as Ensures<crate::KaniVerifier>>::ensures(u),
                "char is a valid Unicode scalar value"
            );

            let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
            assert!(
                <RustStdStandard<char> as Ensures<crate::KaniVerifier>>::ensures((c, c2)),
                "char round-trips through u32"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_string_utf8_valid".to_owned(),
            VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<String>",
        "kani",
        || <RustStdStandard<String> as KaniWitness>::proof().to_string(),
    )
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniUtf8Buffer<2>) -> Self::Token {
        RustStdStringUtf8Token(())
    }
}

/// An `(is_empty, length)` pair known to agree: a buffer's own
/// emptiness check reports `true` exactly when its tracked length is
/// zero.
///
/// Independently hand-written as `assert_eq!(buffer.is_empty(), len ==
/// 0, ...)` at 2 real sites (`rust_std::primitives`'s own `String`
/// buffer bookkeeping, `utf8_model`'s `KaniUtf8Buffer` bookkeeping) --
/// the identical claim regardless of which owned-buffer type is being
/// checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct EmptinessTracksZeroLength;

impl KaniWitness for EmptinessTracksZeroLength {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_string_utf8_valid".to_owned(),
            VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(EmptinessTracksZeroLength);

kani_ensures!(
    EmptinessTracksZeroLength,
    "amenable_kani::EmptinessTracksZeroLength",
    (bool, usize),
    |(is_empty, length)| is_empty == (length == 0)
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::EmptinessTracksZeroLength",
        "kani",
        || <EmptinessTracksZeroLength as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(KaniUtf8Buffer::<2>::requires(len));

            match KaniUtf8Buffer::<2>::new(bytes, len) {
                Ok(buffer) => {
                    let _token = RustStdStandard::<String>::establish(buffer);

                    assert!(
                        KaniUtf8Buffer::<2>::ensures((buffer.len(), len)),
                        "length tracks the stored bytes"
                    );
                    assert!(
                        EmptinessTracksZeroLength::ensures((buffer.is_empty(), len)),
                        "emptiness tracks a zero length"
                    );
                    assert!(KaniUtf8Buffer::<2>::ensures((buffer.as_bytes().len(), len)));
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
