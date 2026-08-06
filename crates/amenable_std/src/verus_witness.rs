//! `VerusWitness` impls for Rust standard-library carriers.
//!
//! This lives here, not alongside the proof functions in `amenable_verus`,
//! for a stronger reason than `amenable_std::creusot_witness`'s: it's not
//! just that the witness/registry machinery is awkward to compile under
//! Verus, it's that `amenable_verus` has *no* `amenable_core`/`inventory`
//! dependency to build that machinery against at all. Verus is invoked as
//! a bare compiler over a single file tree (`verus --crate-type=lib
//! path/to/lib.rs`) — it never reads `Cargo.toml`, so it cannot resolve
//! any external crate, proc-macro or otherwise. Confirmed empirically:
//! pointing `verus` at this crate's pre-split structure (which depended on
//! `amenable_core`/`amenable_derive`/`inventory`) failed immediately with
//! unresolved-crate errors, not proof errors — matching the exact failure
//! `elicitation_verus`'s own real, working proof crate structure avoids by
//! depending on nothing but `verus_builtin_macros`/`vstd`.
//!
//! So there is no `VERIFY_*_SRC` constant to import here the way
//! `creusot_witness` imports one per proof from `amenable_creusot`
//! (`amenable_derive::harness!`, the macro that generates those constants,
//! is itself a proc-macro from a crate Verus can't resolve — it wouldn't
//! compile under Verus's toolchain either). Each `claim` below is captured
//! via `include_str!` instead — a plain Rust language feature (no
//! proc-macro, no external crate), reading `amenable_verus`'s real proof
//! source file directly at compile time, so the claim text can never drift
//! from what `verus` actually checked.
//!
//! Legal under Rust's orphan rule for the same reason `creusot_witness` is:
//! `RustStdStandard<T>` (the `Self` type) is local to this crate. Unlike
//! `creusot_witness`, though, `VerusVerifier`/`VerusVerifierMetadata`/
//! `VerusWitness` are defined *here* too, not in `amenable_verus` — they
//! need `amenable_core::{Verifier, Evidence, ...}`, which `amenable_verus`
//! no longer depends on.

use amenable_core::{Evidence, MetadataEntry, Provenance, Verifier, Witness};

use crate::{RustStdProvenance, RustStdStandard};

/// The Verus verifier, local to this crate: there is only one verifier
/// Verus works with — Verus. Being local here (not imported from
/// `amenable_core`) is what makes the per-type bridges below legal under
/// Rust's orphan rule — a blanket bridge over a bare type parameter is
/// not: the orphan rule requires every uncovered generic parameter to be
/// covered before the first local type, and `Self` in a blanket impl
/// never is.
pub struct VerusVerifier;

/// Provenance surface for the Verus verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "verus"),
            ("authority", "Verus project"),
            ("source_url", "https://verus-lang.github.io/verus/"),
            ("proof_artifact", "Verus proof module token stream"),
            (
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            (
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ];
        FACTS
            .iter()
            .map(|&(k, v)| MetadataEntry::new(k, v))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}

/// Verus-specific witness: identifies the Verus spec (if any) behind a
/// piece of evidence, without ever running it.
pub trait VerusWitness {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the Verus proof relevant to this evidence.
    type ProofArtifact;

    /// Identify the Verus proof artifact for this evidence.
    fn proof() -> Self::ProofArtifact;
}

macro_rules! bridge_verus_witness {
    ($ty:ty) => {
        impl Witness<VerusVerifier> for $ty {
            type SupportingEvidence = <$ty as VerusWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as VerusWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as VerusWitness>::proof()
            }
        }
    };
}

macro_rules! impl_verus_witness_trusted {
    ($($ty:ty),* $(,)?) => {
        $(
            impl VerusWitness for RustStdStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = RustStdProvenance;

                fn proof() -> Self::ProofArtifact {
                    <Self::SupportingEvidence as Evidence>::basis().audit()
                }
            }

            bridge_verus_witness!(RustStdStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    verifier: "verus",
                    describe: || <RustStdStandard<$ty> as VerusWitness>::proof().report().to_string(),
                }
            }
        )*
    };
}

impl_verus_witness_trusted!(
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
    std::convert::Infallible,
    core::ffi::c_void
);

/// Proof artifact for a carrier with a real, machine-checked Verus spec:
/// names the spec function, carries its verbatim source as `claim`, and
/// still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerusCheckedProof {
    /// The Verus spec function that checks this carrier's invariant.
    pub harness: &'static str,
    /// The spec's own source — the whole file it lives in, verbatim
    /// (`include_str!`, not a per-function extraction — Verus proof files
    /// in `amenable_verus` are kept to one carrier's spec function(s) each
    /// so this stays a tight, accurate claim, the same one-claim-per-
    /// carrier granularity `amenable_derive::harness!` gives Kani/Creusot
    /// by capturing one function at a time).
    pub claim: &'static str,
    /// The chain-derived provenance this claim still rests on.
    pub provenance: RustStdProvenance,
}

impl std::fmt::Display for VerusCheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

const VERIFY_CHAR_ROUNDTRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_carrier.rs");

impl VerusWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_char_roundtrip",
            claim: VERIFY_CHAR_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "verus",
        describe: || <RustStdStandard<char> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_STRING_ROUNDTRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/string_carrier.rs");

