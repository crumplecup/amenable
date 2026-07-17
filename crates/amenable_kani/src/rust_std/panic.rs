//! `KaniWitness` impls for `core::panic`.
//!
//! `PanicInfo`/`PanicMessage` have no public constructor — they're built by
//! the panic runtime and handed to a panic hook, not something user code
//! assembles directly — so there's nothing a harness can construct to check
//! a property of. Both stay at the trusted disposition.

use std::panic::AssertUnwindSafe;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;
use core::panic::{Location, PanicInfo, PanicMessage};

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_assert_unwind_safe_derefs_transparently",
            claim: VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<AssertUnwindSafe<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// `AssertUnwindSafe` is a bare assertion wrapper: `Deref`/
        /// `DerefMut` expose the wrapped value with no transformation.
        #[kani::proof]
        fn verify_assert_unwind_safe_derefs_transparently() {
            let value: i32 = kani::any();
            let mut wrapped = AssertUnwindSafe(value);
            assert_eq!(*wrapped, value, "deref exposes the wrapped value");

            let updated: i32 = kani::any();
            *wrapped = updated;
            assert_eq!(
                wrapped.0, updated,
                "deref_mut writes through to the wrapped value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Location<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_location_caller_reflects_the_immediate_call_site",
            claim: VERIFY_LOCATION_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Location<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Location<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Location<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_LOCATION_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC, {
        /// `#[track_caller]` threads through to the immediate call site,
        /// not a fixed location baked into the callee: two calls to the
        /// same `#[track_caller]` function from different lines report
        /// different `line()`s in the same `file()`. This is checked
        /// without hardcoding either line number, since both would be
        /// fragile to any edit of this harness.
        #[kani::proof]
        fn verify_location_caller_reflects_the_immediate_call_site() {
            #[track_caller]
            fn caller_location() -> &'static Location<'static> {
                Location::caller()
            }

            let a = caller_location();
            let b = caller_location();
            assert_eq!(a.file(), b.file(), "both calls originate in the same file");
            assert_ne!(
                a.line(),
                b.line(),
                "different call sites produce different lines"
            );
        }
    }
}

impl_kani_witness_trusted!(PanicInfo<'static>, PanicMessage<'static>);
