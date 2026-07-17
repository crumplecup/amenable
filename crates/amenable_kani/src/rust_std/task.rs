//! `KaniWitness` impls for `core::task`.
//!
//! `RawWaker`/`RawWakerVTable` are the low-level building blocks a `Waker`
//! is assembled from, and meaningfully exercising them requires `unsafe fn`
//! vtable entries — forbidden in this crate (`#![forbid(unsafe_code)]` in
//! `lib.rs`, which blocks even an `unsafe fn` declaration with an empty
//! body, confirmed empirically). `Waker` itself gets a real proof anyway,
//! via the safe `Wake` trait bridge (`Waker::from(Arc<impl Wake>)`) rather
//! than `RawWaker` directly.

use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_context_from_waker_exposes_the_same_waker",
            claim: VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Context<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<Context<'static>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC, {
        /// `Context::from_waker` just bundles the `&Waker` it's given;
        /// `.waker()` hands back a reference that would wake the same
        /// task as the original, checked via `Waker::will_wake` rather
        /// than pointer equality (the documented way to compare wakers).
        #[kani::proof]
        fn verify_context_from_waker_exposes_the_same_waker() {
            struct NoopWake;
            impl std::task::Wake for NoopWake {
                fn wake(self: std::sync::Arc<Self>) {}
            }

            let waker = Waker::from(std::sync::Arc::new(NoopWake));
            let cx = Context::from_waker(&waker);
            assert!(
                cx.waker().will_wake(&waker),
                "Context::from_waker exposes the waker it was built from"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_poll_ready_and_pending_are_disjoint",
            claim: VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Poll<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Poll<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Poll<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC, {
        /// `Ready` and `Pending` are mutually exclusive, and `Ready`
        /// round-trips its value.
        #[kani::proof]
        fn verify_poll_ready_and_pending_are_disjoint() {
            let value: i32 = kani::any();
            let poll: Poll<i32> = Poll::Ready(value);
            assert!(poll.is_ready(), "Ready reports is_ready");
            assert!(!poll.is_pending(), "Ready reports !is_pending");
            match poll {
                Poll::Ready(inner) => assert_eq!(inner, value, "Ready round-trips its value"),
                Poll::Pending => unreachable!("constructed as Ready"),
            }

            let poll: Poll<i32> = Poll::Pending;
            assert!(poll.is_pending(), "Pending reports is_pending");
            assert!(!poll.is_ready(), "Pending reports !is_ready");
        }
    }
}

impl KaniWitness for RustStdStandard<Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_waker_wake_by_ref_invokes_the_wake_impl",
            claim: VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Waker>",
        verifier: "kani",
        describe: || <RustStdStandard<Waker> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC, {
        /// `Waker::from(Arc<impl Wake>)` dispatches through to the
        /// `Wake` impl: calling `wake_by_ref` on the resulting `Waker`
        /// invokes the wrapped type's `wake_by_ref` exactly once,
        /// observed through a shared counter.
        #[kani::proof]
        fn verify_waker_wake_by_ref_invokes_the_wake_impl() {
            struct CountingWake(std::sync::atomic::AtomicUsize);
            impl std::task::Wake for CountingWake {
                fn wake(self: std::sync::Arc<Self>) {
                    self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                fn wake_by_ref(self: &std::sync::Arc<Self>) {
                    self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }

            let inner = std::sync::Arc::new(CountingWake(std::sync::atomic::AtomicUsize::new(0)));
            let waker = Waker::from(inner.clone());
            waker.wake_by_ref();
            assert_eq!(
                inner.0.load(std::sync::atomic::Ordering::SeqCst),
                1,
                "wake_by_ref invokes the Wake impl exactly once"
            );
        }
    }
}

impl_kani_witness_trusted!(RawWaker, RawWakerVTable);
