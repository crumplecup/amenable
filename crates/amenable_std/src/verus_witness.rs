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
//!
//! `SipHasher`'s witness block below references a deprecated item, and
//! `#[expect(deprecated)]` attached to the individual impl/macro-
//! invocation/`inventory::submit!` sites didn't line up with where the
//! lint actually fires through macro expansion (confirmed: those
//! per-site attributes reported "unused attribute" while the warning
//! still fired elsewhere) — expecting it at the whole-module level
//! instead, the same fix `amenable_verus::rust_std::sip_hasher_carrier`
//! already uses for the identical reason.
#![expect(
    deprecated,
    reason = "SipHasher itself is stable (only deprecated as a recommendation to use DefaultHasher instead); covering it is a coverage-completeness question, not a call to use it"
)]

use amenable_core::{Ensures, Evidence, MetadataEntry, Provenance, Requires, Verifier, Witness};
#[cfg(windows)]
use std::os::windows::ffi::EncodeWide;
#[cfg(windows)]
use std::os::windows::io::{
    BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
};

use crate::{
    ArrayIntoIterAdvanceMatchesPosition, ArrayIntoIterStartsAtFirstPosition, AsciiByte,
    IncrementHeadroom, NonNulByte, ObservedValueMatchesInput, RustStdProvenance, RustStdStandard,
    ValidUnicodeScalar, ValueUnchanged, WriteStoresNewValue, YieldsThreeValuesInOrderThenEnds,
};

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
    core::ffi::c_void,
    std::cell::BorrowError,
    std::cell::BorrowMutError,
    std::fmt::Error,
    std::marker::PhantomData<i32>,
    std::marker::PhantomPinned,
    std::net::AddrParseError,
    std::str::ParseBoolError,
    std::io::Stderr,
    std::io::StderrLock<'static>,
    std::io::Stdin,
    std::io::StdinLock<'static>,
    std::io::Stdout,
    std::io::StdoutLock<'static>,
    std::process::ExitCode,
    std::thread::AccessError,
    std::thread::Builder,
    std::thread::JoinHandle<i32>,
    std::thread::Scope<'static, 'static>,
    std::thread::ScopedJoinHandle<'static, i32>,
    std::env::VarError,
    std::env::Vars,
    std::env::VarsOs,
    std::task::RawWaker,
    std::task::RawWakerVTable,
    core::panic::PanicInfo<'static>,
    core::panic::PanicMessage<'static>,
    core::time::TryFromFloatSecsError,
    ()
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

const CHAR_ROUNDTRIP_PRESERVES_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn char_roundtrip_preserves_value(result: char, input: char) -> bool {
    result == input
}"#;

const CHAR_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn char_is_valid_unicode_scalar(value: char) -> bool {
    (value as u32) <= 0xD7FFu32
        || ((value as u32) >= 0xE000u32 && (value as u32) <= 0x10FFFFu32)
}"#;

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

impl Ensures<VerusVerifier> for RustStdStandard<char> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "char_roundtrip_preserves_value(result, c) && char_is_valid_unicode_scalar(c)"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "verus",
        describe: || <RustStdStandard<char> as VerusWitness>::proof().to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<char> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CHAR_ROUNDTRIP_PRESERVES_VALUE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CHAR_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT,
    }
}

/// The [`ValidUnicodeScalar`] contract type reuses `verify_char_roundtrip`
/// rather than adding a new Verus proof — it names the postcondition the
/// spec already checks, it doesn't prove anything new.
impl VerusWitness for ValidUnicodeScalar {
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

bridge_verus_witness!(ValidUnicodeScalar);

impl Ensures<VerusVerifier> for ValidUnicodeScalar {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "char_is_valid_unicode_scalar(c)"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ValidUnicodeScalar",
        verifier: "verus",
        kind: "ensures",
        fragment: || <ValidUnicodeScalar as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::ValidUnicodeScalar",
        verifier: "verus",
        describe: || <ValidUnicodeScalar as VerusWitness>::proof().to_string(),
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
const FP_CATEGORY_CLASSIFY_RESULT_MATCHES_SPECIAL_VALUE_CATEGORIES_VERUS_FRAGMENT: &str = r#"pub open spec fn fp_category_classify_result_matches_special_value_categories(
    value: f64,
    result: FpCategory,
) -> bool {
    (value.is_nan_spec() ==> result == FpCategory::Nan)
        && (value.is_infinite_spec() ==> result == FpCategory::Infinite)
}"#;
const FP_CATEGORY_INPUTS_COVER_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn fp_category_inputs_cover_nan_and_infinite_cases(nan: f64, infinite: f64) -> bool {
    nan.is_nan_spec() && infinite.is_infinite_spec()
}"#;
const FP_CATEGORY_RESULTS_MATCH_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn fp_category_results_match_nan_and_infinite_cases(
    result: (FpCategory, FpCategory),
) -> bool {
    result.0 == FpCategory::Nan && result.1 == FpCategory::Infinite
}"#;

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

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FP_CATEGORY_CLASSIFY_RESULT_MATCHES_SPECIAL_VALUE_CATEGORIES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        verifier: "verus",
        kind: "requires",
        fragment: || FP_CATEGORY_INPUTS_COVER_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FP_CATEGORY_RESULTS_MATCH_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
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

const VERIFY_PARSE_INT_ERROR_MODEL_REPORTS_THE_KIND_OF_THE_FAILURE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/int_error_kind_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_parse_int_error_model_reports_the_kind_of_the_failure",
            claim: VERIFY_PARSE_INT_ERROR_MODEL_REPORTS_THE_KIND_OF_THE_FAILURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::num::ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::num::ParseIntError> as VerusWitness>::proof().to_string()
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

const BOX_DEREFS_AND_WRITES_THROUGH_VERUS_FRAGMENT: &str = r#"pub open spec fn box_derefs_and_writes_through(
    deref_value: int,
    observed_after_write: int,
    initial: int,
    updated: int,
) -> bool {
    deref_value == initial && observed_after_write == updated
}"#;

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

impl Ensures<VerusVerifier> for RustStdStandard<Box<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.0 == value && result.1 == updated"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "verus",
        describe: || <RustStdStandard<Box<i32>> as VerusWitness>::proof().to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<Box<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || BOX_DEREFS_AND_WRITES_THROUGH_VERUS_FRAGMENT,
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

const U32_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn u32_is_valid_unicode_scalar(value: u32) -> bool {
    value <= 0x0010_FFFF && !(0xD800 <= value && value <= 0xDFFF)
}"#;

const CHAR_TRY_FROM_U32_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn char_try_from_u32_succeeds_with_same_scalar(
    value: u32,
    result: Result<char, <char as core::convert::TryFrom<u32>>::Error>,
) -> bool {
    u32_is_valid_unicode_scalar(value) ==> (result is Ok && (result->Ok_0 as u32) == value)
}"#;

const CHAR_TRY_FROM_U32_REJECTS_INVALID_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn char_try_from_u32_rejects_invalid_scalar(
    value: u32,
    result: Result<char, <char as core::convert::TryFrom<u32>>::Error>,
) -> bool {
    !u32_is_valid_unicode_scalar(value) ==> result is Err
}"#;

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

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || U32_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CHAR_TRY_FROM_U32_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CHAR_TRY_FROM_U32_REJECTS_INVALID_SCALAR_VERUS_FRAGMENT,
    }
}

const VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_try_from_carrier.rs");

const CHAR_FITS_IN_U8_VERUS_FRAGMENT: &str = r#"pub open spec fn char_fits_in_u8(value: char) -> bool {
    (value as u32) <= 0xFF
}"#;

const U8_TRY_FROM_CHAR_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn u8_try_from_char_succeeds_with_same_scalar(
    value: char,
    result: Result<u8, <u8 as core::convert::TryFrom<char>>::Error>,
) -> bool {
    char_fits_in_u8(value) ==> (result is Ok && (result->Ok_0 as u32) == (value as u32))
}"#;

const U8_TRY_FROM_CHAR_REJECTS_OUT_OF_RANGE_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn u8_try_from_char_rejects_out_of_range_scalar(
    value: char,
    result: Result<u8, <u8 as core::convert::TryFrom<char>>::Error>,
) -> bool {
    !char_fits_in_u8(value) ==> result is Err
}"#;

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

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CHAR_FITS_IN_U8_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || U8_TRY_FROM_CHAR_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || U8_TRY_FROM_CHAR_REJECTS_OUT_OF_RANGE_SCALAR_VERUS_FRAGMENT,
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
const FROM_UTF16_RESULT_MATCHES_SINGLE_UNIT_EXAMPLES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf16_result_matches_single_unit_examples(
    units: &[u16],
    result: Result<String, FromUtf16Error>,
) -> bool {
    ((units@.len() == 1 && units@[0] == 0x61) ==> result is Ok)
        && ((units@.len() == 1 && units@[0] == 0xD800) ==> result is Err)
        && ((units@.len() == 1 && units@[0] == 0xDC00) ==> result is Err)
}"#;
const FROM_UTF16_INPUTS_COVER_VALID_AND_LONE_SURROGATE_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf16_inputs_cover_valid_and_lone_surrogate_cases(
    valid: &[u16],
    lone_surrogate: &[u16],
    lone_low_surrogate: &[u16],
) -> bool {
    valid@.len() == 1
        && valid@[0] == 0x61
        && lone_surrogate@.len() == 1
        && lone_surrogate@[0] == 0xD800
        && lone_low_surrogate@.len() == 1
        && lone_low_surrogate@[0] == 0xDC00
}"#;
const FROM_UTF16_CASE_RESULTS_MATCH_ACCEPT_REJECT_TRIPLE_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf16_case_results_match_accept_reject_triple(result: (bool, bool, bool)) -> bool {
    result.0 && result.1 && result.2
}"#;

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

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FROM_UTF16_RESULT_MATCHES_SINGLE_UNIT_EXAMPLES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        verifier: "verus",
        kind: "requires",
        fragment: || FROM_UTF16_INPUTS_COVER_VALID_AND_LONE_SURROGATE_CASES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FROM_UTF16_CASE_RESULTS_MATCH_ACCEPT_REJECT_TRIPLE_VERUS_FRAGMENT,
    }
}

const VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cstring_carrier.rs");

const INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT: &str = r#"pub open spec fn into_vec_u8_spec_matches_input_vec(v: Vec<u8>) -> bool {
    into_vec_u8_spec(v) == v@
}"#;

const CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn cstring_new_result_matches_input_bytes<T: Into<Vec<u8>>>(
    bytes: T,
    result: Result<CString, NulError>,
) -> bool {
    (cstring_input_has_no_preterminal_nul(bytes)
        ==> (result is Ok && cstring_bytes_spec(result->Ok_0) == into_vec_u8_spec(bytes)))
        && (cstring_input_has_a_preterminal_nul(bytes) ==> result is Err)
}"#;

const CSTRING_TEST_BYTE_IS_NONZERO_VERUS_FRAGMENT: &str = r#"pub open spec fn cstring_test_byte_is_nonzero(byte: u8) -> bool {
    byte != 0
}"#;

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

impl Requires<VerusVerifier> for RustStdStandard<std::ffi::CString> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "byte != 0"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        verifier: "verus",
        describe: || <RustStdStandard<std::ffi::CString> as VerusWitness>::proof().to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::ffi::CString> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        verifier: "verus",
        kind: "requires",
        fragment: || CSTRING_TEST_BYTE_IS_NONZERO_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        verifier: "verus",
        kind: "ensures",
        fragment: || INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT,
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

impl Requires<VerusVerifier> for RustStdStandard<std::ffi::NulError> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "byte != 0"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        verifier: "verus",
        describe: || <RustStdStandard<std::ffi::NulError> as VerusWitness>::proof().to_string(),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::ffi::NulError> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        verifier: "verus",
        kind: "requires",
        fragment: || CSTRING_TEST_BYTE_IS_NONZERO_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT,
    }
}

const VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/from_vec_with_nul_carrier.rs");

const FROM_VEC_WITH_NUL_RESULT_MATCHES_NUL_PLACEMENT_VERUS_FRAGMENT: &str = r#"pub open spec fn from_vec_with_nul_result_matches_nul_placement(
    bytes: Vec<u8>,
    result: Result<CString, FromVecWithNulError>,
) -> bool {
    (bytes@.len() > 0 && bytes@[bytes@.len() - 1] == 0
        && !(exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0)
        ==> result is Ok)
        && (!exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0 ==> result is Err)
        && ((exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0) ==> result is Err)
}"#;

const FROM_VEC_WITH_NUL_TEST_BYTE_IS_NONZERO_VERUS_FRAGMENT: &str = r#"pub open spec fn from_vec_with_nul_test_byte_is_nonzero(byte: u8) -> bool {
    byte != 0
}"#;

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

impl Requires<VerusVerifier> for RustStdStandard<std::ffi::FromVecWithNulError> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "byte != 0"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ffi::FromVecWithNulError> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::ffi::FromVecWithNulError> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        verifier: "verus",
        kind: "requires",
        fragment: || FROM_VEC_WITH_NUL_TEST_BYTE_IS_NONZERO_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FROM_VEC_WITH_NUL_RESULT_MATCHES_NUL_PLACEMENT_VERUS_FRAGMENT,
    }
}

const VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/parse_char_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::char::ParseCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_parse_char_error_occurs_for_empty_or_multi_character_strings",
            claim: VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::char::ParseCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::ParseCharError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::char::ParseCharError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/rc_carrier.rs");

impl VerusWitness for RustStdStandard<std::rc::Rc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_rc_derefs_to_the_wrapped_value",
            claim: VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::rc::Rc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::rc::Rc<i32>>",
        verifier: "verus",
        describe: || <RustStdStandard<std::rc::Rc<i32>> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_ARC_DEREFS_TO_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/arc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Arc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_arc_derefs_to_the_wrapped_value",
            claim: VERIFY_ARC_DEREFS_TO_THE_WRAPPED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Arc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Arc<i32>>",
        verifier: "verus",
        describe: || <RustStdStandard<std::sync::Arc<i32>> as VerusWitness>::proof().to_string(),
    }
}

const VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/into_string_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_into_string_error_recovers_the_original_cstring",
            claim: VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::IntoStringError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ffi::IntoStringError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cstr_carrier.rs");

const CSTR_FROM_BYTES_UNTIL_NUL_RESULT_MATCHES_NUL_PRESENCE_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_from_bytes_until_nul_result_matches_nul_presence<'a>(
    bytes: &'a [u8],
    result: Result<&'a CStr, std::ffi::FromBytesUntilNulError>,
) -> bool {
    (cstr_bytes_contain_a_nul(bytes) ==> result is Ok)
        && (cstr_bytes_contain_no_nul(bytes) ==> result is Err)
}"#;

