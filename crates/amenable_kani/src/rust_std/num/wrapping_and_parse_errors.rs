use std::num::{
    FpCategory, IntErrorKind, ParseFloatError, ParseIntError, Saturating, TryFromIntError, Wrapping,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use std::num::NonZero;

    pub(super) use amenable_core::Ensures;

    pub(super) use crate::AccessorRecoversTheExpectedValue;
    pub(super) use crate::FallibleOperationReportsFailure;
    pub(super) use crate::FallibleOperationReportsSuccess;
}
#[cfg(kani)]
use mirror::{
    AccessorRecoversTheExpectedValue, Ensures, FallibleOperationReportsFailure,
    FallibleOperationReportsSuccess, NonZero,
};

impl KaniWitness for RustStdStandard<Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_wrapping_add_matches_the_inner_wrapping_add".to_owned(),
            VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        "kani",
        || <RustStdStandard<Wrapping<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, {
        /// `Wrapping<T>`'s `+` operator wraps on overflow exactly like the
        /// inner type's `wrapping_add`.
        #[kani::proof]
        fn verify_wrapping_add_matches_the_inner_wrapping_add() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let result = Wrapping(a) + Wrapping(b);
            assert!(
                RustStdStandard::<i32>::ensures((result.0, a.wrapping_add(b))),
                "Wrapping<T>'s + operator matches the inner type's wrapping_add"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_saturating_add_matches_the_inner_saturating_add".to_owned(),
            VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        "kani",
        || <RustStdStandard<Saturating<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, {
        /// `Saturating<T>`'s `+` operator saturates at the numeric bounds
        /// exactly like the inner type's `saturating_add`.
        #[kani::proof]
        fn verify_saturating_add_matches_the_inner_saturating_add() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let result = Saturating(a) + Saturating(b);
            assert!(
                RustStdStandard::<i32>::ensures((result.0, a.saturating_add(b))),
                "Saturating<T>'s + operator matches the inner type's saturating_add"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_from_int_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        "kani",
        || <RustStdStandard<TryFromIntError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(i32)` fails with `TryFromIntError` exactly when the
        /// source value doesn't fit in `u8`, and succeeds with the same
        /// value otherwise.
        #[kani::proof]
        fn verify_try_from_int_error_occurs_exactly_when_out_of_range() {
            let value: i32 = kani::any();
            let result = u8::try_from(value);
            if (0..=i32::from(u8::MAX)).contains(&value) {
                assert!(
                    AccessorRecoversTheExpectedValue::ensures((result, Ok(value as u8))),
                    "try_from succeeds and preserves the value when it fits the target type"
                );
            } else {
                assert!(
                    FallibleOperationReportsFailure::ensures(result.is_err()),
                    "try_from fails with TryFromIntError when the value doesn't fit the target type"
                );
            }
        }
    }
}

impl KaniWitness for RustStdStandard<IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_int_error_kind_classifies_parse_failures".to_owned(),
            VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<IntErrorKind>);

kani_ensures!(
    RustStdStandard<IntErrorKind>,
    "amenable_std::rust_std::RustStdStandard<IntErrorKind>",
    (IntErrorKind, IntErrorKind),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        "kani",
        || <RustStdStandard<IntErrorKind> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, {
        /// Each representative integer-parse failure mode produces the
        /// matching `IntErrorKind` variant.
        #[kani::proof]
        fn verify_int_error_kind_classifies_parse_failures() {
            assert!(
                RustStdStandard::<IntErrorKind>::ensures((
                    *"".parse::<i32>().unwrap_err().kind(),
                    IntErrorKind::Empty
                )),
                "an empty string parses with IntErrorKind::Empty"
            );
            assert!(
                RustStdStandard::<IntErrorKind>::ensures((
                    *"not a number".parse::<i32>().unwrap_err().kind(),
                    IntErrorKind::InvalidDigit
                )),
                "a non-digit string parses with IntErrorKind::InvalidDigit"
            );
            assert!(
                RustStdStandard::<IntErrorKind>::ensures((
                    *"99999999999999999999".parse::<i32>().unwrap_err().kind(),
                    IntErrorKind::PosOverflow
                )),
                "a value above i32::MAX parses with IntErrorKind::PosOverflow"
            );
            assert!(
                RustStdStandard::<IntErrorKind>::ensures((
                    *"-99999999999999999999".parse::<i32>().unwrap_err().kind(),
                    IntErrorKind::NegOverflow
                )),
                "a value below i32::MIN parses with IntErrorKind::NegOverflow"
            );
            assert!(
                RustStdStandard::<IntErrorKind>::ensures((
                    *"0".parse::<NonZero<i32>>().unwrap_err().kind(),
                    IntErrorKind::Zero
                )),
                "zero parses as NonZero<i32> with IntErrorKind::Zero"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_int_error_reports_the_kind_of_the_failure".to_owned(),
            VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        "kani",
        || <RustStdStandard<ParseIntError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC, {
        /// `ParseIntError::kind()` reports the specific reason the parse
        /// failed, not just that it failed.
        #[kani::proof]
        fn verify_parse_int_error_reports_the_kind_of_the_failure() {
            let err = "not a number".parse::<i32>().expect_err("non-digit input must fail to parse");
            assert!(
                RustStdStandard::<IntErrorKind>::ensures((*err.kind(), IntErrorKind::InvalidDigit)),
                "ParseIntError::kind() reports the specific parse failure reason"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_float_error_occurs_only_for_unparseable_input".to_owned(),
            VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        "kani",
        || <RustStdStandard<ParseFloatError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC, {
        /// A non-numeric string fails to parse as `f64` with
        /// `ParseFloatError`, while a valid numeric string succeeds.
        /// `ParseFloatError`'s public API is Display/Debug/Error only (no
        /// `.kind()`, unlike `ParseIntError`), so there's no further
        /// structure to check beyond success/failure itself.
        #[kani::proof]
        fn verify_parse_float_error_occurs_only_for_unparseable_input() {
            assert!(
                FallibleOperationReportsFailure::ensures("not a float".parse::<f64>().is_err()),
                "a non-numeric string fails to parse as f64"
            );
            assert!(
                FallibleOperationReportsSuccess::ensures("3.14".parse::<f64>().is_ok()),
                "a valid numeric string parses as f64 successfully"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fp_category_matches_the_value_it_classifies".to_owned(),
            VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "kani",
        || <RustStdStandard<FpCategory> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<FpCategory>,
    "amenable_std::rust_std::RustStdStandard<FpCategory>",
    (FpCategory, FpCategory),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC, {
        /// Each representative floating-point value classifies into the
        /// `FpCategory` variant matching its own `is_*` predicates.
        #[kani::proof]
        fn verify_fp_category_matches_the_value_it_classifies() {
            assert!(
                RustStdStandard::<FpCategory>::ensures((f64::NAN.classify(), FpCategory::Nan)),
                "NaN classifies as FpCategory::Nan"
            );
            assert!(
                RustStdStandard::<FpCategory>::ensures((
                    f64::INFINITY.classify(),
                    FpCategory::Infinite
                )),
                "infinity classifies as FpCategory::Infinite"
            );
            assert!(
                RustStdStandard::<FpCategory>::ensures((0.0f64.classify(), FpCategory::Zero)),
                "zero classifies as FpCategory::Zero"
            );
            assert!(
                RustStdStandard::<FpCategory>::ensures((
                    f64::MIN_POSITIVE.classify(),
                    FpCategory::Normal
                )),
                "the smallest positive normal value classifies as FpCategory::Normal"
            );
            let subnormal = f64::MIN_POSITIVE / 2.0;
            assert!(
                RustStdStandard::<FpCategory>::ensures((
                    subnormal.classify(),
                    FpCategory::Subnormal
                )),
                "a value smaller than the smallest normal value classifies as FpCategory::Subnormal"
            );
        }
    }
}
