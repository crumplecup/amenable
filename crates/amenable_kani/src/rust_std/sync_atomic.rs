//! `KaniWitness` impls for `core::sync::atomic`.
//!
//! Each `Atomic*` type is written out literally rather than generated
//! through a wrapping `macro_rules!`, for the same reason `num.rs`'s
//! `NonZero<T>` instantiations are: `amenable_derive::harness!` captures a
//! harness's verbatim source via the group's span, and a span produced by a
//! `macro_rules!` expansion resolves back to the *defining* macro's on-disk
//! text — so a generator macro would capture its own placeholders,
//! unsubstituted, instead of each type's real harness.

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<AtomicBool> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_bool".to_owned(),
            VERIFY_ATOMIC_BOOL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicBool>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicBool>",
        "kani",
        || <RustStdStandard<AtomicBool> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_BOOL_SRC, {
        /// `AtomicBool::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering. Both
        /// assertions call `AtomicLoadReflectsTheLastWrite::ensures`
        /// directly rather than restating the comparison -- see that
        /// type for why this is the one harness its registration reuses
        /// as a witness.
        #[kani::proof]
        fn verify_atomic_bool() {
            let initial: bool = kani::any();
            let atomic = AtomicBool::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicBool::new sets the value observable via load"
            );

            let next: bool = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicBool::store overwrites the value observable via load"
            );
        }
    }
}

