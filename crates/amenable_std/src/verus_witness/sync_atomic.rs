//! Every `std::sync::atomic::AtomicT` instantiation's `VerusWitness` impl,
//! via `impl_sync_atomic_verus_witness!`. The `AtomicPtr` / `Ordering`
//! stragglers and the `std::process` types live in `process_and_atomic_tail`.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

/// Every width's Verus accommodation model states the identical claim
/// (`result == (initial, next)`) since the model function is a plain
/// echo of its own two parameters — trivially true by construction, but
/// still a real, named round-trip claim about the atomic-model type, not
/// scanner-level noise (unlike a bare `result`, whose *content* is
/// invisible to the clause): `Ensures<VerusVerifier>` names it once here
/// rather than at each of the eleven widths' own `ensures` clauses.
macro_rules! impl_sync_atomic_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        // `pub(super)`, not private: `ObservedPairMatchesInput` in
        // iter_adapters_b.rs reuses the generated
        // VERIFY_ATOMIC_BOOL_MODEL_LOAD_STORE_SRC constant rather than
        // adding a new Verus proof -- see that impl's own doc comment.
        pub(super) const $const_name: &str =
            include_str!("../../../amenable_verus/src/rust_std/sync/sync_atomic_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }

        amenable_derive::verus_ensures_witness!(
            RustStdStandard<$ty>,
            concat!(
                "amenable_std::rust_std::RustStdStandard<",
                stringify!($ty),
                ">"
            ),
            $harness
        );
    };
}

impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicBool,
    "verify_atomic_bool_model_load_store",
    VERIFY_ATOMIC_BOOL_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicI8,
    "verify_atomic_i8_model_load_store",
    VERIFY_ATOMIC_I8_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicI16,
    "verify_atomic_i16_model_load_store",
    VERIFY_ATOMIC_I16_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicI32,
    "verify_atomic_i32_model_load_store",
    VERIFY_ATOMIC_I32_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicI64,
    "verify_atomic_i64_model_load_store",
    VERIFY_ATOMIC_I64_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicIsize,
    "verify_atomic_isize_model_load_store",
    VERIFY_ATOMIC_ISIZE_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicU8,
    "verify_atomic_u8_model_load_store",
    VERIFY_ATOMIC_U8_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicU16,
    "verify_atomic_u16_model_load_store",
    VERIFY_ATOMIC_U16_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicU32,
    "verify_atomic_u32_model_load_store",
    VERIFY_ATOMIC_U32_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicU64,
    "verify_atomic_u64_model_load_store",
    VERIFY_ATOMIC_U64_MODEL_LOAD_STORE_SRC
);
impl_sync_atomic_verus_witness!(
    std::sync::atomic::AtomicUsize,
    "verify_atomic_usize_model_load_store",
    VERIFY_ATOMIC_USIZE_MODEL_LOAD_STORE_SRC
);

pub(super) const VERIFY_ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/atomic_ptr_carrier.rs");

pub(super) const ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_VERUS_FRAGMENT: &str = r#"pub open spec fn atomic_ptr_model_load_store_swap_and_compare_exchange(
    load_after_new: int,
    load_after_store: int,
    swap_returned_previous: int,
    load_after_swap: int,
    compare_exchange_returned_previous: int,
    load_after_compare_exchange: int,
    initial: int,
    stored: int,
    swapped_in: int,
    exchange_target: int,
) -> bool {
    load_after_new == initial
        && load_after_store == stored
        && swap_returned_previous == stored
        && load_after_swap == swapped_in
        && compare_exchange_returned_previous == swapped_in
        && load_after_compare_exchange == exchange_target
}"#;