const CSTR_UNTIL_NUL_TEST_INPUTS_COVER_BOTH_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_until_nul_test_inputs_cover_both_cases(
    with_nul: &[u8],
    without_nul: &[u8],
) -> bool {
    cstr_bytes_contain_a_nul(with_nul) && cstr_bytes_contain_no_nul(without_nul)
}"#;

impl VerusWitness for RustStdStandard<core::ffi::FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere",
            claim: VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::FromBytesUntilNulError>);

impl Requires<VerusVerifier> for RustStdStandard<core::ffi::FromBytesUntilNulError> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "exists|i: int| 0 <= i < with_nul@.len() && with_nul@[i] == 0 && !exists|i: int| 0 <= i < without_nul@.len() && without_nul@[i] == 0"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::ffi::FromBytesUntilNulError> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<core::ffi::FromBytesUntilNulError> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        verifier: "verus",
        kind: "requires",
        fragment: || CSTR_UNTIL_NUL_TEST_INPUTS_COVER_BOTH_CASES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CSTR_FROM_BYTES_UNTIL_NUL_RESULT_MATCHES_NUL_PRESENCE_VERUS_FRAGMENT,
    }
}

const VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cstr_carrier.rs");

const CSTR_FROM_BYTES_WITH_NUL_RESULT_MATCHES_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_from_bytes_with_nul_result_matches_bytes<'a>(
    bytes: &'a [u8],
    result: Result<&'a CStr, std::ffi::FromBytesWithNulError>,
) -> bool {
    (cstr_bytes_have_only_a_trailing_nul(bytes) ==> {
        &&& result is Ok
        &&& cstr_bytes_spec(result->Ok_0) == bytes@.subrange(0, bytes@.len() - 1)
    }) && (cstr_bytes_contain_no_nul(bytes) ==> result is Err)
        && (cstr_bytes_have_an_interior_nul(bytes) ==> result is Err)
}"#;

impl VerusWitness for RustStdStandard<core::ffi::FromBytesWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end",
            claim: VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesWithNulError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::ffi::FromBytesWithNulError> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesWithNulError>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CSTR_FROM_BYTES_WITH_NUL_RESULT_MATCHES_BYTES_VERUS_FRAGMENT,
    }
}

const VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/hash_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_build_hasher_default_produces_consistent_hashers",
            claim: VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<
                std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
            > as VerusWitness>::proof()
            .to_string()
        },
    }
}

const VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sip_hasher_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::SipHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_sip_hasher_produces_consistent_hashes",
            claim: VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::SipHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::hash::SipHasher>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::hash::SipHasher> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cow_carrier.rs");

const I32_TO_OWNED_SPEC_IS_IDENTITY_VERUS_FRAGMENT: &str = r#"pub open spec fn i32_to_owned_spec_is_identity(value: i32) -> bool {
    to_owned_spec(&value) == value
}"#;

const COW_INTO_OWNED_PRESERVES_VARIANT_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cow_into_owned_preserves_variant_value<'a, B: ToOwned + ?Sized>(
    cow: Cow<'a, B>,
    result: <B as ToOwned>::Owned,
) -> bool {
    match cow {
        Cow::Borrowed(b) => result == to_owned_spec(b),
        Cow::Owned(o) => result == o,
    }
}"#;

impl VerusWitness for RustStdStandard<std::borrow::Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cow_borrowed_and_owned_agree_on_their_value",
            claim: VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::borrow::Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::borrow::Cow<'static, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || "#[trigger] to_owned_spec(&value) == value",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || I32_TO_OWNED_SPEC_IS_IDENTITY_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || "match cow { Cow::Borrowed(b) => result == to_owned_spec(b), Cow::Owned(o) => result == o, }",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || COW_INTO_OWNED_PRESERVES_VARIANT_VALUE_VERUS_FRAGMENT,
    }
}

const VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/btree_carrier.rs");

const BTREE_MAP_ROUND_TRIP_INPUTS_ARE_DISTINCT_VERUS_FRAGMENT: &str = r#"pub open spec fn btree_map_round_trip_inputs_are_distinct(k1: int, k2: int) -> bool {
    k1 != k2
}"#;

const BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIP_HOLDS_VERUS_FRAGMENT: &str = r#"pub open spec fn btree_map_insert_get_remove_round_trip_holds(result: bool) -> bool {
    result
}"#;

impl VerusWitness for RustStdStandard<std::collections::BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_btree_map_insert_get_remove_round_trips",
            claim: VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BTreeMap<i32, i32>>);

impl Requires<VerusVerifier> for RustStdStandard<std::collections::BTreeMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "k1 != k2"
    }
}

impl Ensures<VerusVerifier> for RustStdStandard<std::collections::BTreeMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::BTreeMap<i32, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::collections::BTreeMap<i32, i32>> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        verifier: "verus",
        kind: "requires",
        fragment: || BTREE_MAP_ROUND_TRIP_INPUTS_ARE_DISTINCT_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::collections::BTreeMap<i32, i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIP_HOLDS_VERUS_FRAGMENT,
    }
}

const VERIFY_BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/btree_carrier.rs");

const BTREE_SET_ROUND_TRIP_INPUTS_ARE_DISTINCT_VERUS_FRAGMENT: &str = r#"pub open spec fn btree_set_round_trip_inputs_are_distinct(a: int, b: int) -> bool {
    a != b
}"#;

const BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIP_HOLDS_VERUS_FRAGMENT: &str = r#"pub open spec fn btree_set_insert_contains_remove_round_trip_holds(result: bool) -> bool {
    result
}"#;

impl VerusWitness for RustStdStandard<std::collections::BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_btree_set_insert_contains_remove_round_trips",
            claim: VERIFY_BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIPS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BTreeSet<i32>>);

impl Requires<VerusVerifier> for RustStdStandard<std::collections::BTreeSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "a != b"
    }
}

impl Ensures<VerusVerifier> for RustStdStandard<std::collections::BTreeSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::BTreeSet<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::collections::BTreeSet<i32>> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        verifier: "verus",
        kind: "requires",
        fragment: || BTREE_SET_ROUND_TRIP_INPUTS_ARE_DISTINCT_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::collections::BTreeSet<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIP_HOLDS_VERUS_FRAGMENT,
    }
}

const VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_deque_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_deque_pushes_and_pops_from_both_ends",
            claim: VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::VecDeque<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::VecDeque<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TRY_RESERVE_PRESERVES_VEC_CONTENTS_REGARDLESS_OF_OUTCOME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/try_reserve_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_try_reserve_preserves_vec_contents_regardless_of_outcome",
            claim: VERIFY_TRY_RESERVE_PRESERVES_VEC_CONTENTS_REGARDLESS_OF_OUTCOME_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::TryReserveError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::TryReserveError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_VEC_INTO_ITER_ROUND_TRIPS_VIA_COLLECT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_into_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_into_iter_round_trips_via_collect",
            claim: VERIFY_VEC_INTO_ITER_ROUND_TRIPS_VIA_COLLECT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::IntoIter<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::vec::IntoIter<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_VEC_DEQUE_ITER_ROUND_TRIPS_VIA_COLLECT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_deque_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_deque_iter_round_trips_via_collect",
            claim: VERIFY_VEC_DEQUE_ITER_ROUND_TRIPS_VIA_COLLECT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_CHARS_YIELDS_CHARACTERS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/chars_carrier.rs");

const CHARS_INPUT_IS_AB_VERUS_FRAGMENT: &str = r#"pub open spec fn chars_input_is_ab(s: &str) -> bool {
    s@.len() == 2 && s@[0] == 'a' && s@[1] == 'b'
}"#;

const CHARS_ITERATION_YIELDS_A_THEN_B_THEN_NONE_VERUS_FRAGMENT: &str = r#"pub open spec fn chars_iteration_yields_a_then_b_then_none(
    result: (Option<char>, Option<char>, Option<char>),
) -> bool {
    result.0 == Some('a') && result.1 == Some('b') && result.2 is None
}"#;

impl VerusWitness for RustStdStandard<std::str::Chars<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_chars_yields_characters_in_order",
            claim: VERIFY_CHARS_YIELDS_CHARACTERS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::str::Chars<'static>>);

impl Requires<VerusVerifier> for RustStdStandard<std::str::Chars<'static>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "s@.len() == 2 && s@[0] == 'a' && s@[1] == 'b'"
    }
}

impl Ensures<VerusVerifier> for RustStdStandard<std::str::Chars<'static>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.0 == Some('a') && result.1 == Some('b') && result.2 is None"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::str::Chars<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::str::Chars<'static>> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        verifier: "verus",
        kind: "requires",
        fragment: || CHARS_INPUT_IS_AB_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::str::Chars<'static>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CHARS_ITERATION_YIELDS_A_THEN_B_THEN_NONE_VERUS_FRAGMENT,
    }
}

const VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/binary_heap_carrier.rs");

const BINARY_HEAP_MODEL_RECORDS_VALUES_IN_HEAP_ORDER_VERUS_FRAGMENT: &str = r#"pub open spec fn binary_heap_model_records_values_in_heap_order(
    observed_max: i32,
    observed_min: i32,
    a: i32,
    b: i32,
) -> bool {
    observed_max == if a >= b { a } else { b }
        && observed_min == if a >= b { b } else { a }
}"#;
const BINARY_HEAP_MODEL_POP_RETURNS_RECORDED_ORDER_VERUS_FRAGMENT: &str = r#"pub open spec fn binary_heap_model_pop_returns_recorded_order(
    first: i32,
    second: i32,
    max: i32,
    min: i32,
) -> bool {
    first == max && second == min
}"#;
const BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_VERUS_FRAGMENT: &str = r#"pub open spec fn binary_heap_pop_yields_the_maximum_first(
    first: int,
    second: int,
    a: int,
    b: int,
) -> bool {
    first == if a >= b { a } else { b }
        && second == if a >= b { b } else { a }
}"#;

impl VerusWitness for RustStdStandard<std::collections::BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_max_heap_pair_pops_the_maximum_first",
            claim: VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BinaryHeap<i32>>);

impl Ensures<VerusVerifier> for RustStdStandard<std::collections::BinaryHeap<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.0 == if a >= b { a } else { b } && result.1 == if a >= b { b } else { a }"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::BinaryHeap<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::collections::BinaryHeap<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || BINARY_HEAP_MODEL_RECORDS_VALUES_IN_HEAP_ORDER_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || BINARY_HEAP_MODEL_POP_RETURNS_RECORDED_ORDER_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_VERUS_FRAGMENT,
    }
}

const VERIFY_FIFO_QUEUE_PAIR_POPS_IN_PUSH_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/linked_list_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_fifo_queue_pair_pops_in_push_order",
            claim: VERIFY_FIFO_QUEUE_PAIR_POPS_IN_PUSH_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::LinkedList<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::LinkedList<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cell_carrier.rs");

const CELL_MODEL_NEW_STORES_INITIAL_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_new_stores_initial_value(observed: int, initial: int) -> bool {
    observed == initial
}"#;
const CELL_MODEL_GET_READS_CURRENT_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_get_reads_current_value(observed: int, current: int) -> bool {
    observed == current
}"#;
const CELL_MODEL_REPLACE_RETURNS_PREVIOUS_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_replace_returns_previous_value(observed: int, previous: int) -> bool {
    observed == previous
}"#;
const CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_HOLDS_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_get_set_replace_round_trip_holds(
    stores_initial: bool,
    set_overwrites: bool,
    replace_returns_old: bool,
    replace_stores_new: bool,
) -> bool {
    stores_initial && set_overwrites && replace_returns_old && replace_stores_new
}"#;

impl VerusWitness for RustStdStandard<std::cell::Cell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cell_model_get_set_replace_round_trip",
            claim: VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::Cell<i32>>);

impl Ensures<VerusVerifier> for RustStdStandard<std::cell::Cell<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.0 && result.1 && result.2 && result.3"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::Cell<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::cell::Cell<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CELL_MODEL_NEW_STORES_INITIAL_VALUE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CELL_MODEL_GET_READS_CURRENT_VALUE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CELL_MODEL_REPLACE_RETURNS_PREVIOUS_VALUE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_HOLDS_VERUS_FRAGMENT,
    }
}

/// [`WriteStoresNewValue`] reuses `Cell`'s own round-trip harness rather
/// than adding a new Verus proof: it names the shared write-through law
/// the harness already establishes.
impl VerusWitness for WriteStoresNewValue {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cell_model_get_set_replace_round_trip",
            claim: VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(WriteStoresNewValue);

impl Ensures<VerusVerifier> for WriteStoresNewValue {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "final (self) . value == new_value"
    }
}

const WRITE_STORES_NEW_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn write_stores_new_value(new_value: int, observed: int) -> bool {
    observed == new_value
}"#;

// `Ensures::ensures()` keeps the descriptive `final(self).value ==
// new_value` spelling as WriteStoresNewValue's canonical text, while the
// real Verus proof sites in `cell_carrier`, `ref_cell_carrier`,
// `unsafe_cell_carrier`, and `ordered_pair_iter_mut_carrier` call the
// shared `write_stores_new_value` spec fn defined in
// `amenable_verus::rust_std::cell_carrier`.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::WriteStoresNewValue",
        verifier: "verus",
        kind: "ensures",
        fragment: || <WriteStoresNewValue as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::WriteStoresNewValue",
        verifier: "verus",
        kind: "ensures",
        fragment: || WRITE_STORES_NEW_VALUE_VERUS_FRAGMENT,
    }
}

const VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/array_into_iter_carrier.rs");

/// [`ArrayIntoIterStartsAtFirstPosition`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the model's
/// initial-state law.
impl VerusWitness for ArrayIntoIterStartsAtFirstPosition {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_array_into_iter_model_yields_elements_in_order",
            claim: VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(ArrayIntoIterStartsAtFirstPosition);

impl Ensures<VerusVerifier> for ArrayIntoIterStartsAtFirstPosition {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.first == first && result.second == second && result.third == third && result.position == 0"
    }
}

const ARRAY_INTO_ITER_STARTS_AT_FIRST_POSITION_VERUS_FRAGMENT: &str = r#"pub open spec fn array_into_iter_model_starts_at_first_position(
    observed_first: i32,
    observed_second: i32,
    observed_third: i32,
    observed_position: u8,
    first: i32,
    second: i32,
    third: i32,
) -> bool {
    observed_first == first
        && observed_second == second
        && observed_third == third
        && observed_position == 0
}"#;

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArrayIntoIterStartsAtFirstPosition",
        verifier: "verus",
        kind: "ensures",
        fragment: || <ArrayIntoIterStartsAtFirstPosition as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArrayIntoIterStartsAtFirstPosition",
        verifier: "verus",
        kind: "ensures",
        fragment: || ARRAY_INTO_ITER_STARTS_AT_FIRST_POSITION_VERUS_FRAGMENT,
    }
}

