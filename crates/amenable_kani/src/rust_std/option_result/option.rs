//! `KaniWitness` impls for `core::option`.

use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

use super::super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

#[cfg(kani)]
use crate::{
    AccessorRecoversTheExpectedValue, CollectedSequenceMatchesExpected,
    DerefReflectsTheStoredValue, IteratorYieldsAReferenceToTheStoredValue,
    IteratorYieldsNoneWhenExhausted,
};
#[cfg(kani)]
use amenable_core::Ensures;

impl KaniWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

/// A `bool` known to be the `true` `.is_some()` reports when an `Option` is
/// actually the `Some` variant.
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

/// The `.is_none()` sibling of [`OptionIsSomeReportsTrue`].
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
        /// `Some` round-trips its value through `unwrap`, and `None` falls
        /// back to `unwrap_or`'s default.
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

impl KaniWitness for RustStdStandard<core::option::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
        /// `Some(value).into_iter()` yields the owned value once, then stops;
        /// `None.into_iter()` yields nothing at all.
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
        /// `.iter_mut()` yields a mutable reference to the contained value,
        /// and a write through it is visible in the `Option` afterward.
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
