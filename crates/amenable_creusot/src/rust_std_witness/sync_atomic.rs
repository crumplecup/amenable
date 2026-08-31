use super::CheckedProof;

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering as AtomicOrdering,
};

use crate::{
    CreusotVerifier, CreusotWitness, RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_HOLDS_SRC,
    VERIFY_ATOMIC_BOOL_LOAD_STORE_SRC, VERIFY_ATOMIC_I8_LOAD_STORE_SRC,
    VERIFY_ATOMIC_I16_LOAD_STORE_SRC, VERIFY_ATOMIC_I32_LOAD_STORE_SRC,
    VERIFY_ATOMIC_I64_LOAD_STORE_SRC, VERIFY_ATOMIC_ISIZE_LOAD_STORE_SRC,
    VERIFY_ATOMIC_PTR_LOAD_STORE_SRC, VERIFY_ATOMIC_U8_LOAD_STORE_SRC,
    VERIFY_ATOMIC_U16_LOAD_STORE_SRC, VERIFY_ATOMIC_U32_LOAD_STORE_SRC,
    VERIFY_ATOMIC_U64_LOAD_STORE_SRC, VERIFY_ATOMIC_USIZE_LOAD_STORE_SRC,
    VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC,
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

macro_rules! impl_creusot_atomic_checked_witness {
    ($ty:ty, $harness:literal, $claim:ident) => {
        impl CreusotWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = CheckedProof;

            fn proof() -> Self::ProofArtifact {
                CheckedProof::new(
                    $harness.to_string(),
                    $claim.to_string(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_creusot_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "creusot",
                || <RustStdStandard<$ty> as CreusotWitness>::proof().to_string(),
            )
        }
    };
}

impl_creusot_atomic_checked_witness!(
    AtomicBool,
    "verify_atomic_bool_load_store",
    VERIFY_ATOMIC_BOOL_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI8,
    "verify_atomic_i8_load_store",
    VERIFY_ATOMIC_I8_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI16,
    "verify_atomic_i16_load_store",
    VERIFY_ATOMIC_I16_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI32,
    "verify_atomic_i32_load_store",
    VERIFY_ATOMIC_I32_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicI64,
    "verify_atomic_i64_load_store",
    VERIFY_ATOMIC_I64_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicIsize,
    "verify_atomic_isize_load_store",
    VERIFY_ATOMIC_ISIZE_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU8,
    "verify_atomic_u8_load_store",
    VERIFY_ATOMIC_U8_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU16,
    "verify_atomic_u16_load_store",
    VERIFY_ATOMIC_U16_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU32,
    "verify_atomic_u32_load_store",
    VERIFY_ATOMIC_U32_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicU64,
    "verify_atomic_u64_load_store",
    VERIFY_ATOMIC_U64_LOAD_STORE_SRC
);
impl_creusot_atomic_checked_witness!(
    AtomicUsize,
    "verify_atomic_usize_load_store",
    VERIFY_ATOMIC_USIZE_LOAD_STORE_SRC
);

impl_creusot_atomic_checked_witness!(
    AtomicPtr<i32>,
    "verify_atomic_ptr_load_store",
    VERIFY_ATOMIC_PTR_LOAD_STORE_SRC
);

// Bare `Ordering`, matching `amenable_std::rust_std::sync_atomic`'s own
// registration exactly: unlike `std::cmp::Ordering`, this evidence string
// intentionally stays unqualified so the checklist row resolves to the
// atomic carrier rather than the comparison carrier.
impl CreusotWitness for RustStdStandard<AtomicOrdering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_relaxed_ordering_still_makes_a_store_observable".to_string(),
            VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<AtomicOrdering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ordering>",
        "creusot",
        || <RustStdStandard<AtomicOrdering> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// relaxed_ordering_still_makes_a_store_observable_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<AtomicOrdering> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ordering>",
        "creusot",
        "ensures",
        || <RustStdStandard<AtomicOrdering> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