/// [`ArrayIntoIterAdvanceMatchesPosition`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the model's
/// one-step transition law.
impl VerusWitness for ArrayIntoIterAdvanceMatchesPosition {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_array_into_iter_model_yields_elements_in_order",
            claim: VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(ArrayIntoIterAdvanceMatchesPosition);

impl Ensures<VerusVerifier> for ArrayIntoIterAdvanceMatchesPosition {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "old(self).position drives the yielded value, final position, and element preservation"
    }
}

const ARRAY_INTO_ITER_ADVANCE_MATCHES_POSITION_VERUS_FRAGMENT: &str = r#"pub open spec fn array_into_iter_advance_matches_position(
    old_position: u8,
    yielded: Option<i32>,
    final_position: u8,
    old_first: i32,
    old_second: i32,
    old_third: i32,
    final_first: i32,
    final_second: i32,
    final_third: i32,
) -> bool {
    (old_position == 0 ==> yielded == Some(old_first) && final_position == 1)
        && (old_position == 1 ==> yielded == Some(old_second) && final_position == 2)
        && (old_position == 2 ==> yielded == Some(old_third) && final_position == 3)
        && (old_position >= 3 ==> yielded is None && final_position == old_position)
        && final_first == old_first
        && final_second == old_second
        && final_third == old_third
}"#;

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArrayIntoIterAdvanceMatchesPosition",
        verifier: "verus",
        kind: "ensures",
        fragment: || <ArrayIntoIterAdvanceMatchesPosition as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ArrayIntoIterAdvanceMatchesPosition",
        verifier: "verus",
        kind: "ensures",
        fragment: || ARRAY_INTO_ITER_ADVANCE_MATCHES_POSITION_VERUS_FRAGMENT,
    }
}

/// [`YieldsThreeValuesInOrderThenEnds`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the
/// fixed-length consuming-iterator law the carrier already establishes.
impl VerusWitness for YieldsThreeValuesInOrderThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_array_into_iter_model_yields_elements_in_order",
            claim: VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(YieldsThreeValuesInOrderThenEnds);

impl Ensures<VerusVerifier> for YieldsThreeValuesInOrderThenEnds {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.0 == Some(first) && result.1 == Some(second) && result.2 == Some(third) && result.3 is None"
    }
}

const YIELDS_THREE_VALUES_IN_ORDER_THEN_ENDS_VERUS_FRAGMENT: &str = r#"pub open spec fn yields_three_values_in_order_then_ends(
    observed_first: Option<i32>,
    observed_second: Option<i32>,
    observed_third: Option<i32>,
    exhausted: Option<i32>,
    first: i32,
    second: i32,
    third: i32,
) -> bool {
    observed_first == Some(first)
        && observed_second == Some(second)
        && observed_third == Some(third)
        && exhausted is None
}"#;

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::YieldsThreeValuesInOrderThenEnds",
        verifier: "verus",
        kind: "ensures",
        fragment: || <YieldsThreeValuesInOrderThenEnds as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::YieldsThreeValuesInOrderThenEnds",
        verifier: "verus",
        kind: "ensures",
        fragment: || YIELDS_THREE_VALUES_IN_ORDER_THEN_ENDS_VERUS_FRAGMENT,
    }
}

impl VerusWitness for RustStdStandard<std::array::IntoIter<i32, 3>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_array_into_iter_model_yields_elements_in_order",
            claim: VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::array::IntoIter<i32, 3>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::array::IntoIter<i32, 3>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::array::IntoIter<i32, 3>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ref_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::RefCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ref_cell_model_dynamic_borrow_rules",
            claim: VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::RefCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::RefCell<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

impl Ensures<VerusVerifier> for RustStdStandard<std::cell::RefCell<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "final (self) . value == new_value"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::cell::RefCell<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

const VERIFY_ONCE_CELL_MODEL_INITIALIZES_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/once_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::OnceCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_once_cell_model_initializes_exactly_once",
            claim: VERIFY_ONCE_CELL_MODEL_INITIALIZES_EXACTLY_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::OnceCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::OnceCell<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::OnceCell<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_UNSAFE_CELL_MODEL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/unsafe_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::UnsafeCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_unsafe_cell_model_get_mut_and_into_inner_round_trip",
            claim: VERIFY_UNSAFE_CELL_MODEL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::UnsafeCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::UnsafeCell<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::UnsafeCell<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

impl Ensures<VerusVerifier> for RustStdStandard<std::cell::UnsafeCell<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "final (self) . value == new_value"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::UnsafeCell<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::cell::UnsafeCell<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

const VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/lazy_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_lazy_cell_model_caches_its_initializer_result",
            claim: VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_lazy_cell_model_caches_its_initializer_result",
            claim: VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/weak_carrier.rs");

impl VerusWitness for RustStdStandard<std::rc::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero",
            claim: VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::rc::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::rc::Weak<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::rc::Weak<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::sync::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero",
            claim: VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Weak<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::Weak<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_FROM_UTF8_ERROR_MODEL_RECOVERS_THE_ORIGINAL_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/from_utf8_error_carrier.rs");

const FROM_UTF8_ERROR_MODEL_NEW_PRESERVES_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf8_error_model_new_preserves_bytes(
    bytes: Vec<u8>,
    result: VerusFromUtf8ErrorModel,
) -> bool {
    result.bytes@ == bytes@
}"#;
const FROM_UTF8_ERROR_MODEL_AS_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf8_error_model_as_bytes_preserves_bytes(
    model: &VerusFromUtf8ErrorModel,
    result: &Vec<u8>,
) -> bool {
    result@ == model.bytes@
}"#;
const FROM_UTF8_ERROR_MODEL_INTO_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf8_error_model_into_bytes_preserves_bytes(
    model: VerusFromUtf8ErrorModel,
    result: Vec<u8>,
) -> bool {
    result@ == model.bytes@
}"#;

impl VerusWitness for RustStdStandard<std::string::FromUtf8Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_utf8_error_model_recovers_the_original_bytes",
            claim: VERIFY_FROM_UTF8_ERROR_MODEL_RECOVERS_THE_ORIGINAL_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::string::FromUtf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::string::FromUtf8Error> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FROM_UTF8_ERROR_MODEL_NEW_PRESERVES_BYTES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FROM_UTF8_ERROR_MODEL_AS_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FROM_UTF8_ERROR_MODEL_INTO_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    }
}

const VERIFY_ESCAPE_DEFAULT_MODEL_ESCAPES_A_CONTROL_BYTE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ascii_escape_carrier.rs");

impl VerusWitness for RustStdStandard<core::ascii::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_escape_default_model_escapes_a_control_byte",
            claim: VERIFY_ESCAPE_DEFAULT_MODEL_ESCAPES_A_CONTROL_BYTE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::ascii::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ascii::EscapeDefault>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::ascii::EscapeDefault> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/cstr_carrier.rs");

const CSTR_TO_BYTES_MATCHES_MODEL_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_to_bytes_matches_model(cstr: &CStr, result: &[u8]) -> bool {
    result@ == cstr_bytes_spec(cstr)
}"#;

impl VerusWitness for RustStdStandard<core::ffi::CStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cstr_excludes_the_terminating_nul_from_to_bytes",
            claim: VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::CStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::CStr>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::ffi::CStr> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ffi::CStr>",
        verifier: "verus",
        kind: "ensures",
        fragment: || CSTR_TO_BYTES_MATCHES_MODEL_VERUS_FRAGMENT,
    }
}

/// [`NonNulByte`] reuses the same harness rather than adding a new Verus
/// proof — it names the precondition every `CStr`/`CString`-family proof
/// in this crate already requires, it doesn't prove anything new.
impl VerusWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cstr_excludes_the_terminating_nul_from_to_bytes",
            claim: VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(NonNulByte);

const NON_NUL_BYTE_VALUE_IS_NONZERO_VERUS_FRAGMENT: &str = r#"pub open spec fn non_nul_byte_value_is_nonzero(byte: u8) -> bool {
    byte != 0
}"#;

impl Requires<VerusVerifier> for NonNulByte {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "byte != 0"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::NonNulByte",
        verifier: "verus",
        kind: "requires",
        fragment: || <NonNulByte as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::NonNulByte",
        verifier: "verus",
        kind: "requires",
        fragment: || NON_NUL_BYTE_VALUE_IS_NONZERO_VERUS_FRAGMENT,
    }
}

const VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ordered_pair_into_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_into_iter_model_yields_owned_values_in_order",
            claim: VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::Drain<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::vec::Drain<'static, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::vec_deque::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_into_iter_model_yields_owned_values_in_order",
            claim: VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::vec_deque::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::linked_list::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_into_iter_model_yields_owned_values_in_order",
            claim: VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::linked_list::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::linked_list::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_into_iter_model_yields_owned_values_in_order",
            claim: VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::linked_list::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::string::Drain<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_into_iter_model_yields_owned_values_in_order",
            claim: VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::string::Drain<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::string::Drain<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_extract_if_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_extract_if_model_partitions_by_the_predicate",
            claim: VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness
    for RustStdStandard<
        std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_extract_if_model_partitions_by_the_predicate",
            claim: VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_SPLICE_MODEL_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_splice_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_splice_model_replaces_a_range_and_yields_what_it_removed",
            claim: VERIFY_SPLICE_MODEL_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_map_model_applies_its_closure_to_each_item",
            claim: VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_FILTER_MODEL_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_filter_model_yields_only_items_matching_the_predicate",
            claim: VERIFY_FILTER_MODEL_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_FILTER_MAP_MODEL_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_filter_map_model_applies_and_filters_in_one_step",
            claim: VERIFY_FILTER_MAP_MODEL_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_MAP_WHILE_MODEL_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_map_while_model_maps_items_while_the_closure_returns_some",
            claim: VERIFY_MAP_WHILE_MODEL_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_CLONED_MODEL_CLONES_EACH_REFERENCED_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cloned_model_clones_each_referenced_item",
            claim: VERIFY_CLONED_MODEL_CLONES_EACH_REFERENCED_ITEM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_COPIED_MODEL_COPIES_EACH_REFERENCED_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_copied_model_copies_each_referenced_item",
            claim: VERIFY_COPIED_MODEL_COPIES_EACH_REFERENCED_ITEM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_CHAIN_MODEL_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_chain_model_sequences_two_iterators_end_to_end",
            claim: VERIFY_CHAIN_MODEL_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_ZIP_MODEL_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_zip_model_pairs_items_from_two_iterators",
            claim: VERIFY_ZIP_MODEL_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_enumerate_model_pairs_each_item_with_its_index",
            claim: VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

/// [`IncrementHeadroom`] reuses the same harness rather than adding a new
/// Verus proof — it names the precondition the harness already requires,
/// it doesn't prove anything new. Three supplementary fragments cover the
/// wider margin `slice_chunks_carrier`'s own models need (`a`/`b`/`c <=
/// i32::MAX - 10`, not registered through the `Requires` trait itself
/// since only one fragment can be the "canonical" one per type).
impl VerusWitness for IncrementHeadroom {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_enumerate_model_pairs_each_item_with_its_index",
            claim: VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(IncrementHeadroom);

impl Requires<VerusVerifier> for IncrementHeadroom {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "a < i32 :: MAX - 1"
    }
}

const INCREMENT_HEADROOM_HOLDS_VERUS_FRAGMENT: &str = r#"pub open spec fn increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX - 1
}"#;

const SINGLE_INCREMENT_HEADROOM_HOLDS_VERUS_FRAGMENT: &str = r#"pub open spec fn single_increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX
}"#;

const TEN_INCREMENT_HEADROOM_HOLDS_VERUS_FRAGMENT: &str = r#"pub open spec fn ten_increment_headroom_holds(a: i32) -> bool {
    a <= i32::MAX - 10
}"#;

// `Requires::requires()` still returns the tight-margin spelling as
// IncrementHeadroom's canonical descriptive text (for cross-backend
// enumeration), but no real site restates it raw anymore. Four sites
// need the tight, two-increment margin (verify_enumerate_model_pairs_
// each_item_with_its_index, verify_rev_model_reverses_iteration_order,
// verify_cycle_model_repeats_its_sequence_forever, verify_peekable_
// model_peek_does_not_consume) and call increment_headroom_holds
// directly; eight more need only the loosest, one-increment margin
// (verify_chain_model_sequences_two_iterators_end_to_end,
// verify_zip_model_pairs_items_from_two_iterators,
// verify_fuse_model_keeps_returning_none_once_exhausted,
// verify_inspect_model_calls_once_per_item_without_changing_values,
// verify_fn_pointer_model_calls_the_underlying_function,
// verify_map_model_applies_its_closure_to_each_item) and call
// single_increment_headroom_holds. The slice-chunk write-through models
// need a wider margin still and call ten_increment_headroom_holds. All
// three are real, shared `open spec fn`s in
// amenable_verus::rust_std::iter_sequence_carrier confirmed
// under real verus to give every call site genuine proof credit across
// carrier files -- see amenable_std::verus_gallery's
// cross_file_spec_fn_reuse_gets_real_proof_credit case.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IncrementHeadroom",
        verifier: "verus",
        kind: "requires",
        fragment: || <IncrementHeadroom as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IncrementHeadroom",
        verifier: "verus",
        kind: "requires",
        fragment: || INCREMENT_HEADROOM_HOLDS_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IncrementHeadroom",
        verifier: "verus",
        kind: "requires",
        fragment: || SINGLE_INCREMENT_HEADROOM_HOLDS_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::IncrementHeadroom",
        verifier: "verus",
        kind: "requires",
        fragment: || TEN_INCREMENT_HEADROOM_HOLDS_VERUS_FRAGMENT,
    }
}

/// [`ValueUnchanged`] reuses `RefCell`'s own borrow-rules harness rather
/// than adding a new Verus proof — the harness's own `ensures` clauses
/// already establish this frame condition for `try_borrow`/
/// `try_borrow_mut`/`release_shared`; `Weak::drop_strong` states the
/// identical claim, covered by the same fragment text (no variable-name
/// variance this time — all four sites use `self`/`value` verbatim).
impl VerusWitness for ValueUnchanged {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ref_cell_model_dynamic_borrow_rules",
            claim: VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(ValueUnchanged);

impl Ensures<VerusVerifier> for ValueUnchanged {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "final (self) . value == old (self) . value"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ValueUnchanged",
        verifier: "verus",
        kind: "ensures",
        fragment: || <ValueUnchanged as Ensures<VerusVerifier>>::ensures(()),
    }
}

/// [`ObservedValueMatchesInput`] reuses the shared-reference harness
/// rather than adding a new Verus proof — it names the direct identity
/// postcondition that many simple scalar-observation carriers now state
/// through one shared Verus `spec fn`.
impl VerusWitness for ObservedValueMatchesInput {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_shared_reference_model_dereferences_to_the_referent",
            claim: VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(ObservedValueMatchesInput);

impl Ensures<VerusVerifier> for ObservedValueMatchesInput {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result == value"
    }
}

