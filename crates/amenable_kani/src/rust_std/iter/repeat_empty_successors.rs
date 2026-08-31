use std::iter::{RepeatN, RepeatWith, Successors};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
use crate::CheckedProof;
#[cfg(kani)]
use crate::FirstValueIsLessThanTheSecond;
use crate::KaniWitness;
#[cfg(kani)]
use crate::PeekRevealsTheStoredReference;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<std::iter::Repeat<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_repeat_yields_the_same_value_forever".to_owned(),
            VERIFY_REPEAT_YIELDS_THE_SAME_VALUE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Repeat<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
        "kani",
        || <RustStdStandard<std::iter::Repeat<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::iter::Repeat<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_REPEAT_YIELDS_THE_SAME_VALUE_FOREVER_SRC, {
        /// `repeat` yields the same value on every call, checked across
        /// three calls.
        #[kani::proof]
        fn verify_repeat_yields_the_same_value_forever() {
            let value: i32 = kani::any();
            let mut r = std::iter::repeat(value);
            assert!(RustStdStandard::<std::iter::Repeat<i32>>::ensures((r.next(), Some(value))));
            assert!(RustStdStandard::<std::iter::Repeat<i32>>::ensures((r.next(), Some(value))));
            assert!(
                RustStdStandard::<std::iter::Repeat<i32>>::ensures((r.next(), Some(value))),
                "repeat yields the same value on every call"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RepeatWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_repeat_with_calls_its_closure_once_per_item".to_owned(),
            VERIFY_REPEAT_WITH_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RepeatWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RepeatWith<fn() -> i32>>",
        "kani",
        || <RustStdStandard<RepeatWith<fn() -> i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_REPEAT_WITH_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC, {
        /// Unlike `LazyCell`, `RepeatWith` never caches: it calls its
        /// closure once per item, every time, observed through a shared
        /// counter across three calls.
        #[kani::proof]
        fn verify_repeat_with_calls_its_closure_once_per_item() {
            static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn produce() -> i32 {
                CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                0
            }

            let mut r = std::iter::repeat_with(produce as fn() -> i32);
            r.next();
            r.next();
            r.next();
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    CALLS.load(std::sync::atomic::Ordering::SeqCst),
                    3
                )),
                "repeat_with calls its closure once per item, never caching"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RepeatN<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_repeat_n_yields_the_value_exactly_n_times".to_owned(),
            VERIFY_REPEAT_N_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RepeatN<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RepeatN<i32>>",
        "kani",
        || <RustStdStandard<RepeatN<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<RepeatN<i32>>,
    "amenable_std::rust_std::RustStdStandard<RepeatN<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_REPEAT_N_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC, {
        /// `repeat_n(value, n)` yields `value` exactly `n` times, then
        /// stops — the bounded counterpart to `repeat`.
        #[kani::proof]
        fn verify_repeat_n_yields_the_value_exactly_n_times() {
            let value: i32 = kani::any();
            let mut r = std::iter::repeat_n(value, 2);
            assert!(RustStdStandard::<RepeatN<i32>>::ensures((r.next(), Some(value))));
            assert!(RustStdStandard::<RepeatN<i32>>::ensures((r.next(), Some(value))));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(r.next()),
                "repeat_n stops after exactly n items"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Empty<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_empty_yields_nothing".to_owned(),
            VERIFY_EMPTY_YIELDS_NOTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Empty<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Empty<i32>>",
        "kani",
        || <RustStdStandard<std::iter::Empty<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_EMPTY_YIELDS_NOTHING_SRC, {
        /// `empty` never yields a value. Both assertions call
        /// `IteratorYieldsNoneWhenExhausted::ensures` directly rather than
        /// restating the comparison -- see that type for why this is the
        /// one harness its registration reuses as a witness.
        #[kani::proof]
        fn verify_empty_yields_nothing() {
            let mut e: std::iter::Empty<i32> = std::iter::empty();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(e.next()),
                "empty never yields a value"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(e.next()),
                "empty continues yielding nothing on repeated next calls"
            );
        }
    }
}

/// An `Option<T>` known to be the `None` an exhausted iterator adapter
/// keeps returning on every subsequent `.next()` call.
///
/// Independently hand-written as `assert_eq!(iter.next(), None, ...)` at
/// dozens of real proof sites across nearly every iterator adapter this
/// crate proves, spanning this file plus `alloc_collections`, `alloc_vec`,
/// `array`, `option_result`, `path`, `slice`, and `str` -- the identical
/// claim regardless of the adapter's item type. A single contract type
/// generic over the item, rather than one registration per concrete
/// adapter, resolves every site at once.
///
/// Unlike every other named bound in this crate, this one needs a
/// hand-written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros: both macros emit a
/// non-generic `impl Trait for $ty`, which can't carry the `impl<T>`
/// generic parameter list this type's single, item-type-agnostic
/// registration depends on. Call sites never construct an instance --
/// `ensures` is a static associated function, resolved by unifying
/// `Self::Input = Option<T>` against the argument's type -- so this type
/// carries no fields beyond the `PhantomData<T>` Rust requires to use `T`
/// at all, and every call site writes the bare type name with no
/// turbofish, letting inference pick `T`.
pub struct IteratorYieldsNoneWhenExhausted<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for IteratorYieldsNoneWhenExhausted<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IteratorYieldsNoneWhenExhausted<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for IteratorYieldsNoneWhenExhausted<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_empty_yields_nothing".to_owned(),
            VERIFY_EMPTY_YIELDS_NOTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for IteratorYieldsNoneWhenExhausted<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T> amenable_core::Ensures<crate::KaniVerifier> for IteratorYieldsNoneWhenExhausted<T> {
    type Input = Option<T>;
    type Bound = bool;

    fn ensures(input: Option<T>) -> bool {
        input.is_none()
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::IteratorYieldsNoneWhenExhausted",
        "kani",
        "ensures",
        || stringify!(input.is_none()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IteratorYieldsNoneWhenExhausted",
        "kani",
        || <IteratorYieldsNoneWhenExhausted<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_successors_generates_from_the_previous_item".to_owned(),
            VERIFY_SUCCESSORS_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>",
        "kani",
        || <RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SUCCESSORS_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC, {
        /// `successors` yields the seed first, then computes each next
        /// item from the previous one via its closure.
        #[kani::proof]
        fn verify_successors_generates_from_the_previous_item() {
            fn next_step(x: &i32) -> Option<i32> {
                if *x < 100 { Some(x + 1) } else { None }
            }
            let seed: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((seed, 100)));
            let mut s = std::iter::successors(Some(seed), next_step);
            assert!(
                RustStdStandard::<Successors<i32, fn(&i32) -> Option<i32>>>::ensures((
                    s.next(),
                    Some(seed)
                )),
                "successors yields the seed first"
            );
            assert!(
                RustStdStandard::<Successors<i32, fn(&i32) -> Option<i32>>>::ensures((
                    s.next(),
                    Some(seed + 1)
                )),
                "successors computes the next item from the previous one"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_fn_yields_until_the_closure_returns_none".to_owned(),
            VERIFY_FROM_FN_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
        "kani",
        || <RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_FROM_FN_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC, {
        /// `from_fn` yields whatever its closure produces, and stops the
        /// moment the closure returns `None` — a shared counter drives a
        /// deterministic two-item-then-stop sequence, since the closure
        /// itself is a bare `fn` and can't capture state directly.
        #[kani::proof]
        fn verify_from_fn_yields_until_the_closure_returns_none() {
            static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn produce() -> Option<i32> {
                let n = CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if n < 2 { Some(n as i32) } else { None }
            }

            let mut f = std::iter::from_fn(produce as fn() -> Option<i32>);
            assert!(RustStdStandard::<std::iter::FromFn<fn() -> Option<i32>>>::ensures((
                f.next(),
                Some(0)
            )));
            assert!(RustStdStandard::<std::iter::FromFn<fn() -> Option<i32>>>::ensures((
                f.next(),
                Some(1)
            )));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(f.next()),
                "from_fn stops once its closure returns None"
            );
        }
    }
}
