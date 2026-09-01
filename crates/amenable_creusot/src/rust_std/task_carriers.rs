/// The `#[cfg(creusot)]` imports this file needs, consolidated into one
/// gate on this `mod` instead of one per item -- see `stoplight::mirror`'s
/// own doc comment for the general rationale. Every name is re-exported:
/// the `harness! { .. }` blocks below need all of them, unqualified, at
/// this file's own top level.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
    pub(super) use std::future::{Pending, PollFn, Ready};
    pub(super) use std::sync::atomic::Ordering as AtomicOrdering;
    pub(super) use std::task::Waker;
    pub(super) use std::task::{Context, Poll};
}
#[cfg(creusot)]
use mirror::{
    AtomicOrdering, Context, Pending, Poll, PollFn, Ready, Waker, check, ensures, extern_spec,
    logic, requires, trusted,
};

// `creusot-std` 0.11.0 ships no `core::future` / `core::task` contract
// surface at all (checked directly against the installed sources), so
// `Future::poll`, `Context`, `Waker`, and the std `Pending` / `Ready` /
// `PollFn` carriers are outside Creusot's concrete reasoning boundary
// today. These harnesses therefore keep the same representative
// observations as Kani while making that trusted boundary explicit.
//
// `Poll<T>` itself is the one exception here: it is just a plain enum, so
// Creusot can check the Ready/Pending disjointness law directly even though
// the surrounding task ecosystem stays outside its current std-contract
// surface.
amenable_derive::harness! {
    creusot, POLL_READY_AND_PENDING_ARE_DISJOINT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Poll<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn poll_ready_and_pending_are_disjoint_holds(
            value: i32,
            poll_result: (bool, bool, i32, bool, bool),
        ) -> bool {
            pearlite! {
                match poll_result {
                    (ready_is_ready, ready_is_pending, ready_value, pending_is_pending, pending_is_ready) =>
                        ready_is_ready
                            && !ready_is_pending
                            && ready_value == value
                            && pending_is_pending
                            && !pending_is_ready,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_POLL_READY_AND_PENDING_ARE_DISJOINT_SRC, {
        /// `Poll::Ready` and `Poll::Pending` are disjoint, and `Ready`
        /// round-trips its payload.
        #[requires(true)]
        #[ensures(poll_ready_and_pending_are_disjoint_holds(value, result))]
        fn verify_poll_ready_and_pending_are_disjoint(
            value: i32,
        ) -> (bool, bool, i32, bool, bool) {
            let ready = Poll::Ready(value);
            let ready_is_ready = match ready {
                Poll::Ready(_) => true,
                Poll::Pending => false,
            };
            let ready_is_pending = match ready {
                Poll::Ready(_) => false,
                Poll::Pending => true,
            };
            let ready_value = match ready {
                Poll::Ready(inner) => inner,
                Poll::Pending => value,
            };

            let pending: Poll<i32> = Poll::Pending;
            let pending_is_pending = match pending {
                Poll::Ready(_) => false,
                Poll::Pending => true,
            };
            let pending_is_ready = match pending {
                Poll::Ready(_) => true,
                Poll::Pending => false,
            };

            (
                ready_is_ready,
                ready_is_pending,
                ready_value,
                pending_is_pending,
                pending_is_ready,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CONTEXT_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC, {
        /// `Context::from_waker` exposes the same wake target through
        /// `Context::waker`.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_context_from_waker_exposes_the_same_waker() -> bool {
            use std::sync::Arc;
            use std::task::Wake;

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let cx = Context::from_waker(&waker);
            cx.waker().will_wake(&waker)
        }
    }
}

amenable_derive::harness! {
    creusot, PENDING_NEVER_RESOLVES_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Pending<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn pending_never_resolves_holds(pending_result: (Poll<i32>, Poll<i32>)) -> bool {
            pearlite! {
                match pending_result {
                    (first_poll, second_poll) =>
                        first_poll == Poll::Pending && second_poll == Poll::Pending,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PENDING_NEVER_RESOLVES_SRC, {
        /// `Pending` always reports `Poll::Pending` when polled,
        /// including repeated polls.
        #[trusted]
        #[requires(true)]
        #[ensures(pending_never_resolves_holds(result))]
        fn verify_pending_never_resolves() -> (Poll<i32>, Poll<i32>) {
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut: Pending<i32> = std::future::pending();
            let mut fut = pin!(fut);
            let first_poll = fut.as_mut().poll(&mut cx);
            let second_poll = fut.as_mut().poll(&mut cx);
            (first_poll, second_poll)
        }
    }
}

amenable_derive::harness! {
    creusot, READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Ready<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn ready_resolves_immediately_with_its_value_holds(
            value: i32,
            poll_result: Poll<i32>,
        ) -> bool {
            pearlite! { poll_result == Poll::Ready(value) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_READY_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC, {
        /// `Ready` resolves immediately with the value it was
        /// constructed from.
        #[trusted]
        #[requires(true)]
        #[ensures(ready_resolves_immediately_with_its_value_holds(value, result))]
        fn verify_ready_resolves_immediately_with_its_value(value: i32) -> Poll<i32> {
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut: Ready<i32> = std::future::ready(value);
            let mut fut = pin!(fut);
            fut.as_mut().poll(&mut cx)
        }
    }
}

amenable_derive::harness! {
    creusot, WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Waker>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn waker_wake_by_ref_invokes_the_wake_impl_holds(wake_count_result: usize) -> bool {
            pearlite! { wake_count_result == 1usize }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WAKER_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC, {
        /// `Waker::wake_by_ref` dispatches through to the wrapped
        /// `Wake` implementation exactly once.
        #[trusted]
        #[requires(true)]
        #[ensures(waker_wake_by_ref_invokes_the_wake_impl_holds(result))]
        fn verify_waker_wake_by_ref_invokes_the_wake_impl() -> usize {
            use std::sync::Arc;
            use std::task::Wake;

            struct CountingWake(std::sync::atomic::AtomicUsize);
            impl Wake for CountingWake {
                fn wake(self: Arc<Self>) {
                    self.0.fetch_add(1, AtomicOrdering::SeqCst);
                }

                fn wake_by_ref(self: &Arc<Self>) {
                    self.0.fetch_add(1, AtomicOrdering::SeqCst);
                }
            }

            let inner = Arc::new(CountingWake(std::sync::atomic::AtomicUsize::new(0)));
            let waker = Waker::from(inner.clone());
            waker.wake_by_ref();
            inner.0.load(AtomicOrdering::SeqCst)
        }
    }
}

amenable_derive::harness! {
    creusot, RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<AtomicOrdering>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn relaxed_ordering_still_makes_a_store_observable_holds(
            value: i32,
            load_result: (i32, i32),
        ) -> bool {
            pearlite! {
                match load_result {
                    (before, after) => before == 0i32 && after == value,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC, {
        /// A `Relaxed` store is still observable through a later
        /// `Relaxed` load on the same atomic in the same thread.
        #[trusted]
        #[requires(true)]
        #[ensures(relaxed_ordering_still_makes_a_store_observable_holds(value, result))]
        fn verify_relaxed_ordering_still_makes_a_store_observable(
            value: i32,
        ) -> (i32, i32) {
            let atomic = std::sync::atomic::AtomicI32::new(0);
            let before = atomic.load(AtomicOrdering::Relaxed);
            atomic.store(value, AtomicOrdering::Relaxed);
            let after = atomic.load(AtomicOrdering::Relaxed);
            (before, after)
        }
    }
}

amenable_derive::harness! {
    creusot, POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut
        /// Context<'_>) -> Poll<i32>>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn poll_fn_dispatches_through_to_its_closure_holds(
            value: i32,
            poll_fn_result: (Poll<i32>, bool),
        ) -> bool {
            pearlite! {
                match poll_fn_result {
                    (poll_result, called) => poll_result == Poll::Ready(value) && called,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_POLL_FN_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC, {
        /// `poll_fn` turns a poll-shaped function into a `Future`, and
        /// polling that future dispatches straight through to the
        /// wrapped function result.
        #[trusted]
        #[requires(true)]
        #[ensures(poll_fn_dispatches_through_to_its_closure_holds(value, result))]
        fn verify_poll_fn_dispatches_through_to_its_closure(
            value: i32,
        ) -> (Poll<i32>, bool) {
            use std::cell::Cell;
            use std::pin::pin;
            use std::sync::Arc;
            use std::task::{Wake, Waker};

            struct NoopWake;
            impl Wake for NoopWake {
                fn wake(self: Arc<Self>) {}
            }

            let called = Cell::new(false);
            let waker = Waker::from(Arc::new(NoopWake));
            let mut cx = Context::from_waker(&waker);
            let fut: PollFn<_> = std::future::poll_fn(|_cx| {
                called.set(true);
                Poll::Ready(value)
            });
            let mut fut = pin!(fut);
            let poll_result = fut.as_mut().poll(&mut cx);
            (poll_result, called.get())
        }
    }
}
