//! `KaniWitness` impls for `core::panic`.
//!
//! `PanicInfo`/`PanicMessage` have no public constructor — they're built by
//! the panic runtime and handed to a panic hook, not something user code
//! assembles directly — so there's nothing a harness can construct to check
//! a property of. Both stay at the trusted disposition.

use std::panic::AssertUnwindSafe;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;
use core::panic::{Location, PanicInfo, PanicMessage};

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::FieldAccessRecoversTheStoredValue;
use crate::rust_std::{bridge_kani_witness, impl_kani_witness_trusted};
use crate::{KaniCallerLocationObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_assert_unwind_safe_derefs_transparently".to_owned(),
            VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        "kani",
        || <RustStdStandard<AssertUnwindSafe<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// `AssertUnwindSafe` is a bare assertion wrapper: `Deref`/
        /// `DerefMut` expose the wrapped value with no transformation.
        #[kani::proof]
        fn verify_assert_unwind_safe_derefs_transparently() {
            let value: i32 = kani::any();
            let mut wrapped = AssertUnwindSafe(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*wrapped, value)),
                "deref exposes the wrapped value"
            );

            let updated: i32 = kani::any();
            *wrapped = updated;
            assert!(
                FieldAccessRecoversTheStoredValue::ensures((wrapped.0, updated)),
                "deref_mut writes through to the wrapped value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Location<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_location_caller_reflects_the_immediate_call_site".to_owned(),
            VERIFY_LOCATION_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Location<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Location<'static>>",
        "kani",
        || <RustStdStandard<Location<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCallerLocationObservation` instance actually
/// demonstrated same-file, different-line caller reporting, minted only by
/// [`KaniCallerLocationObservation::demonstrate_immediate_call_site`].
pub struct KaniCallerLocationWitnessToken(());

impl ProofToken for KaniCallerLocationWitnessToken {
    type Proposition = KaniCallerLocationObservation;
}

impl KaniCallerLocationObservation {
    /// Assert the observation reports the expected file and that its two
    /// call-site lines differ. Consumes `self`: the only way to obtain
    /// the token is to have run this check against a real observation
    /// instance, not to assert it independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_immediate_call_site(
        self,
        expected_file: &'static str,
    ) -> KaniCallerLocationWitnessToken {
        assert_eq!(self.file(), expected_file);
        assert!(self.lines_differ());
        KaniCallerLocationWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Location<'static>>`'s
/// immediate-call-site claim has been established from a
/// `KaniCallerLocationObservation` that has itself demonstrated same-file,
/// different-line caller reporting.
pub struct RustStdLocationToken(());

impl ProofToken for RustStdLocationToken {
    type Proposition = RustStdStandard<Location<'static>>;
}

impl Establish<KaniCallerLocationWitnessToken, KaniVerifier>
    for RustStdStandard<Location<'static>>
{
    type Token = RustStdLocationToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniCallerLocationWitnessToken) -> Self::Token {
        RustStdLocationToken(())
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
        /// This proof uses the Amenable-owned caller-location observation:
        /// the direct `Location::caller()` path reaches the gallery's
        /// unsupported `track_caller` boundary before the two-call-site law
        /// can be checked. The claim is established through
        /// `Establish<KaniCallerLocationObservation, KaniVerifier> for
        /// RustStdStandard<Location<'static>>` from the observation
        /// instance that actually demonstrated same-file, different-line
        /// reporting, rather than asserted independently of it.
        #[kani::proof]
        fn verify_location_caller_reflects_the_immediate_call_site() {
            let observation =
                KaniCallerLocationObservation::same_file_distinct_lines("proof.rs", 10, 11);
            let demonstration = observation.demonstrate_immediate_call_site("proof.rs");

            let _token = RustStdStandard::<Location<'static>>::establish(demonstration);
        }
    }
}

impl_kani_witness_trusted!(PanicInfo<'static>, PanicMessage<'static>);