const OBSERVED_VALUE_MATCHES_INPUT_VERUS_FRAGMENT: &str = r#"pub open spec fn observed_value_matches_input(observed: int, input: int) -> bool {
    observed == input
}"#;

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ObservedValueMatchesInput",
        verifier: "verus",
        kind: "ensures",
        fragment: || <ObservedValueMatchesInput as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::ObservedValueMatchesInput",
        verifier: "verus",
        kind: "ensures",
        fragment: || OBSERVED_VALUE_MATCHES_INPUT_VERUS_FRAGMENT,
    }
}

const VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_rev_model_reverses_iteration_order",
            claim: VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_skip_model_discards_the_first_n_items",
            claim: VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_skip_while_model_discards_items_while_the_predicate_holds",
            claim: VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_step_by_model_yields_every_nth_item",
            claim: VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Take<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_take_model_yields_at_most_n_items",
            claim: VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Take<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Take<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Take<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_take_while_model_yields_items_while_the_predicate_holds",
            claim: VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Once<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_once_model_yields_exactly_one_value",
            claim: VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Once<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Once<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::OnceWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_once_with_model_calls_its_closure_exactly_once",
            claim: VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::OnceWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::OnceWith<fn() -> i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::OnceWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Repeat<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_repeat_model_yields_the_same_value_forever",
            claim: VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Repeat<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Repeat<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_repeat_with_model_calls_its_closure_once_per_item",
            claim: VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::RepeatWith<fn() -> i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::RepeatWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatN<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_repeat_n_model_yields_the_value_exactly_n_times",
            claim: VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatN<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::RepeatN<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::RepeatN<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Empty<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_empty_model_yields_nothing",
            claim: VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Empty<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Empty<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Empty<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CYCLE_MODEL_REPEATS_ITS_SEQUENCE_FOREVER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cycle_model_repeats_its_sequence_forever",
            claim: VERIFY_CYCLE_MODEL_REPEATS_ITS_SEQUENCE_FOREVER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_FUSE_MODEL_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_fuse_model_keeps_returning_none_once_exhausted",
            claim: VERIFY_FUSE_MODEL_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_INSPECT_MODEL_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_inspect_model_calls_once_per_item_without_changing_values",
            claim: VERIFY_INSPECT_MODEL_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_PEEKABLE_MODEL_PEEK_DOES_NOT_CONSUME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_peekable_model_peek_does_not_consume",
            claim: VERIFY_PEEKABLE_MODEL_PEEK_DOES_NOT_CONSUME_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_SCAN_MODEL_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<
        std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_scan_model_threads_state_through_its_closure",
            claim: VERIFY_SCAN_MODEL_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_FLAT_MAP_MODEL_FLATTENS_EACH_GENERATED_ITERATOR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<
        std::iter::FlatMap<
            std::array::IntoIter<i32, 1>,
            std::ops::Range<i32>,
            fn(i32) -> std::ops::Range<i32>,
        >,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_flat_map_model_flattens_each_generated_iterator",
            claim: VERIFY_FLAT_MAP_MODEL_FLATTENS_EACH_GENERATED_ITERATOR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<
        std::iter::FlatMap<
            std::array::IntoIter<i32, 1>,
            std::ops::Range<i32>,
            fn(i32) -> std::ops::Range<i32>,
        >,
    >
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::FlatMap<std::array::IntoIter<i32, 1>, std::ops::Range<i32>, fn(i32) -> std::ops::Range<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::FlatMap<std::array::IntoIter<i32, 1>, std::ops::Range<i32>, fn(i32) -> std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_FLATTEN_MODEL_CONCATENATES_THE_INNER_ITERATORS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_flatten_model_concatenates_the_inner_iterators",
            claim: VERIFY_FLATTEN_MODEL_CONCATENATES_THE_INNER_ITERATORS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_successors_model_generates_from_the_previous_item",
            claim: VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_FROM_FN_MODEL_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_fn_model_yields_until_the_closure_returns_none",
            claim: VERIFY_FROM_FN_MODEL_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_ALIGNMENT_MODEL_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::Alignment> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_alignment_model_reaches_the_formatter_from_the_format_spec",
            claim: VERIFY_ALIGNMENT_MODEL_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Alignment>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::Alignment>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::Alignment> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_FORMATTER_MODEL_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::Formatter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_formatter_model_exposes_the_parsed_width_and_precision",
            claim: VERIFY_FORMATTER_MODEL_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Formatter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::Formatter<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::Formatter<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ARGUMENTS_MODEL_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");
const FMT_ARGUMENTS_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_arguments_result_matches_display_token(display_token: i32, result: i32) -> bool {
    result == display_token
}"#;
const FMT_FROM_FN_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_from_fn_result_matches_display_token(display_token: i32, result: i32) -> bool {
    result == display_token
}"#;
const FMT_DEBUG_STRUCT_RESULT_MATCHES_NAMED_FIELDS_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_struct_result_matches_named_fields(
    type_label: u8,
    field_label: u8,
    value_token: i32,
    result: (u8, u8, i32),
) -> bool {
    result == (type_label, field_label, value_token)
}"#;
const FMT_DEBUG_TUPLE_RESULT_MATCHES_POSITIONAL_FIELDS_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_tuple_result_matches_positional_fields(
    type_label: u8,
    value_token: i32,
    result: (u8, i32),
) -> bool {
    result == (type_label, value_token)
}"#;
const FMT_DEBUG_LIST_RESULT_MATCHES_ENTRIES_IN_BRACKETS_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_list_result_matches_entries_in_brackets(
    first_token: i32,
    second_token: i32,
    result: (i32, i32),
) -> bool {
    result == (first_token, second_token)
}"#;
const FMT_DEBUG_SET_RESULT_MATCHES_ENTRIES_IN_BRACES_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_set_result_matches_entries_in_braces(
    first_token: i32,
    second_token: i32,
    result: (i32, i32),
) -> bool {
    result == (first_token, second_token)
}"#;
const FMT_DEBUG_MAP_RESULT_MATCHES_KEY_VALUE_PAIR_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_map_result_matches_key_value_pair(
    key_label: u8,
    value_token: i32,
    result: (u8, i32),
) -> bool {
    result == (key_label, value_token)
}"#;

impl VerusWitness for RustStdStandard<std::fmt::Arguments<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_arguments_model_renders_the_same_as_the_value_itself",
            claim: VERIFY_ARGUMENTS_MODEL_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Arguments<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::Arguments<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::Arguments<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::Arguments<'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_ARGUMENTS_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT,
    }
}

const VERIFY_FROM_FN_MODEL_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_from_fn_model_forwards_display_to_the_supplied_closure",
            claim: VERIFY_FROM_FN_MODEL_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_FROM_FN_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT,
    }
}

const VERIFY_DEBUG_STRUCT_MODEL_RENDERS_NAMED_FIELDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugStruct<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_debug_struct_model_renders_named_fields",
            claim: VERIFY_DEBUG_STRUCT_MODEL_RENDERS_NAMED_FIELDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugStruct<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugStruct<'static, 'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::DebugStruct<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugStruct<'static, 'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_DEBUG_STRUCT_RESULT_MATCHES_NAMED_FIELDS_VERUS_FRAGMENT,
    }
}

const VERIFY_DEBUG_TUPLE_MODEL_RENDERS_POSITIONAL_FIELDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugTuple<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_debug_tuple_model_renders_positional_fields",
            claim: VERIFY_DEBUG_TUPLE_MODEL_RENDERS_POSITIONAL_FIELDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugTuple<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugTuple<'static, 'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::DebugTuple<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugTuple<'static, 'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_DEBUG_TUPLE_RESULT_MATCHES_POSITIONAL_FIELDS_VERUS_FRAGMENT,
    }
}

const VERIFY_DEBUG_LIST_MODEL_RENDERS_ENTRIES_IN_BRACKETS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugList<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_debug_list_model_renders_entries_in_brackets",
            claim: VERIFY_DEBUG_LIST_MODEL_RENDERS_ENTRIES_IN_BRACKETS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugList<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugList<'static, 'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::DebugList<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugList<'static, 'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_DEBUG_LIST_RESULT_MATCHES_ENTRIES_IN_BRACKETS_VERUS_FRAGMENT,
    }
}

const VERIFY_DEBUG_SET_MODEL_RENDERS_ENTRIES_IN_BRACES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugSet<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_debug_set_model_renders_entries_in_braces",
            claim: VERIFY_DEBUG_SET_MODEL_RENDERS_ENTRIES_IN_BRACES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugSet<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugSet<'static, 'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::DebugSet<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugSet<'static, 'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_DEBUG_SET_RESULT_MATCHES_ENTRIES_IN_BRACES_VERUS_FRAGMENT,
    }
}

const VERIFY_DEBUG_MAP_MODEL_RENDERS_KEY_VALUE_PAIRS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugMap<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_debug_map_model_renders_key_value_pairs",
            claim: VERIFY_DEBUG_MAP_MODEL_RENDERS_KEY_VALUE_PAIRS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugMap<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugMap<'static, 'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fmt::DebugMap<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fmt::DebugMap<'static, 'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FMT_DEBUG_MAP_RESULT_MATCHES_KEY_VALUE_PAIR_VERUS_FRAGMENT,
    }
}

const VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/discriminant_carrier.rs");

impl VerusWitness for RustStdStandard<std::mem::Discriminant<Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_discriminant_model_identifies_variant_not_payload",
            claim: VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::mem::Discriminant<Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::mem::Discriminant<Option<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::mem::Discriminant<Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

/// `RustStdStandard<NonZero<T>>`'s Verus proof states its claim as two
/// separate `ensures` clauses (`value != 0 ==> result`, `value == 0 ==>
/// !result`) — an iff split into its two implications, not one expression.
/// `Ensures<VerusVerifier>` still returns the first direction as the
/// canonical descriptive text for cross-backend enumeration, while the
/// real Verus proof sites call shared `non_zero_new_accepts_nonzero` and
/// `non_zero_new_rejects_zero` spec fns defined in
/// `amenable_verus::rust_std::non_zero_carrier`.
const NON_ZERO_NEW_ACCEPTS_NONZERO_VERUS_FRAGMENT: &str = r#"pub open spec fn non_zero_new_accepts_nonzero(value: int, result: bool) -> bool {
    value != 0 ==> result
}"#;

const NON_ZERO_NEW_REJECTS_ZERO_VERUS_FRAGMENT: &str = r#"pub open spec fn non_zero_new_rejects_zero(value: int, result: bool) -> bool {
    value == 0 ==> !result
}"#;

macro_rules! impl_non_zero_verus_witness {
    ($($ty:ty => $harness:literal),* $(,)?) => {
        $(
            impl VerusWitness for RustStdStandard<std::num::NonZero<$ty>> {
                type SupportingEvidence = Self;
                type ProofArtifact = VerusCheckedProof;

                fn proof() -> Self::ProofArtifact {
                    VerusCheckedProof {
                        harness: $harness,
                        claim: include_str!("../../amenable_verus/src/rust_std/non_zero_carrier.rs"),
                        provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                    }
                }
            }

            bridge_verus_witness!(RustStdStandard<std::num::NonZero<$ty>>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                    verifier: "verus",
                    describe: || <RustStdStandard<std::num::NonZero<$ty>> as VerusWitness>::proof().to_string(),
                }
            }

            impl Ensures<VerusVerifier> for RustStdStandard<std::num::NonZero<$ty>> {
                type Input = ();
                type Bound = &'static str;

                fn ensures(_: ()) -> &'static str {
                    "value != 0 ==> result"
                }
            }

            ::inventory::submit! {
                ::amenable_core::ContractRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                    verifier: "verus",
                    kind: "ensures",
                    fragment: || <RustStdStandard<std::num::NonZero<$ty>> as Ensures<VerusVerifier>>::ensures(()),
                }
            }

            ::inventory::submit! {
                ::amenable_core::ContractRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                    verifier: "verus",
                    kind: "ensures",
                    fragment: || NON_ZERO_NEW_ACCEPTS_NONZERO_VERUS_FRAGMENT,
                }
            }

            ::inventory::submit! {
                ::amenable_core::ContractRecord {
                    evidence: concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                    verifier: "verus",
                    kind: "ensures",
                    fragment: || NON_ZERO_NEW_REJECTS_ZERO_VERUS_FRAGMENT,
                }
            }
        )*
    };
}

const VERIFY_ITER_MODEL_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/slice_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_iter_model_yields_shared_references_in_order",
            claim: VERIFY_ITER_MODEL_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::slice::Iter<'static, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ITER_MUT_MODEL_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/slice_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_iter_mut_model_yields_mutable_references_that_write_through",
            claim: VERIFY_ITER_MUT_MODEL_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::slice::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_IPV4_ADDR_MODEL_OCTETS_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Ipv4Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ipv4_addr_model_octets_round_trip",
            claim: VERIFY_IPV4_ADDR_MODEL_OCTETS_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Ipv4Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::Ipv4Addr>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::Ipv4Addr> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_IPV6_ADDR_MODEL_SEGMENTS_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Ipv6Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ipv6_addr_model_segments_round_trip",
            claim: VERIFY_IPV6_ADDR_MODEL_SEGMENTS_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Ipv6Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::Ipv6Addr>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::Ipv6Addr> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_IP_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::IpAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ip_addr_model_variant_matches_its_kind",
            claim: VERIFY_IP_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::IpAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::IpAddr>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::IpAddr> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SOCKET_ADDR_V4_MODEL_ROUND_TRIPS_IP_AND_PORT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddrV4> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_socket_addr_v4_model_round_trips_ip_and_port",
            claim: VERIFY_SOCKET_ADDR_V4_MODEL_ROUND_TRIPS_IP_AND_PORT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddrV4>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::SocketAddrV4>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::SocketAddrV4> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SOCKET_ADDR_V6_MODEL_ROUND_TRIPS_ALL_FIELDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddrV6> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_socket_addr_v6_model_round_trips_all_fields",
            claim: VERIFY_SOCKET_ADDR_V6_MODEL_ROUND_TRIPS_ALL_FIELDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddrV6>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::SocketAddrV6>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::SocketAddrV6> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SOCKET_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_socket_addr_model_variant_matches_its_kind",
            claim: VERIFY_SOCKET_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::SocketAddr>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::SocketAddr> as VerusWitness>::proof().to_string()
        },
    }
}

