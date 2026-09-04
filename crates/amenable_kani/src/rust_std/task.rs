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

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};

impl KaniWitness for RustStdStandard<Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_context_from_waker_exposes_the_same_waker".to_owned(),
            VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Context<'static>>",
        "kani",
        || <RustStdStandard<Context<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` `Waker::will_wake` reports when the
/// two wakers really would wake the same task -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct WillWakeReportsTrue;

impl KaniWitness for WillWakeReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_context_from_waker_exposes_the_same_waker".to_owned(),
            VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(WillWakeReportsTrue);

kani_ensures!(
    WillWakeReportsTrue,
    "amenable_kani::WillWakeReportsTrue",
    bool,
    |will_wake| will_wake
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::WillWakeReportsTrue",
        "kani",
        || <WillWakeReportsTrue as KaniWitness>::proof().to_string(),
    )
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
                WillWakeReportsTrue::ensures(cx.waker().will_wake(&waker)),
                "Context::from_waker exposes the waker it was built from"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_ready_and_pending_are_disjoint".to_owned(),
            VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Poll<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Poll<i32>>",
        "kani",
        || <RustStdStandard<Poll<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Poll<i32>>,
    "amenable_std::rust_std::RustStdStandard<Poll<i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

/// A `bool` known to be the `true` `Poll::is_ready()` reports when the
/// poll is actually the `Ready` variant -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct PollIsReadyReportsTrue;

impl KaniWitness for PollIsReadyReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_ready_and_pending_are_disjoint".to_owned(),
            VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(PollIsReadyReportsTrue);

kani_ensures!(
    PollIsReadyReportsTrue,
    "amenable_kani::PollIsReadyReportsTrue",
    bool,
    |is_ready| is_ready
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PollIsReadyReportsTrue",
        "kani",
        || <PollIsReadyReportsTrue as KaniWitness>::proof().to_string(),
    )
}

/// The `.is_pending()` sibling of [`PollIsReadyReportsTrue`], same
/// reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct PollIsPendingReportsTrue;

impl KaniWitness for PollIsPendingReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_ready_and_pending_are_disjoint".to_owned(),
            VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(PollIsPendingReportsTrue);

kani_ensures!(
    PollIsPendingReportsTrue,
    "amenable_kani::PollIsPendingReportsTrue",
    bool,
    |is_pending| is_pending
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PollIsPendingReportsTrue",
        "kani",
        || <PollIsPendingReportsTrue as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC, {
        /// `Ready` and `Pending` are mutually exclusive, and `Ready`
        /// round-trips its value.
        #[kani::proof]
        fn verify_poll_ready_and_pending_are_disjoint() {
            let value: i32 = kani::any();
            let poll: Poll<i32> = Poll::Ready(value);
            assert!(
                PollIsReadyReportsTrue::ensures(poll.is_ready()),
                "Ready reports is_ready"
            );
            assert!(
                !PollIsPendingReportsTrue::ensures(poll.is_pending()),
                "Ready reports !is_pending"
            );
            match poll {
                Poll::Ready(inner) => assert!(
                    RustStdStandard::<Poll<i32>>::ensures((inner, value)),
                    "Ready round-trips its value"
                ),
                Poll::Pending => unreachable!("constructed as Ready"),
            }

            let poll: Poll<i32> = Poll::Pending;
            assert!(
                PollIsPendingReportsTrue::ensures(poll.is_pending()),
                "Pending reports is_pending"
            );
            assert!(
                !PollIsReadyReportsTrue::ensures(poll.is_ready()),
                "Pending reports !is_ready"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_waker_wake_by_ref_invokes_the_wake_impl".to_owned(),
            VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Waker>",
        "kani",
        || <RustStdStandard<Waker> as KaniWitness>::proof().to_string(),
    )
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
            assert!(
                RustStdStandard::<usize>::ensures((
                    inner.0.load(std::sync::atomic::Ordering::SeqCst),
                    1
                )),
                "wake_by_ref invokes the Wake impl exactly once"
            );
        }
    }
}

impl_kani_witness_trusted!(RawWaker, RawWakerVTable);
