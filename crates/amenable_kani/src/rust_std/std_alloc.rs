//! `KaniWitness` impls for `std::alloc`.

use std::alloc::System;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<System> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_system_allocates_and_deallocates_a_layout".to_owned(),
            VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<System>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<System>",
        "kani",
        || <RustStdStandard<System> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<System>,
    "amenable_std::rust_std::RustStdStandard<System>",
    (i32, i32),
    |(boxed_value, value)| boxed_value == value
);

amenable_derive::harness! {
    kani, VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC, {
        /// `System` is the process's default global allocator (nothing
        /// in this crate overrides it via `#[global_allocator]`), so a
        /// `Box` allocation and drop is serviced by `System::alloc`/
        /// `dealloc` even though neither is named directly. Exercised
        /// this way rather than by calling `GlobalAlloc::alloc`/
        /// `dealloc` directly: both are `unsafe fn`, and this crate
        /// forbids `unsafe` in its own source (`#![forbid(unsafe_code)]`
        /// in `lib.rs`) — the `unsafe` inside `Box`'s own implementation
        /// doesn't count against that, since the forbid lint only
        /// checks code written in this crate. The assertion calls
        /// `RustStdStandard::<System>::ensures` directly rather than
        /// restating the comparison.
        #[kani::proof]
        fn verify_system_allocates_and_deallocates_a_layout() {
            let value: i32 = kani::any();
            let boxed = Box::new(value);
            assert!(
                RustStdStandard::<System>::ensures((*boxed, value)),
                "a Box allocation serviced by the default System allocator round-trips its value"
            );
            drop(boxed);
        }
    }
}
