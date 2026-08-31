use super::CheckedProof;

use std::cmp::Reverse;

use crate::{
    CreusotVerifier, CreusotWitness, ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_HOLDS_SRC,
    REVERSE_INVERTS_COMPARISON_HOLDS_SRC, VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
    VERIFY_REVERSE_INVERTS_COMPARISON_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::RustStdStandard;

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
// Fully qualified, matching `amenable_kani::rust_std::cmp` and
// `amenable_std::rust_std::cmp`'s own registration exactly: there's also
// a `core::sync::atomic::Ordering`, so the evidence string must say
// `std::cmp::Ordering`, not the bare name, or alias resolution won't
// match this proof to the checklist row.
impl CreusotWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ordering_reverse_swaps_less_and_greater".to_string(),
            VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        "creusot",
        || <RustStdStandard<std::cmp::Ordering> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn ordering_reverse_swaps_less_and_greater_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<std::cmp::Ordering> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        "creusot",
        "ensures",
        || <RustStdStandard<std::cmp::Ordering> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

// Bare `Reverse<i32>`, matching `amenable_std::rust_std::cmp`'s own
// registration exactly (`register_rust_std_standard_evidence!(std::cmp::
// Ordering, Reverse<i32>)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Reverse<i32>>`).
impl CreusotWitness for RustStdStandard<Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_reverse_inverts_comparison".to_string(),
            VERIFY_REVERSE_INVERTS_COMPARISON_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Reverse<i32>>);

/// Returns `amenable_creusot::REVERSE_INVERTS_COMPARISON_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn reverse_inverts_comparison_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Reverse<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        REVERSE_INVERTS_COMPARISON_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Reverse<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Reverse<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Reverse<i32>>",
        "creusot",
        || <RustStdStandard<Reverse<i32>> as CreusotWitness>::proof().to_string(),
    )
}
