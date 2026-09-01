//! `KaniWitness` impls for `core::option` and `core::result`.

use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::Ensures;

    pub(super) use crate::AccessorRecoversTheExpectedValue;
    pub(super) use crate::CollectedSequenceMatchesExpected;
    pub(super) use crate::DerefReflectsTheStoredValue;
    pub(super) use crate::IteratorYieldsAReferenceToTheStoredValue;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
}
#[cfg(kani)]
use mirror::{
    AccessorRecoversTheExpectedValue, CollectedSequenceMatchesExpected,
    DerefReflectsTheStoredValue, Ensures, IteratorYieldsAReferenceToTheStoredValue,
    IteratorYieldsNoneWhenExhausted,
};

impl KaniWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_some_and_none_are_disjoint".to_owned(),
            VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        "kani",
        || <RustStdStandard<Option<i32>> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` `.is_some()` reports when an
/// `Option` is actually the `Some` variant -- following
/// `IsContinueVariantReportsTrue`'s established shape for a raw
/// boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct OptionIsSomeReportsTrue;

impl KaniWitness for OptionIsSomeReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_some_and_none_are_disjoint".to_owned(),
            VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(OptionIsSomeReportsTrue);

kani_ensures!(
    OptionIsSomeReportsTrue,
    "amenable_kani::OptionIsSomeReportsTrue",
    bool,
    |is_some| is_some
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::OptionIsSomeReportsTrue",
        "kani",
        || <OptionIsSomeReportsTrue as KaniWitness>::proof().to_string(),
    )
}

/// The `.is_none()` sibling of [`OptionIsSomeReportsTrue`], same
/// reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct OptionIsNoneReportsTrue;

impl KaniWitness for OptionIsNoneReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_some_and_none_are_disjoint".to_owned(),
            VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(OptionIsNoneReportsTrue);

kani_ensures!(
    OptionIsNoneReportsTrue,
    "amenable_kani::OptionIsNoneReportsTrue",
    bool,
    |is_none| is_none
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::OptionIsNoneReportsTrue",
        "kani",
        || <OptionIsNoneReportsTrue as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC, {
        /// `Some` round-trips its value through `unwrap`, and `None`
        /// falls back to `unwrap_or`'s default.
        #[kani::proof]
        fn verify_option_some_and_none_are_disjoint() {
            let value: i32 = kani::any();
            let some: Option<i32> = Some(value);
            assert!(OptionIsSomeReportsTrue::ensures(some.is_some()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((some.unwrap(), value)),
                "Some round-trips its value"
            );

            let none: Option<i32> = None;
            assert!(OptionIsNoneReportsTrue::ensures(none.is_none()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((none.unwrap_or(0), 0)),
                "None falls back to the default"
            );
        }
    }
}

/// A fallible operation's outcome, once computed, known to report
/// failure: whatever real reason (a length mismatch, an out-of-range
/// conversion, a rejected alignment, ...) the operation's own body
/// decided on, `.is_err()` reflects it.
///
/// Independently hand-written as `assert!(result.is_err(), ...)` at 8
/// real sites across `TryFromSliceError`, `char::try_from`'s surrogate
/// rejection, `TryFromCharError`, `Layout::from_size_align`'s alignment
/// rejection, `TryFromIntError`, a `Result<i32, i32>` freshly
/// constructed as `Err`, and `HandleOrInvalid`'s sentinel-conversion
/// rejection -- the identical claim shape regardless of which fallible
/// operation or real rejection reason produced the `Err`. Same "trust
/// the body, name the flag" reasoning as `EmptiedContainerReportsEmpty`
/// (`rust_std::alloc_collections`): needs no type parameter, since
/// every real site already computes the `bool` before asserting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FallibleOperationReportsFailure;

impl KaniWitness for FallibleOperationReportsFailure {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_owned(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(FallibleOperationReportsFailure);

kani_ensures!(
    FallibleOperationReportsFailure,
    "amenable_kani::FallibleOperationReportsFailure",
    bool,
    |is_err| is_err
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FallibleOperationReportsFailure",
        "kani",
        || <FallibleOperationReportsFailure as KaniWitness>::proof().to_string(),
    )
}

/// The positive counterpart to [`FallibleOperationReportsFailure`]:
/// a `bool` known to be the `true` a fallible operation's own success
/// check reports when it actually succeeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FallibleOperationReportsSuccess;

impl KaniWitness for FallibleOperationReportsSuccess {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_owned(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(FallibleOperationReportsSuccess);

kani_ensures!(
    FallibleOperationReportsSuccess,
    "amenable_kani::FallibleOperationReportsSuccess",
    bool,
    |is_ok| is_ok
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FallibleOperationReportsSuccess",
        "kani",
        || <FallibleOperationReportsSuccess as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_owned(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        "kani",
        || <RustStdStandard<Result<i32, i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Result<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
    (Result<i32, i32>, Result<i32, i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, {
        /// `Ok` round-trips its value through `unwrap`, and `Err`
        /// round-trips its value through `unwrap_err`.
        #[kani::proof]
        fn verify_result_ok_and_err_are_disjoint() {
            let value: i32 = kani::any();
            let ok: Result<i32, i32> = Ok(value);
            assert!(FallibleOperationReportsSuccess::ensures(ok.is_ok()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((ok.unwrap(), value)),
                "Ok round-trips its value"
            );

            let err_value: i32 = kani::any();
            let err: Result<i32, i32> = Err(err_value);
            assert!(FallibleOperationReportsFailure::ensures(err.is_err()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((err.unwrap_err(), err_value)),
                "Err round-trips its value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::option::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_into_iter_yields_zero_or_one_owned_value".to_owned(),
            VERIFY_OPTION_INTO_ITER_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::option::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::option::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<core::option::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::option::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<core::option::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_OPTION_INTO_ITER_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC, {
        /// `Some(value).into_iter()` yields the owned value once, then
        /// stops; `None.into_iter()` yields nothing at all.
        #[kani::proof]
        fn verify_option_into_iter_yields_zero_or_one_owned_value() {
            let value: i32 = kani::any();
            let mut it = Some(value).into_iter();
            assert!(RustStdStandard::<core::option::IntoIter<i32>>::ensures((it.next(), Some(value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));

            let mut empty_it = None::<i32>.into_iter();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(empty_it.next()),
                "None's into_iter yields nothing"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::option::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_iter_yields_zero_or_one_reference".to_owned(),
            VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::option::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::option::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<core::option::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC, {
        /// `.iter()` borrows instead of consuming: it yields a shared
        /// reference to the value, not the value itself.
        #[kani::proof]
        fn verify_option_iter_yields_zero_or_one_reference() {
            let value: i32 = kani::any();
            let opt = Some(value);
            let mut it = opt.iter();
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<core::option::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_iter_mut_writes_through_to_the_option".to_owned(),
            VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::option::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::option::IterMut<'static, i32>>",
        "kani",
        || <RustStdStandard<core::option::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC, {
        /// `.iter_mut()` yields a mutable reference to the contained
        /// value, and a write through it is visible in the `Option`
        /// afterward.
        #[kani::proof]
        fn verify_option_iter_mut_writes_through_to_the_option() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mut opt = Some(value);
            {
                let mut it = opt.iter_mut();
                let first = it.next().unwrap();
                assert!(DerefReflectsTheStoredValue::ensures((*first, value)));
                *first = updated;
                assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            }
            assert!(
                CollectedSequenceMatchesExpected::ensures((opt, Some(updated))),
                "the write through iter_mut is visible"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_into_iter_yields_the_ok_value_only".to_owned(),
            VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::result::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<core::result::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::result::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<core::result::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC, {
        /// `Ok(value).into_iter()` yields the owned value once, then
        /// stops; `Err(_).into_iter()` yields nothing — the `Err`
        /// payload is never exposed through iteration.
        #[kani::proof]
        fn verify_result_into_iter_yields_the_ok_value_only() {
            let value: i32 = kani::any();
            let mut it = Ok::<i32, i32>(value).into_iter();
            assert!(RustStdStandard::<core::result::IntoIter<i32>>::ensures((it.next(), Some(value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));

            let err_value: i32 = kani::any();
            let mut empty_it = Err::<i32, i32>(err_value).into_iter();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(empty_it.next()),
                "Err's into_iter yields nothing"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_iter_yields_a_reference_to_the_ok_value".to_owned(),
            VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::result::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<core::result::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC, {
        /// `.iter()` borrows instead of consuming: it yields a shared
        /// reference to the `Ok` value, not the value itself.
        #[kani::proof]
        fn verify_result_iter_yields_a_reference_to_the_ok_value() {
            let value: i32 = kani::any();
            let res: Result<i32, i32> = Ok(value);
            let mut it = res.iter();
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_iter_mut_writes_through_to_the_result".to_owned(),
            VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::result::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::IterMut<'static, i32>>",
        "kani",
        || <RustStdStandard<core::result::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC, {
        /// `.iter_mut()` yields a mutable reference to the `Ok` value,
        /// and a write through it is visible in the `Result` afterward.
        #[kani::proof]
        fn verify_result_iter_mut_writes_through_to_the_result() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mut res: Result<i32, i32> = Ok(value);
            {
                let mut it = res.iter_mut();
                let first = it.next().unwrap();
                assert!(DerefReflectsTheStoredValue::ensures((*first, value)));
                *first = updated;
                assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            }
            assert!(
                RustStdStandard::<Result<i32, i32>>::ensures((res, Ok(updated))),
                "the write through iter_mut is visible"
            );
        }
    }
}
