//! Creusot verifier backend for the `amenable` constitutional trait family.
//!
//! `CreusotVerifier` and the [`CreusotWitness`] trait are defined *here* —
//! there is only one verifier Creusot works with, Creusot, so the marker
//! belongs with the crate that means it. But unlike Kani and Verus, the
//! impls bridging `CreusotWitness`/`Witness<CreusotVerifier>` to concrete
//! std carriers (`RustStdStandard<T>`) live in `amenable_std` instead of
//! here — see `amenable_std::creusot_witness`'s doc comment for why (in
//! short: `creusot-rustc`'s whole-crate translation pass can't handle the
//! ordinary Rust machinery that bridge needs, so this crate stays pure
//! Pearlite proof-function content, the thing `cargo creusot` actually
//! translates). That split is legal under Rust's orphan rule via a
//! different justification than usual: it's `RustStdStandard<T>` (the
//! `Self` type, local to `amenable_std`) satisfying the "one local type"
//! requirement there, rather than the verifier marker (local here).
//!
//! `rust_std.rs` holds the actual harness functions; `witness.rs` holds
//! the trait/marker definitions `amenable_std` implements against.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod rust_std;
mod witness;

pub use rust_std::{
    VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC,
    VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
    VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, VERIFY_CHAR_ROUNDTRIP_SRC,
    VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC,
    VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
    VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
    VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
    VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
    VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
    VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
    VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC,
    VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
    VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
    VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC,
    VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, VERIFY_REVERSE_INVERTS_COMPARISON_SRC,
    VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, VERIFY_STRING_ROUNDTRIP_SRC,
    VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
    VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC,
    VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC,
    VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC,
    VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
};
pub use witness::{CreusotVerifier, CreusotVerifierMetadata, CreusotWitness};
