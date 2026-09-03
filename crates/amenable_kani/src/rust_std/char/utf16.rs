use std::char::{DecodeUtf16, DecodeUtf16Error};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ValueIsOutsideInclusiveRange;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_decode_utf16_round_trips_a_bmp_code_unit".to_owned(),
            VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "kani",
        || <RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC, {
        /// A single non-surrogate UTF-16 code unit decodes to the char
        /// with that same scalar value.
        #[kani::proof]
        fn verify_decode_utf16_round_trips_a_bmp_code_unit() {
            let unit: u16 = kani::any();
            kani::assume(ValueIsOutsideInclusiveRange::requires((unit, 0xD800, 0xDFFF)));

            let mut iter = char::decode_utf16([unit]);
            let decoded = iter
                .next()
                .expect("a one-element iterator yields exactly one result")
                .expect("a non-surrogate BMP code unit always decodes successfully");
            assert!(
                AccessorRecoversTheExpectedValue::ensures((decoded as u32, u32::from(unit))),
                "decoding a BMP code unit yields the char with that same scalar value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<DecodeUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_decode_utf16_error_reports_the_unpaired_surrogate".to_owned(),
            VERIFY_DECODE_UTF16_ERROR_REPORTS_THE_UNPAIRED_SURROGATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DecodeUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DecodeUtf16Error>",
        "kani",
        || <RustStdStandard<DecodeUtf16Error> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DECODE_UTF16_ERROR_REPORTS_THE_UNPAIRED_SURROGATE_SRC, {
        /// A lone high surrogate with no following low surrogate fails to
        /// decode, and the error reports the exact unpaired code unit.
        #[kani::proof]
        fn verify_decode_utf16_error_reports_the_unpaired_surrogate() {
            let lone_surrogate: u16 = 0xD800;
            let mut iter = char::decode_utf16([lone_surrogate]);
            let result = iter
                .next()
                .expect("a one-element iterator yields exactly one result");
            let err = result.expect_err("a lone surrogate with no pair must fail to decode");
            assert!(
                AccessorRecoversTheExpectedValue::ensures((
                    err.unpaired_surrogate(),
                    lone_surrogate
                )),
                "DecodeUtf16Error reports the exact unpaired code unit"
            );
        }
    }
}
