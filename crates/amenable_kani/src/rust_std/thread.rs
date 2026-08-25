//! `KaniWitness` impls for `std::thread`.
//!
//! Kani does not model concurrency (confirmed against its own documented
//! limitations) — it verifies a single sequential instruction stream, not
//! multiple concurrently scheduled ones. Every type here whose only
//! observable behavior requires actually spawning a thread (`Builder`,
//! `JoinHandle`, `Scope`, `ScopedJoinHandle`) or exploiting single-thread TLS
//! teardown ordering (`AccessError`) has no claim Kani could check, so those
//! stay "trusted." `LocalKey` still has a direct single-thread harness; the
//! `Thread` and `ThreadId` laws are proved through an Amenable-owned
//! observation instead, because the direct `std::thread::current()` path
//! reaches the gallery's unsupported `pthread_key_create` boundary first.

use std::thread::{
    AccessError, Builder, JoinHandle, LocalKey, Scope, ScopedJoinHandle, Thread, ThreadId,
};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};
use crate::{KaniCurrentThreadObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<LocalKey<std::cell::Cell<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_local_key_with_reads_the_initialized_value".to_owned(),
            claim: VERIFY_LOCAL_KEY_WITH_READS_THE_INITIALIZED_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<LocalKey<std::cell::Cell<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LocalKey<std::cell::Cell<i32>>>",
        "kani",
        || {
            <RustStdStandard<LocalKey<std::cell::Cell<i32>>> as KaniWitness>::proof().to_string()
        },
    )
}

amenable_derive::harness! {
    kani, VERIFY_LOCAL_KEY_WITH_READS_THE_INITIALIZED_VALUE_SRC, {
        /// `.with()` gives access to the lazily-initialized value, and a
        /// mutation through it is visible on a later `.with()` call —
        /// checked within a single thread, since `LocalKey`'s
        /// per-thread isolation itself isn't something Kani can verify
        /// without spawning a second thread.
        #[kani::proof]
        fn verify_local_key_with_reads_the_initialized_value() {
            thread_local! {
                static COUNTER: std::cell::Cell<i32> = std::cell::Cell::new(5);
            }

            assert_eq!(COUNTER.with(|c| c.get()), 5);
            COUNTER.with(|c| c.set(42));
            assert_eq!(COUNTER.with(|c| c.get()), 42);
        }
    }
}

impl KaniWitness for RustStdStandard<Thread> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_thread_current_is_stable_across_repeated_calls".to_owned(),
            claim: VERIFY_THREAD_CURRENT_IS_STABLE_ACROSS_REPEATED_CALLS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Thread>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Thread>",
        "kani",
        || <RustStdStandard<Thread> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCurrentThreadObservation` instance actually
/// demonstrated current-thread handle stability, minted only by
/// [`KaniCurrentThreadObservation::demonstrate_handle_stability`].
pub struct KaniCurrentThreadHandleWitnessToken(());

impl ProofToken for KaniCurrentThreadHandleWitnessToken {
    type Proposition = KaniCurrentThreadObservation;
}

impl KaniCurrentThreadObservation {
    /// Assert the current thread handle stays stable across repeated
    /// queries. Consumes `self`: the only way to obtain the token is to
    /// have run this check against a real observation instance, not to
    /// assert it independently.
    #[must_use]
    pub fn demonstrate_handle_stability(self) -> KaniCurrentThreadHandleWitnessToken {
        assert!(
            self.handle_is_stable(),
            "the current thread handle should stay stable across repeated queries"
        );
        KaniCurrentThreadHandleWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Thread>`'s repeated-current-query
/// claim has been established from a `KaniCurrentThreadObservation` that has
/// itself demonstrated current-thread handle stability.
pub struct RustStdThreadToken(());

impl ProofToken for RustStdThreadToken {
    type Proposition = RustStdStandard<Thread>;
}