/// A `(loaded, expected)` pair known to agree: an atomic's `.load()`
/// reflects the value most recently established for it, by `new`,
/// `store`, `swap`, `compare_exchange`, or `fetch_add`.
///
/// Independently hand-written as `assert_eq!(atomic.load(Ordering::SeqCst),
/// expected, ...)` at 29 real sites across every `Atomic*` integer/bool
/// type in this file plus two call-counter sites elsewhere
/// (`rust_std::iter::verify_repeat_with_calls_its_closure_once_per_item`,
/// `rust_std::sync_lock::verify_once_runs_its_closure_exactly_once`) --
/// the identical claim regardless of the atomic's value type. Generic
/// over that value type rather than one registration per `Atomic*` type,
/// the same reasoning (and the same reason it needs a hand-written
/// `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `IteratorYieldsNoneWhenExhausted` in `rust_std::iter`.
///
/// `AtomicPtr<i32>`'s own `RustStdStandard<AtomicPtr<i32>>` carrier
/// already has a *different* `Ensures<KaniVerifier>` bound occupying its
/// slot (`.swap()` returning the previous value, just below) -- a second,
/// distinct claim about the same type needs its own type regardless, per
/// the associated-type-uniqueness rule, so `AtomicPtr`'s four `.load()`
/// sites could never have used a per-carrier registration even if every
/// other `Atomic*` type had.
pub struct AtomicLoadReflectsTheLastWrite<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for AtomicLoadReflectsTheLastWrite<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for AtomicLoadReflectsTheLastWrite<T> {
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

impl<T> KaniWitness for AtomicLoadReflectsTheLastWrite<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_bool".to_owned(),
            VERIFY_ATOMIC_BOOL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for AtomicLoadReflectsTheLastWrite<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for AtomicLoadReflectsTheLastWrite<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((loaded, expected): (T, T)) -> bool {
        loaded == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::AtomicLoadReflectsTheLastWrite",
        "kani",
        "ensures",
        || stringify!(loaded == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::AtomicLoadReflectsTheLastWrite",
        "kani",
        || <AtomicLoadReflectsTheLastWrite<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<AtomicI8> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i8".to_owned(),
            VERIFY_ATOMIC_I8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI8>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI8>",
        "kani",
        || <RustStdStandard<AtomicI8> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I8_SRC, {
        /// `AtomicI8::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i8() {
            let initial: i8 = kani::any();
            let atomic = AtomicI8::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI8::new sets the value observable via load"
            );

            let next: i8 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI8::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI16> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i16".to_owned(),
            VERIFY_ATOMIC_I16_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI16>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI16>",
        "kani",
        || <RustStdStandard<AtomicI16> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I16_SRC, {
        /// `AtomicI16::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i16() {
            let initial: i16 = kani::any();
            let atomic = AtomicI16::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI16::new sets the value observable via load"
            );

            let next: i16 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI16::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i32".to_owned(),
            VERIFY_ATOMIC_I32_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI32>",
        "kani",
        || <RustStdStandard<AtomicI32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I32_SRC, {
        /// `AtomicI32::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i32() {
            let initial: i32 = kani::any();
            let atomic = AtomicI32::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI32::new sets the value observable via load"
            );

            let next: i32 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI32::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI64> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i64".to_owned(),
            VERIFY_ATOMIC_I64_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI64>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI64>",
        "kani",
        || <RustStdStandard<AtomicI64> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I64_SRC, {
        /// `AtomicI64::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i64() {
            let initial: i64 = kani::any();
            let atomic = AtomicI64::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI64::new sets the value observable via load"
            );

            let next: i64 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI64::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicIsize> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_isize".to_owned(),
            VERIFY_ATOMIC_ISIZE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicIsize>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicIsize>",
        "kani",
        || <RustStdStandard<AtomicIsize> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_ISIZE_SRC, {
        /// `AtomicIsize::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_isize() {
            let initial: isize = kani::any();
            let atomic = AtomicIsize::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicIsize::new sets the value observable via load"
            );

            let next: isize = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicIsize::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU8> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u8".to_owned(),
            VERIFY_ATOMIC_U8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU8>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU8>",
        "kani",
        || <RustStdStandard<AtomicU8> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U8_SRC, {
        /// `AtomicU8::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u8() {
            let initial: u8 = kani::any();
            let atomic = AtomicU8::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU8::new sets the value observable via load"
            );

            let next: u8 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU8::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU16> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u16".to_owned(),
            VERIFY_ATOMIC_U16_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU16>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU16>",
        "kani",
        || <RustStdStandard<AtomicU16> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U16_SRC, {
        /// `AtomicU16::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u16() {
            let initial: u16 = kani::any();
            let atomic = AtomicU16::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU16::new sets the value observable via load"
            );

            let next: u16 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU16::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u32".to_owned(),
            VERIFY_ATOMIC_U32_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU32>",
        "kani",
        || <RustStdStandard<AtomicU32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U32_SRC, {
        /// `AtomicU32::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u32() {
            let initial: u32 = kani::any();
            let atomic = AtomicU32::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU32::new sets the value observable via load"
            );

            let next: u32 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU32::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU64> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u64".to_owned(),
            VERIFY_ATOMIC_U64_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU64>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU64>",
        "kani",
        || <RustStdStandard<AtomicU64> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U64_SRC, {
        /// `AtomicU64::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u64() {
            let initial: u64 = kani::any();
            let atomic = AtomicU64::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU64::new sets the value observable via load"
            );

            let next: u64 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU64::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicUsize> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_usize".to_owned(),
            VERIFY_ATOMIC_USIZE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicUsize>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicUsize>",
        "kani",
        || <RustStdStandard<AtomicUsize> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_USIZE_SRC, {
        /// `AtomicUsize::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_usize() {
            let initial: usize = kani::any();
            let atomic = AtomicUsize::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicUsize::new sets the value observable via load"
            );

            let next: usize = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicUsize::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicPtr<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_ptr_load_store_swap_and_compare_exchange".to_owned(),
            VERIFY_ATOMIC_PTR_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicPtr<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicPtr<i32>>",
        "kani",
        || <RustStdStandard<AtomicPtr<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<AtomicPtr<i32>>,
    "amenable_std::rust_std::RustStdStandard<AtomicPtr<i32>>",
    (*mut i32, *mut i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_PTR_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC, {
        /// `AtomicPtr::new` sets the pointer value observable via `load`;
        /// `store` overwrites it; `swap` returns the previous pointer and
        /// installs the new one; `compare_exchange` updates on a matching
        /// current value and reports the previous value on success.
        /// Pointer values are the addresses of distinct local slots (each
        /// holding a symbolic `i32`), so every value is pairwise-distinct
        /// by construction and no pointer is ever dereferenced — the
        /// whole proof stays free of `unsafe`.
        ///
        /// `compare_exchange`'s *failure* branch (mismatched current,
        /// reporting the real current value unchanged) is deliberately
        /// not checked here: Kani's model of the underlying
        /// `atomic_compare_exchange` intrinsic does not correctly track
        /// the reported value on that branch, and that false trail remains
        /// preserved in `gallery::atomic_ptr_compare_exchange` — a minimal,
        /// isolated reproduction (a single `compare_exchange` call against
        /// a freshly constructed `AtomicPtr`, no `swap` or prior exchange
        /// involved) that fails the same way, confirming this is a genuine
        /// Kani/CBMC limitation rather than a logic error in this proof or
        /// in std.
        #[kani::proof]
        fn verify_atomic_ptr_load_store_swap_and_compare_exchange() {
            let mut initial_slot: i32 = kani::any();
            let mut stored_slot: i32 = kani::any();
            let mut swapped_in_slot: i32 = kani::any();
            let mut exchange_target_slot: i32 = kani::any();

            let initial: *mut i32 = &mut initial_slot;
            let atomic = AtomicPtr::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicPtr::new sets the value observable via load"
            );

            let stored: *mut i32 = &mut stored_slot;
            atomic.store(stored, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    stored
                )),
                "AtomicPtr::store overwrites the value observable via load"
            );

            let swapped_in: *mut i32 = &mut swapped_in_slot;
            let previous = atomic.swap(swapped_in, std::sync::atomic::Ordering::SeqCst);
            assert!(
                <RustStdStandard<AtomicPtr<i32>> as Ensures<crate::KaniVerifier>>::ensures((
                    previous, stored
                )),
                "AtomicPtr::swap returns the value that was there before"
            );
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    swapped_in
                )),
                "AtomicPtr::swap installs the new value observable via load"
            );

            let exchange_target: *mut i32 = &mut exchange_target_slot;
            let success_result = atomic.compare_exchange(
                swapped_in,
                exchange_target,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((success_result, Ok(swapped_in))),
                "compare_exchange succeeds and returns the previous value when current matches"
            );
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    exchange_target
                )),
                "compare_exchange installs the new value on success"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::atomic::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_relaxed_ordering_still_makes_a_store_observable".to_owned(),
            VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::atomic::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ordering>",
        "kani",
        || <RustStdStandard<std::sync::atomic::Ordering> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC, {
        /// Every proof elsewhere in this module uses `SeqCst`; this one
        /// checks `Ordering::Relaxed` specifically -- a store under
        /// `Relaxed` is still observable via a `Relaxed` load on the
        /// same atomic (the ordering variant only affects cross-thread
        /// synchronization, not single-thread visibility).
        #[kani::proof]
        fn verify_relaxed_ordering_still_makes_a_store_observable() {
            let value: i32 = kani::any();
            let atomic = AtomicI32::new(0);
            atomic.store(value, std::sync::atomic::Ordering::Relaxed);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::Relaxed),
                    value
                )),
                "a Relaxed store is observable via a Relaxed load on the same atomic"
            );
        }
    }
}
