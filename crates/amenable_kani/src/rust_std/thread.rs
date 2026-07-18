//! `KaniWitness` impls for `std::thread`.
//!
//! Kani does not model concurrency (confirmed against its own
//! documented limitations) — it verifies a single sequential
//! instruction stream, not multiple concurrently scheduled ones. Every
//! type here whose only observable behavior requires actually spawning
//! a thread (`Builder`, `JoinHandle`, `Scope`, `ScopedJoinHandle`) or
//! exploiting single-thread TLS teardown ordering (`AccessError`) has
//! no claim Kani could check, so those stay "trusted." `LocalKey`,
//! `Thread`, and `ThreadId` all have real, single-threaded-observable
//! behavior — accessing a `thread_local!` value, or querying the
//! *current* (already-running) thread's own handle — so those get real
//! harnesses.

use std::thread::{
    AccessError, Builder, JoinHandle, LocalKey, Scope, ScopedJoinHandle, Thread, ThreadId,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<LocalKey<std::cell::Cell<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_local_key_with_reads_the_initialized_value",
            claim: VERIFY_LOCAL_KEY_WITH_READS_THE_INITIALIZED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<LocalKey<std::cell::Cell<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LocalKey<std::cell::Cell<i32>>>",
        verifier: "kani",
        describe: || {
            <RustStdStandard<LocalKey<std::cell::Cell<i32>>> as KaniWitness>::proof().to_string()
        },
    }
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
            harness: "verify_thread_current_is_stable_across_repeated_calls",
            claim: VERIFY_THREAD_CURRENT_IS_STABLE_ACROSS_REPEATED_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Thread>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Thread>",
        verifier: "kani",
        describe: || <RustStdStandard<Thread> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_THREAD_CURRENT_IS_STABLE_ACROSS_REPEATED_CALLS_SRC, {
        /// `std::thread::current()` doesn't spawn anything — it just
        /// hands back a handle to the thread already running the
        /// harness, so two calls report the same name.
        #[kani::proof]
        fn verify_thread_current_is_stable_across_repeated_calls() {
            let first = std::thread::current();
            let second = std::thread::current();
            assert_eq!(first.name(), second.name());
        }
    }
}

impl KaniWitness for RustStdStandard<ThreadId> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_thread_id_is_stable_across_repeated_calls",
            claim: VERIFY_THREAD_ID_IS_STABLE_ACROSS_REPEATED_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ThreadId>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ThreadId>",
        verifier: "kani",
        describe: || <RustStdStandard<ThreadId> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_THREAD_ID_IS_STABLE_ACROSS_REPEATED_CALLS_SRC, {
        /// The current thread's id, queried twice without spawning
        /// anything in between, compares equal both ways.
        #[kani::proof]
        fn verify_thread_id_is_stable_across_repeated_calls() {
            let first = std::thread::current().id();
            let second = std::thread::current().id();
            assert_eq!(first, second);
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
