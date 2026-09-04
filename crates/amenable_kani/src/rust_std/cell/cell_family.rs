//! `Cell<i32/u32/usize>`'s `KaniWitness` impls and the shared
//! get/set/replace/take round-trip harness (`Cell<u32>`/`Cell<usize>` reuse
//! `Cell<i32>`'s harness at a different scalar width).

use std::cell::Cell;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Cell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cell_get_set_replace_take_round_trip".to_owned(),
            VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cell<i32>>",
        "kani",
        || <RustStdStandard<Cell<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Cell<i32>>,
    "amenable_std::rust_std::RustStdStandard<Cell<i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC, {
        /// `Cell`'s whole interface is get/set-by-value: `new` stores the
        /// initial value, `set` overwrites it, `replace` overwrites it and
        /// hands back the old value, and `take` (needs `T: Default`) does
        /// the same against `T::default()`. The first, second, and last
        /// assertions call `RustStdStandard::<Cell<i32>>::ensures`
        /// directly rather than restating the comparison.
        #[kani::proof]
        fn verify_cell_get_set_replace_take_round_trip() {
            let initial: i32 = kani::any();
            let cell = Cell::new(initial);
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), initial)),
                "new stores the initial value"
            );

            let updated: i32 = kani::any();
            cell.set(updated);
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), updated)),
                "set overwrites the stored value"
            );

            let replacement: i32 = kani::any();
            let old = cell.replace(replacement);
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((old, updated)),
                "replace returns the previous value"
            );
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), replacement)),
                "replace stores the new value"
            );

            let taken = cell.take();
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((taken, replacement)),
                "take returns the stored value"
            );
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), i32::default())),
                "take leaves the default value behind"
            );
        }
    }
}

/// [`RustStdStandard<Cell<u32>>`] reuses [`RustStdStandard<Cell<i32>>`]'s
/// harness rather than adding a new Kani proof — every proof across this
/// crate that counts drops via a `Cell<u32>` witness relies on exactly
/// the get/set round-trip `verify_cell_get_set_replace_take_round_trip`
/// already checks for `i32`, just at a different scalar width.
impl KaniWitness for RustStdStandard<Cell<u32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cell_get_set_replace_take_round_trip".to_owned(),
            VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cell<u32>>);

kani_ensures!(
    RustStdStandard<Cell<u32>>,
    "amenable_std::rust_std::RustStdStandard<Cell<u32>>",
    (u32, u32),
    |(actual, expected)| actual == expected
);

/// [`RustStdStandard<Cell<usize>>`] reuses the same harness for the same
/// reason [`RustStdStandard<Cell<u32>>`] does — a call-counter witness
/// relies on the identical get/set round-trip, at `usize` width.
impl KaniWitness for RustStdStandard<Cell<usize>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cell_get_set_replace_take_round_trip".to_owned(),
            VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cell<usize>>);

kani_ensures!(
    RustStdStandard<Cell<usize>>,
    "amenable_std::rust_std::RustStdStandard<Cell<usize>>",
    (usize, usize),
    |(actual, expected)| actual == expected
);