impl_non_zero_verus_witness!(
    i8 => "verify_non_zero_i8_model_round_trips_iff_nonzero",
    i16 => "verify_non_zero_i16_model_round_trips_iff_nonzero",
    i32 => "verify_non_zero_i32_model_round_trips_iff_nonzero",
    i64 => "verify_non_zero_i64_model_round_trips_iff_nonzero",
    i128 => "verify_non_zero_i128_model_round_trips_iff_nonzero",
    isize => "verify_non_zero_isize_model_round_trips_iff_nonzero",
    u8 => "verify_non_zero_u8_model_round_trips_iff_nonzero",
    u16 => "verify_non_zero_u16_model_round_trips_iff_nonzero",
    u32 => "verify_non_zero_u32_model_round_trips_iff_nonzero",
    u64 => "verify_non_zero_u64_model_round_trips_iff_nonzero",
    u128 => "verify_non_zero_u128_model_round_trips_iff_nonzero",
    usize => "verify_non_zero_usize_model_round_trips_iff_nonzero",
);

const VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ordered_pair_iter_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_iter_mut_model_writes_through_in_order",
            claim: VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ordered_pair_iter_mut_model_writes_through_in_order",
            claim: VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/unordered_pair_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_unordered_pair_model_yields_every_element_once",
            claim: VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_unordered_pair_model_yields_every_element_once",
            claim: VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::binary_heap::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_unordered_pair_model_yields_every_element_once",
            claim: VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_max_heap_pair_pops_the_maximum_first",
            claim: VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_REF_MODEL_DEREFS_TO_THE_BORROWED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ref_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::Ref<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ref_model_derefs_to_the_borrowed_value",
            claim: VERIFY_REF_MODEL_DEREFS_TO_THE_BORROWED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::Ref<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::Ref<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::Ref<'static, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_REF_MUT_MODEL_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ref_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::RefMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ref_mut_model_derefs_and_writes_through_to_the_cell",
            claim: VERIFY_REF_MUT_MODEL_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::RefMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::cell::RefMut<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::cell::RefMut<'static, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/decode_utf16_carrier.rs");

const DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_unit_is_non_surrogate(unit: u16) -> bool {
    unit < 0xD800 || unit > 0xDFFF
}"#;
const DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_unit_is_surrogate(unit: u16) -> bool {
    0xD800 <= unit <= 0xDFFF
}"#;
const DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_bmp_unit_decodes_to_same_scalar(
    unit: u16,
    result: Option<u32>,
) -> bool {
    result == Some(unit as u32)
}"#;

const DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_lone_surrogate_reports_same_unit(
    unit: u16,
    result: Result<u32, u16>,
) -> bool {
    result == Err(unit)
}"#;

const DECODE_UTF16_TEST_INPUTS_COVER_BOTH_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_test_inputs_cover_both_cases(
    bmp_unit: u16,
    lone_surrogate: u16,
) -> bool {
    decode_utf16_unit_is_non_surrogate(bmp_unit)
        && decode_utf16_unit_is_surrogate(lone_surrogate)
}"#;

impl VerusWitness for RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates",
            claim: VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>);

impl Requires<VerusVerifier>
    for RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>
{
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "(bmp_unit < 0xD800 || bmp_unit > 0xDFFF) && 0xD800 <= lone_surrogate <= 0xDFFF"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        kind: "requires",
        fragment: || DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        kind: "requires",
        fragment: || DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        kind: "requires",
        fragment: || DECODE_UTF16_TEST_INPUTS_COVER_BOTH_CASES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    }
}

impl VerusWitness for RustStdStandard<std::char::DecodeUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates",
            claim: VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::DecodeUtf16Error>);

impl Requires<VerusVerifier> for RustStdStandard<std::char::DecodeUtf16Error> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "(bmp_unit < 0xD800 || bmp_unit > 0xDFFF) && 0xD800 <= lone_surrogate <= 0xDFFF"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::DecodeUtf16Error> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        kind: "requires",
        fragment: || <RustStdStandard<std::char::DecodeUtf16Error> as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        kind: "requires",
        fragment: || DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        kind: "requires",
        fragment: || DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        kind: "requires",
        fragment: || DECODE_UTF16_TEST_INPUTS_COVER_BOTH_CASES_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        verifier: "verus",
        kind: "ensures",
        fragment: || DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    }
}

const VERIFY_TO_LOWERCASE_MODEL_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::ToLowercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_to_lowercase_model_maps_an_uppercase_ascii_letter",
            claim: VERIFY_TO_LOWERCASE_MODEL_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::ToLowercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::ToLowercase>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::ToLowercase> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TO_UPPERCASE_MODEL_MAPS_A_LOWERCASE_ASCII_LETTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::ToUppercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_to_uppercase_model_maps_a_lowercase_ascii_letter",
            claim: VERIFY_TO_UPPERCASE_MODEL_MAPS_A_LOWERCASE_ASCII_LETTER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::ToUppercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::ToUppercase>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::ToUppercase> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHAR_ESCAPE_DEBUG_MODEL_ESCAPES_A_NEWLINE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeDebug> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_char_escape_debug_model_escapes_a_newline",
            claim: VERIFY_CHAR_ESCAPE_DEBUG_MODEL_ESCAPES_A_NEWLINE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeDebug>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::EscapeDebug>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::EscapeDebug> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHAR_ESCAPE_DEFAULT_MODEL_ESCAPES_A_NEWLINE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_char_escape_default_model_escapes_a_newline",
            claim: VERIFY_CHAR_ESCAPE_DEFAULT_MODEL_ESCAPES_A_NEWLINE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::EscapeDefault>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::EscapeDefault> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHAR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeUnicode> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_char_escape_unicode_model_renders_the_codepoint_escape",
            claim: VERIFY_CHAR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeUnicode>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::char::EscapeUnicode>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::char::EscapeUnicode> as VerusWitness>::proof().to_string()
        },
    }
}

macro_rules! impl_slice_chunks_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/slice_chunks_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

const TEN_INCREMENT_WRITE_THROUGH_VERUS_FRAGMENT: &str = r#"pub open spec fn ten_increment_write_through(before: int, after: int) -> bool {
    after == before + 10
}"#;

macro_rules! register_slice_chunks_increment_fragment {
    ($ty:ty) => {
        ::inventory::submit! {
            ::amenable_core::ContractRecord {
                evidence: concat!(
                    "amenable_std::rust_std::RustStdStandard<",
                    stringify!($ty),
                    ">"
                ),
                verifier: "verus",
                kind: "ensures",
                fragment: || TEN_INCREMENT_WRITE_THROUGH_VERUS_FRAGMENT,
            }
        }
    };
}

impl_slice_chunks_verus_witness!(
    std::slice::Chunks<'static, i32>,
    "verify_chunks_model_yields_non_overlapping_groups_with_a_short_last_chunk",
    VERIFY_CHUNKS_MODEL_YIELDS_NON_OVERLAPPING_GROUPS_WITH_A_SHORT_LAST_CHUNK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::ChunksExact<'static, i32>,
    "verify_chunks_exact_model_discards_a_short_remainder",
    VERIFY_CHUNKS_EXACT_MODEL_DISCARDS_A_SHORT_REMAINDER_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::ChunksMut<'static, i32>,
    "verify_chunks_mut_model_writes_through_every_chunk",
    VERIFY_CHUNKS_MUT_MODEL_WRITES_THROUGH_EVERY_CHUNK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::ChunksExactMut<'static, i32>,
    "verify_chunks_exact_mut_model_leaves_the_remainder_untouched",
    VERIFY_CHUNKS_EXACT_MUT_MODEL_LEAVES_THE_REMAINDER_UNTOUCHED_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunks<'static, i32>,
    "verify_rchunks_model_groups_from_the_back",
    VERIFY_RCHUNKS_MODEL_GROUPS_FROM_THE_BACK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunksExact<'static, i32>,
    "verify_rchunks_exact_model_discards_a_short_remainder_at_the_front",
    VERIFY_RCHUNKS_EXACT_MODEL_DISCARDS_A_SHORT_REMAINDER_AT_THE_FRONT_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunksExactMut<'static, i32>,
    "verify_rchunks_exact_mut_model_leaves_the_front_remainder_untouched",
    VERIFY_RCHUNKS_EXACT_MUT_MODEL_LEAVES_THE_FRONT_REMAINDER_UNTOUCHED_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunksMut<'static, i32>,
    "verify_rchunks_mut_model_writes_through_every_chunk",
    VERIFY_RCHUNKS_MUT_MODEL_WRITES_THROUGH_EVERY_CHUNK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::Windows<'static, i32>,
    "verify_windows_model_yields_overlapping_slices",
    VERIFY_WINDOWS_MODEL_YIELDS_OVERLAPPING_SLICES_SRC
);

register_slice_chunks_increment_fragment!(std::slice::ChunksMut<'static, i32>);
register_slice_chunks_increment_fragment!(std::slice::ChunksExactMut<'static, i32>);
register_slice_chunks_increment_fragment!(std::slice::RChunksExactMut<'static, i32>);
register_slice_chunks_increment_fragment!(std::slice::RChunksMut<'static, i32>);

const VERIFY_CHUNK_BY_MODEL_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/slice_chunk_by_carrier.rs");

macro_rules! impl_chunk_by_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate",
                    claim: VERIFY_CHUNK_BY_MODEL_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_chunk_by_verus_witness!(std::slice::ChunkBy<'static, i32, fn(&i32, &i32) -> bool>);
impl_chunk_by_verus_witness!(std::slice::ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>);

macro_rules! impl_slice_split_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/slice_split_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_slice_split_verus_witness!(
    std::slice::Split<'static, i32, fn(&i32) -> bool>,
    "verify_split_model_yields_subslices_between_matches",
    VERIFY_SPLIT_MODEL_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitMut<'static, i32, fn(&i32) -> bool>,
    "verify_split_mut_model_writes_through_the_first_piece",
    VERIFY_SPLIT_MUT_MODEL_WRITES_THROUGH_THE_FIRST_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>,
    "verify_split_inclusive_model_keeps_the_match_at_the_end_of_each_piece",
    VERIFY_SPLIT_INCLUSIVE_MODEL_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
    "verify_split_inclusive_mut_model_keeps_the_match_at_the_end_of_each_piece",
    VERIFY_SPLIT_INCLUSIVE_MUT_MODEL_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitN<'static, i32, fn(&i32) -> bool>,
    "verify_split_n_model_caps_the_number_of_pieces",
    VERIFY_SPLIT_N_MODEL_CAPS_THE_NUMBER_OF_PIECES_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitNMut<'static, i32, fn(&i32) -> bool>,
    "verify_split_n_model_caps_the_number_of_pieces",
    VERIFY_SPLIT_N_MUT_MODEL_CAPS_THE_NUMBER_OF_PIECES_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplit<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_model_yields_subslices_from_the_back",
    VERIFY_RSPLIT_MODEL_YIELDS_SUBSLICES_FROM_THE_BACK_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplitMut<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_mut_model_writes_through_the_rearmost_piece",
    VERIFY_RSPLIT_MUT_MODEL_WRITES_THROUGH_THE_REARMOST_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplitN<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back",
    VERIFY_RSPLIT_N_MODEL_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplitNMut<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back",
    VERIFY_RSPLIT_N_MUT_MODEL_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC
);

const VERIFY_ESCAPE_ASCII_MODEL_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/escape_ascii_carrier.rs");
const ESCAPE_ASCII_INPUT_IS_PRINTABLE_ASCII_VERUS_FRAGMENT: &str = r#"pub open spec fn escape_ascii_input_is_printable_ascii(printable: u8) -> bool {
    32 <= printable && printable <= 126
}"#;
const ESCAPE_ASCII_RESULT_MATCHES_PRINTABLE_PLUS_NEWLINE_ESCAPE_VERUS_FRAGMENT: &str = r#"pub open spec fn escape_ascii_result_matches_printable_plus_newline_escape(
    printable: u8,
    result: (u8, u8, u8),
) -> bool {
    result.0 == printable && result.1 == 92 && result.2 == 110
}"#;

impl VerusWitness for RustStdStandard<std::slice::EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_escape_ascii_model_leaves_printable_bytes_unescaped",
            claim: VERIFY_ESCAPE_ASCII_MODEL_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::EscapeAscii<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::slice::EscapeAscii<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        verifier: "verus",
        kind: "requires",
        fragment: || ESCAPE_ASCII_INPUT_IS_PRINTABLE_ASCII_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || ESCAPE_ASCII_RESULT_MATCHES_PRINTABLE_PLUS_NEWLINE_ESCAPE_VERUS_FRAGMENT,
    }
}

const VERIFY_GET_DISJOINT_MUT_MODEL_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/get_disjoint_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_get_disjoint_mut_model_rejects_overlap_and_out_of_bounds",
            claim: VERIFY_GET_DISJOINT_MUT_MODEL_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::GetDisjointMutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::slice::GetDisjointMutError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::slice::GetDisjointMutError> as VerusWitness>::proof().to_string()
        },
    }
}

macro_rules! impl_str_ascii_iter_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_ascii_iter_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_ascii_iter_verus_witness!(
    std::str::Bytes<'static>,
    "verify_bytes_model_yields_the_utf8_encoding",
    VERIFY_BYTES_MODEL_YIELDS_THE_UTF8_ENCODING_SRC
);
impl_str_ascii_iter_verus_witness!(
    std::str::CharIndices<'static>,
    "verify_char_indices_model_pairs_each_char_with_its_byte_offset",
    VERIFY_CHAR_INDICES_MODEL_PAIRS_EACH_CHAR_WITH_ITS_BYTE_OFFSET_SRC
);
impl_str_ascii_iter_verus_witness!(
    std::str::EncodeUtf16<'static>,
    "verify_encode_utf16_model_yields_utf16_code_units",
    VERIFY_ENCODE_UTF16_MODEL_YIELDS_UTF16_CODE_UNITS_SRC
);

macro_rules! impl_str_escape_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_escape_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_escape_verus_witness!(
    std::str::EscapeDebug<'static>,
    "verify_str_escape_debug_model_escapes_control_characters",
    VERIFY_STR_ESCAPE_DEBUG_MODEL_ESCAPES_CONTROL_CHARACTERS_SRC
);
impl_str_escape_verus_witness!(
    std::str::EscapeDefault<'static>,
    "verify_str_escape_default_model_escapes_control_characters",
    VERIFY_STR_ESCAPE_DEFAULT_MODEL_ESCAPES_CONTROL_CHARACTERS_SRC
);
impl_str_escape_verus_witness!(
    std::str::EscapeUnicode<'static>,
    "verify_str_escape_unicode_model_renders_the_codepoint_escape",
    VERIFY_STR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC
);

const VERIFY_LINES_MODEL_SPLITS_ON_LINE_ENDINGS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::Lines<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_lines_model_splits_on_line_endings",
            claim: VERIFY_LINES_MODEL_SPLITS_ON_LINE_ENDINGS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::str::Lines<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::str::Lines<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_LINES_ANY_MODEL_SPLITS_ON_ANY_LINE_ENDING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::LinesAny<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_lines_any_model_splits_on_any_line_ending",
            claim: VERIFY_LINES_ANY_MODEL_SPLITS_ON_ANY_LINE_ENDING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::str::LinesAny<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::LinesAny<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::str::LinesAny<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

