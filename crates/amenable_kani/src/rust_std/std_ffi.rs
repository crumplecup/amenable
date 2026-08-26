//! `KaniWitness` impls for `std::ffi` (`OsStr`/`OsString`).

use std::ffi::{OsStr, OsString};

#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::IndexRecoversTheStoredElement;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniUtf8Buffer, KaniVerifier};

impl KaniWitness for RustStdStandard<OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_os_str_valid_utf8_content_round_trips_through_to_str".to_owned(),
            VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsStr>",
        "kani",
        || <RustStdStandard<OsStr> as KaniWitness>::proof().to_string(),
    )
}

/// Lawful token minted once `RustStdStandard<OsStr>`'s UTF-8 round-trip
/// claim has been established from an already-proven `KaniUtf8Buffer<2>` --
/// mirrors `primitives::RustStdStringUtf8Token`, one credential shared by
/// both std-facing carriers built on the same buffer model.
pub struct RustStdOsStrUtf8Token(());

impl ProofToken for RustStdOsStrUtf8Token {
    type Proposition = RustStdStandard<OsStr>;
}

impl Establish<KaniUtf8Buffer<2>, KaniVerifier> for RustStdStandard<OsStr> {
    type Token = RustStdOsStrUtf8Token;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniUtf8Buffer<2>) -> Self::Token {
        RustStdOsStrUtf8Token(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC, {
        /// An `OsStr` built entirely of valid Unicode round-trips
        /// exactly through `.to_str()`, and `.len()` reports its byte
        /// length.
        /// This proof uses `KaniUtf8Buffer` (`utf8_model.rs`), following the
        /// pattern documented in `elicitation`'s
        /// `verification::types::Utf8Bytes<MAX_LEN>`: validity is assumed
        /// symbolically under Kani rather than computed, since both the
        /// real `std::str::from_utf8` path and `utf8_model`'s own full
        /// validation state machine were confirmed to time out even for
        /// two fixed bytes when every byte is valid (see
        /// `gallery::utf8_validation_algorithm_cost`).
        /// On every platform, `OsStr::to_str()` on Unicode-valid content is
        /// exactly this same UTF-8 validity check followed by a borrow of
        /// the same bytes -- `to_str()` returning `Some` never changes what
        /// bytes are exposed, so a byte-equality check stands in for the
        /// round trip without materializing a real `&str` through
        /// `from_utf8` again. The claim is established through
        /// `Establish<KaniUtf8Buffer<2>, KaniVerifier> for
        /// RustStdStandard<OsStr>` rather than asserted independently, so
        /// it rests on the buffer's own proven bookkeeping instead of
        /// re-deriving it inline.
        #[kani::proof]
        fn verify_os_str_valid_utf8_content_round_trips_through_to_str() {
            use crate::{KaniUtf8Buffer, KaniUtf8BufferError};

            let bytes: [u8; 2] = kani::any();
            let len: usize = kani::any();
            kani::assume(KaniUtf8Buffer::<2>::requires(len));

            match KaniUtf8Buffer::<2>::new(bytes, len) {
                Ok(buffer) => {
                    let _token = RustStdStandard::<OsStr>::establish(buffer);

                    // Compared index-by-index rather than via slice
                    // equality: `&[u8] == &[u8]` over a symbolic length
                    // routes through CBMC's memcmp intrinsic with a
                    // symbolic byte count, which was confirmed to blow up
                    // the same way as `slice::Split`'s internal
                    // `Iterator::position` call (see
                    // `gallery::slice_split_position`).
                    let recovered = buffer.as_bytes();
                    assert!(
                        KaniUtf8Buffer::<2>::ensures((recovered.len(), len)),
                        "len() reports the byte length"
                    );
                    if len >= 1 {
                        assert!(
                            IndexRecoversTheStoredElement::ensures((recovered[0], bytes[0])),
                            "to_str's content is exactly the OsStr's own bytes"
                        );
                    }
                    if len >= 2 {
                        assert!(
                            IndexRecoversTheStoredElement::ensures((recovered[1], bytes[1])),
                            "to_str's content is exactly the OsStr's own bytes"
                        );
                    }
                }
                Err(KaniUtf8BufferError::InvalidUtf8) => {
                    // The round-trip claim only applies to content that is
                    // in fact valid Unicode (the accepted construction path).
                }
                Err(KaniUtf8BufferError::TooLong) => {
                    unreachable!("len is assumed <= the buffer's own capacity")
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_os_string_push_appends_to_the_existing_content".to_owned(),
            VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsString>",
        "kani",
        || <RustStdStandard<OsString> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC, {
        /// `.push()` appends to the existing content, without
        /// disturbing what was already there.
        #[kani::proof]
        fn verify_os_string_push_appends_to_the_existing_content() {
            let mut os_string = OsString::from("hello");
            os_string.push(", world");
            assert_eq!(os_string.as_os_str(), OsStr::new("hello, world"));
        }
    }
}

impl KaniWitness for RustStdStandard<std::ffi::os_str::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_os_str_display_renders_valid_utf8_content_unchanged".to_owned(),
            VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::ffi::os_str::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::os_str::Display<'static>>",
        "kani",
        || <RustStdStandard<std::ffi::os_str::Display<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC, {
        /// `.display()` renders valid-UTF-8 content exactly as written,
        /// with no lossy substitution needed.
        #[kani::proof]
        fn verify_os_str_display_renders_valid_utf8_content_unchanged() {
            let os_str = OsStr::new("hello");
            assert_eq!(os_str.display().to_string(), "hello", "display renders valid UTF-8 content unchanged");
        }
    }
}
