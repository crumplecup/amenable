//! `CreusotWitness` impls for Rust standard-library carriers.
//!
//! This lives here, not alongside the proof functions in `amenable_creusot`,
//! because `creusot-rustc`'s translator sweeps every local item in a
//! `creusot-std`-dependent crate — including ones no `#[cfg(creusot)]` gate
//! protects, since Rust items don't need to *run* to be enumerated. Ordinary
//! infrastructure that's completely unremarkable to plain `rustc` turned out
//! to be unsupported there: a return-position `impl Trait` on a local `impl`
//! panicked its intrinsics-gathering pass outright (a real ICE, not a
//! hypothetical), and the `static` item `::inventory::submit!` expands to
//! hit "unsupported definition kind ... Static" the same way. So
//! `amenable_creusot` stays pure Pearlite proof-function content — the thing
//! `cargo creusot -- -p amenable_creusot` actually needs to translate — and
//! everything else about *finding* those proofs (the witness bridge, the
//! registry) moves to the crate that already owns the types being proved
//! about.
//!
//! This is legal under Rust's orphan rule for a reason distinct from every
//! other verifier backend's bridge (see `amenable_creusot::witness`'s own
//! doc comment for the usual justification: the verifier marker type is
//! local): here, `RustStdStandard<T>` — the `Self` type — is local to this
//! crate instead. The orphan rule only requires *one* of {trait, Self type,
//! trait's own type parameters} to be local to the implementing crate, not
//! any particular one, and a real 3-crate test confirmed a Self-type-local
//! justification compiles exactly the same as a trait-parameter-local one.
//!
//! One block per concrete type: a Creusot-checkable property doesn't
//! generalize across types the way provenance does, so there is no blanket
//! impl here — each type gets exactly the contract that's actually true of
//! it. The bridge to `Witness<CreusotVerifier>` is mechanical (delegates
//! straight to `CreusotWitness`), so it's generated per type by a macro
//! rather than hand-repeated.
//!
//! Most of these carriers have no invariant beyond what the type system
//! already guarantees — every bit pattern of an `i8` is a valid `i8`, so
//! there is nothing for Creusot to check. Their `proof()` is trusted: it
//! returns the chain-derived provenance reached through
//! `SupportingEvidence::basis().audit()` and nothing more — not a special
//! case, just what a `proof()` implementation looks like when there's no
//! contract content to add. `char` and `String` do carry a genuine
//! constraint, so their `proof()` also names the Creusot contract function
//! that checks it, alongside the same chain-derived provenance.
//!
//! Each type also registers a [`amenable_core::ProofRecord`] alongside its
//! `Witness` bridge, so `proof()`'s output is discoverable by name for
//! audit — see `amenable_core::chain::proof_chain`. The registered
//! `evidence` name is a hardcoded module-path literal matching
//! `RustStdStandard`'s own registration in `rust_std`, so both sides agree
//! on the same string without one computing it from the other.
//!
//! A "checked" carrier's [`CheckedProof::claim`] is the contract's own
//! verbatim source (`#[requires]`/`#[ensures]` included), captured via
//! [`amenable_derive::harness!`] *in `amenable_creusot`* — this crate only
//! imports the resulting `&'static str` constant, never the harness
//! function itself, so the claim can never drift from the real contract
//! without also touching the crate that's actually translated.

use std::borrow::Cow;
use std::boxed::Box;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, TryReserveError};
use std::mem::ManuallyDrop;
use std::num::{NonZero, Saturating, Wrapping};
use std::time::{Duration, TryFromFloatSecsError};

use amenable_core::{Evidence, Provenance, Witness};
use amenable_creusot::{
    CreusotVerifier, CreusotWitness, VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC,
    VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
    VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, VERIFY_CHAR_ROUNDTRIP_SRC,
    VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC,
    VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
    VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
    VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
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
    VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
};

use crate::{RustStdProvenance, RustStdStandard};

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}

macro_rules! impl_creusot_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl CreusotWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_creusot_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "creusot",
                    describe: || <RustStdStandard<$ty> as CreusotWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_creusot_witness_trusted!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    TryFromFloatSecsError
);

/// Proof artifact for a carrier with a real, machine-checked Creusot
/// contract: names the contract function, carries its verbatim source as
/// `claim`, and still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// The Creusot contract function that checks this carrier's invariant.
    pub harness: &'static str,
    /// The contract's own source — what it actually requires/ensures,
    /// verbatim.
    pub claim: &'static str,
    /// The chain-derived provenance this claim still rests on.
    pub provenance: RustStdProvenance,
}

impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

impl CreusotWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_roundtrip",
            claim: VERIFY_CHAR_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "creusot",
        describe: || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_roundtrip",
            claim: VERIFY_STRING_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "creusot",
        describe: || <RustStdStandard<String> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Cow<'static, i32>`, matching `amenable_std::rust_std::
