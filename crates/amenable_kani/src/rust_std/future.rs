//! `KaniWitness` impls for `core::future`.
//!
//! Every harness polls its future directly via `Future::poll`, pinned
//! locally with `std::pin::pin!` (no heap allocation needed) and driven by
//! a real, safe `Waker` built through the `Wake` trait bridge (the same
//! pattern established in `rust_std::task`'s `Waker` proof) — not an async
//! executor, since none is available or needed for a single poll. `pin!`/
//! `Wake`/`Waker`/`Arc` are imported locally inside each harness rather
//! than at module level: they're only ever referenced inside
//! `#[cfg(kani)]`-gated code, which this machine's plain `cargo check`
//! never compiles, so a top-level import would show as genuinely unused
//! here (confirmed empirically in earlier batches).

use std::future::{Pending, PollFn, Ready};
use std::task::{Context, Poll};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Pending<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_pending_never_resolves".to_owned(),
            VERIFY_PENDING_NEVER_RESOLVES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Pending<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        "kani",
        || <RustStdStandard<Pending<i32>> as KaniWitness>::proof().to_string(),
    )
}

/// A `(actual, expected)` pair of `Future::poll` results known to
/// agree: distinct from `RustStdStandard<Poll<i32>>`'s own
/// `Ensures<KaniVerifier>` slot (`rust_std::task`, which compares the
/// unwrapped `i32` inside a known-`Ready` result), since that slot was
/// already occupied -- this compares the whole `Poll<i32>` value
/// itself, `Pending` included, per the associated-type-uniqueness rule
/// (see `VecLengthTracksPushesAndPops`'s doc comment for the same
/// reasoning).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct PollResultMatchesExpected;

impl KaniWitness for PollResultMatchesExpected {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_pending_never_resolves".to_owned(),
            VERIFY_PENDING_NEVER_RESOLVES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(PollResultMatchesExpected);

kani_ensures!(
    PollResultMatchesExpected,
    "amenable_kani::PollResultMatchesExpected",
    (Poll<i32>, Poll<i32>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PollResultMatchesExpected",
        "kani",
        || <PollResultMatchesExpected as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` a callback's own tracking flag
/// reports once the callback has actually run -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim: the bound is the identity function, since the claim
/// itself *is* "this boolean is true."
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct CallbackWasInvoked;

impl KaniWitness for CallbackWasInvoked {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_fn_dispatches_through_to_its_closure".to_owned(),
            VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(CallbackWasInvoked);

kani_ensures!(
    CallbackWasInvoked,
    "amenable_kani::CallbackWasInvoked",
    bool,
    |invoked| invoked
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CallbackWasInvoked",
        "kani",
        || <CallbackWasInvoked as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PENDING_NEVER_RESOLVES_SRC, {
        /// `Pending` always reports `Poll::Pending` when polled.
        #[kani::proof]
        fn verify_pending_never_resolves() {
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut = std::future::pending::<i32>();
            let mut fut = pin!(fut);
            assert!(
                PollResultMatchesExpected::ensures((fut.as_mut().poll(&mut cx), Poll::Pending)),
                "the first poll reports Poll::Pending"
            );
            assert!(
                PollResultMatchesExpected::ensures((fut.as_mut().poll(&mut cx), Poll::Pending)),
                "a repeated poll still reports Poll::Pending"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Ready<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ready_resolves_immediately_with_its_value".to_owned(),
            VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Ready<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        "kani",
        || <RustStdStandard<Ready<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC, {
        /// `Ready`'s first poll resolves immediately with the value it
        /// was constructed from — the opposite of `Pending`.
        #[kani::proof]
        fn verify_ready_resolves_immediately_with_its_value() {
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let value: i32 = kani::any();
            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut = std::future::ready(value);
            let mut fut = pin!(fut);
            assert!(
                PollResultMatchesExpected::ensures((
                    fut.as_mut().poll(&mut cx),
                    Poll::Ready(value)
                )),
                "ready resolves immediately with its value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poll_fn_dispatches_through_to_its_closure".to_owned(),
            VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        "kani",
        || <RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC, {
        /// `poll_fn` turns a plain `poll`-shaped function into a
        /// `Future`: polling it calls straight through to the wrapped
        /// function.
        #[kani::proof]
        fn verify_poll_fn_dispatches_through_to_its_closure() {
            use std::cell::Cell;
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let value: i32 = kani::any();
            let called = Cell::new(false);
            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut = std::future::poll_fn(|_cx| {
                called.set(true);
                Poll::Ready(value)
            });
            let mut fut = pin!(fut);
            assert!(
                PollResultMatchesExpected::ensures((
                    fut.as_mut().poll(&mut cx),
                    Poll::Ready(value)
                )),
                "poll_fn preserves the wrapped closure result"
            );
            assert!(
                CallbackWasInvoked::ensures(called.get()),
                "poll_fn invokes the wrapped closure"
            );
        }
    }
}
