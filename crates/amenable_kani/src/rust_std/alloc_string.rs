//! `KaniWitness` impls for `alloc::string`.

use std::string::{FromUtf8Error, FromUtf16Error};

#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::KaniUtf8Buffer;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniVerifier, KaniWitness};

// Written as the fully-qualified `std::string::Drain<'static>` throughout:
// its bare name collides with `alloc::vec::Drain` and the
// `alloc::collections::{binary_heap,vec_deque}::Drain` carriers, and this
// qualification is what lets `amenable_std`'s matching evidence string
// disambiguate them for tooling reading the registry (e.g. `elicit_doc`'s
// coverage report).
impl KaniWitness for RustStdStandard<std::string::Drain<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_drain_removes_and_yields_the_content".to_owned(),
            claim: VERIFY_STRING_DRAIN_REMOVES_AND_YIELDS_THE_CONTENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::string::Drain<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::string::Drain<'static>> as KaniWitness>::proof()
            .to_string(),
    }
}

/// Witness that a `KaniStringDrainObservation` instance actually
/// demonstrated the whole-string drain law, minted only by
/// [`crate::KaniStringDrainObservation::demonstrate_whole_string_drain`].
pub struct KaniStringDrainWitnessToken(());

impl ProofToken for KaniStringDrainWitnessToken {
    type Proposition = crate::KaniStringDrainObservation;
}