macro_rules! impl_str_whitespace_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_whitespace_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_whitespace_verus_witness!(
    std::str::SplitAsciiWhitespace<'static>,
    "verify_split_ascii_whitespace_model_collapses_runs_of_whitespace",
    VERIFY_SPLIT_ASCII_WHITESPACE_MODEL_COLLAPSES_RUNS_OF_WHITESPACE_SRC
);
impl_str_whitespace_verus_witness!(
    std::str::SplitWhitespace<'static>,
    "verify_split_whitespace_model_collapses_runs_of_whitespace",
    VERIFY_SPLIT_WHITESPACE_MODEL_COLLAPSES_RUNS_OF_WHITESPACE_SRC
);

macro_rules! impl_str_utf8_chunks_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_utf8_chunks_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Chunks<'static>,
    "verify_utf8_chunks_model_yields_one_chunk_for_wholly_valid_input",
    VERIFY_UTF8_CHUNKS_MODEL_YIELDS_ONE_CHUNK_FOR_WHOLLY_VALID_INPUT_SRC
);
impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Chunk<'static>,
    "verify_utf8_chunk_model_separates_the_valid_prefix_from_invalid_bytes",
    VERIFY_UTF8_CHUNK_MODEL_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC
);
impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Error,
    "verify_utf8_error_model_reports_the_valid_prefix_length_and_error_span",
    VERIFY_UTF8_ERROR_MODEL_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC
);

macro_rules! impl_str_pattern_split_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_pattern_split_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_pattern_split_verus_witness!(
    std::str::Split<'static, char>,
    "verify_str_split_model_yields_substrings_between_pattern_matches",
    VERIFY_STR_SPLIT_MODEL_YIELDS_SUBSTRINGS_BETWEEN_PATTERN_MATCHES_SRC
);
impl_str_pattern_split_verus_witness!(
    std::str::SplitN<'static, char>,
    "verify_str_splitn_model_limits_to_n_substrings",
    VERIFY_STR_SPLITN_MODEL_LIMITS_TO_N_SUBSTRINGS_SRC
);
impl_str_pattern_split_verus_witness!(
    std::str::SplitInclusive<'static, char>,
    "verify_str_split_inclusive_model_keeps_the_delimiter_attached",
    VERIFY_STR_SPLIT_INCLUSIVE_MODEL_KEEPS_THE_DELIMITER_ATTACHED_SRC
);

macro_rules! impl_str_pattern_reverse_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_pattern_reverse_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_pattern_reverse_verus_witness!(
    std::str::RSplit<'static, char>,
    "verify_str_rsplit_model_yields_substrings_from_the_back",
    VERIFY_STR_RSPLIT_MODEL_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC
);
impl_str_pattern_reverse_verus_witness!(
    std::str::RSplitN<'static, char>,
    "verify_str_rsplitn_model_limits_to_n_substrings_from_the_back",
    VERIFY_STR_RSPLITN_MODEL_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC
);

macro_rules! impl_str_pattern_terminator_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_pattern_terminator_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_pattern_terminator_verus_witness!(
    std::str::SplitTerminator<'static, char>,
    "verify_str_split_terminator_model_suppresses_a_trailing_empty_substring",
    VERIFY_STR_SPLIT_TERMINATOR_MODEL_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC
);
impl_str_pattern_terminator_verus_witness!(
    std::str::RSplitTerminator<'static, char>,
    "verify_str_rsplit_terminator_model_suppresses_a_trailing_empty_substring_from_the_back",
    VERIFY_STR_RSPLIT_TERMINATOR_MODEL_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC
);

const VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_pattern_match_carrier.rs");

macro_rules! impl_str_matches_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_str_matches_model_yields_every_non_overlapping_occurrence",
                    claim: VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_str_matches_verus_witness!(std::str::Matches<'static, char>);
impl_str_matches_verus_witness!(std::str::RMatches<'static, char>);

/// [`AsciiByte`] reuses the same harness rather than adding a new Verus
/// proof — it names the precondition the harness already requires, it
/// doesn't prove anything new. The precondition recurs across six carrier
/// files under six different local variable names (`pattern`, `c`, `a`,
/// `b`, `before`, `after`) for the identical claim; `Requires<VerusVerifier>`
/// carries the `pattern` spelling (this harness's own), and the other five
/// are registered directly as supplementary `ContractRecord`s.
impl VerusWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_str_matches_model_yields_every_non_overlapping_occurrence",
            claim: VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(AsciiByte);

impl Requires<VerusVerifier> for AsciiByte {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        "(pattern as u32) < 128"
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || <AsciiByte as Requires<VerusVerifier>>::requires(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || "(c as u32) < 128",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || "(a as u32) < 128",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || "(b as u32) < 128",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || "(before as u32) < 128",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || "(after as u32) < 128",
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::AsciiByte",
        verifier: "verus",
        kind: "requires",
        fragment: || "byte < 128",
    }
}

const VERIFY_STR_MATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_pattern_match_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::MatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_str_match_indices_model_pairs_each_match_with_its_byte_offset",
            claim: VERIFY_STR_MATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::str::MatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::MatchIndices<'static, char>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::str::MatchIndices<'static, char>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_STR_RMATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC:
    &str = include_str!("../../amenable_verus/src/rust_std/str_pattern_match_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::RMatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_str_rmatch_indices_model_pairs_each_match_with_its_byte_offset_from_the_back",
            claim: VERIFY_STR_RMATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::str::RMatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::str::RMatchIndices<'static, char>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::str::RMatchIndices<'static, char>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_BUF_READER_MODEL_READS_THE_UNDERLYING_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_buffered_read_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::BufReader<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_buf_reader_model_reads_the_underlying_bytes",
            claim: VERIFY_BUF_READER_MODEL_READS_THE_UNDERLYING_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::BufReader<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::BufReader<&'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::BufReader<&'static [u8]>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_BUF_WRITER_MODEL_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_buf_writer_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::BufWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_buf_writer_model_flushes_to_the_underlying_writer",
            claim: VERIFY_BUF_WRITER_MODEL_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::BufWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::BufWriter<Vec<u8>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::BufWriter<Vec<u8>>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_BYTES_MODEL_YIELDS_ONE_BYTE_AT_A_TIME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_bytes_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Bytes<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_bytes_model_yields_one_byte_at_a_time",
            claim: VERIFY_BYTES_MODEL_YIELDS_ONE_BYTE_AT_A_TIME_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Bytes<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Bytes<&'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Bytes<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_INTO_INNER_ERROR_MODEL_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_into_inner_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_into_inner_error_model_recovers_the_writer_and_the_flush_error",
            claim: VERIFY_INTO_INNER_ERROR_MODEL_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_LINE_WRITER_MODEL_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_line_writer_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::LineWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_line_writer_model_flushes_on_a_newline_but_not_before_one",
            claim: VERIFY_LINE_WRITER_MODEL_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::LineWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::LineWriter<Vec<u8>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::LineWriter<Vec<u8>>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_LINES_MODEL_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Lines<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_lines_model_splits_on_newlines_and_drops_the_terminator",
            claim: VERIFY_LINES_MODEL_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Lines<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Lines<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_pipe_carrier.rs");

macro_rules! impl_io_pipe_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_pipe_model_delivers_written_bytes_to_the_paired_reader",
                    claim: VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_io_pipe_verus_witness!(std::io::PipeReader);
impl_io_pipe_verus_witness!(std::io::PipeWriter);

const VERIFY_SPLIT_MODEL_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_split_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Split<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_split_model_segments_on_the_given_byte_and_drops_it",
            claim: VERIFY_SPLIT_MODEL_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Split<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Split<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_WRITER_PANICKED_MODEL_RECOVERS_THE_BUFFERED_DATA_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_writer_panicked_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::WriterPanicked> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_writer_panicked_model_recovers_the_buffered_data",
            claim: VERIFY_WRITER_PANICKED_MODEL_RECOVERS_THE_BUFFERED_DATA_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::WriterPanicked>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::WriterPanicked>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::WriterPanicked> as VerusWitness>::proof().to_string()
        },
    }
}

macro_rules! impl_io_empty_repeat_sink_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/io_empty_repeat_sink_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_io_empty_repeat_sink_verus_witness!(
    std::io::Empty,
    "verify_empty_model_read_reports_end_of_file",
    VERIFY_EMPTY_MODEL_READ_REPORTS_END_OF_FILE_SRC
);
impl_io_empty_repeat_sink_verus_witness!(
    std::io::Repeat,
    "verify_repeat_model_fills_the_buffer_with_the_given_byte",
    VERIFY_REPEAT_MODEL_FILLS_THE_BUFFER_WITH_THE_GIVEN_BYTE_SRC
);
impl_io_empty_repeat_sink_verus_witness!(
    std::io::Sink,
    "verify_sink_model_write_reports_full_length_and_discards_content",
    VERIFY_SINK_MODEL_WRITE_REPORTS_FULL_LENGTH_AND_DISCARDS_CONTENT_SRC
);

const VERIFY_SEEK_FROM_MODEL_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_seek_from_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::SeekFrom> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_seek_from_model_round_trips_each_variants_offset",
            claim: VERIFY_SEEK_FROM_MODEL_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::SeekFrom>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::SeekFrom> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHAIN_MODEL_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_chain_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_chain_model_reads_the_first_source_then_the_second",
            claim: VERIFY_CHAIN_MODEL_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_CURSOR_MODEL_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_cursor_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Cursor<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_cursor_model_read_advances_position_and_seek_repositions_it",
            claim: VERIFY_CURSOR_MODEL_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Cursor<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Cursor<&'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Cursor<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ERROR_MODEL_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_error_model_from_error_kind_preserves_the_kind",
            claim: VERIFY_ERROR_MODEL_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Error>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Error> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_IO_SLICE_MODEL_DEREFS_TO_THE_WRAPPED_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IoSlice<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_io_slice_model_derefs_to_the_wrapped_bytes",
            claim: VERIFY_IO_SLICE_MODEL_DEREFS_TO_THE_WRAPPED_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IoSlice<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::IoSlice<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::IoSlice<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_IO_SLICE_MUT_MODEL_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IoSliceMut<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_io_slice_mut_model_derefs_to_and_permits_mutating_the_wrapped_bytes",
            claim: VERIFY_IO_SLICE_MUT_MODEL_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IoSliceMut<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::IoSliceMut<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::IoSliceMut<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TAKE_MODEL_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_take_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Take<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_take_model_caps_reads_at_the_remaining_limit",
            claim: VERIFY_TAKE_MODEL_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Take<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::io::Take<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    }
}

/// Every width's Verus accommodation model states the identical claim
/// (`result == (initial, next)`) since the model function is a plain
/// echo of its own two parameters — trivially true by construction, but
/// still a real, named round-trip claim about the atomic-model type, not
/// scanner-level noise (unlike a bare `result`, whose *content* is
/// invisible to the clause): `Ensures<VerusVerifier>` names it once here
/// rather than at each of the eleven widths' own `ensures` clauses.
macro_rules! impl_sync_atomic_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/sync_atomic_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: $harness,
                    claim: $const_name,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }

        impl Ensures<VerusVerifier> for RustStdStandard<$ty> {
            type Input = ();
            type Bound = &'static str;

            fn ensures(_: ()) -> &'static str {
                "result == (initial, next)"
            }
        }

        ::inventory::submit! {
            ::amenable_core::ContractRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                kind: "ensures",
                fragment: || <RustStdStandard<$ty> as Ensures<VerusVerifier>>::ensures(()),
            }
        }
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

const VERIFY_ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/atomic_ptr_carrier.rs");

const ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_VERUS_FRAGMENT: &str = r#"pub open spec fn atomic_ptr_model_load_store_swap_and_compare_exchange(
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

impl VerusWitness for RustStdStandard<std::sync::atomic::AtomicPtr<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_atomic_ptr_model_load_store_swap_and_compare_exchange",
            claim: VERIFY_ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::atomic::AtomicPtr<i32>>);

impl Ensures<VerusVerifier> for RustStdStandard<std::sync::atomic::AtomicPtr<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "result.0 == initial && result.1 == stored && result.2 == stored && result.3 == swapped_in && result.4 == swapped_in && result.5 == exchange_target"
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::atomic::AtomicPtr<i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<std::sync::atomic::AtomicPtr<i32>> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        verifier: "verus",
        kind: "ensures",
        fragment: || ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_VERUS_FRAGMENT,
    }
}

const VERIFY_ATOMIC_ORDERING_MODEL_RELAXED_STORE_IS_OBSERVABLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_atomic_ordering_carrier.rs");