// alloc_borrow`'s own registration exactly (confirmed against the
// checklist's own `evidence_name` column:
// `RustStdStandard<Cow<'static, i32>>`).
impl CreusotWitness for RustStdStandard<Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cow_destructure_recovers_the_wrapped_value",
            claim: VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Cow<'static, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_map_iterates_in_key_order",
            claim: VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BTreeMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BTreeMap<i32, i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_set_iterates_in_sorted_order",
            claim: VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BTreeSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BTreeSet<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_pop_yields_the_maximum_first",
            claim: VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeap<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<BinaryHeap<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_is_fifo_through_back_and_front",
            claim: VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LinkedList<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<LinkedList<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_reserve_rejects_an_impossible_capacity",
            claim: VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryReserveError>",
        verifier: "creusot",
        describe: || <RustStdStandard<TryReserveError> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_box_new_preserves_the_wrapped_value",
            claim: VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Box<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Box<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_duration_new_normalizes_nanos_and_carries_into_secs",
            claim: VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Duration>",
        verifier: "creusot",
        describe: || <RustStdStandard<Duration> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<NonZero<i16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i16_roundtrips",
            claim: VERIFY_NONZERO_I16_ROUNDTRIPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<NonZero<i16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        verifier: "creusot",
        describe: || <RustStdStandard<NonZero<i16>> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_kani::rust_std::cmp` and
// `amenable_std::rust_std::cmp`'s own registration exactly: there's also
// a `core::sync::atomic::Ordering`, so the evidence string must say
// `std::cmp::Ordering`, not the bare name, or alias resolution won't
// match this proof to the checklist row.
impl CreusotWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ordering_reverse_swaps_less_and_greater",
            claim: VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        verifier: "creusot",
        describe: || <RustStdStandard<std::cmp::Ordering> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_wrapping_i32_add_wraps",
            claim: VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Wrapping<i32>> as CreusotWitness>::proof().to_string(),
    }
}

impl CreusotWitness for RustStdStandard<Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_saturating_i32_add_clamps",
            claim: VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Saturating<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::IntErrorKind, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::IntErrorKind>`).
impl CreusotWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_int_error_kind_classifies_parse_failures",
            claim: VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        verifier: "creusot",
        describe: || <RustStdStandard<core::num::IntErrorKind> as CreusotWitness>::proof().to_string(),
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::TryFromIntError, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::TryFromIntError>`).
impl CreusotWitness for RustStdStandard<core::num::TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_from_int_error_occurs_exactly_when_out_of_range",
            claim: VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::TryFromIntError> as CreusotWitness>::proof().to_string()
        },
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::ParseIntError, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::ParseIntError>`).
impl CreusotWitness for RustStdStandard<core::num::ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_parse_int_error_reports_the_kind_of_the_failure",
            claim: VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::ParseIntError> as CreusotWitness>::proof().to_string()
        },
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::FpCategory, ...)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<core::num::FpCategory>`).
//
// `#[trusted]`, like `NonZero<i16>` above: `f64` has no `View` impl in
// `creusot-std`, and a bare float literal in Pearlite panics
// `creusot-rustc` outright — both confirmed real blockers, not a
// convenience shortcut; see `amenable_std::creusot_gallery`'s
// `f64_has_no_view_impl_at_all`/`float_literals_in_pearlite_ice_the_compiler`
// findings.
impl CreusotWitness for RustStdStandard<core::num::FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_fp_category_matches_the_value_it_classifies",
            claim: VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::FpCategory> as CreusotWitness>::proof().to_string()
        },
    }
}

// Fully qualified, matching `amenable_std::rust_std::num`'s own
// registration exactly (`register_rust_std_standard_evidence!(...,
// core::num::ParseFloatError, ...)`, confirmed against the checklist's
// own `evidence_name` column: `RustStdStandard<core::num::ParseFloatError>`).
//
// `#[trusted]`: a real extern_spec for `FromStr for f64` translates
// cleanly but `why3find prove` doesn't discharge the harness's own goal
// against it — confirmed reproducible, not a convenience shortcut; see
// `amenable_std::creusot_gallery`'s
// `parse_float_error_extern_spec_translates_but_wont_discharge` finding.
impl CreusotWitness for RustStdStandard<core::num::ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_parse_float_error_occurs_only_for_unparseable_input",
            claim: VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<core::num::ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<core::num::ParseFloatError> as CreusotWitness>::proof().to_string()
        },
    }
}

// Bare `Reverse<i32>`, matching `amenable_std::rust_std::cmp`'s own
// registration exactly (`register_rust_std_standard_evidence!(std::cmp::
// Ordering, Reverse<i32>)`, confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Reverse<i32>>`).
impl CreusotWitness for RustStdStandard<Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_reverse_inverts_comparison",
            claim: VERIFY_REVERSE_INVERTS_COMPARISON_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Reverse<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Reverse<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Option<i32>`, matching `amenable_std::rust_std::option_result`'s
// own registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Option<i32>>`).
impl CreusotWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_some_and_none_are_disjoint",
            claim: VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Option<i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `Result<i32, i32>`, matching `amenable_std::rust_std::option_result`'s
// own registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<Result<i32, i32>>`).
impl CreusotWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_ok_and_err_are_disjoint",
            claim: VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        verifier: "creusot",
        describe: || <RustStdStandard<Result<i32, i32>> as CreusotWitness>::proof().to_string(),
    }
}

// Bare `ManuallyDrop<i32>`, matching `amenable_std::rust_std::mem`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<ManuallyDrop<i32>>`).
impl CreusotWitness for RustStdStandard<ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_manually_drop_derefs_and_into_inner_round_trip",
            claim: VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_creusot_witness!(RustStdStandard<ManuallyDrop<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ManuallyDrop<i32>>",
        verifier: "creusot",
        describe: || {
            <RustStdStandard<ManuallyDrop<i32>> as CreusotWitness>::proof().to_string()
        },
    }
}