impl VerusWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_string_roundtrip",
            claim: VERIFY_STRING_ROUNDTRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "verus",
        describe: || <RustStdStandard<String> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ordering_carrier.rs");

impl VerusWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordering_reverse_swaps_less_and_greater",
            claim: VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cmp::Ordering> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_OPTION_UNWRAP_RETURNS_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/option_carrier.rs");

impl VerusWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_option_unwrap_returns_the_wrapped_value",
            claim: VERIFY_OPTION_UNWRAP_RETURNS_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        verifier: "verus",
        describe: || <RustStdStandard<Option<i32>> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_RESULT_UNWRAP_RETURNS_THE_OK_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/result_carrier.rs");

impl VerusWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_result_unwrap_returns_the_ok_value",
            claim: VERIFY_RESULT_UNWRAP_RETURNS_THE_OK_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<Result<i32, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_WRAPPING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/wrapping_carrier.rs");

impl VerusWitness for RustStdStandard<std::num::Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_wrapping_field_roundtrips_the_constructed_value",
            claim: VERIFY_WRAPPING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::num::Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::num::Wrapping<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::num::Wrapping<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SATURATING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/saturating_carrier.rs");

impl VerusWitness for RustStdStandard<std::num::Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_saturating_field_roundtrips_the_constructed_value",
            claim: VERIFY_SATURATING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::num::Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::num::Saturating<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::num::Saturating<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_REVERSE_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/reverse_carrier.rs");

impl VerusWitness for RustStdStandard<std::cmp::Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_reverse_field_roundtrips_the_constructed_value",
            claim: VERIFY_REVERSE_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cmp::Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cmp::Reverse<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cmp::Reverse<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/manually_drop_carrier.rs");

impl VerusWitness for RustStdStandard<std::mem::ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_manually_drop_derefs_and_into_inner_round_trip",
            claim: VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::mem::ManuallyDrop<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::mem::ManuallyDrop<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::mem::ManuallyDrop<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fp_category_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_fp_category_matches_the_value_it_classifies",
            claim: VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::num::FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::num::FpCategory> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/int_error_kind_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_int_error_kind_classifies_parse_failures",
            claim: VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::num::IntErrorKind> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/parse_float_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_parse_float_error_occurs_only_for_unparseable_input",
            claim: VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::num::ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::num::ParseFloatError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/try_from_int_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_try_from_int_error_occurs_exactly_when_out_of_range",
            claim: VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::num::TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::num::TryFromIntError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/box_carrier.rs");

impl VerusWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_box_derefs_and_writes_through",
            claim: VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<Box<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "verus",
        describe: || <RustStdStandard<Box<i32>> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/layout_carrier.rs");

impl VerusWitness for RustStdStandard<core::alloc::Layout> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment",
            claim: VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::alloc::Layout>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::alloc::Layout>",
        verifier: "verus",
        describe: || <RustStdStandard<core::alloc::Layout> as VerusWitness>::proof().to_string(),
    }
}

impl VerusWitness for RustStdStandard<core::alloc::LayoutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment",
            claim: VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::alloc::LayoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::alloc::LayoutError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::alloc::LayoutError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_carrier.rs");

impl VerusWitness for RustStdStandard<Vec<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_push_pop_round_trips",
            claim: VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<Vec<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
        verifier: "verus",
        describe: || <RustStdStandard<Vec<i32>> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_try_from_carrier.rs");

impl VerusWitness for RustStdStandard<core::char::CharTryFromError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range",
            claim: VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::char::CharTryFromError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::char::CharTryFromError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_try_from_carrier.rs");

impl VerusWitness for RustStdStandard<core::char::TryFromCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_try_from_char_error_occurs_exactly_when_out_of_range",
            claim: VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::char::TryFromCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::char::TryFromCharError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/type_id_carrier.rs");

impl VerusWitness for RustStdStandard<core::any::TypeId> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_type_id_is_reflexive_and_distinguishes_distinct_types",
            claim: VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::any::TypeId>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::any::TypeId>",
        verifier: "verus",
        describe: || <RustStdStandard<core::any::TypeId> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/try_from_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::array::TryFromSliceError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_try_from_slice_rejects_a_length_mismatch",
            claim: VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::array::TryFromSliceError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::array::TryFromSliceError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::array::TryFromSliceError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/from_utf16_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::string::FromUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_utf16_rejects_a_lone_surrogate",
            claim: VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::string::FromUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::string::FromUtf16Error> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cstring_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cstring_excludes_the_terminator_and_rejects_interior_nul",
            claim: VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::CString>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        verifier: "verus",
        describe: || <RustStdStandard<std::ffi::CString> as VerusWitness>::proof().to_string(),
    }
}

impl VerusWitness for RustStdStandard<std::ffi::NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cstring_excludes_the_terminator_and_rejects_interior_nul",
            claim: VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::NulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        verifier: "verus",
        describe: || <RustStdStandard<std::ffi::NulError> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/from_vec_with_nul_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_vec_with_nul_requires_the_nul_only_at_the_end",
            claim: VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::FromVecWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ffi::FromVecWithNulError> as VerusWitness>::proof().to_string()
        },
    }
}