// Bare `Ordering`, matching `amenable_std::rust_std::sync_atomic`'s own
// registration and the Kani/Creusot witnesses' evidence strings for the
// same type — this is `core::sync::atomic::Ordering`, not
// `std::cmp::Ordering` (see `ordering_carrier.rs`/that type's own
// witness above for the comparison-result enum).
impl VerusWitness for RustStdStandard<std::sync::atomic::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_atomic_ordering_model_relaxed_store_is_observable",
            claim: VERIFY_ATOMIC_ORDERING_MODEL_RELAXED_STORE_IS_OBSERVABLE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::atomic::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ordering>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::atomic::Ordering> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHILD_MODEL_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Child> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_child_model_has_a_process_id_and_can_be_waited_on",
            claim: VERIFY_CHILD_MODEL_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Child>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::Child>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::Child> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHILD_STDERR_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStderr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_child_stderr_model_captures_what_the_child_wrote_to_stderr",
            claim: VERIFY_CHILD_STDERR_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStderr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::ChildStderr>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::ChildStderr> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHILD_STDIN_MODEL_IS_READABLE_BY_THE_CHILD_PROCESS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStdin> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_child_stdin_model_is_readable_by_the_child_process",
            claim: VERIFY_CHILD_STDIN_MODEL_IS_READABLE_BY_THE_CHILD_PROCESS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStdin>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::ChildStdin>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::ChildStdin> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHILD_STDOUT_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStdout> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_child_stdout_model_captures_what_the_child_wrote_to_stdout",
            claim: VERIFY_CHILD_STDOUT_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStdout>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::ChildStdout>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::ChildStdout> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_COMMAND_MODEL_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_command_model_env_override_is_visible_to_the_spawned_process",
            claim: VERIFY_COMMAND_MODEL_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Command>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::Command>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::Command> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_COMMAND_ARGS_MODEL_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_command_args_model_reports_the_configured_arguments",
            claim: VERIFY_COMMAND_ARGS_MODEL_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::CommandArgs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::CommandArgs<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::CommandArgs<'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_COMMAND_ENVS_MODEL_REPORTS_THE_CONFIGURED_OVERRIDES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_command_envs_model_reports_the_configured_overrides",
            claim: VERIFY_COMMAND_ENVS_MODEL_REPORTS_THE_CONFIGURED_OVERRIDES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::CommandEnvs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::CommandEnvs<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::CommandEnvs<'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_EXIT_STATUS_MODEL_REPORTS_A_NONZERO_EXIT_CODE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_exit_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ExitStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_exit_status_model_reports_a_nonzero_exit_code",
            claim: VERIFY_EXIT_STATUS_MODEL_REPORTS_A_NONZERO_EXIT_CODE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ExitStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::ExitStatus>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::ExitStatus> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_OUTPUT_MODEL_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_exit_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Output> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_output_model_captures_stdout_and_the_exit_status",
            claim: VERIFY_OUTPUT_MODEL_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Output>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::Output>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::Output> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_STDIO_MODEL_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_stdio_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Stdio> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_stdio_model_null_discards_the_childs_output_handle",
            claim: VERIFY_STDIO_MODEL_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Stdio>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::process::Stdio>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::process::Stdio> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_ancestors_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Ancestors<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ancestors_model_yields_self_then_each_parent_up_to_root",
            claim: VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Ancestors<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Ancestors<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Ancestors<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_COMPONENT_MODEL_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Component<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_component_model_distinguishes_root_from_normal_segments",
            claim: VERIFY_COMPONENT_MODEL_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Component<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Component<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Component<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_COMPONENTS_MODEL_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Components<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_components_model_yields_root_then_named_segments_in_order",
            claim: VERIFY_COMPONENTS_MODEL_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Components<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Components<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Components<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ITER_MODEL_YIELDS_THE_NAMED_SEGMENTS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Iter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_iter_model_yields_the_named_segments",
            claim: VERIFY_ITER_MODEL_YIELDS_THE_NAMED_SEGMENTS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Iter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Iter<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_DISPLAY_MODEL_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_display_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_display_model_renders_a_valid_utf8_path_verbatim",
            claim: VERIFY_DISPLAY_MODEL_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Display<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Display<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PATH_MODEL_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Path> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_path_model_derives_extension_file_name_and_parent",
            claim: VERIFY_PATH_MODEL_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Path>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Path>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Path> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PATH_BUF_MODEL_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_buf_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::PathBuf> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_path_buf_model_push_pop_and_join_build_the_expected_path",
            claim: VERIFY_PATH_BUF_MODEL_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::PathBuf>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::PathBuf>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::PathBuf> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PREFIX_MODEL_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Prefix<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_prefix_model_disk_identifies_the_drive_letter",
            claim: VERIFY_PREFIX_MODEL_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Prefix<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::Prefix<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::Prefix<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PREFIX_COMPONENT_MODEL_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::PrefixComponent<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_prefix_component_model_pairs_raw_text_with_parsed_prefix",
            claim: VERIFY_PREFIX_COMPONENT_MODEL_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::PrefixComponent<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::PrefixComponent<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::PrefixComponent<'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_STRIP_PREFIX_ERROR_MODEL_REPORTS_A_NON_MATCHING_PREFIX_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_strip_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::StripPrefixError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_strip_prefix_error_model_reports_a_non_matching_prefix",
            claim: VERIFY_STRIP_PREFIX_ERROR_MODEL_REPORTS_A_NON_MATCHING_PREFIX_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::path::StripPrefixError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::path::StripPrefixError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::path::StripPrefixError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_path_carrier.rs");

const DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_VERUS_FRAGMENT: &str = r#"pub open spec fn dir_builder_model_creates_nested_directories_recursively(
    a: char,
    b: char,
    c: char,
    result: DirBuilderResult,
) -> bool {
    &&& result.0 == (a,)
    &&& result.1 == (a, b)
    &&& result.2 == (a, b, c)
}"#;

impl VerusWitness for RustStdStandard<std::fs::DirBuilder> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_dir_builder_model_creates_nested_directories_recursively",
            claim: VERIFY_DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::DirBuilder>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::DirBuilder>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::DirBuilder> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::DirBuilder>",
        verifier: "verus",
        kind: "ensures",
        fragment: || DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_VERUS_FRAGMENT,
    }
}

const VERIFY_DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_path_carrier.rs");

const DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_VERUS_FRAGMENT: &str = r#"pub open spec fn dir_entry_model_reports_the_created_files_name_and_path(
    parent: char,
    name: char,
    result: (char, (char, char)),
) -> bool {
    &&& result.0 == name
    &&& result.1 == (parent, name)
}"#;

impl VerusWitness for RustStdStandard<std::fs::DirEntry> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_dir_entry_model_reports_the_created_files_name_and_path",
            claim: VERIFY_DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::DirEntry>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::DirEntry>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::DirEntry> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::DirEntry>",
        verifier: "verus",
        kind: "ensures",
        fragment: || DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_VERUS_FRAGMENT,
    }
}

const VERIFY_READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_path_carrier.rs");

const READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_VERUS_FRAGMENT: &str = r#"pub open spec fn read_dir_model_iterates_every_entry_in_the_directory(
    first_name: char,
    second_name: char,
    result: (u32, char, char),
) -> bool {
    &&& result.0 == 2
    &&& result.1 == first_name
    &&& result.2 == second_name
}"#;

impl VerusWitness for RustStdStandard<std::fs::ReadDir> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_read_dir_model_iterates_every_entry_in_the_directory",
            claim: VERIFY_READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::ReadDir>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::ReadDir>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::ReadDir> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::ReadDir>",
        verifier: "verus",
        kind: "ensures",
        fragment: || READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_VERUS_FRAGMENT,
    }
}

const VERIFY_FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_content_carrier.rs");

const FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn file_model_write_then_read_round_trips_the_bytes(
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    result: (u8, u8, u8, u8),
) -> bool {
    result == (a, b, c, d)
}"#;

impl VerusWitness for RustStdStandard<std::fs::File> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_file_model_write_then_read_round_trips_the_bytes",
            claim: VERIFY_FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::File>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::File>",
        verifier: "verus",
        describe: || { <RustStdStandard<std::fs::File> as VerusWitness>::proof().to_string() },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::File>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_VERUS_FRAGMENT,
    }
}

const VERIFY_FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_content_carrier.rs");

const FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_VERUS_FRAGMENT: &str = r#"pub open spec fn file_times_model_sets_the_recorded_modification_time(
    target_unix_seconds: u64,
    result: u64,
) -> bool {
    result == target_unix_seconds
}"#;

impl VerusWitness for RustStdStandard<std::fs::FileTimes> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_file_times_model_sets_the_recorded_modification_time",
            claim: VERIFY_FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::FileTimes>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::FileTimes>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::FileTimes> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::FileTimes>",
        verifier: "verus",
        kind: "ensures",
        fragment: || FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_VERUS_FRAGMENT,
    }
}

const VERIFY_METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_content_carrier.rs");

const METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_VERUS_FRAGMENT: &str = r#"pub open spec fn metadata_model_reports_the_written_length(
    byte_count: u8,
    result: (u64, bool),
) -> bool {
    &&& result.0 == byte_count as u64
    &&& result.1 == (byte_count == 0)
}"#;

impl VerusWitness for RustStdStandard<std::fs::Metadata> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_metadata_model_reports_the_written_length",
            claim: VERIFY_METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::Metadata>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::Metadata>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::Metadata> as VerusWitness>::proof().to_string()
        },
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::Metadata>",
        verifier: "verus",
        kind: "ensures",
        fragment: || METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_VERUS_FRAGMENT,
    }
}

const VERIFY_FILE_TYPE_MODEL_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::FileType> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_file_type_model_distinguishes_files_from_directories",
            claim: VERIFY_FILE_TYPE_MODEL_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::FileType>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::FileType>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::FileType> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_OPEN_OPTIONS_MODEL_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::OpenOptions> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_open_options_model_create_new_rejects_an_existing_file",
            claim: VERIFY_OPEN_OPTIONS_MODEL_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::OpenOptions>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::OpenOptions>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::OpenOptions> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PERMISSIONS_MODEL_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::Permissions> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_permissions_model_readonly_round_trips_through_set_permissions",
            claim: VERIFY_PERMISSIONS_MODEL_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::Permissions>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::Permissions>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::Permissions> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TRY_LOCK_ERROR_MODEL_REPORTS_A_LOCK_ALREADY_HELD_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::TryLockError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_try_lock_error_model_reports_a_lock_already_held",
            claim: VERIFY_TRY_LOCK_ERROR_MODEL_REPORTS_A_LOCK_ALREADY_HELD_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::TryLockError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::TryLockError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::fs::TryLockError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/thread_local_key_carrier.rs");

impl VerusWitness for RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_local_key_model_with_reads_the_initialized_value",
            claim: VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence:
            "amenable_std::rust_std::RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/thread_current_carrier.rs");

impl VerusWitness for RustStdStandard<std::thread::Thread> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_thread_current_model_is_stable_across_repeated_calls",
            claim: VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::Thread>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::thread::Thread>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::thread::Thread> as VerusWitness>::proof().to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::thread::ThreadId> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_thread_current_model_is_stable_across_repeated_calls",
            claim: VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::ThreadId>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::thread::ThreadId>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::thread::ThreadId> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ARGS_MODEL_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/env_carrier.rs");

const ARGS_MODEL_COUNT_MATCHES_PROGRAM_PLUS_EXTRA_VERUS_FRAGMENT: &str = r#"pub open spec fn args_model_count_matches_program_plus_extra(
    extra_count: u8,
    result: u32,
) -> bool {
    result >= 1 && result == 1 + extra_count as u32
}"#;

macro_rules! impl_env_args_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_args_model_reports_at_least_the_program_path",
                    claim: VERIFY_ARGS_MODEL_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }

        ::inventory::submit! {
            ::amenable_core::ContractRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                kind: "ensures",
                fragment: || ARGS_MODEL_COUNT_MATCHES_PROGRAM_PLUS_EXTRA_VERUS_FRAGMENT,
            }
        }
    };
}

impl_env_args_verus_witness!(std::env::Args);
impl_env_args_verus_witness!(std::env::ArgsOs);

const VERIFY_JOIN_PATHS_ERROR_MODEL_REPORTS_AN_UNJOINABLE_PATH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/env_carrier.rs");

impl VerusWitness for RustStdStandard<std::env::JoinPathsError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_join_paths_error_model_reports_an_unjoinable_path",
            claim: VERIFY_JOIN_PATHS_ERROR_MODEL_REPORTS_AN_UNJOINABLE_PATH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::env::JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::env::JoinPathsError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::env::JoinPathsError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SPLIT_PATHS_MODEL_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/env_carrier.rs");

impl VerusWitness for RustStdStandard<std::env::SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_split_paths_model_recovers_paths_joined_by_join_paths",
            claim: VERIFY_SPLIT_PATHS_MODEL_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::env::SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::env::SplitPaths<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::env::SplitPaths<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHANNEL_MODEL_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_mpsc_carrier.rs");

macro_rules! impl_mpsc_channel_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_channel_model_delivers_to_the_paired_receiver",
                    claim: VERIFY_CHANNEL_MODEL_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_mpsc_channel_verus_witness!(std::sync::mpsc::Sender<i32>);
impl_mpsc_channel_verus_witness!(std::sync::mpsc::SyncSender<i32>);

const VERIFY_RECEIVER_MODEL_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_mpsc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::mpsc::Receiver<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_receiver_model_fails_once_every_sender_is_dropped",
            claim: VERIFY_RECEIVER_MODEL_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::mpsc::Receiver<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Receiver<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::mpsc::Receiver<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CHANNEL_ITER_MODEL_YIELDS_SENT_VALUES_THEN_STOPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_mpsc_carrier.rs");

macro_rules! impl_mpsc_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_channel_iter_model_yields_sent_values_then_stops",
                    claim: VERIFY_CHANNEL_ITER_MODEL_YIELDS_SENT_VALUES_THEN_STOPS_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_mpsc_iter_verus_witness!(std::sync::mpsc::IntoIter<i32>);
