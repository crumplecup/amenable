#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::Requires;
#[cfg(kani)]
use amenable_std::AsciiByte;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
#[cfg(kani)]
use crate::IndexRecoversTheStoredElement;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_indexing_and_length".to_owned(),
            VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        "kani",
        || <RustStdStandard<[i32; 3]> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<[i32; 3]>,
    "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC, {
        /// A fixed-size array's `.len()` is the compile-time-known
        /// element count, and each index recovers the element it was
        /// constructed with.
        #[kani::proof]
        fn verify_array_indexing_and_length() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let arr = [a, b, c];
            assert!(
                RustStdStandard::<[i32; 3]>::ensures((arr.len(), 3)),
                "the array's length is its fixed compile-time size"
            );
            assert!(IndexRecoversTheStoredElement::ensures((arr[0], a)));
            assert!(IndexRecoversTheStoredElement::ensures((arr[1], b)));
            assert!(IndexRecoversTheStoredElement::ensures((arr[2], c)));
        }
    }
}

impl KaniWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_slice_indexing_and_length".to_owned(),
            VERIFY_SLICE_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32]>",
        "kani",
        || <RustStdStandard<[i32]> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<[i32]>,
    "amenable_std::rust_std::RustStdStandard<[i32]>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SLICE_INDEXING_AND_LENGTH_SRC, {
        /// A slice's `.len()` reports the number of elements it views,
        /// and each index recovers the underlying element. Checked via a
        /// safe unsizing coercion from a concrete array (`&arr` coerces
        /// `&[i32; 3]` to `&[i32]`): a bare `[i32]` value can never exist
        /// as a local, only a `&[i32]` reference can, so this is the
        /// only way any code -- proof or otherwise -- interacts with a
        /// slice value at all.
        #[kani::proof]
        fn verify_slice_indexing_and_length() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let arr = [a, b, c];
            let s: &[i32] = &arr;
            assert!(
                RustStdStandard::<[i32]>::ensures((s.len(), 3)),
                "the slice's length is the number of elements it views"
            );
            assert!(IndexRecoversTheStoredElement::ensures((s[0], a)));
            assert!(IndexRecoversTheStoredElement::ensures((s[1], b)));
            assert!(IndexRecoversTheStoredElement::ensures((s[2], c)));
        }
    }
}

impl KaniWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<str>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "kani",
        || <RustStdStandard<str> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<str>,
    "amenable_std::rust_std::RustStdStandard<str>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC, {
        /// A `str`'s `.len()` reports its UTF-8 byte length, and its
        /// bytes are exactly its content's UTF-8 encoding -- checked for
        /// any single-byte (ASCII) character, mirroring
        /// `rust_std::str`'s own symbolic-byte convention. A bare `str`
        /// value can never exist as a local, only a `&str` reference
        /// can (here, borrowed from an owned `String`), so this is the
        /// only way any code interacts with a `str` value at all.
        ///
        /// The `kani::assume` call below calls
        /// `AsciiByte::requires` directly rather than restating its
        /// expression — the same precondition every symbolic
        /// single-byte-character proof in `rust_std::str` assumes.
        #[kani::proof]
        fn verify_str_byte_length_and_content() {
            let byte: u8 = kani::any();
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let owned = (byte as char).to_string();
            let s: &str = &owned;
            assert!(
                <RustStdStandard<str> as Ensures<crate::KaniVerifier>>::ensures((s.len(), 1)),
                "a single ASCII char is exactly one UTF-8 byte"
            );
            assert!(IndexRecoversTheStoredElement::ensures((s.as_bytes()[0], byte)));
        }
    }
}
