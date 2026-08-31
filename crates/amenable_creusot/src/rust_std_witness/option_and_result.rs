use super::CheckedProof;

use crate::{
    CreusotVerifier, CreusotWitness, ITER_YIELDS_OK_VALUE_ONCE_THEN_ENDS_SRC,
    ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC, OPTION_SOME_AND_NONE_ARE_DISJOINT_HOLDS_SRC,
    RESULT_OK_AND_ERR_ARE_DISJOINT_HOLDS_SRC,
    VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC,
    VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC,
    VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC,
    VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC,
    VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC,
    VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::{IterYieldsValueOnceThenEnds, RustStdStandard};

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
// Bare `Option<i32>`, matching `amenable_std::rust_std::option_result`'s
// own registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Option<i32>>`).
impl CreusotWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_some_and_none_are_disjoint".to_string(),
            VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        "creusot",
        || <RustStdStandard<Option<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::OPTION_SOME_AND_NONE_ARE_DISJOINT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn option_some_and_none_are_disjoint_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Option<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        OPTION_SOME_AND_NONE_ARE_DISJOINT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Option<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Bare `Result<i32, i32>`, matching `amenable_std::rust_std::option_result`'s
// own registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Result<i32, i32>>`).
impl CreusotWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_string(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        "creusot",
        || <RustStdStandard<Result<i32, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::RESULT_OK_AND_ERR_ARE_DISJOINT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn result_ok_and_err_are_disjoint_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Result<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        RESULT_OK_AND_ERR_ARE_DISJOINT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Result<i32, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Fully qualified to distinguish `core::option::Iter` from the many other
// std carriers also named `Iter` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::option::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_iter_yields_zero_or_one_reference".to_string(),
            VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::option::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::option::Iter<'static, i32>>",
        "creusot",
        || <RustStdStandard<core::option::Iter<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    )
}

// Fully qualified to distinguish `core::option::IterMut` from the many other
// std carriers also named `IterMut` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::option::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_iter_mut_writes_through_to_the_option".to_string(),
            VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::option::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::option::IterMut<'static, i32>>",
        "creusot",
        || <RustStdStandard<core::option::IterMut<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    )
}

// Fully qualified to distinguish `core::result::Iter` from the many other
// std carriers also named `Iter` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::result::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_iter_yields_a_reference_to_the_ok_value".to_string(),
            VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::result::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::Iter<'static, i32>>",
        "creusot",
        || <RustStdStandard<core::result::Iter<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    )
}

// Fully qualified to distinguish `core::result::IterMut` from the many other
// std carriers also named `IterMut` in the checklist and registry.
impl CreusotWitness for RustStdStandard<core::result::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_iter_mut_writes_through_to_the_result".to_string(),
            VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<core::result::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::IterMut<'static, i32>>",
        "creusot",
        || <RustStdStandard<core::result::IterMut<'static, i32>> as CreusotWitness>::proof()
            .to_string(),
    )
}

/// [`IterYieldsValueOnceThenEnds`] reuses the `Option::iter` harness
/// rather than adding a new Creusot proof — it names the postcondition
/// all four `Option`/`Result` value-iterator proofs already share, it
/// doesn't prove anything new.
impl CreusotWitness for IterYieldsValueOnceThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_option_iter_yields_zero_or_one_reference".to_string(),
            VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(IterYieldsValueOnceThenEnds);

/// Returns `amenable_creusot::ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn iter_yields_value_once_then_ends` both
/// `Option`-shaped sites call, not a hand-retyped copy of its
/// expression. The `Result`-shaped sibling
/// `ITER_YIELDS_OK_VALUE_ONCE_THEN_ENDS_SRC` is registered as a second
/// `ContractRecord` just below, under the same evidence -- Creusot
/// matching is by predicate name, not evidence, so one contract type
/// can name more than one real call shape.
impl Ensures<CreusotVerifier> for IterYieldsValueOnceThenEnds {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ITER_YIELDS_VALUE_ONCE_THEN_ENDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::IterYieldsValueOnceThenEnds",
        "creusot",
        "ensures",
        || <IterYieldsValueOnceThenEnds as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// The `Result`-shaped sibling of the registration above -- both real
// `#[logic(open)]` predicates `IterYieldsValueOnceThenEnds` names
// (`iter_yields_value_once_then_ends` for `Option`, this one for
// `Result`), not a second, competing definition of the same claim.
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::IterYieldsValueOnceThenEnds",
        "creusot",
        "ensures",
        || ITER_YIELDS_OK_VALUE_ONCE_THEN_ENDS_SRC,
    )
}