impl_mpsc_iter_verus_witness!(std::sync::mpsc::Iter<'static, i32>);

const VERIFY_TRY_ITER_MODEL_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_mpsc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_try_iter_model_does_not_block_on_an_empty_open_channel",
            claim: VERIFY_TRY_ITER_MODEL_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Once> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_once_model_runs_its_closure_exactly_once",
            claim: VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Once>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
        verifier: "verus",
        describe: || { <RustStdStandard<std::sync::Once> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_ONCE_STATE_MODEL_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::OnceState> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_once_state_model_reports_not_poisoned_on_a_clean_run",
            claim: VERIFY_ONCE_STATE_MODEL_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::OnceState>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::OnceState>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::OnceState> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ONCE_LOCK_MODEL_INITIALIZES_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::OnceLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_once_lock_model_initializes_exactly_once",
            claim: VERIFY_ONCE_LOCK_MODEL_INITIALIZES_EXACTLY_ONCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::OnceLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::OnceLock<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::OnceLock<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_barrier_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Barrier> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_barrier_model_of_one_is_its_own_leader",
            claim: VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Barrier>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Barrier>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::Barrier> as VerusWitness>::proof().to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::sync::BarrierWaitResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_barrier_model_of_one_is_its_own_leader",
            claim: VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::BarrierWaitResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::BarrierWaitResult>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::BarrierWaitResult> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_WAIT_TIMEOUT_RESULT_MODEL_REPORTS_TIMED_OUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_wait_timeout_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::WaitTimeoutResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_wait_timeout_result_model_reports_timed_out",
            claim: VERIFY_WAIT_TIMEOUT_RESULT_MODEL_REPORTS_TIMED_OUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::WaitTimeoutResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::WaitTimeoutResult>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::sync::WaitTimeoutResult> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_INCOMING_MODEL_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Incoming<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_incoming_model_yields_an_already_queued_connection",
            claim: VERIFY_INCOMING_MODEL_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Incoming<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::Incoming<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::Incoming<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SHUTDOWN_MODEL_WRITE_PREVENTS_FURTHER_WRITES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Shutdown> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_shutdown_model_write_prevents_further_writes",
            claim: VERIFY_SHUTDOWN_MODEL_WRITE_PREVENTS_FURTHER_WRITES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Shutdown>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::Shutdown>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::Shutdown> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TCP_LISTENER_MODEL_ACCEPTS_A_CONNECTING_STREAM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::TcpListener> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_tcp_listener_model_accepts_a_connecting_stream",
            claim: VERIFY_TCP_LISTENER_MODEL_ACCEPTS_A_CONNECTING_STREAM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::TcpListener>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::TcpListener>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::TcpListener> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_TCP_STREAM_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::TcpStream> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_tcp_stream_model_delivers_written_bytes_to_the_accepted_peer",
            claim: VERIFY_TCP_STREAM_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::TcpStream>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::TcpStream>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::TcpStream> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_UDP_SOCKET_MODEL_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::UdpSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_udp_socket_model_send_to_recv_from_round_trips_a_datagram",
            claim: VERIFY_UDP_SOCKET_MODEL_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::net::UdpSocket>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::net::UdpSocket>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::net::UdpSocket> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CONTEXT_MODEL_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_context_model_from_waker_exposes_the_same_waker",
            claim: VERIFY_CONTEXT_MODEL_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::task::Context<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::task::Context<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_POLL_MODEL_READY_AND_PENDING_ARE_DISJOINT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_poll_model_ready_and_pending_are_disjoint",
            claim: VERIFY_POLL_MODEL_READY_AND_PENDING_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Poll<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::task::Poll<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::task::Poll<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_WAKER_MODEL_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_waker_model_wake_by_ref_invokes_the_wake_impl",
            claim: VERIFY_WAKER_MODEL_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::task::Waker>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::task::Waker> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/panic_carrier.rs");

impl VerusWitness for RustStdStandard<std::panic::AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_assert_unwind_safe_model_derefs_transparently",
            claim: VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::panic::AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::panic::AssertUnwindSafe<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::panic::AssertUnwindSafe<i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_LOCATION_MODEL_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/panic_carrier.rs");

impl VerusWitness for RustStdStandard<core::panic::Location<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_location_model_caller_reflects_the_immediate_call_site",
            claim: VERIFY_LOCATION_MODEL_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<core::panic::Location<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::panic::Location<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<core::panic::Location<'static>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_RANGE_TO_MODEL_CONTAINS_MATCHES_BOUND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::RangeTo<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_range_to_model_contains_matches_bound",
            claim: VERIFY_RANGE_TO_MODEL_CONTAINS_MATCHES_BOUND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::RangeTo<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ops::RangeTo<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ops::RangeTo<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_RANGE_FULL_MODEL_CONTAINS_EVERYTHING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::RangeFull> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_range_full_model_contains_everything",
            claim: VERIFY_RANGE_FULL_MODEL_CONTAINS_EVERYTHING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::RangeFull>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RangeFull>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ops::RangeFull> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_BOUND_MODEL_ROUND_TRIPS_ITS_ENDPOINT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::Bound<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_bound_model_round_trips_its_endpoint",
            claim: VERIFY_BOUND_MODEL_ROUND_TRIPS_ITS_ENDPOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::Bound<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ops::Bound<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_CONTROL_FLOW_MODEL_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::ControlFlow<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_control_flow_model_continue_and_break_are_disjoint",
            claim: VERIFY_CONTROL_FLOW_MODEL_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::ControlFlow<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ops::ControlFlow<i32, i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Instant> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_instant_model_is_monotonically_nondecreasing",
            claim: VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Instant>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Instant>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::time::Instant> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_system_time_model_duration_since_computes_the_elapsed_span",
            claim: VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SystemTime>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::time::SystemTime> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTimeError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_system_time_error_model_recovers_how_far_backward_it_went",
            claim: VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTimeError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SystemTimeError>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::time::SystemTimeError> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_duration_model_new_normalizes_nanos_and_carries_into_secs",
            claim: VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Duration>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::time::Duration> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_INTO_ITER_MODEL_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/option_result_iter_carrier.rs");

macro_rules! impl_option_result_into_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_into_iter_model_yields_zero_or_one_owned_value",
                    claim: VERIFY_INTO_ITER_MODEL_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_option_result_into_iter_verus_witness!(core::option::IntoIter<i32>);
impl_option_result_into_iter_verus_witness!(core::result::IntoIter<i32>);

const VERIFY_ITER_MODEL_YIELDS_ZERO_OR_ONE_REFERENCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/option_result_iter_carrier.rs");

macro_rules! impl_option_result_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_iter_model_yields_zero_or_one_reference",
                    claim: VERIFY_ITER_MODEL_YIELDS_ZERO_OR_ONE_REFERENCE_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_option_result_iter_verus_witness!(core::option::Iter<'static, i32>);
impl_option_result_iter_verus_witness!(core::result::Iter<'static, i32>);

const VERIFY_ITER_MUT_MODEL_WRITES_THROUGH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/option_result_iter_carrier.rs");

macro_rules! impl_option_result_iter_mut_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof {
                    harness: "verify_iter_mut_model_writes_through",
                    claim: VERIFY_ITER_MUT_MODEL_WRITES_THROUGH_SRC,
                    provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
                }
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord {
                evidence: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                verifier: "verus",
                describe: || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            }
        }
    };
}

impl_option_result_iter_mut_verus_witness!(core::option::IterMut<'static, i32>);
impl_option_result_iter_mut_verus_witness!(core::result::IterMut<'static, i32>);

const VERIFY_PENDING_MODEL_NEVER_RESOLVES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/future_carrier.rs");

impl VerusWitness for RustStdStandard<std::future::Pending<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_pending_model_never_resolves",
            claim: VERIFY_PENDING_MODEL_NEVER_RESOLVES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::future::Pending<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::future::Pending<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_READY_MODEL_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/future_carrier.rs");

impl VerusWitness for RustStdStandard<std::future::Ready<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_ready_model_resolves_immediately_with_its_value",
            claim: VERIFY_READY_MODEL_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::future::Ready<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::future::Ready<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_POLL_FN_MODEL_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/future_carrier.rs");

impl VerusWitness
    for RustStdStandard<
        std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_poll_fn_model_dispatches_through_to_its_closure",
            claim: VERIFY_POLL_FN_MODEL_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(
    RustStdStandard<std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

const VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_array_model_indexing_and_length",
            claim: VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        verifier: "verus",
        describe: || { <RustStdStandard<[i32; 3]> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_slice_model_indexing_and_length",
            claim: VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<[i32]>",
        verifier: "verus",
        describe: || { <RustStdStandard<[i32]> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_str_model_byte_length_and_content",
            claim: VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<str>);

impl Ensures<VerusVerifier> for RustStdStandard<str> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        "observed @ =~= expected @"
    }
}

const TEXT_VIEW_MATCHES_EXPECTED_VERUS_FRAGMENT: &str = r#"pub open spec fn text_view_matches_expected(observed: Seq<char>, expected: Seq<char>) -> bool {
    observed =~= expected
}"#;

// `Ensures::ensures()` keeps a descriptive text-view-equality spelling
// for cross-backend enumeration, while the real Verus proof sites across
// the `str`, `path`, `process`, `env`, and panic carriers call the
// shared `text_view_matches_expected` spec fn directly.
::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "verus",
        kind: "ensures",
        fragment: || <RustStdStandard<str> as Ensures<VerusVerifier>>::ensures(()),
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "verus",
        kind: "ensures",
        fragment: || TEXT_VIEW_MATCHES_EXPECTED_VERUS_FRAGMENT,
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "verus",
        describe: || { <RustStdStandard<str> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_tuple_model_field_access",
            claim: VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        verifier: "verus",
        describe: || { <RustStdStandard<(i32, i32)> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_fn_pointer_model_calls_the_underlying_function",
            claim: VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        verifier: "verus",
        describe: || { <RustStdStandard<fn(i32) -> i32> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_const_pointer_model_cast_is_reproducible",
            claim: VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<*const i32>",
        verifier: "verus",
        describe: || { <RustStdStandard<*const i32> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_mut_pointer_model_cast_is_reproducible",
            claim: VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<*mut i32>",
        verifier: "verus",
        describe: || { <RustStdStandard<*mut i32> as VerusWitness>::proof().to_string() },
    }
}

impl VerusWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_shared_reference_model_dereferences_to_the_referent",
            claim: VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static i32>",
        verifier: "verus",
        describe: || { <RustStdStandard<&'static i32> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_mutable_reference_model_dereferences_to_and_updates_the_referent",
            claim: VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        verifier: "verus",
        describe: || { <RustStdStandard<&'static mut i32> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_default_hasher_model_is_deterministic_across_fresh_instances",
            claim: VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::DefaultHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::hash::DefaultHasher> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_RANDOM_STATE_MODEL_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::RandomState> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_random_state_model_gives_the_same_hasher_seed_across_calls",
            claim: VERIFY_RANDOM_STATE_MODEL_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::RandomState>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RandomState>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::hash::RandomState> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_OS_STR_MODEL_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_os_str_model_valid_utf8_content_round_trips_through_to_str",
            claim: VERIFY_OS_STR_MODEL_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsStr>",
        verifier: "verus",
        describe: || { <RustStdStandard<std::ffi::OsStr> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_OS_STRING_MODEL_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_os_string_model_push_appends_to_the_existing_content",
            claim: VERIFY_OS_STRING_MODEL_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsString>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ffi::OsString> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_OS_STR_DISPLAY_MODEL_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::os_str::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_os_str_display_model_renders_valid_utf8_content_unchanged",
            claim: VERIFY_OS_STR_DISPLAY_MODEL_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::os_str::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::ffi::os_str::Display<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ffi::os_str::Display<'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_HASH_MAP_MODEL_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_collections_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::HashMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_hash_map_model_insert_then_get_recovers_the_value",
            claim: VERIFY_HASH_MAP_MODEL_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::HashMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::HashMap<i32, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_HASH_SET_MODEL_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_collections_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::HashSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_hash_set_model_insert_then_contains_reports_membership",
            claim: VERIFY_HASH_SET_MODEL_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::HashSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::HashSet<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PIN_MODEL_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::pin::Pin<Box<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_pin_model_derefs_and_get_mut_round_trip",
            claim: VERIFY_PIN_MODEL_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::pin::Pin<Box<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Pin<Box<i32>>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::pin::Pin<Box<i32>>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_NON_NULL_MODEL_REJECTS_THE_NULL_POINTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::ptr::NonNull<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_non_null_model_rejects_the_null_pointer",
            claim: VERIFY_NON_NULL_MODEL_REJECTS_THE_NULL_POINTER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::ptr::NonNull<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonNull<i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::ptr::NonNull<i32>> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_SYSTEM_MODEL_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::alloc::System> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_system_model_allocates_and_deallocates_a_layout",
            claim: VERIFY_SYSTEM_MODEL_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::alloc::System>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<System>",
        verifier: "verus",
        describe: || { <RustStdStandard<std::alloc::System> as VerusWitness>::proof().to_string() },
    }
}

const VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::backtrace::Backtrace> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_backtrace_model_force_capture_always_actually_captures",
            claim: VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::backtrace::Backtrace>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Backtrace>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::backtrace::Backtrace> as VerusWitness>::proof().to_string()
        },
    }
}

impl VerusWitness for RustStdStandard<std::backtrace::BacktraceStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_backtrace_model_force_capture_always_actually_captures",
            claim: VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::backtrace::BacktraceStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BacktraceStatus>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::backtrace::BacktraceStatus> as VerusWitness>::proof().to_string()
        },
    }
}

const VERIFY_PANIC_HOOK_INFO_MODEL_REPORTS_THE_PANICS_OWN_MESSAGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::panic::PanicHookInfo<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_panic_hook_info_model_reports_the_panics_own_message",
            claim: VERIFY_PANIC_HOOK_INFO_MODEL_REPORTS_THE_PANICS_OWN_MESSAGE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::panic::PanicHookInfo<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PanicHookInfo<'static>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::panic::PanicHookInfo<'static>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

const VERIFY_VEC_DEQUE_DRAIN_MODEL_REMOVES_AND_YIELDS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "verify_vec_deque_drain_model_removes_and_yields_in_order",
            claim: VERIFY_VEC_DEQUE_DRAIN_MODEL_REMOVES_AND_YIELDS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        verifier: "verus",
        describe: || {
            <RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    }
}

// `std::os::windows::*` witnesses: `#[cfg(windows)]`-gated per item,
// mirroring `rust_std::os_windows`'s own gating. Unlike every other
// `VerusWitness` impl in this file, `amenable_verus::rust_std::
// os_windows_carrier` (the `claim` these `include_str!` in) has never
// been checked by `verus` on this crate's primary development host
// (Linux) — only the `verus-windows` GitHub Actions workflow
// (`workflow_dispatch`, `windows-latest`) can. See that carrier's own
// module doc comment for the full reasoning.

#[cfg(windows)]
const VERIFY_ENCODE_WIDE_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<EncodeWide<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "<EncodeWide<'_> as Iterator>::next",
            claim: VERIFY_ENCODE_WIDE_AXIOM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<EncodeWide<'static>>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        verifier: "verus",
        describe: || { <RustStdStandard<EncodeWide<'static>> as VerusWitness>::proof().to_string() },
    }
}

#[cfg(windows)]
const VERIFY_BORROWED_HANDLE_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<BorrowedHandle<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "<BorrowedHandle<'_> as AsRawHandle>::as_raw_handle",
            claim: VERIFY_BORROWED_HANDLE_AXIOM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<BorrowedHandle<'static>>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        verifier: "verus",
        describe: || { <RustStdStandard<BorrowedHandle<'static>> as VerusWitness>::proof().to_string() },
    }
}

#[cfg(windows)]
const VERIFY_BORROWED_SOCKET_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<BorrowedSocket<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "<BorrowedSocket<'_> as AsRawSocket>::as_raw_socket",
            claim: VERIFY_BORROWED_SOCKET_AXIOM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<BorrowedSocket<'static>>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        verifier: "verus",
        describe: || { <RustStdStandard<BorrowedSocket<'static>> as VerusWitness>::proof().to_string() },
    }
}

#[cfg(windows)]
const VERIFY_HANDLE_OR_INVALID_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<HandleOrInvalid> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "<OwnedHandle as TryFrom<HandleOrInvalid>>::try_from",
            claim: VERIFY_HANDLE_OR_INVALID_AXIOM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<HandleOrInvalid>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        verifier: "verus",
        describe: || { <RustStdStandard<HandleOrInvalid> as VerusWitness>::proof().to_string() },
    }
}

#[cfg(windows)]
const VERIFY_OWNED_HANDLE_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<OwnedHandle> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "<OwnedHandle as AsRawHandle>::as_raw_handle",
            claim: VERIFY_OWNED_HANDLE_AXIOM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<OwnedHandle>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        verifier: "verus",
        describe: || { <RustStdStandard<OwnedHandle> as VerusWitness>::proof().to_string() },
    }
}

#[cfg(windows)]
const VERIFY_OWNED_SOCKET_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<OwnedSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof {
            harness: "<OwnedSocket as AsRawSocket>::as_raw_socket",
            claim: VERIFY_OWNED_SOCKET_AXIOM_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<OwnedSocket>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        verifier: "verus",
        describe: || { <RustStdStandard<OwnedSocket> as VerusWitness>::proof().to_string() },
    }
}