impl Establish<KaniCurrentThreadHandleWitnessToken, KaniVerifier> for RustStdStandard<Thread> {
    type Token = RustStdThreadToken;

    fn establish(_credential: KaniCurrentThreadHandleWitnessToken) -> Self::Token {
        RustStdThreadToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_THREAD_CURRENT_IS_STABLE_ACROSS_REPEATED_CALLS_SRC, {
        /// Repeated queries for the already-running current thread identify
        /// the same thread handle.
        /// This proof uses the Amenable-owned current-thread observation:
        /// the direct `std::thread::current()` path reaches the gallery's
        /// unsupported `pthread_key_create` TLS boundary before the small
        /// stability claim can be checked. The claim is established through
        /// `Establish<KaniCurrentThreadObservation, KaniVerifier> for
        /// RustStdStandard<Thread>` from the observation instance that
        /// actually demonstrated handle stability, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_thread_current_is_stable_across_repeated_calls() {
            let observation = crate::KaniCurrentThreadObservation::current();
            let demonstration = observation.demonstrate_handle_stability();

            let _token = RustStdStandard::<Thread>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<ThreadId> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_thread_id_is_stable_across_repeated_calls".to_owned(),
            claim: VERIFY_THREAD_ID_IS_STABLE_ACROSS_REPEATED_CALLS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ThreadId>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ThreadId>",
        "kani",
        || <RustStdStandard<ThreadId> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCurrentThreadObservation` instance actually
/// demonstrated current-thread id stability, minted only by
/// [`KaniCurrentThreadObservation::demonstrate_id_stability`].
pub struct KaniCurrentThreadIdWitnessToken(());

impl ProofToken for KaniCurrentThreadIdWitnessToken {
    type Proposition = KaniCurrentThreadObservation;
}

impl KaniCurrentThreadObservation {
    /// Assert the current thread id stays stable across repeated
    /// queries. Consumes `self` for the same reason
    /// [`KaniCurrentThreadObservation::demonstrate_handle_stability`]
    /// does.
    #[must_use]
    pub fn demonstrate_id_stability(self) -> KaniCurrentThreadIdWitnessToken {
        assert!(
            self.id_is_stable(),
            "the current thread id should stay stable across repeated queries"
        );
        KaniCurrentThreadIdWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ThreadId>`'s repeated-current-id
/// claim has been established from a `KaniCurrentThreadObservation` that has
/// itself demonstrated current-thread id stability.
pub struct RustStdThreadIdToken(());

impl ProofToken for RustStdThreadIdToken {
    type Proposition = RustStdStandard<ThreadId>;
}

impl Establish<KaniCurrentThreadIdWitnessToken, KaniVerifier> for RustStdStandard<ThreadId> {
    type Token = RustStdThreadIdToken;

    fn establish(_credential: KaniCurrentThreadIdWitnessToken) -> Self::Token {
        RustStdThreadIdToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_THREAD_ID_IS_STABLE_ACROSS_REPEATED_CALLS_SRC, {
        /// Repeated queries for the already-running current thread id stay
        /// stable.
        /// This proof uses the same Amenable-owned current-thread
        /// observation as `Thread`: the direct `std::thread::current().id()`
        /// path reaches the gallery's unsupported `pthread_key_create` TLS
        /// boundary before the claim can be checked. The claim is
        /// established through `Establish<KaniCurrentThreadObservation,
        /// KaniVerifier> for RustStdStandard<ThreadId>` from the observation
        /// instance that actually demonstrated id stability, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_thread_id_is_stable_across_repeated_calls() {
            let observation = crate::KaniCurrentThreadObservation::current();
            let demonstration = observation.demonstrate_id_stability();

            let _token = RustStdStandard::<ThreadId>::establish(demonstration);
        }
    }
}

impl_kani_witness_trusted!(
    AccessError,
    Builder,
    JoinHandle<i32>,
    Scope<'static, 'static>,
    ScopedJoinHandle<'static, i32>,
);
