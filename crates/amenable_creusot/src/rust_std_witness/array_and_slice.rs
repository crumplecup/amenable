use super::CheckedProof;

use crate::{
    CreusotVerifier, CreusotWitness, INDEXING_AND_LENGTH_HOLDS_SRC,
    SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
    SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC,
    VERIFY_SLICE_INDEXING_AND_LENGTH_SRC,
    VERIFY_SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
    VERIFY_SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::{IndexingAndLength, RustStdStandard};

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_indexing_and_length".to_string(),
            VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        "creusot",
        || <RustStdStandard<[i32; 3]> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_slice_indexing_and_length".to_string(),
            VERIFY_SLICE_INDEXING_AND_LENGTH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32]>",
        "creusot",
        || <RustStdStandard<[i32]> as CreusotWitness>::proof().to_string(),
    )
}

/// [`IndexingAndLength`] reuses the array-indexing harness rather than
/// adding a new Creusot proof — it names the postcondition both the
/// array and slice indexing/length proofs already share, it doesn't
/// prove anything new.
impl CreusotWitness for IndexingAndLength {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_indexing_and_length".to_string(),
            VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(IndexingAndLength);

/// Returns `amenable_creusot::INDEXING_AND_LENGTH_HOLDS_SRC` directly --
/// the verbatim, `harness!`-captured source of the real `#[logic(open)]
/// fn indexing_and_length_holds` both `verify_array_indexing_and_length`
/// and `verify_slice_indexing_and_length` call, not a hand-retyped copy
/// of its expression. There is exactly one place this postcondition's
/// text exists in the whole codebase.
impl Ensures<CreusotVerifier> for IndexingAndLength {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        INDEXING_AND_LENGTH_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::IndexingAndLength",
        "creusot",
        "ensures",
        || <IndexingAndLength as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_slice_iter_yields_shared_references_in_order".to_string(),
            VERIFY_SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        "creusot",
        || <RustStdStandard<std::slice::Iter<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    )
}

/// Returns `amenable_creusot::SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn slice_iter_yields_shared_references_in_order` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<std::slice::Iter<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SLICE_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<std::slice::Iter<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_slice_iter_mut_yields_mutable_references_that_write_through".to_string(),
            VERIFY_SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        "creusot",
        || {
            <RustStdStandard<std::slice::IterMut<'static, i32>> as CreusotWitness>::proof()
                .to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn slice_iter_mut_yields_mutable_references_that_
/// write_through` the real site calls, not a hand-retyped copy of its
/// expression.
impl Ensures<CreusotVerifier> for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SLICE_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<std::slice::IterMut<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
