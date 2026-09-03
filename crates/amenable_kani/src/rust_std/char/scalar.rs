use std::char::{CharTryFromError, TryFromCharError};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::{RustStdStandard, ValidUnicodeScalar};

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<CharTryFromError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range".to_owned(),
            VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CharTryFromError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CharTryFromError>",
        "kani",
        || <RustStdStandard<CharTryFromError> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<u32>,
    "amenable_std::rust_std::RustStdStandard<u32>",
    (u32, u32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC, {
        /// `char::try_from(u32)` succeeds exactly for valid Unicode scalar
        /// values (at most `U+10FFFF`, excluding the surrogate range), and
        /// preserves the value; it fails with `CharTryFromError` otherwise.
        /// `is_valid_scalar` calls `ValidUnicodeScalar::ensures` directly
        /// rather than restating its expression.
        #[kani::proof]
        fn verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range() {
            let value: u32 = kani::any();
            let result = char::try_from(value);
            let is_valid_scalar =
                <ValidUnicodeScalar as Ensures<crate::KaniVerifier>>::ensures(value);
            if is_valid_scalar {
                let parsed = result.expect("a valid Unicode scalar value must convert");
                assert!(
                    RustStdStandard::<u32>::ensures((parsed as u32, value)),
                    "char::try_from preserves the scalar value"
                );
            } else {
                assert!(
                    FallibleOperationReportsFailure::ensures(result.is_err()),
                    "char::try_from fails for surrogate or out-of-range values"
                );
            }
        }
    }
}

/// The [`ValidUnicodeScalar`] contract type reuses
/// `verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range`
/// rather than adding a new Kani harness — it names the bound the harness
/// already checks, it doesn't prove anything new.
impl KaniWitness for ValidUnicodeScalar {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range".to_owned(),
            VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ValidUnicodeScalar);

kani_ensures!(
    ValidUnicodeScalar,
    "amenable_std::ValidUnicodeScalar",
    u32,
    |value| value <= 0x0010_FFFF && !(0xD800..=0xDFFF).contains(&value)
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::ValidUnicodeScalar",
        "kani",
        || <ValidUnicodeScalar as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<TryFromCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_from_char_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TryFromCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TryFromCharError>",
        "kani",
        || <RustStdStandard<TryFromCharError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(char)` fails with `TryFromCharError` exactly when
        /// the char's scalar value doesn't fit in `u8`, and succeeds with
        /// the same value otherwise.
        #[kani::proof]
        fn verify_try_from_char_error_occurs_exactly_when_out_of_range() {
            let c: char = kani::any();
            let result = u8::try_from(c);
            if (c as u32) <= u32::from(u8::MAX) {
                assert!(
                    AccessorRecoversTheExpectedValue::ensures((result, Ok(c as u8))),
                    "try_from succeeds and preserves the value when it fits u8"
                );
            } else {
                assert!(
                    FallibleOperationReportsFailure::ensures(result.is_err()),
                    "try_from fails with TryFromCharError when the char doesn't fit u8"
                );
            }
        }
    }
}