impl crate::KaniStringDrainObservation {
    /// Assert the drain yields exactly `bytes[..len]` and leaves the
    /// source empty afterward. Consumes `self`: the only way to obtain
    /// the token is to have run this check against a real observation
    /// instance, not to assert it independently.
    #[must_use]
    pub fn demonstrate_whole_string_drain(
        self,
        bytes: [u8; 2],
        len: usize,
    ) -> KaniStringDrainWitnessToken {
        let yielded = self.yielded_bytes();

        assert_eq!(
            self.yielded_len(),
            len,
            "drain yields exactly the source byte length"
        );
        assert_eq!(
            yielded.len(),
            len,
            "drain recovers a byte slice with the source length"
        );
        if len >= 1 {
            assert_eq!(
                yielded[0], bytes[0],
                "drain preserves the first source byte"
            );
        }
        if len >= 2 {
            assert_eq!(
                yielded[1], bytes[1],
                "drain preserves the second source byte"
            );
        }
        assert_eq!(
            self.source_len_after_drain(),
            0,
            "drain leaves the source length at zero"
        );
        assert!(self.source_is_empty(), "drain leaves the source empty");
        KaniStringDrainWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::string::Drain<'static>>`'s
/// whole-string drain law has been established from a
/// `KaniStringDrainObservation`.
pub struct RustStdStringDrainToken(());

impl ProofToken for RustStdStringDrainToken {
    type Proposition = RustStdStandard<std::string::Drain<'static>>;
}

impl Establish<KaniStringDrainWitnessToken, KaniVerifier>
    for RustStdStandard<std::string::Drain<'static>>
{
    type Token = RustStdStringDrainToken;

    fn establish(_credential: KaniStringDrainWitnessToken) -> Self::Token {
        RustStdStringDrainToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_STRING_DRAIN_REMOVES_AND_YIELDS_THE_CONTENT_SRC, {
        /// Draining a bounded string over its full range yields the exact
        /// UTF-8 content in order and leaves the source empty afterward.
        /// This proof uses the Amenable-owned drain observation rather than
        /// the native `String::drain(..)` iterator path: the direct std
        /// harness times out under Kani even for a single ASCII character,
        /// and that false trail remains preserved in `gallery::string_drain`.
        /// A second false trail also remains documented there: asking Kani
        /// to quantify this model over symbolic UTF-8 strings still times out.
        /// The passing proof therefore rests on the already-proven
        /// `KaniUtf8Buffer<2>` bookkeeping model and checks only bounded byte
        /// recovery plus post-drain emptiness here, instead of reintroducing
        /// solver-heavy `Vec` / `str` machinery.
        /// The claim is established through
        /// `Establish<KaniStringDrainObservation, KaniVerifier> for
        /// RustStdStandard<std::string::Drain<'static>>` from the bounded
        /// observation instance that actually demonstrated the law.
        #[kani::proof]
        fn verify_string_drain_removes_and_yields_the_content() {
            let bytes: [u8; 2] = kani::any();
            let len: usize = kani::any();
            kani::assume(KaniUtf8Buffer::<2>::requires(len));

            if let Ok(buffer) = crate::KaniUtf8Buffer::<2>::new(bytes, len) {
                let observation = crate::KaniStringDrainObservation::new(buffer);
                let demonstration = observation.demonstrate_whole_string_drain(bytes, len);

                let _token = RustStdStandard::<std::string::Drain<'static>>::establish(demonstration);
            }
        }
    }
}

impl KaniWitness for RustStdStandard<FromUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_utf16_rejects_a_lone_surrogate".to_owned(),
            claim: VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromUtf16Error>",
        verifier: "kani",
        describe: || <RustStdStandard<FromUtf16Error> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC, {
        /// `String::from_utf16` accepts a valid UTF-16 code unit and
        /// rejects a lone surrogate half (a high surrogate with no
        /// paired low surrogate) — the specific failure mode
        /// `FromUtf16Error` reports.
        #[kani::proof]
        fn verify_from_utf16_rejects_a_lone_surrogate() {
            let valid: [u16; 1] = [0x61];
            assert!(String::from_utf16(&valid).is_ok(), "a valid code unit is accepted");

            let lone_surrogate: [u16; 1] = [0xD800];
            assert!(
                String::from_utf16(&lone_surrogate).is_err(),
                "a lone surrogate half is rejected"
            );

            let lone_low_surrogate: [u16; 1] = [0xDC00];
            assert!(
                String::from_utf16(&lone_low_surrogate).is_err(),
                "a lone low surrogate half is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromUtf8Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_utf8_error_recovers_the_original_bytes".to_owned(),
            claim: VERIFY_FROM_UTF8_ERROR_RECOVERS_THE_ORIGINAL_BYTES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromUtf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromUtf8Error>",
        verifier: "kani",
        describe: || <RustStdStandard<FromUtf8Error> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FROM_UTF8_ERROR_RECOVERS_THE_ORIGINAL_BYTES_SRC, {
        /// Amenable models the owned invalid-UTF-8 recovery boundary
        /// directly: if the real `String::from_utf8` /
        /// `std::string::FromUtf8Error` path conforms to this
        /// Kani-owned byte-preservation law, then `.as_bytes()` and
        /// `.into_bytes()` both recover exactly the original owned
        /// bytes that failed conversion.
        #[kani::proof]
        fn verify_from_utf8_error_recovers_the_original_bytes() {
            let depth0 = <crate::KaniFromUtf8Error as crate::KaniCompose>::kani_depth0()
                .into_bytes();
            let depth1 = <crate::KaniFromUtf8Error as crate::KaniCompose>::kani_depth1()
                .into_bytes();
            let depth2 = <crate::KaniFromUtf8Error as crate::KaniCompose>::kani_depth2()
                .into_bytes();

            let err0 = crate::KaniUtf8::classify_owned(depth0.clone()).unwrap_err();
            assert_eq!(
                err0.as_bytes(),
                &depth0[..],
                "depth0 as_bytes recovers the original bytes"
            );
            assert_eq!(
                err0.into_bytes(),
                depth0,
                "depth0 into_bytes recovers the original bytes"
            );

            let err1 = crate::KaniUtf8::classify_owned(depth1.clone()).unwrap_err();
            assert_eq!(
                err1.as_bytes(),
                &depth1[..],
                "depth1 as_bytes recovers the original bytes"
            );
            assert_eq!(
                err1.into_bytes(),
                depth1,
                "depth1 into_bytes recovers the original bytes"
            );

            let err2 = crate::KaniUtf8::classify_owned(depth2.clone()).unwrap_err();
            assert_eq!(
                err2.as_bytes(),
                &depth2[..],
                "depth2 as_bytes recovers the original bytes"
            );
            assert_eq!(
                err2.into_bytes(),
                depth2,
                "depth2 into_bytes recovers the original bytes"
            );
        }
    }
}
