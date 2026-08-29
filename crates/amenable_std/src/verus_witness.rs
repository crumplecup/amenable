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

use amenable_core::{
    ClassifiedWitness, Evidence, MetadataEntry, Provenance, Verifier, Witness, WitnessArtifact,
    WitnessArtifactNode, WitnessSupportKind, WitnessSupportSummary,
};
#[cfg(windows)]
use std::os::windows::ffi::EncodeWide;
#[cfg(windows)]
use std::os::windows::io::{
    BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
};

use crate::{
    ArrayIntoIterAdvanceMatchesPosition, ArrayIntoIterStartsAtFirstPosition, AsciiByte,
    IncrementHeadroom, NonNulByte, ObservedOptionMatchesInput, ObservedPairMatchesInput,
    ObservedValueMatchesInput, RustStdProvenance, RustStdStandard, ValidUnicodeScalar,
    ValueUnchanged, WriteStoresNewValue, YieldsThreeValuesInOrderThenEnds,
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

/// Register explicit Verus witness exports for concrete instantiated
/// types.
///
/// Verus compiles a separate source tree and cannot discover every
/// derived witness automatically. This macro records the concrete types a
/// crate wants to materialize in that separate Verus pipeline while
/// keeping the registration itself in ordinary Rust macro expansion.
#[macro_export]
macro_rules! emit_verus_witnesses {
    ($($ty:ty),* $(,)?) => {
        ::amenable_core::register_witness_exports!(
            verifier = $crate::VerusVerifier;
            $($ty),*
        );
    };
}

trait VerusProofArtifactSupport {
    fn support() -> WitnessSupportSummary;
}

macro_rules! bridge_verus_witness {
    ($ty:ty) => {
        impl Witness<VerusVerifier> for $ty {
            type SupportingEvidence = <$ty as VerusWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as VerusWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as VerusWitness>::proof()
            }

            fn support() -> WitnessSupportSummary {
                <<$ty as VerusWitness>::ProofArtifact as VerusProofArtifactSupport>::support()
            }
        }

        // Every VerusWitness-bridged type resolves to a real VerusCheckedProof
        // or RustStdProvenance artifact -- both genuinely classified (Checked
        // or Trusted respectively, see VerusProofArtifactSupport below), never
        // the Witness::support() default (Opaque) -- so this impl is always
        // sound to add unconditionally alongside the bridge.
        impl ClassifiedWitness<VerusVerifier> for $ty {}
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
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    "verus",
                    || <RustStdStandard<$ty> as VerusWitness>::proof().report().to_string(),
                )
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

impl VerusProofArtifactSupport for RustStdProvenance {
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trusted_leaf()
    }
}

impl WitnessArtifact for RustStdProvenance {
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf_with_metadata(
            WitnessSupportKind::Trusted,
            WitnessSupportSummary::trusted_leaf(),
            self.report().to_string(),
            self.metadata(),
        )
    }
}

/// Proof artifact for a carrier with a real, machine-checked Verus spec:
/// names the spec function, carries its verbatim source as `claim`, and
/// still rests on the chain-derived provenance.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct VerusCheckedProof {
    /// The Verus spec function that checks this carrier's invariant.
    harness: String,
    /// The spec's own source — the whole file it lives in, verbatim
    /// (`include_str!`, not a per-function extraction — Verus proof files
    /// in `amenable_verus` are kept to one carrier's spec function(s) each
    /// so this stays a tight, accurate claim, the same one-claim-per-
    /// carrier granularity `amenable_derive::harness!` gives Kani/Creusot
    /// by capturing one function at a time).
    claim: String,
    /// The chain-derived provenance this claim still rests on.
    provenance: RustStdProvenance,
}

impl VerusProofArtifactSupport for VerusCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl WitnessArtifact for VerusCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf_with_metadata(
            WitnessSupportKind::Checked,
            WitnessSupportSummary::checked_leaf(),
            format!("harness: {}", self.harness),
            [
                MetadataEntry::new("verifier", VerusVerifier::name()),
                MetadataEntry::new("harness", self.harness.clone()),
                MetadataEntry::new("claim", self.claim.clone()),
            ],
        )
    }
}

impl std::fmt::Display for VerusCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}

/// One symbolic input a real Verus harness takes, in declaration order.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct VerusParam {
    /// The parameter's real name in the harness signature.
    name: String,
    /// The parameter's real Verus type, as written in the signature.
    ty: String,
}

/// A real predicate/spec-fn a harness's clause templates cite, together
/// with its own defining module — not necessarily the harness's own
/// module. Confirmed as a real, not hypothetical, distinction against
/// the real `verus` tool: `RefCell`'s harness cites
/// `observed_value_matches_input`, which is *defined* in
/// `primitive_shapes_carrier` and only privately `use`d by
/// `ref_cell_carrier` — importing it via the harness's own module path
/// (`crate::rust_std::ref_cell_carrier::observed_value_matches_input`)
/// failed with `E0603: function import ... is private`.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct VerusImport {
    /// The predicate/spec-fn's own defining module.
    module_path: String,
    /// The predicate/spec-fn's real name.
    name: String,
}

/// How a compositional renderer should invoke a leaf's real Verus proof,
/// rather than assuming its conclusion as a free boolean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerusCallKind {
    /// A bare `open spec fn` predicate, directly citable by name — no
    /// call needed, a composite's own spec fn can just conjoin it.
    Predicate,
    /// A value-returning function: call it, bind the result, cite its
    /// real `ensures` predicate with the bound result in scope.
    Function {
        /// The harness's real return type.
        returns: String,
    },
}

/// Structural, machine-usable call shape for a real Verus harness —
/// enough for a compositional renderer to emit a literal call to (or
/// citation of) the real proof, instead of assuming its conclusion.
///
/// `requires`/`ensures` are the harness's own real clause text, copied
/// verbatim, with `$name` placeholders standing in for whatever local
/// name a composite ends up choosing (`$result` for the harness's own
/// bound return value, `$paramname` for one of its own named
/// parameters) — never restated by hand, never restructured into a
/// predicate-call-only shape. A first design here tried a structured
/// `predicate(args)`-only representation, which worked for
/// `char_roundtrip`/`escape_ascii`'s harnesses but broke on
/// `RefCell`'s: its own top-level harness's `ensures` includes raw
/// tuple-field projections (`result.0`, `!result.1`, ...) alongside a
/// named-predicate call whose own argument is itself a projection-and-
/// cast (`result.5 as int`) — neither fits a "bare call" shape. Plain
/// text templates handle both uniformly, since the renderer never needs
/// to parse the clause's grammar, only substitute placeholder tokens.
///
/// A separate, additive registry (see [`VerusCallShapeRecord`]) rather
/// than a field on [`VerusCheckedProof`] itself: [`VerusCheckedProof`]
/// already has ~280 construction sites across this file, almost none of
/// which are opted into Verus export (`register_witness_exports!` is
/// deliberately opt-in — see its own doc comment). Requiring every one
/// of those sites to supply a call shape up front, before any renderer
/// exists to use it, would force touching all of them for no immediate
/// benefit. Registering a call shape only for harnesses actually opted
/// into export keeps the two concerns (this crate's own witness
/// registrations vs. what a downstream Verus-rendering tool needs) from
/// forcing lockstep changes on each other.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct VerusCallShape {
    /// The crate-relative module path the harness lives in.
    module_path: String,
    /// The harness function's real name.
    name: String,
    /// The harness's real symbolic parameters, in order.
    params: Vec<VerusParam>,
    /// The harness's own real precondition templates, propagated
    /// upward into a composite's own `requires` when this leaf composes.
    requires: Vec<String>,
    /// The harness's own real postcondition templates, cited (never
    /// restated) in a composite's own `ensures` when this leaf composes.
    ensures: Vec<String>,
    /// Real predicate/spec-fns the templates above reference and that
    /// need an explicit `use` to resolve — listed separately from the
    /// templates themselves rather than parsed out of them, since a
    /// template may be a raw expression with no callable name in it at
    /// all (e.g. `$result.0`).
    imports: Vec<VerusImport>,
    /// How to invoke this specific harness.
    kind: VerusCallKind,
}

/// A statically registered call shape for one Verus harness, keyed by
/// harness name.
///
/// Additive and opt-in, matching [`amenable_core::WitnessExportRecord`]'s
/// own opt-in registration story: only harnesses a compositional
/// renderer actually needs to call get one.
///
/// Hand-written `const fn new`/getters, not derived: this record is
/// itself passed to `inventory::submit!`, which requires a
/// `const`-evaluable value, and `derive_new::new` cannot generate a
/// `const fn`. `VerusCallShape` itself has no such requirement -- it's
/// built at call time inside the stored closure -- so it uses the
/// ordinary derives above.
pub struct VerusCallShapeRecord {
    harness: &'static str,
    call_shape: fn() -> VerusCallShape,
}

impl VerusCallShapeRecord {
    /// Register a harness's real call shape constructor.
    #[must_use]
    pub const fn new(harness: &'static str, call_shape: fn() -> VerusCallShape) -> Self {
        Self {
            harness,
            call_shape,
        }
    }

    /// The harness name this call shape describes.
    #[must_use]
    pub const fn harness(&self) -> &'static str {
        self.harness
    }

    /// Build the real call shape.
    #[must_use]
    pub const fn call_shape(&self) -> fn() -> VerusCallShape {
        self.call_shape
    }
}

inventory::collect!(VerusCallShapeRecord);

/// Look up a harness's real call shape: an explicit
/// `register_verus_call_shape!` registration first (an escape hatch for
/// synthetic/test-only shapes with no real carrier file behind them,
/// e.g. `amenable`'s own renderer tests), falling back to deriving it by
/// parsing the harness's real carrier source directly -- the single
/// source of truth for every real harness, with nothing to keep in sync
/// by hand. See `verus_call_shape_derive`'s own doc comment.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub fn verus_call_shape(harness: &str) -> Option<VerusCallShape> {
    inventory::iter::<VerusCallShapeRecord>()
        .find(|record| record.harness == harness)
        .map(|record| (record.call_shape)())
        .or_else(|| crate::verus_call_shape_derive::derive_call_shape(harness))
}

/// Manually register a Verus harness's call shape — an escape hatch for
/// synthetic/test-only shapes with no real carrier file behind them
/// (e.g. `amenable`'s own renderer tests). Every real harness gets its
/// call shape derived automatically instead, by
/// `verus_call_shape_derive` parsing the real carrier source directly;
/// this macro exists only for the case a real source file can't back
/// the shape at all.
///
/// `requires`/`ensures` entries are the harness's own real clause text,
/// verbatim, with `$result`/`$paramname` placeholders in place of
/// whatever local names a composite ends up choosing. `imports` lists
/// the real `(module_path, name)` of each predicate/spec-fn those
/// templates reference, so the renderer knows what needs a `use` — its
/// own defining module, not necessarily the harness's own (a shared
/// predicate like `observed_value_matches_input` is defined once in
/// `primitive_shapes_carrier` and merely `use`d by many carriers,
/// including a harness's own).
///
/// ```ignore
/// register_verus_call_shape! {
///     harness = "verify_char_roundtrip",
///     module_path = "crate::rust_std::char_carrier",
///     params = [("c", "char")],
///     returns = "char",
///     requires = [],
///     ensures = [
///         "char_roundtrip_preserves_value($result, $c)",
///         "char_is_valid_unicode_scalar($c)",
///     ],
///     imports = [
///         ("crate::rust_std::char_carrier", "char_roundtrip_preserves_value"),
///         ("crate::rust_std::char_carrier", "char_is_valid_unicode_scalar"),
///     ],
/// }
/// ```
#[macro_export]
macro_rules! register_verus_call_shape {
    (
        harness = $harness:literal,
        module_path = $module_path:literal,
        params = [$(($param_name:literal, $param_ty:literal)),* $(,)?],
        returns = $returns:literal,
        requires = [$($requires_template:literal),* $(,)?],
        ensures = [$($ensures_template:literal),* $(,)?],
        imports = [$(($import_module:literal, $import_name:literal $(,)?)),* $(,)?] $(,)?
    ) => {
        ::inventory::submit! {
            $crate::VerusCallShapeRecord::new(
                $harness,
                || $crate::VerusCallShape::new(
                    $module_path.to_owned(),
                    $harness.to_owned(),
                    ::std::vec![
                        $($crate::VerusParam::new(
                            $param_name.to_owned(),
                            $param_ty.to_owned(),
                        )),*
                    ],
                    ::std::vec![$($requires_template.to_owned()),*],
                    ::std::vec![$($ensures_template.to_owned()),*],
                    ::std::vec![
                        $($crate::VerusImport::new(
                            $import_module.to_owned(),
                            $import_name.to_owned(),
                        )),*
                    ],
                    $crate::VerusCallKind::Function {
                        returns: $returns.to_owned(),
                    },
                ),
            )
        }
    };
}

const VERIFY_CHAR_ROUNDTRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_carrier.rs");

// `verify_char_roundtrip`'s real VerusCallShape (params/requires/ensures/
// imports) is no longer registered by hand here -- `verus_call_shape`
// derives it by parsing the real signature directly from
// crates/amenable_verus/src/rust_std/char_carrier.rs. Reused by
// RustStdStandard<char>, ValidUnicodeScalar, and the Verus derive-witness
// canary's CheckedVerusExportLeaf, all keyed by this one harness name.

impl VerusWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_roundtrip".to_owned(),
            VERIFY_CHAR_ROUNDTRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<char>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<char>,
    "amenable_std::rust_std::RustStdStandard<char>",
    "verify_char_roundtrip"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "verus",
        || <RustStdStandard<char> as VerusWitness>::proof().to_string(),
    )
}

/// The [`ValidUnicodeScalar`] contract type reuses `verify_char_roundtrip`
/// rather than adding a new Verus proof — it names the postcondition the
/// spec already checks, it doesn't prove anything new.
impl VerusWitness for ValidUnicodeScalar {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_roundtrip".to_owned(),
            VERIFY_CHAR_ROUNDTRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ValidUnicodeScalar);

// verify_char_roundtrip's second real clause only -- see
// RustStdStandard<char> just above for its first.
amenable_derive::verus_ensures_witness!(
    ValidUnicodeScalar,
    "amenable_std::ValidUnicodeScalar",
    "verify_char_roundtrip",
    [1]
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::ValidUnicodeScalar",
        "verus",
        || <ValidUnicodeScalar as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_STRING_ROUNDTRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/string_carrier.rs");

impl VerusWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_string_roundtrip".to_owned(),
            VERIFY_STRING_ROUNDTRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<String>",
        "verus",
        || <RustStdStandard<String> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ordering_carrier.rs");

impl VerusWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordering_reverse_swaps_less_and_greater".to_owned(),
            VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        "verus",
        || {
            <RustStdStandard<std::cmp::Ordering> as VerusWitness>::proof().to_string()
        },
    )
}

// The real law `.reverse()` obeys -- named once, called from both the
// trusted `assume_specification` on the real method and its own
// re-derivation, instead of restated at each.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cmp::Ordering>,
    "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
    "ordering_reverse_swaps_less_and_greater"
);

const VERIFY_OPTION_UNWRAP_RETURNS_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/option_carrier.rs");

impl VerusWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_option_unwrap_returns_the_wrapped_value".to_owned(),
            VERIFY_OPTION_UNWRAP_RETURNS_THE_WRAPPED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        "verus",
        || <RustStdStandard<Option<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_RESULT_UNWRAP_RETURNS_THE_OK_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/result_carrier.rs");

impl VerusWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_result_unwrap_returns_the_ok_value".to_owned(),
            VERIFY_RESULT_UNWRAP_RETURNS_THE_OK_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<Result<i32, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_WRAPPING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/wrapping_carrier.rs");

impl VerusWitness for RustStdStandard<std::num::Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_wrapping_field_roundtrips_the_constructed_value".to_owned(),
            VERIFY_WRAPPING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::num::Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::num::Wrapping<i32>>",
        "verus",
        || {
            <RustStdStandard<std::num::Wrapping<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SATURATING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/saturating_carrier.rs");

impl VerusWitness for RustStdStandard<std::num::Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_saturating_field_roundtrips_the_constructed_value".to_owned(),
            VERIFY_SATURATING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::num::Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::num::Saturating<i32>>",
        "verus",
        || {
            <RustStdStandard<std::num::Saturating<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REVERSE_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/reverse_carrier.rs");

impl VerusWitness for RustStdStandard<std::cmp::Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_reverse_field_roundtrips_the_constructed_value".to_owned(),
            VERIFY_REVERSE_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cmp::Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Reverse<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cmp::Reverse<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/manually_drop_carrier.rs");

impl VerusWitness for RustStdStandard<std::mem::ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_manually_drop_derefs_and_into_inner_round_trip".to_owned(),
            VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::mem::ManuallyDrop<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::mem::ManuallyDrop<i32>>",
        "verus",
        || {
            <RustStdStandard<std::mem::ManuallyDrop<i32>> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_fp_category_matches_the_value_it_classifies".to_owned(),
            VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        || {
            <RustStdStandard<core::num::FpCategory> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        "ensures",
        || FP_CATEGORY_CLASSIFY_RESULT_MATCHES_SPECIAL_VALUE_CATEGORIES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        "requires",
        || FP_CATEGORY_INPUTS_COVER_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        "ensures",
        || FP_CATEGORY_RESULTS_MATCH_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    )
}

const VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/int_error_kind_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_int_error_kind_classifies_parse_failures".to_owned(),
            VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        "verus",
        || {
            <RustStdStandard<core::num::IntErrorKind> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PARSE_INT_ERROR_MODEL_REPORTS_THE_KIND_OF_THE_FAILURE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/int_error_kind_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_parse_int_error_model_reports_the_kind_of_the_failure".to_owned(),
            VERIFY_PARSE_INT_ERROR_MODEL_REPORTS_THE_KIND_OF_THE_FAILURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        "verus",
        || {
            <RustStdStandard<core::num::ParseIntError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/parse_float_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_parse_float_error_occurs_only_for_unparseable_input".to_owned(),
            VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        "verus",
        || {
            <RustStdStandard<core::num::ParseFloatError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/try_from_int_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_from_int_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        "verus",
        || {
            <RustStdStandard<core::num::TryFromIntError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/box_carrier.rs");

impl VerusWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_box_derefs_and_writes_through".to_owned(),
            VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Box<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<Box<i32>>,
    "amenable_std::rust_std::RustStdStandard<Box<i32>>",
    "verify_box_derefs_and_writes_through"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        "verus",
        || <RustStdStandard<Box<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/layout_carrier.rs");

impl VerusWitness for RustStdStandard<core::alloc::Layout> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment".to_owned(),
            VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::alloc::Layout>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::alloc::Layout>",
        "verus",
        || <RustStdStandard<core::alloc::Layout> as VerusWitness>::proof().to_string(),
    )
}

impl VerusWitness for RustStdStandard<core::alloc::LayoutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment".to_owned(),
            VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::alloc::LayoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::alloc::LayoutError>",
        "verus",
        || {
            <RustStdStandard<core::alloc::LayoutError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_carrier.rs");

impl VerusWitness for RustStdStandard<Vec<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_push_pop_round_trips".to_owned(),
            VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Vec<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
        "verus",
        || <RustStdStandard<Vec<i32>> as VerusWitness>::proof().to_string(),
    )
}

// A singleton contract: this len-after-one-push fact is never restated
// anywhere else, but still gets a real, named, callable predicate
// rather than staying an unnamed raw literal -- a named contract's
// whole point is giving an assumption an explicit, auditable source,
// not just deduplicating repeated text.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<Vec<i32>>,
    "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
    "vec_len_after_one_push_is_one"
);

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
        VerusCheckedProof::new(
            "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range".to_owned(),
            VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::char::CharTryFromError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        || {
            <RustStdStandard<core::char::CharTryFromError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        "ensures",
        || U32_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        "ensures",
        || CHAR_TRY_FROM_U32_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        "ensures",
        || CHAR_TRY_FROM_U32_REJECTS_INVALID_SCALAR_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_try_from_char_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::char::TryFromCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        || {
            <RustStdStandard<core::char::TryFromCharError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        "ensures",
        || CHAR_FITS_IN_U8_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        "ensures",
        || U8_TRY_FROM_CHAR_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        "ensures",
        || U8_TRY_FROM_CHAR_REJECTS_OUT_OF_RANGE_SCALAR_VERUS_FRAGMENT,
    )
}

const VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/type_id_carrier.rs");

impl VerusWitness for RustStdStandard<core::any::TypeId> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_type_id_is_reflexive_and_distinguishes_distinct_types".to_owned(),
            VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::any::TypeId>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::any::TypeId>",
        "verus",
        || <RustStdStandard<core::any::TypeId> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/try_from_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::array::TryFromSliceError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_from_slice_rejects_a_length_mismatch".to_owned(),
            VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::array::TryFromSliceError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::array::TryFromSliceError>",
        "verus",
        || {
            <RustStdStandard<std::array::TryFromSliceError> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_from_utf16_rejects_a_lone_surrogate".to_owned(),
            VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::string::FromUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        || {
            <RustStdStandard<std::string::FromUtf16Error> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        "ensures",
        || FROM_UTF16_RESULT_MATCHES_SINGLE_UNIT_EXAMPLES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        "requires",
        || FROM_UTF16_INPUTS_COVER_VALID_AND_LONE_SURROGATE_CASES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        "ensures",
        || FROM_UTF16_CASE_RESULTS_MATCH_ACCEPT_REJECT_TRIPLE_VERUS_FRAGMENT,
    )
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

impl VerusWitness for RustStdStandard<std::ffi::CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul".to_owned(),
            VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::CString>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::ffi::CString>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
    "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        "verus",
        || <RustStdStandard<std::ffi::CString> as VerusWitness>::proof().to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        "verus",
        "ensures",
        || INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        "verus",
        "ensures",
        || CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT,
    )
}

impl VerusWitness for RustStdStandard<std::ffi::NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul".to_owned(),
            VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::NulError>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::ffi::NulError>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
    "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        "verus",
        || <RustStdStandard<std::ffi::NulError> as VerusWitness>::proof().to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        "verus",
        "ensures",
        || INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        "verus",
        "ensures",
        || CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT,
    )
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

impl VerusWitness for RustStdStandard<std::ffi::FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::FromVecWithNulError>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::ffi::FromVecWithNulError>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
    "verify_from_vec_with_nul_requires_the_nul_only_at_the_end"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        "verus",
        || {
            <RustStdStandard<std::ffi::FromVecWithNulError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        "verus",
        "ensures",
        || FROM_VEC_WITH_NUL_RESULT_MATCHES_NUL_PLACEMENT_VERUS_FRAGMENT,
    )
}

const VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/parse_char_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::char::ParseCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_parse_char_error_occurs_for_empty_or_multi_character_strings".to_owned(),
            VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::char::ParseCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::ParseCharError>",
        "verus",
        || {
            <RustStdStandard<core::char::ParseCharError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/rc_carrier.rs");

impl VerusWitness for RustStdStandard<std::rc::Rc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_rc_derefs_to_the_wrapped_value".to_owned(),
            VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::rc::Rc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::rc::Rc<i32>>",
        "verus",
        || <RustStdStandard<std::rc::Rc<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_ARC_DEREFS_TO_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/arc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Arc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_arc_derefs_to_the_wrapped_value".to_owned(),
            VERIFY_ARC_DEREFS_TO_THE_WRAPPED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Arc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Arc<i32>>",
        "verus",
        || <RustStdStandard<std::sync::Arc<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/into_string_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_into_string_error_recovers_the_original_cstring".to_owned(),
            VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::IntoStringError>",
        "verus",
        || {
            <RustStdStandard<std::ffi::IntoStringError> as VerusWitness>::proof().to_string()
        },
    )
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

impl VerusWitness for RustStdStandard<core::ffi::FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere".to_owned(),
            VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::FromBytesUntilNulError>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<core::ffi::FromBytesUntilNulError>,
    "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
    "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        "verus",
        || {
            <RustStdStandard<core::ffi::FromBytesUntilNulError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        "verus",
        "ensures",
        || CSTR_FROM_BYTES_UNTIL_NUL_RESULT_MATCHES_NUL_PRESENCE_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesWithNulError>",
        "verus",
        || {
            <RustStdStandard<core::ffi::FromBytesWithNulError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesWithNulError>",
        "verus",
        "ensures",
        || CSTR_FROM_BYTES_WITH_NUL_RESULT_MATCHES_BYTES_VERUS_FRAGMENT,
    )
}

const VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/hash_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_build_hasher_default_produces_consistent_hashers".to_owned(),
            VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>",
        "verus",
        || {
            <RustStdStandard<
                std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
            > as VerusWitness>::proof()
            .to_string()
        },
    )
}

const VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sip_hasher_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::SipHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_sip_hasher_produces_consistent_hashes".to_owned(),
            VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::SipHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::hash::SipHasher>",
        "verus",
        || {
            <RustStdStandard<std::hash::SipHasher> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_cow_borrowed_and_owned_agree_on_their_value".to_owned(),
            VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::borrow::Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::borrow::Cow<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        "verus",
        "ensures",
        || I32_TO_OWNED_SPEC_IS_IDENTITY_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        "verus",
        "ensures",
        || COW_INTO_OWNED_PRESERVES_VARIANT_VALUE_VERUS_FRAGMENT,
    )
}

const VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/btree_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_btree_map_insert_get_remove_round_trips".to_owned(),
            VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BTreeMap<i32, i32>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::collections::BTreeMap<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
    "verify_btree_map_insert_get_remove_round_trips"
);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::collections::BTreeMap<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
    "verify_btree_map_insert_get_remove_round_trips"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::BTreeMap<i32, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/btree_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_btree_set_insert_contains_remove_round_trips".to_owned(),
            VERIFY_BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BTreeSet<i32>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::collections::BTreeSet<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
    "verify_btree_set_insert_contains_remove_round_trips"
);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::collections::BTreeSet<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
    "verify_btree_set_insert_contains_remove_round_trips"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::BTreeSet<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_deque_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_deque_pushes_and_pops_from_both_ends".to_owned(),
            VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::VecDeque<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::VecDeque<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TRY_RESERVE_PRESERVES_VEC_CONTENTS_REGARDLESS_OF_OUTCOME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/try_reserve_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_reserve_preserves_vec_contents_regardless_of_outcome".to_owned(),
            VERIFY_TRY_RESERVE_PRESERVES_VEC_CONTENTS_REGARDLESS_OF_OUTCOME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::TryReserveError>",
        "verus",
        || {
            <RustStdStandard<std::collections::TryReserveError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_INTO_ITER_ROUND_TRIPS_VIA_COLLECT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_into_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_into_iter_round_trips_via_collect".to_owned(),
            VERIFY_VEC_INTO_ITER_ROUND_TRIPS_VIA_COLLECT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::vec::IntoIter<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_DEQUE_ITER_ROUND_TRIPS_VIA_COLLECT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_deque_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_deque_iter_round_trips_via_collect".to_owned(),
            VERIFY_VEC_DEQUE_ITER_ROUND_TRIPS_VIA_COLLECT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CHARS_YIELDS_CHARACTERS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/chars_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::Chars<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_chars_yields_characters_in_order".to_owned(),
            VERIFY_CHARS_YIELDS_CHARACTERS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::Chars<'static>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::str::Chars<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
    "verify_chars_yields_characters_in_order"
);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::str::Chars<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
    "verify_chars_yields_characters_in_order"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        "verus",
        || {
            <RustStdStandard<std::str::Chars<'static>> as VerusWitness>::proof().to_string()
        },
    )
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

impl VerusWitness for RustStdStandard<std::collections::BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_max_heap_pair_pops_the_maximum_first".to_owned(),
            VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BinaryHeap<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::collections::BinaryHeap<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
    "verify_max_heap_pair_pops_the_maximum_first"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::BinaryHeap<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        "verus",
        "ensures",
        || BINARY_HEAP_MODEL_RECORDS_VALUES_IN_HEAP_ORDER_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        "verus",
        "ensures",
        || BINARY_HEAP_MODEL_POP_RETURNS_RECORDED_ORDER_VERUS_FRAGMENT,
    )
}

const VERIFY_FIFO_QUEUE_PAIR_POPS_IN_PUSH_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/linked_list_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fifo_queue_pair_pops_in_push_order".to_owned(),
            VERIFY_FIFO_QUEUE_PAIR_POPS_IN_PUSH_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::LinkedList<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::LinkedList<i32>> as VerusWitness>::proof().to_string()
        },
    )
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

impl VerusWitness for RustStdStandard<std::cell::Cell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cell_model_get_set_replace_round_trip".to_owned(),
            VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::Cell<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::cell::Cell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
    "verify_cell_model_get_set_replace_round_trip"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::Cell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        "ensures",
        || CELL_MODEL_NEW_STORES_INITIAL_VALUE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        "ensures",
        || CELL_MODEL_GET_READS_CURRENT_VALUE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        "ensures",
        || CELL_MODEL_REPLACE_RETURNS_PREVIOUS_VALUE_VERUS_FRAGMENT,
    )
}

/// [`WriteStoresNewValue`] reuses `Cell`'s own round-trip harness rather
/// than adding a new Verus proof: it names the shared write-through law
/// the harness already establishes.
impl VerusWitness for WriteStoresNewValue {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cell_model_get_set_replace_round_trip".to_owned(),
            VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(WriteStoresNewValue);

// `write_stores_new_value` is shared across `cell_carrier`,
// `ref_cell_carrier`, `unsafe_cell_carrier`, and
// `ordered_pair_iter_mut_carrier` -- no single harness to derive a
// clause-index selector from, so this derives from the predicate's own
// real declaration (`observed == new_value`, in its own parameter
// names) rather than any one caller's argument-substituted instance of
// it (previously `final(self).value == new_value`, `cell_carrier`'s own
// call-site spelling -- also real, just a different, less general
// representation of the same shared law).
amenable_derive::verus_ensures_predicate!(
    WriteStoresNewValue,
    "amenable_std::WriteStoresNewValue",
    "write_stores_new_value"
);

const VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/array_into_iter_carrier.rs");

/// [`ArrayIntoIterStartsAtFirstPosition`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the model's
/// initial-state law.
impl VerusWitness for ArrayIntoIterStartsAtFirstPosition {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ArrayIntoIterStartsAtFirstPosition);

amenable_derive::verus_ensures_predicate!(
    ArrayIntoIterStartsAtFirstPosition,
    "amenable_std::ArrayIntoIterStartsAtFirstPosition",
    "array_into_iter_model_starts_at_first_position"
);

/// [`ArrayIntoIterAdvanceMatchesPosition`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the model's
/// one-step transition law.
impl VerusWitness for ArrayIntoIterAdvanceMatchesPosition {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ArrayIntoIterAdvanceMatchesPosition);

amenable_derive::verus_ensures_predicate!(
    ArrayIntoIterAdvanceMatchesPosition,
    "amenable_std::ArrayIntoIterAdvanceMatchesPosition",
    "array_into_iter_advance_matches_position"
);

/// [`YieldsThreeValuesInOrderThenEnds`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the
/// fixed-length consuming-iterator law the carrier already establishes.
impl VerusWitness for YieldsThreeValuesInOrderThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(YieldsThreeValuesInOrderThenEnds);

amenable_derive::verus_ensures_witness!(
    YieldsThreeValuesInOrderThenEnds,
    "amenable_std::YieldsThreeValuesInOrderThenEnds",
    "verify_array_into_iter_model_yields_elements_in_order"
);

impl VerusWitness for RustStdStandard<std::array::IntoIter<i32, 3>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::array::IntoIter<i32, 3>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::array::IntoIter<i32, 3>>",
        "verus",
        || {
            <RustStdStandard<std::array::IntoIter<i32, 3>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ref_cell_carrier.rs");

// `verify_ref_cell_model_dynamic_borrow_rules`'s real VerusCallShape is
// no longer registered by hand here -- `verus_call_shape` derives it by
// parsing the real signature directly from
// crates/amenable_verus/src/rust_std/ref_cell_carrier.rs. Its own
// `&mut self`/`old`/`final` methods (`try_borrow`, `release_shared`,
// etc.) are purely internal to this one harness's body -- never
// independently registered or composed; this top-level harness is a
// plain value-returning function like any other. Its own `ensures` mixes
// raw tuple-field projections (some negated) with one named-predicate
// citation whose own argument is itself a projection-and-cast
// (`result.5 as int`) -- the reason `VerusCallShape.ensures`/`.requires`
// are plain `$placeholder` text templates rather than a structured
// predicate-call-only representation (a first design tried the latter
// and it didn't fit this harness at all), and derivation walks tokens
// directly rather than `Expr`'s own AST shape for the same reason.

impl VerusWitness for RustStdStandard<std::cell::RefCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_cell_model_dynamic_borrow_rules".to_owned(),
            VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::RefCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::RefCell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::RefCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
    "write_stores_new_value"
);

const VERIFY_ONCE_CELL_MODEL_INITIALIZES_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/once_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::OnceCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_cell_model_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_CELL_MODEL_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::OnceCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::OnceCell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::OnceCell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

// `get()` reads back exactly the stored `Option<i32>` -- neither
// `observed_value_matches_input`/`observed_pair_matches_input`'s more
// specific typed shape fits an `Option<i32>`-vs-`Option<i32>` read-back,
// so this uses the generic positive-equality predicate instead.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::OnceCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::OnceCell<i32>>",
    "values_are_equal"
);

const VERIFY_UNSAFE_CELL_MODEL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/unsafe_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::UnsafeCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unsafe_cell_model_get_mut_and_into_inner_round_trip".to_owned(),
            VERIFY_UNSAFE_CELL_MODEL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::UnsafeCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::UnsafeCell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::UnsafeCell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::UnsafeCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::UnsafeCell<i32>>",
    "write_stores_new_value"
);

const VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/lazy_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lazy_cell_model_caches_its_initializer_result".to_owned(),
            VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lazy_cell_model_caches_its_initializer_result".to_owned(),
            VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/weak_carrier.rs");

impl VerusWitness for RustStdStandard<std::rc::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::rc::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::rc::Weak<i32>>",
        "verus",
        || {
            <RustStdStandard<std::rc::Weak<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::sync::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Weak<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::Weak<i32>> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_from_utf8_error_model_recovers_the_original_bytes".to_owned(),
            VERIFY_FROM_UTF8_ERROR_MODEL_RECOVERS_THE_ORIGINAL_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::string::FromUtf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        || {
            <RustStdStandard<std::string::FromUtf8Error> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        "ensures",
        || FROM_UTF8_ERROR_MODEL_NEW_PRESERVES_BYTES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        "ensures",
        || FROM_UTF8_ERROR_MODEL_AS_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        "ensures",
        || FROM_UTF8_ERROR_MODEL_INTO_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    )
}

const VERIFY_ESCAPE_DEFAULT_MODEL_ESCAPES_A_CONTROL_BYTE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ascii_escape_carrier.rs");

impl VerusWitness for RustStdStandard<core::ascii::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_escape_default_model_escapes_a_control_byte".to_owned(),
            VERIFY_ESCAPE_DEFAULT_MODEL_ESCAPES_A_CONTROL_BYTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ascii::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ascii::EscapeDefault>",
        "verus",
        || {
            <RustStdStandard<core::ascii::EscapeDefault> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::CStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::CStr>",
        "verus",
        || {
            <RustStdStandard<core::ffi::CStr> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::CStr>",
        "verus",
        "ensures",
        || CSTR_TO_BYTES_MATCHES_MODEL_VERUS_FRAGMENT,
    )
}

/// [`NonNulByte`] reuses the same harness rather than adding a new Verus
/// proof — it names the precondition every `CStr`/`CString`-family proof
/// in this crate already requires, it doesn't prove anything new.
impl VerusWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(NonNulByte);

amenable_derive::verus_requires_witness!(
    NonNulByte,
    "amenable_std::NonNulByte",
    "verify_cstr_excludes_the_terminating_nul_from_to_bytes"
);

const VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ordered_pair_into_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::Drain<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::vec::Drain<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract, registered once per real type this carrier backs
// (matching the harness registration above): a freshly-constructed
// model always starts positioned before the first element.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::vec::Drain<'static, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::vec::Drain<'static, i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::collections::vec_deque::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::vec_deque::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::collections::linked_list::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::linked_list::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::collections::linked_list::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::linked_list::Iter<'static, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::string::Drain<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::string::Drain<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
        "verus",
        || {
            <RustStdStandard<std::string::Drain<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::string::Drain<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

const VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_extract_if_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_extract_if_model_partitions_by_the_predicate".to_owned(),
            VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness
    for RustStdStandard<
        std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_extract_if_model_partitions_by_the_predicate".to_owned(),
            VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SPLICE_MODEL_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/vec_splice_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_splice_model_replaces_a_range_and_yields_what_it_removed".to_owned(),
            VERIFY_SPLICE_MODEL_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_map_model_applies_its_closure_to_each_item".to_owned(),
            VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FILTER_MODEL_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_filter_model_yields_only_items_matching_the_predicate".to_owned(),
            VERIFY_FILTER_MODEL_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FILTER_MAP_MODEL_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_filter_map_model_applies_and_filters_in_one_step".to_owned(),
            VERIFY_FILTER_MAP_MODEL_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_MAP_WHILE_MODEL_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_map_while_model_maps_items_while_the_closure_returns_some".to_owned(),
            VERIFY_MAP_WHILE_MODEL_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CLONED_MODEL_CLONES_EACH_REFERENCED_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cloned_model_clones_each_referenced_item".to_owned(),
            VERIFY_CLONED_MODEL_CLONES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_COPIED_MODEL_COPIES_EACH_REFERENCED_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_copied_model_copies_each_referenced_item".to_owned(),
            VERIFY_COPIED_MODEL_COPIES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CHAIN_MODEL_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_chain_model_sequences_two_iterators_end_to_end".to_owned(),
            VERIFY_CHAIN_MODEL_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ZIP_MODEL_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_zip_model_pairs_items_from_two_iterators".to_owned(),
            VERIFY_ZIP_MODEL_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_enumerate_model_pairs_each_item_with_its_index".to_owned(),
            VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_enumerate_model_pairs_each_item_with_its_index".to_owned(),
            VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(IncrementHeadroom);

// Four sites need the tight, two-increment margin
// (verify_enumerate_model_pairs_each_item_with_its_index, verify_rev_
// model_reverses_iteration_order, verify_cycle_model_repeats_its_
// sequence_forever, verify_peekable_model_peek_does_not_consume) and call
// increment_headroom_holds directly; eight more need only the loosest,
// one-increment margin (verify_chain_model_sequences_two_iterators_end_
// to_end, verify_zip_model_pairs_items_from_two_iterators, verify_fuse_
// model_keeps_returning_none_once_exhausted, verify_inspect_model_calls_
// once_per_item_without_changing_values, verify_fn_pointer_model_calls_
// the_underlying_function, verify_map_model_applies_its_closure_to_each_
// item) and call single_increment_headroom_holds. The slice-chunk
// write-through models need a wider margin still and call
// ten_increment_headroom_holds. All three are real, shared
// `open spec fn`s in amenable_verus::rust_std::iter_sequence_carrier
// confirmed under real verus to give every call site genuine proof
// credit across carrier files -- see amenable_std::verus_gallery's
// cross_file_spec_fn_reuse_gets_real_proof_credit case.
amenable_derive::verus_requires_predicate!(
    IncrementHeadroom,
    "amenable_std::IncrementHeadroom",
    [
        "increment_headroom_holds",
        "single_increment_headroom_holds",
        "ten_increment_headroom_holds"
    ]
);

/// [`ValueUnchanged`] reuses `RefCell`'s own borrow-rules harness rather
/// than adding a new Verus proof — the harness's own `ensures` clauses
/// already establish this frame condition for `try_borrow`/
/// `try_borrow_mut`/`release_shared` (and `Weak::drop_strong` states the
/// identical claim) through one shared Verus `spec fn`, `value_unchanged`.
impl VerusWitness for ValueUnchanged {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_cell_model_dynamic_borrow_rules".to_owned(),
            VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ValueUnchanged);

amenable_derive::verus_ensures_predicate!(
    ValueUnchanged,
    "amenable_std::ValueUnchanged",
    "value_unchanged"
);

/// [`ObservedValueMatchesInput`] reuses the shared-reference harness
/// rather than adding a new Verus proof — it names the direct identity
/// postcondition that many simple scalar-observation carriers now state
/// through one shared Verus `spec fn`.
impl VerusWitness for ObservedValueMatchesInput {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_shared_reference_model_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ObservedValueMatchesInput);

// Registered under both "ensures" and "requires": `ref_cell_carrier.rs`'s
// `release_exclusive` states the identical direct-identity claim as a
// precondition too (`old(self).borrow_state == -1`), reusing the same
// real spec fn rather than adding a requires-only twin.
amenable_derive::verus_ensures_predicate!(
    ObservedValueMatchesInput,
    "amenable_std::ObservedValueMatchesInput",
    "observed_value_matches_input"
);

amenable_derive::verus_requires_predicate!(
    ObservedValueMatchesInput,
    "amenable_std::ObservedValueMatchesInput",
    "observed_value_matches_input"
);

/// [`ObservedOptionMatchesInput`] reuses the `Once` harness rather than
/// adding a new Verus proof — it names the direct `Option`-wrapped
/// identity postcondition that several `core::iter` generator carriers
/// now state through one shared Verus `spec fn`, the `Option`-wrapped
/// counterpart to [`ObservedValueMatchesInput`]'s bare-scalar version.
impl VerusWitness for ObservedOptionMatchesInput {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_yields_exactly_one_value".to_owned(),
            VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ObservedOptionMatchesInput);

amenable_derive::verus_ensures_predicate!(
    ObservedOptionMatchesInput,
    "amenable_std::ObservedOptionMatchesInput",
    "observed_option_matches_input"
);

/// [`ObservedPairMatchesInput`] reuses the `AtomicBool` load-store
/// harness rather than adding a new Verus proof — it names the direct
/// pair-identity postcondition that several accommodation models now
/// state through one shared, generic Verus `spec fn`.
impl VerusWitness for ObservedPairMatchesInput {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_atomic_bool_model_load_store".to_owned(),
            VERIFY_ATOMIC_BOOL_MODEL_LOAD_STORE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ObservedPairMatchesInput);

amenable_derive::verus_ensures_predicate!(
    ObservedPairMatchesInput,
    "amenable_std::ObservedPairMatchesInput",
    "observed_pair_matches_input"
);

const VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_rev_model_reverses_iteration_order".to_owned(),
            VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_skip_model_discards_the_first_n_items".to_owned(),
            VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_skip_while_model_discards_items_while_the_predicate_holds".to_owned(),
            VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_step_by_model_yields_every_nth_item".to_owned(),
            VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Take<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_model_yields_at_most_n_items".to_owned(),
            VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Take<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Take<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Take<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_while_model_yields_items_while_the_predicate_holds".to_owned(),
            VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Once<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_yields_exactly_one_value".to_owned(),
            VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Once<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Once<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::OnceWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_with_model_calls_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::OnceWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::OnceWith<fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::OnceWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Repeat<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_model_yields_the_same_value_forever".to_owned(),
            VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Repeat<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Repeat<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_with_model_calls_its_closure_once_per_item".to_owned(),
            VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::RepeatWith<fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::RepeatWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatN<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_n_model_yields_the_value_exactly_n_times".to_owned(),
            VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatN<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::RepeatN<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::RepeatN<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Empty<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_empty_model_yields_nothing".to_owned(),
            VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Empty<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Empty<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Empty<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CYCLE_MODEL_REPEATS_ITS_SEQUENCE_FOREVER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cycle_model_repeats_its_sequence_forever".to_owned(),
            VERIFY_CYCLE_MODEL_REPEATS_ITS_SEQUENCE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FUSE_MODEL_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fuse_model_keeps_returning_none_once_exhausted".to_owned(),
            VERIFY_FUSE_MODEL_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_INSPECT_MODEL_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_inspect_model_calls_once_per_item_without_changing_values".to_owned(),
            VERIFY_INSPECT_MODEL_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_PEEKABLE_MODEL_PEEK_DOES_NOT_CONSUME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_peekable_model_peek_does_not_consume".to_owned(),
            VERIFY_PEEKABLE_MODEL_PEEK_DOES_NOT_CONSUME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_scan_model_threads_state_through_its_closure".to_owned(),
            VERIFY_SCAN_MODEL_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_flat_map_model_flattens_each_generated_iterator".to_owned(),
            VERIFY_FLAT_MAP_MODEL_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
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
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FlatMap<std::array::IntoIter<i32, 1>, std::ops::Range<i32>, fn(i32) -> std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::FlatMap<std::array::IntoIter<i32, 1>, std::ops::Range<i32>, fn(i32) -> std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FLATTEN_MODEL_CONCATENATES_THE_INNER_ITERATORS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_flatten_model_concatenates_the_inner_iterators".to_owned(),
            VERIFY_FLATTEN_MODEL_CONCATENATES_THE_INNER_ITERATORS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_successors_model_generates_from_the_previous_item".to_owned(),
            VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FROM_FN_MODEL_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_fn_model_yields_until_the_closure_returns_none".to_owned(),
            VERIFY_FROM_FN_MODEL_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ALIGNMENT_MODEL_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::Alignment> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_alignment_model_reaches_the_formatter_from_the_format_spec".to_owned(),
            VERIFY_ALIGNMENT_MODEL_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Alignment>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Alignment>",
        "verus",
        || {
            <RustStdStandard<std::fmt::Alignment> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_FORMATTER_MODEL_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::Formatter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_formatter_model_exposes_the_parsed_width_and_precision".to_owned(),
            VERIFY_FORMATTER_MODEL_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Formatter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Formatter<'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::Formatter<'static>> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_arguments_model_renders_the_same_as_the_value_itself".to_owned(),
            VERIFY_ARGUMENTS_MODEL_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Arguments<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Arguments<'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::Arguments<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Arguments<'static>>",
        "verus",
        "ensures",
        || FMT_ARGUMENTS_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT,
    )
}

const VERIFY_FROM_FN_MODEL_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_fn_model_forwards_display_to_the_supplied_closure".to_owned(),
            VERIFY_FROM_FN_MODEL_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>",
        "verus",
        "ensures",
        || FMT_FROM_FN_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_STRUCT_MODEL_RENDERS_NAMED_FIELDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugStruct<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_struct_model_renders_named_fields".to_owned(),
            VERIFY_DEBUG_STRUCT_MODEL_RENDERS_NAMED_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugStruct<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugStruct<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugStruct<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugStruct<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_STRUCT_RESULT_MATCHES_NAMED_FIELDS_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_TUPLE_MODEL_RENDERS_POSITIONAL_FIELDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugTuple<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_tuple_model_renders_positional_fields".to_owned(),
            VERIFY_DEBUG_TUPLE_MODEL_RENDERS_POSITIONAL_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugTuple<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugTuple<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugTuple<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugTuple<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_TUPLE_RESULT_MATCHES_POSITIONAL_FIELDS_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_LIST_MODEL_RENDERS_ENTRIES_IN_BRACKETS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugList<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_list_model_renders_entries_in_brackets".to_owned(),
            VERIFY_DEBUG_LIST_MODEL_RENDERS_ENTRIES_IN_BRACKETS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugList<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugList<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugList<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugList<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_LIST_RESULT_MATCHES_ENTRIES_IN_BRACKETS_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_SET_MODEL_RENDERS_ENTRIES_IN_BRACES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugSet<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_set_model_renders_entries_in_braces".to_owned(),
            VERIFY_DEBUG_SET_MODEL_RENDERS_ENTRIES_IN_BRACES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugSet<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugSet<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugSet<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugSet<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_SET_RESULT_MATCHES_ENTRIES_IN_BRACES_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_MAP_MODEL_RENDERS_KEY_VALUE_PAIRS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugMap<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_map_model_renders_key_value_pairs".to_owned(),
            VERIFY_DEBUG_MAP_MODEL_RENDERS_KEY_VALUE_PAIRS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugMap<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugMap<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugMap<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugMap<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_MAP_RESULT_MATCHES_KEY_VALUE_PAIR_VERUS_FRAGMENT,
    )
}

const VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/discriminant_carrier.rs");

impl VerusWitness for RustStdStandard<std::mem::Discriminant<Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_discriminant_model_identifies_variant_not_payload".to_owned(),
            VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::mem::Discriminant<Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::mem::Discriminant<Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::mem::Discriminant<Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

/// `RustStdStandard<NonZero<T>>`'s Verus proof states its claim as two
/// separate real `ensures` clauses (`non_zero_new_accepts_nonzero`,
/// `non_zero_new_rejects_zero`) — an iff split into its two
/// implications, not one expression, since Verus has no single iff
/// operator. `Ensures<VerusVerifier>::Bound = &'static [&'static str]`
/// holds both, uniformly, as first-class elements of the trait's own
/// value -- the original motivating case for that shape (see Design E
/// in `docs/VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`'s companion
/// contract-type work): no more picking one direction as "canonical"
/// and smuggling the other in through a bespoke supplementary
/// `ContractRecord`.
macro_rules! impl_non_zero_verus_witness {
    ($($ty:ty => $harness:literal),* $(,)?) => {
        $(
            impl VerusWitness for RustStdStandard<std::num::NonZero<$ty>> {
                type SupportingEvidence = Self;
                type ProofArtifact = VerusCheckedProof;

                fn proof() -> Self::ProofArtifact {
                    VerusCheckedProof::new(
                        $harness.to_owned(),
                        include_str!("../../amenable_verus/src/rust_std/non_zero_carrier.rs").to_owned(),
                        <Self::SupportingEvidence as Evidence>::basis().audit(),
                    )
                }
            }

            bridge_verus_witness!(RustStdStandard<std::num::NonZero<$ty>>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                    "verus",
                    || <RustStdStandard<std::num::NonZero<$ty>> as VerusWitness>::proof().to_string(),
                )
            }

            amenable_derive::verus_ensures_witness!(
                RustStdStandard<std::num::NonZero<$ty>>,
                concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                $harness
            );
        )*
    };
}

const VERIFY_ITER_MODEL_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/slice_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_iter_model_yields_shared_references_in_order".to_owned(),
            VERIFY_ITER_MODEL_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::slice::Iter<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ITER_MUT_MODEL_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/slice_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_iter_mut_model_yields_mutable_references_that_write_through".to_owned(),
            VERIFY_ITER_MUT_MODEL_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::slice::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_IPV4_ADDR_MODEL_OCTETS_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Ipv4Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ipv4_addr_model_octets_round_trip".to_owned(),
            VERIFY_IPV4_ADDR_MODEL_OCTETS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Ipv4Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Ipv4Addr>",
        "verus",
        || {
            <RustStdStandard<std::net::Ipv4Addr> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IPV6_ADDR_MODEL_SEGMENTS_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Ipv6Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ipv6_addr_model_segments_round_trip".to_owned(),
            VERIFY_IPV6_ADDR_MODEL_SEGMENTS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Ipv6Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Ipv6Addr>",
        "verus",
        || {
            <RustStdStandard<std::net::Ipv6Addr> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IP_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::IpAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ip_addr_model_variant_matches_its_kind".to_owned(),
            VERIFY_IP_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::IpAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::IpAddr>",
        "verus",
        || {
            <RustStdStandard<std::net::IpAddr> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: `IpAddr`'s `V4` variant always round-trips its
// wrapped octets exactly.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::net::IpAddr>,
    "amenable_std::rust_std::RustStdStandard<std::net::IpAddr>",
    "ip_addr_model_v4_octets_match_input"
);

const VERIFY_SOCKET_ADDR_V4_MODEL_ROUND_TRIPS_IP_AND_PORT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddrV4> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_socket_addr_v4_model_round_trips_ip_and_port".to_owned(),
            VERIFY_SOCKET_ADDR_V4_MODEL_ROUND_TRIPS_IP_AND_PORT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddrV4>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::SocketAddrV4>",
        "verus",
        || {
            <RustStdStandard<std::net::SocketAddrV4> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SOCKET_ADDR_V6_MODEL_ROUND_TRIPS_ALL_FIELDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddrV6> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_socket_addr_v6_model_round_trips_all_fields".to_owned(),
            VERIFY_SOCKET_ADDR_V6_MODEL_ROUND_TRIPS_ALL_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddrV6>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::SocketAddrV6>",
        "verus",
        || {
            <RustStdStandard<std::net::SocketAddrV6> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SOCKET_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_socket_addr_model_variant_matches_its_kind".to_owned(),
            VERIFY_SOCKET_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::SocketAddr>",
        "verus",
        || {
            <RustStdStandard<std::net::SocketAddr> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_ordered_pair_iter_mut_model_writes_through_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_iter_mut_model_writes_through_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/unordered_pair_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unordered_pair_model_yields_every_element_once".to_owned(),
            VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unordered_pair_model_yields_every_element_once".to_owned(),
            VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unordered_pair_model_yields_every_element_once".to_owned(),
            VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_max_heap_pair_pops_the_maximum_first".to_owned(),
            VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_REF_MODEL_DEREFS_TO_THE_BORROWED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ref_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::Ref<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_model_derefs_to_the_borrowed_value".to_owned(),
            VERIFY_REF_MODEL_DEREFS_TO_THE_BORROWED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::Ref<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Ref<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::Ref<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REF_MUT_MODEL_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ref_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::RefMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_mut_model_derefs_and_writes_through_to_the_cell".to_owned(),
            VERIFY_REF_MUT_MODEL_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::RefMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::RefMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::RefMut<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
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

impl VerusWitness for RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates".to_owned(),
            VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>,
    "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
    "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        || {
            <RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "ensures",
        || DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "ensures",
        || DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    )
}

impl VerusWitness for RustStdStandard<std::char::DecodeUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates".to_owned(),
            VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::DecodeUtf16Error>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::char::DecodeUtf16Error>,
    "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
    "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        || {
            <RustStdStandard<std::char::DecodeUtf16Error> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "ensures",
        || DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "ensures",
        || DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    )
}

const VERIFY_TO_LOWERCASE_MODEL_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::ToLowercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_to_lowercase_model_maps_an_uppercase_ascii_letter".to_owned(),
            VERIFY_TO_LOWERCASE_MODEL_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::ToLowercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::ToLowercase>",
        "verus",
        || {
            <RustStdStandard<std::char::ToLowercase> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TO_UPPERCASE_MODEL_MAPS_A_LOWERCASE_ASCII_LETTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::ToUppercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_to_uppercase_model_maps_a_lowercase_ascii_letter".to_owned(),
            VERIFY_TO_UPPERCASE_MODEL_MAPS_A_LOWERCASE_ASCII_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::ToUppercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::ToUppercase>",
        "verus",
        || {
            <RustStdStandard<std::char::ToUppercase> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAR_ESCAPE_DEBUG_MODEL_ESCAPES_A_NEWLINE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeDebug> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_escape_debug_model_escapes_a_newline".to_owned(),
            VERIFY_CHAR_ESCAPE_DEBUG_MODEL_ESCAPES_A_NEWLINE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeDebug>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::EscapeDebug>",
        "verus",
        || {
            <RustStdStandard<std::char::EscapeDebug> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAR_ESCAPE_DEFAULT_MODEL_ESCAPES_A_NEWLINE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_escape_default_model_escapes_a_newline".to_owned(),
            VERIFY_CHAR_ESCAPE_DEFAULT_MODEL_ESCAPES_A_NEWLINE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::EscapeDefault>",
        "verus",
        || {
            <RustStdStandard<std::char::EscapeDefault> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeUnicode> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_escape_unicode_model_renders_the_codepoint_escape".to_owned(),
            VERIFY_CHAR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeUnicode>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::EscapeUnicode>",
        "verus",
        || {
            <RustStdStandard<std::char::EscapeUnicode> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_slice_chunks_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/slice_chunks_carrier.rs");

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
    };
}

const TEN_INCREMENT_WRITE_THROUGH_VERUS_FRAGMENT: &str = r#"pub open spec fn ten_increment_write_through(before: int, after: int) -> bool {
    after == before + 10
}"#;

macro_rules! register_slice_chunks_increment_fragment {
    ($ty:ty) => {
        ::inventory::submit! {
            ::amenable_core::ContractRecord::new(
                concat!(
                    "amenable_std::rust_std::RustStdStandard<",
                    stringify!($ty),
                    ">"
                ),
                "verus",
                "ensures",
                || TEN_INCREMENT_WRITE_THROUGH_VERUS_FRAGMENT,
            )
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
                VerusCheckedProof::new(
                    "verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate"
                        .to_owned(),
                    VERIFY_CHUNK_BY_MODEL_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC
                        .to_owned(),
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

// `verify_escape_ascii_model_leaves_printable_bytes_unescaped`'s real
// VerusCallShape is no longer registered by hand here -- `verus_call_shape`
// derives it (including its `requires` clause -- the first harness with a
// real precondition, exercising the compositional renderer's
// requires-propagation) by parsing the real signature directly from
// crates/amenable_verus/src/rust_std/escape_ascii_carrier.rs.

impl VerusWitness for RustStdStandard<std::slice::EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_escape_ascii_model_leaves_printable_bytes_unescaped".to_owned(),
            VERIFY_ESCAPE_ASCII_MODEL_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::EscapeAscii<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        "verus",
        || {
            <RustStdStandard<std::slice::EscapeAscii<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        "verus",
        "requires",
        || ESCAPE_ASCII_INPUT_IS_PRINTABLE_ASCII_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        "verus",
        "ensures",
        || ESCAPE_ASCII_RESULT_MATCHES_PRINTABLE_PLUS_NEWLINE_ESCAPE_VERUS_FRAGMENT,
    )
}

const VERIFY_GET_DISJOINT_MUT_MODEL_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/get_disjoint_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_get_disjoint_mut_model_rejects_overlap_and_out_of_bounds".to_owned(),
            VERIFY_GET_DISJOINT_MUT_MODEL_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::GetDisjointMutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::GetDisjointMutError>",
        "verus",
        || {
            <RustStdStandard<std::slice::GetDisjointMutError> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_str_ascii_iter_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_ascii_iter_carrier.rs");

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

// Singleton contract: the sole char's byte offset in a one-character
// str is always 0.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::CharIndices<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::CharIndices<'static>>",
    "char_indices_first_offset_is_zero"
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
        VerusCheckedProof::new(
            "verify_lines_model_splits_on_line_endings".to_owned(),
            VERIFY_LINES_MODEL_SPLITS_ON_LINE_ENDINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::Lines<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
        "verus",
        || {
            <RustStdStandard<std::str::Lines<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_LINES_ANY_MODEL_SPLITS_ON_ANY_LINE_ENDING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::LinesAny<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lines_any_model_splits_on_any_line_ending".to_owned(),
            VERIFY_LINES_ANY_MODEL_SPLITS_ON_ANY_LINE_ENDING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::LinesAny<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::LinesAny<'static>>",
        "verus",
        || {
            <RustStdStandard<std::str::LinesAny<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_str_whitespace_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_whitespace_carrier.rs");

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

// Singleton contract: the one bad byte is always exactly 0xFF in this
// fixed example.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::Utf8Chunk<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Utf8Chunk<'static>>",
    "utf8_chunk_invalid_byte_is_0xff"
);

impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Error,
    "verify_utf8_error_model_reports_the_valid_prefix_length_and_error_span",
    VERIFY_UTF8_ERROR_MODEL_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC
);

// Singleton contract: the fixed example's valid-prefix length (2) and
// error span (1).
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::Utf8Error>,
    "amenable_std::rust_std::RustStdStandard<std::str::Utf8Error>",
    "utf8_error_reports_length_and_span"
);

macro_rules! impl_str_pattern_split_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_pattern_split_carrier.rs");

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
    };
}

impl_str_pattern_reverse_verus_witness!(
    std::str::RSplit<'static, char>,
    "verify_str_rsplit_model_yields_substrings_from_the_back",
    VERIFY_STR_RSPLIT_MODEL_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::RSplit<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::RSplit<'static, char>>",
    "values_are_distinct"
);

impl_str_pattern_reverse_verus_witness!(
    std::str::RSplitN<'static, char>,
    "verify_str_rsplitn_model_limits_to_n_substrings_from_the_back",
    VERIFY_STR_RSPLITN_MODEL_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::RSplitN<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::RSplitN<'static, char>>",
    "values_are_distinct"
);

macro_rules! impl_str_pattern_terminator_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/str_pattern_terminator_carrier.rs");

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
    };
}

impl_str_pattern_terminator_verus_witness!(
    std::str::SplitTerminator<'static, char>,
    "verify_str_split_terminator_model_suppresses_a_trailing_empty_substring",
    VERIFY_STR_SPLIT_TERMINATOR_MODEL_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::SplitTerminator<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::SplitTerminator<'static, char>>",
    "values_are_distinct"
);

impl_str_pattern_terminator_verus_witness!(
    std::str::RSplitTerminator<'static, char>,
    "verify_str_rsplit_terminator_model_suppresses_a_trailing_empty_substring_from_the_back",
    VERIFY_STR_RSPLIT_TERMINATOR_MODEL_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::RSplitTerminator<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::RSplitTerminator<'static, char>>",
    "values_are_distinct"
);

const VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_pattern_match_carrier.rs");

macro_rules! impl_str_matches_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_str_matches_model_yields_every_non_overlapping_occurrence".to_owned(),
                    VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
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
    };
}

impl_str_matches_verus_witness!(std::str::Matches<'static, char>);
impl_str_matches_verus_witness!(std::str::RMatches<'static, char>);

/// [`AsciiByte`] reuses the same harness rather than adding a new Verus
/// proof — it names the precondition the harness already requires, it
/// doesn't prove anything new. The precondition recurs across four
/// carrier files (`str_ascii_iter_carrier`, `str_pattern_match_carrier`,
/// `str_pattern_reverse_carrier`, `str_pattern_terminator_carrier`) —
/// every real site now calls the one shared spec fn,
/// `primitive_shapes_carrier::is_ascii_byte`, registered below via
/// [`IS_ASCII_BYTE_VERUS_FRAGMENT`]. (An earlier version of this
/// registration hand-typed one inert, non-`fn` string per carrier's
/// local variable spelling instead of a real shared predicate — that
/// text could never satisfy the call-shape recognition rule and never
/// actually named any of these sites; replaced rather than kept
/// alongside the real fragment.)
impl VerusWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_matches_model_yields_every_non_overlapping_occurrence".to_owned(),
            VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(AsciiByte);

amenable_derive::verus_requires_predicate!(AsciiByte, "amenable_std::AsciiByte", "is_ascii_byte");

const VERIFY_STR_MATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/str_pattern_match_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::MatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_match_indices_model_pairs_each_match_with_its_byte_offset".to_owned(),
            VERIFY_STR_MATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::MatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::MatchIndices<'static, char>>",
        "verus",
        || {
            <RustStdStandard<std::str::MatchIndices<'static, char>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STR_RMATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC:
    &str = include_str!("../../amenable_verus/src/rust_std/str_pattern_match_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::RMatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_rmatch_indices_model_pairs_each_match_with_its_byte_offset_from_the_back"
                .to_owned(),
            VERIFY_STR_RMATCH_INDICES_MODEL_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::RMatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::RMatchIndices<'static, char>>",
        "verus",
        || {
            <RustStdStandard<std::str::RMatchIndices<'static, char>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_BUF_READER_MODEL_READS_THE_UNDERLYING_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_buffered_read_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::BufReader<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_buf_reader_model_reads_the_underlying_bytes".to_owned(),
            VERIFY_BUF_READER_MODEL_READS_THE_UNDERLYING_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::BufReader<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::BufReader<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::BufReader<&'static [u8]>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_BUF_WRITER_MODEL_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_buf_writer_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::BufWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_buf_writer_model_flushes_to_the_underlying_writer".to_owned(),
            VERIFY_BUF_WRITER_MODEL_FLUSHES_TO_THE_UNDERLYING_WRITER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::BufWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::BufWriter<Vec<u8>>>",
        "verus",
        || {
            <RustStdStandard<std::io::BufWriter<Vec<u8>>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BYTES_MODEL_YIELDS_ONE_BYTE_AT_A_TIME_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_bytes_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Bytes<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_bytes_model_yields_one_byte_at_a_time".to_owned(),
            VERIFY_BYTES_MODEL_YIELDS_ONE_BYTE_AT_A_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Bytes<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Bytes<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Bytes<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INTO_INNER_ERROR_MODEL_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_into_inner_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_into_inner_error_model_recovers_the_writer_and_the_flush_error".to_owned(),
            VERIFY_INTO_INNER_ERROR_MODEL_RECOVERS_THE_WRITER_AND_THE_FLUSH_ERROR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>>",
        "verus",
        || {
            <RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_LINE_WRITER_MODEL_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_line_writer_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::LineWriter<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_line_writer_model_flushes_on_a_newline_but_not_before_one".to_owned(),
            VERIFY_LINE_WRITER_MODEL_FLUSHES_ON_A_NEWLINE_BUT_NOT_BEFORE_ONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::LineWriter<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::LineWriter<Vec<u8>>>",
        "verus",
        || {
            <RustStdStandard<std::io::LineWriter<Vec<u8>>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::LineWriter<Vec<u8>>>,
    "amenable_std::rust_std::RustStdStandard<std::io::LineWriter<Vec<u8>>>",
    "is_not_a_newline_byte"
);

const VERIFY_LINES_MODEL_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Lines<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lines_model_splits_on_newlines_and_drops_the_terminator".to_owned(),
            VERIFY_LINES_MODEL_SPLITS_ON_NEWLINES_AND_DROPS_THE_TERMINATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Lines<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Lines<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::Lines<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Lines<&'static [u8]>>",
    "is_not_a_line_terminator_byte"
);

const VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_pipe_carrier.rs");

macro_rules! impl_io_pipe_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_pipe_model_delivers_written_bytes_to_the_paired_reader".to_owned(),
                    VERIFY_PIPE_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_PAIRED_READER_SRC.to_owned(),
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
        VerusCheckedProof::new(
            "verify_split_model_segments_on_the_given_byte_and_drops_it".to_owned(),
            VERIFY_SPLIT_MODEL_SEGMENTS_ON_THE_GIVEN_BYTE_AND_DROPS_IT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Split<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Split<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

// The shared pairwise-distinctness precondition `amenable_std::
// verus_witness` registers for several accommodation models that build
// a symbolic non-overlapping match/split window.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::io::Split<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Split<&'static [u8]>>",
    "values_are_distinct"
);

const VERIFY_WRITER_PANICKED_MODEL_RECOVERS_THE_BUFFERED_DATA_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_writer_panicked_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::WriterPanicked> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_writer_panicked_model_recovers_the_buffered_data".to_owned(),
            VERIFY_WRITER_PANICKED_MODEL_RECOVERS_THE_BUFFERED_DATA_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::WriterPanicked>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::WriterPanicked>",
        "verus",
        || {
            <RustStdStandard<std::io::WriterPanicked> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_io_empty_repeat_sink_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../amenable_verus/src/rust_std/io_empty_repeat_sink_carrier.rs");

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
    };
}

impl_io_empty_repeat_sink_verus_witness!(
    std::io::Empty,
    "verify_empty_model_read_reports_end_of_file",
    VERIFY_EMPTY_MODEL_READ_REPORTS_END_OF_FILE_SRC
);

// Singleton contract: `Empty::read` always reports the literal `0`
// bytes read.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Empty>,
    "amenable_std::rust_std::RustStdStandard<std::io::Empty>",
    "empty_read_reports_zero_bytes"
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
        VerusCheckedProof::new(
            "verify_seek_from_model_round_trips_each_variants_offset".to_owned(),
            VERIFY_SEEK_FROM_MODEL_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::SeekFrom>",
        "verus",
        || {
            <RustStdStandard<std::io::SeekFrom> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAIN_MODEL_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_chain_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_chain_model_reads_the_first_source_then_the_second".to_owned(),
            VERIFY_CHAIN_MODEL_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CURSOR_MODEL_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_cursor_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Cursor<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cursor_model_read_advances_position_and_seek_repositions_it".to_owned(),
            VERIFY_CURSOR_MODEL_READ_ADVANCES_POSITION_AND_SEEK_REPOSITIONS_IT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Cursor<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Cursor<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Cursor<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: reading two bytes from position zero advances the
// position to exactly 2, and seeking back to `Start(0)` resets it to
// exactly 0.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Cursor<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Cursor<&'static [u8]>>",
    "cursor_positions_after_read_then_seek"
);

const VERIFY_ERROR_MODEL_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_error_model_from_error_kind_preserves_the_kind".to_owned(),
            VERIFY_ERROR_MODEL_FROM_ERROR_KIND_PRESERVES_THE_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Error>",
        "verus",
        || {
            <RustStdStandard<std::io::Error> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IO_SLICE_MODEL_DEREFS_TO_THE_WRAPPED_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IoSlice<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_io_slice_model_derefs_to_the_wrapped_bytes".to_owned(),
            VERIFY_IO_SLICE_MODEL_DEREFS_TO_THE_WRAPPED_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IoSlice<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::IoSlice<'static>>",
        "verus",
        || {
            <RustStdStandard<std::io::IoSlice<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IO_SLICE_MUT_MODEL_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::IoSliceMut<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_io_slice_mut_model_derefs_to_and_permits_mutating_the_wrapped_bytes".to_owned(),
            VERIFY_IO_SLICE_MUT_MODEL_DEREFS_TO_AND_PERMITS_MUTATING_THE_WRAPPED_BYTES_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::IoSliceMut<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::IoSliceMut<'static>>",
        "verus",
        || {
            <RustStdStandard<std::io::IoSliceMut<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TAKE_MODEL_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/io_take_carrier.rs");

impl VerusWitness for RustStdStandard<std::io::Take<&'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_model_caps_reads_at_the_remaining_limit".to_owned(),
            VERIFY_TAKE_MODEL_CAPS_READS_AT_THE_REMAINING_LIMIT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::io::Take<&'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
        "verus",
        || {
            <RustStdStandard<std::io::Take<&'static [u8]>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: the allowance is always exactly exhausted (0)
// after a read that consumes the whole limit.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::io::Take<&'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Take<&'static [u8]>>",
    "take_allowance_is_exhausted"
);

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
        VerusCheckedProof::new(
            "verify_atomic_ptr_model_load_store_swap_and_compare_exchange".to_owned(),
            VERIFY_ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::atomic::AtomicPtr<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::sync::atomic::AtomicPtr<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
    "verify_atomic_ptr_model_load_store_swap_and_compare_exchange"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::atomic::AtomicPtr<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::atomic::AtomicPtr<i32>>",
        "verus",
        "ensures",
        || ATOMIC_PTR_MODEL_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_atomic_ordering_model_relaxed_store_is_observable".to_owned(),
            VERIFY_ATOMIC_ORDERING_MODEL_RELAXED_STORE_IS_OBSERVABLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::atomic::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ordering>",
        "verus",
        || {
            <RustStdStandard<std::sync::atomic::Ordering> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHILD_MODEL_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Child> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_model_has_a_process_id_and_can_be_waited_on".to_owned(),
            VERIFY_CHILD_MODEL_HAS_A_PROCESS_ID_AND_CAN_BE_WAITED_ON_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Child>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Child>",
        "verus",
        || {
            <RustStdStandard<std::process::Child> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::process::Child>,
    "amenable_std::rust_std::RustStdStandard<std::process::Child>",
    "process_id_is_nonzero"
);

const VERIFY_CHILD_STDERR_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStderr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_stderr_model_captures_what_the_child_wrote_to_stderr".to_owned(),
            VERIFY_CHILD_STDERR_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDERR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStderr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ChildStderr>",
        "verus",
        || {
            <RustStdStandard<std::process::ChildStderr> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHILD_STDIN_MODEL_IS_READABLE_BY_THE_CHILD_PROCESS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStdin> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_stdin_model_is_readable_by_the_child_process".to_owned(),
            VERIFY_CHILD_STDIN_MODEL_IS_READABLE_BY_THE_CHILD_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStdin>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ChildStdin>",
        "verus",
        || {
            <RustStdStandard<std::process::ChildStdin> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHILD_STDOUT_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_child_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ChildStdout> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_child_stdout_model_captures_what_the_child_wrote_to_stdout".to_owned(),
            VERIFY_CHILD_STDOUT_MODEL_CAPTURES_WHAT_THE_CHILD_WROTE_TO_STDOUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ChildStdout>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ChildStdout>",
        "verus",
        || {
            <RustStdStandard<std::process::ChildStdout> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMMAND_MODEL_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Command> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_command_model_env_override_is_visible_to_the_spawned_process".to_owned(),
            VERIFY_COMMAND_MODEL_ENV_OVERRIDE_IS_VISIBLE_TO_THE_SPAWNED_PROCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Command>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Command>",
        "verus",
        || {
            <RustStdStandard<std::process::Command> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMMAND_ARGS_MODEL_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::CommandArgs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_command_args_model_reports_the_configured_arguments".to_owned(),
            VERIFY_COMMAND_ARGS_MODEL_REPORTS_THE_CONFIGURED_ARGUMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::CommandArgs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::CommandArgs<'static>>",
        "verus",
        || {
            <RustStdStandard<std::process::CommandArgs<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_COMMAND_ENVS_MODEL_REPORTS_THE_CONFIGURED_OVERRIDES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_command_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::CommandEnvs<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_command_envs_model_reports_the_configured_overrides".to_owned(),
            VERIFY_COMMAND_ENVS_MODEL_REPORTS_THE_CONFIGURED_OVERRIDES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::CommandEnvs<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::CommandEnvs<'static>>",
        "verus",
        || {
            <RustStdStandard<std::process::CommandEnvs<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_EXIT_STATUS_MODEL_REPORTS_A_NONZERO_EXIT_CODE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_exit_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::ExitStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_exit_status_model_reports_a_nonzero_exit_code".to_owned(),
            VERIFY_EXIT_STATUS_MODEL_REPORTS_A_NONZERO_EXIT_CODE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::ExitStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::ExitStatus>",
        "verus",
        || {
            <RustStdStandard<std::process::ExitStatus> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::process::ExitStatus>,
    "amenable_std::rust_std::RustStdStandard<std::process::ExitStatus>",
    "exit_code_is_nonzero"
);

const VERIFY_OUTPUT_MODEL_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_exit_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Output> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_output_model_captures_stdout_and_the_exit_status".to_owned(),
            VERIFY_OUTPUT_MODEL_CAPTURES_STDOUT_AND_THE_EXIT_STATUS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Output>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Output>",
        "verus",
        || {
            <RustStdStandard<std::process::Output> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: this fixed example's exit code is always 0
// (success).
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::process::Output>,
    "amenable_std::rust_std::RustStdStandard<std::process::Output>",
    "output_exit_code_is_success"
);

const VERIFY_STDIO_MODEL_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/process_stdio_carrier.rs");

impl VerusWitness for RustStdStandard<std::process::Stdio> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_stdio_model_null_discards_the_childs_output_handle".to_owned(),
            VERIFY_STDIO_MODEL_NULL_DISCARDS_THE_CHILDS_OUTPUT_HANDLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::process::Stdio>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::process::Stdio>",
        "verus",
        || {
            <RustStdStandard<std::process::Stdio> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_ancestors_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Ancestors<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ancestors_model_yields_self_then_each_parent_up_to_root".to_owned(),
            VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Ancestors<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Ancestors<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Ancestors<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMPONENT_MODEL_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Component<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_component_model_distinguishes_root_from_normal_segments".to_owned(),
            VERIFY_COMPONENT_MODEL_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Component<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Component<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Component<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMPONENTS_MODEL_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Components<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_components_model_yields_root_then_named_segments_in_order".to_owned(),
            VERIFY_COMPONENTS_MODEL_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Components<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Components<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Components<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ITER_MODEL_YIELDS_THE_NAMED_SEGMENTS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Iter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_iter_model_yields_the_named_segments".to_owned(),
            VERIFY_ITER_MODEL_YIELDS_THE_NAMED_SEGMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Iter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Iter<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: `.iter()` over the fixed example `"/a/b"` always
// yields exactly 3 segments.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::path::Iter<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
    "path_iter_yields_three_segments"
);

const VERIFY_DISPLAY_MODEL_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_display_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_display_model_renders_a_valid_utf8_path_verbatim".to_owned(),
            VERIFY_DISPLAY_MODEL_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Display<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Display<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PATH_MODEL_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Path> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_path_model_derives_extension_file_name_and_parent".to_owned(),
            VERIFY_PATH_MODEL_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Path>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Path>",
        "verus",
        || {
            <RustStdStandard<std::path::Path> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PATH_BUF_MODEL_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_buf_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::PathBuf> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_path_buf_model_push_pop_and_join_build_the_expected_path".to_owned(),
            VERIFY_PATH_BUF_MODEL_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::PathBuf>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::PathBuf>",
        "verus",
        || {
            <RustStdStandard<std::path::PathBuf> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PREFIX_MODEL_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Prefix<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_prefix_model_disk_identifies_the_drive_letter".to_owned(),
            VERIFY_PREFIX_MODEL_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Prefix<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Prefix<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Prefix<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PREFIX_COMPONENT_MODEL_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::PrefixComponent<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_prefix_component_model_pairs_raw_text_with_parsed_prefix".to_owned(),
            VERIFY_PREFIX_COMPONENT_MODEL_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::PrefixComponent<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::PrefixComponent<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::PrefixComponent<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STRIP_PREFIX_ERROR_MODEL_REPORTS_A_NON_MATCHING_PREFIX_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/path_strip_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::StripPrefixError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_strip_prefix_error_model_reports_a_non_matching_prefix".to_owned(),
            VERIFY_STRIP_PREFIX_ERROR_MODEL_REPORTS_A_NON_MATCHING_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::StripPrefixError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::StripPrefixError>",
        "verus",
        || {
            <RustStdStandard<std::path::StripPrefixError> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_dir_builder_model_creates_nested_directories_recursively".to_owned(),
            VERIFY_DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::DirBuilder>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirBuilder>",
        "verus",
        || {
            <RustStdStandard<std::fs::DirBuilder> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirBuilder>",
        "verus",
        "ensures",
        || DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_dir_entry_model_reports_the_created_files_name_and_path".to_owned(),
            VERIFY_DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::DirEntry>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirEntry>",
        "verus",
        || {
            <RustStdStandard<std::fs::DirEntry> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirEntry>",
        "verus",
        "ensures",
        || DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_read_dir_model_iterates_every_entry_in_the_directory".to_owned(),
            VERIFY_READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::ReadDir>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::ReadDir>",
        "verus",
        || {
            <RustStdStandard<std::fs::ReadDir> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::ReadDir>",
        "verus",
        "ensures",
        || READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_file_model_write_then_read_round_trips_the_bytes".to_owned(),
            VERIFY_FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::File>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::File>",
        "verus",
        || { <RustStdStandard<std::fs::File> as VerusWitness>::proof().to_string() },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::File>",
        "verus",
        "ensures",
        || FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_file_times_model_sets_the_recorded_modification_time".to_owned(),
            VERIFY_FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::FileTimes>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::FileTimes>",
        "verus",
        || {
            <RustStdStandard<std::fs::FileTimes> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::FileTimes>",
        "verus",
        "ensures",
        || FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_VERUS_FRAGMENT,
    )
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
        VerusCheckedProof::new(
            "verify_metadata_model_reports_the_written_length".to_owned(),
            VERIFY_METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::Metadata>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::Metadata>",
        "verus",
        || {
            <RustStdStandard<std::fs::Metadata> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::Metadata>",
        "verus",
        "ensures",
        || METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_VERUS_FRAGMENT,
    )
}

const VERIFY_FILE_TYPE_MODEL_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::FileType> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_file_type_model_distinguishes_files_from_directories".to_owned(),
            VERIFY_FILE_TYPE_MODEL_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::FileType>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::FileType>",
        "verus",
        || {
            <RustStdStandard<std::fs::FileType> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_OPEN_OPTIONS_MODEL_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::OpenOptions> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_open_options_model_create_new_rejects_an_existing_file".to_owned(),
            VERIFY_OPEN_OPTIONS_MODEL_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::OpenOptions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::OpenOptions>",
        "verus",
        || {
            <RustStdStandard<std::fs::OpenOptions> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PERMISSIONS_MODEL_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::Permissions> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_permissions_model_readonly_round_trips_through_set_permissions".to_owned(),
            VERIFY_PERMISSIONS_MODEL_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::Permissions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::Permissions>",
        "verus",
        || {
            <RustStdStandard<std::fs::Permissions> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TRY_LOCK_ERROR_MODEL_REPORTS_A_LOCK_ALREADY_HELD_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::TryLockError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_lock_error_model_reports_a_lock_already_held".to_owned(),
            VERIFY_TRY_LOCK_ERROR_MODEL_REPORTS_A_LOCK_ALREADY_HELD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::TryLockError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::TryLockError>",
        "verus",
        || {
            <RustStdStandard<std::fs::TryLockError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/thread_local_key_carrier.rs");

impl VerusWitness for RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_local_key_model_with_reads_the_initialized_value".to_owned(),
            VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

// Singleton contract: the fixed example's initial value (5) and its
// value after one mutation (42).
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::thread::LocalKey<std::cell::Cell<i32>>>",
    "local_key_observes_initial_then_updated"
);

const VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/thread_current_carrier.rs");

impl VerusWitness for RustStdStandard<std::thread::Thread> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_thread_current_model_is_stable_across_repeated_calls".to_owned(),
            VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::Thread>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::thread::Thread>",
        "verus",
        || {
            <RustStdStandard<std::thread::Thread> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::thread::ThreadId> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_thread_current_model_is_stable_across_repeated_calls".to_owned(),
            VERIFY_THREAD_CURRENT_MODEL_IS_STABLE_ACROSS_REPEATED_CALLS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::thread::ThreadId>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::thread::ThreadId>",
        "verus",
        || {
            <RustStdStandard<std::thread::ThreadId> as VerusWitness>::proof().to_string()
        },
    )
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
                VerusCheckedProof::new(
                    "verify_args_model_reports_at_least_the_program_path".to_owned(),
                    VERIFY_ARGS_MODEL_REPORTS_AT_LEAST_THE_PROGRAM_PATH_SRC.to_owned(),
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

        ::inventory::submit! {
            ::amenable_core::ContractRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                "ensures",
                || ARGS_MODEL_COUNT_MATCHES_PROGRAM_PLUS_EXTRA_VERUS_FRAGMENT,
            )
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
        VerusCheckedProof::new(
            "verify_join_paths_error_model_reports_an_unjoinable_path".to_owned(),
            VERIFY_JOIN_PATHS_ERROR_MODEL_REPORTS_AN_UNJOINABLE_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::env::JoinPathsError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::env::JoinPathsError>",
        "verus",
        || {
            <RustStdStandard<std::env::JoinPathsError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SPLIT_PATHS_MODEL_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/env_carrier.rs");

impl VerusWitness for RustStdStandard<std::env::SplitPaths<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_split_paths_model_recovers_paths_joined_by_join_paths".to_owned(),
            VERIFY_SPLIT_PATHS_MODEL_RECOVERS_PATHS_JOINED_BY_JOIN_PATHS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::env::SplitPaths<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::env::SplitPaths<'static>>",
        "verus",
        || {
            <RustStdStandard<std::env::SplitPaths<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHANNEL_MODEL_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_mpsc_carrier.rs");

macro_rules! impl_mpsc_channel_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_channel_model_delivers_to_the_paired_receiver".to_owned(),
                    VERIFY_CHANNEL_MODEL_DELIVERS_TO_THE_PAIRED_RECEIVER_SRC.to_owned(),
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
        VerusCheckedProof::new(
            "verify_receiver_model_fails_once_every_sender_is_dropped".to_owned(),
            VERIFY_RECEIVER_MODEL_FAILS_ONCE_EVERY_SENDER_IS_DROPPED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::mpsc::Receiver<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::Receiver<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::mpsc::Receiver<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHANNEL_ITER_MODEL_YIELDS_SENT_VALUES_THEN_STOPS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_mpsc_carrier.rs");

macro_rules! impl_mpsc_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_channel_iter_model_yields_sent_values_then_stops".to_owned(),
                    VERIFY_CHANNEL_ITER_MODEL_YIELDS_SENT_VALUES_THEN_STOPS_SRC.to_owned(),
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
        VerusCheckedProof::new(
            "verify_try_iter_model_does_not_block_on_an_empty_open_channel".to_owned(),
            VERIFY_TRY_ITER_MODEL_DOES_NOT_BLOCK_ON_AN_EMPTY_OPEN_CHANNEL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::mpsc::TryIter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Once> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_runs_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Once>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
        "verus",
        || { <RustStdStandard<std::sync::Once> as VerusWitness>::proof().to_string() },
    )
}

// The shared "exactly once" invocation-count postcondition `amenable_
// std::verus_witness` registers for `Once`/`Waker`.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::sync::Once>,
    "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
    "invoked_exactly_once"
);

const VERIFY_ONCE_STATE_MODEL_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::OnceState> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_state_model_reports_not_poisoned_on_a_clean_run".to_owned(),
            VERIFY_ONCE_STATE_MODEL_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::OnceState>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::OnceState>",
        "verus",
        || {
            <RustStdStandard<std::sync::OnceState> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ONCE_LOCK_MODEL_INITIALIZES_EXACTLY_ONCE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::OnceLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_lock_model_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_LOCK_MODEL_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::OnceLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::OnceLock<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::OnceLock<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_barrier_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Barrier> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_barrier_model_of_one_is_its_own_leader".to_owned(),
            VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Barrier>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Barrier>",
        "verus",
        || {
            <RustStdStandard<std::sync::Barrier> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::sync::BarrierWaitResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_barrier_model_of_one_is_its_own_leader".to_owned(),
            VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::BarrierWaitResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::BarrierWaitResult>",
        "verus",
        || {
            <RustStdStandard<std::sync::BarrierWaitResult> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_WAIT_TIMEOUT_RESULT_MODEL_REPORTS_TIMED_OUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/sync_wait_timeout_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::WaitTimeoutResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_wait_timeout_result_model_reports_timed_out".to_owned(),
            VERIFY_WAIT_TIMEOUT_RESULT_MODEL_REPORTS_TIMED_OUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::WaitTimeoutResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::WaitTimeoutResult>",
        "verus",
        || {
            <RustStdStandard<std::sync::WaitTimeoutResult> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INCOMING_MODEL_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Incoming<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_incoming_model_yields_an_already_queued_connection".to_owned(),
            VERIFY_INCOMING_MODEL_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Incoming<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Incoming<'static>>",
        "verus",
        || {
            <RustStdStandard<std::net::Incoming<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SHUTDOWN_MODEL_WRITE_PREVENTS_FURTHER_WRITES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Shutdown> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_shutdown_model_write_prevents_further_writes".to_owned(),
            VERIFY_SHUTDOWN_MODEL_WRITE_PREVENTS_FURTHER_WRITES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Shutdown>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Shutdown>",
        "verus",
        || {
            <RustStdStandard<std::net::Shutdown> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TCP_LISTENER_MODEL_ACCEPTS_A_CONNECTING_STREAM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::TcpListener> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tcp_listener_model_accepts_a_connecting_stream".to_owned(),
            VERIFY_TCP_LISTENER_MODEL_ACCEPTS_A_CONNECTING_STREAM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::TcpListener>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::TcpListener>",
        "verus",
        || {
            <RustStdStandard<std::net::TcpListener> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TCP_STREAM_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::TcpStream> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tcp_stream_model_delivers_written_bytes_to_the_accepted_peer".to_owned(),
            VERIFY_TCP_STREAM_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::TcpStream>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::TcpStream>",
        "verus",
        || {
            <RustStdStandard<std::net::TcpStream> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_UDP_SOCKET_MODEL_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::UdpSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_udp_socket_model_send_to_recv_from_round_trips_a_datagram".to_owned(),
            VERIFY_UDP_SOCKET_MODEL_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::UdpSocket>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::UdpSocket>",
        "verus",
        || {
            <RustStdStandard<std::net::UdpSocket> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CONTEXT_MODEL_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_context_model_from_waker_exposes_the_same_waker".to_owned(),
            VERIFY_CONTEXT_MODEL_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::task::Context<'static>>",
        "verus",
        || {
            <RustStdStandard<std::task::Context<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_POLL_MODEL_READY_AND_PENDING_ARE_DISJOINT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_poll_model_ready_and_pending_are_disjoint".to_owned(),
            VERIFY_POLL_MODEL_READY_AND_PENDING_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Poll<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::task::Poll<i32>>",
        "verus",
        || {
            <RustStdStandard<std::task::Poll<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_WAKER_MODEL_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_waker_model_wake_by_ref_invokes_the_wake_impl".to_owned(),
            VERIFY_WAKER_MODEL_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::task::Waker>",
        "verus",
        || {
            <RustStdStandard<std::task::Waker> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::task::Waker>,
    "amenable_std::rust_std::RustStdStandard<std::task::Waker>",
    "invoked_exactly_once"
);

const VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/panic_carrier.rs");

impl VerusWitness for RustStdStandard<std::panic::AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_assert_unwind_safe_model_derefs_transparently".to_owned(),
            VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::panic::AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::panic::AssertUnwindSafe<i32>>",
        "verus",
        || {
            <RustStdStandard<std::panic::AssertUnwindSafe<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_LOCATION_MODEL_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/panic_carrier.rs");

impl VerusWitness for RustStdStandard<core::panic::Location<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_location_model_caller_reflects_the_immediate_call_site".to_owned(),
            VERIFY_LOCATION_MODEL_CALLER_REFLECTS_THE_IMMEDIATE_CALL_SITE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::panic::Location<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::panic::Location<'static>>",
        "verus",
        || {
            <RustStdStandard<core::panic::Location<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<core::panic::Location<'static>>,
    "amenable_std::rust_std::RustStdStandard<core::panic::Location<'static>>",
    "values_are_distinct"
);

const VERIFY_RANGE_TO_MODEL_CONTAINS_MATCHES_BOUND_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::RangeTo<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_range_to_model_contains_matches_bound".to_owned(),
            VERIFY_RANGE_TO_MODEL_CONTAINS_MATCHES_BOUND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::RangeTo<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ops::RangeTo<i32>>",
        "verus",
        || {
            <RustStdStandard<std::ops::RangeTo<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_RANGE_FULL_MODEL_CONTAINS_EVERYTHING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::RangeFull> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_range_full_model_contains_everything".to_owned(),
            VERIFY_RANGE_FULL_MODEL_CONTAINS_EVERYTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::RangeFull>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeFull>",
        "verus",
        || {
            <RustStdStandard<std::ops::RangeFull> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BOUND_MODEL_ROUND_TRIPS_ITS_ENDPOINT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::Bound<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_bound_model_round_trips_its_endpoint".to_owned(),
            VERIFY_BOUND_MODEL_ROUND_TRIPS_ITS_ENDPOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::Bound<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        "verus",
        || {
            <RustStdStandard<std::ops::Bound<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CONTROL_FLOW_MODEL_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/ops_carrier.rs");

impl VerusWitness for RustStdStandard<std::ops::ControlFlow<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_control_flow_model_continue_and_break_are_disjoint".to_owned(),
            VERIFY_CONTROL_FLOW_MODEL_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ops::ControlFlow<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<std::ops::ControlFlow<i32, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Instant> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_instant_model_is_monotonically_nondecreasing".to_owned(),
            VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Instant>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Instant>",
        "verus",
        || {
            <RustStdStandard<std::time::Instant> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_time_model_duration_since_computes_the_elapsed_span".to_owned(),
            VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTime>",
        "verus",
        || {
            <RustStdStandard<std::time::SystemTime> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTimeError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_time_error_model_recovers_how_far_backward_it_went".to_owned(),
            VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTimeError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTimeError>",
        "verus",
        || {
            <RustStdStandard<std::time::SystemTimeError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_duration_model_new_normalizes_nanos_and_carries_into_secs".to_owned(),
            VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "verus",
        || {
            <RustStdStandard<std::time::Duration> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INTO_ITER_MODEL_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/option_result_iter_carrier.rs");

macro_rules! impl_option_result_into_iter_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_into_iter_model_yields_zero_or_one_owned_value".to_owned(),
                    VERIFY_INTO_ITER_MODEL_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC.to_owned(),
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
                VerusCheckedProof::new(
                    "verify_iter_model_yields_zero_or_one_reference".to_owned(),
                    VERIFY_ITER_MODEL_YIELDS_ZERO_OR_ONE_REFERENCE_SRC.to_owned(),
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
                VerusCheckedProof::new(
                    "verify_iter_mut_model_writes_through".to_owned(),
                    VERIFY_ITER_MUT_MODEL_WRITES_THROUGH_SRC.to_owned(),
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
        VerusCheckedProof::new(
            "verify_pending_model_never_resolves".to_owned(),
            VERIFY_PENDING_MODEL_NEVER_RESOLVES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::future::Pending<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pending<i32>>",
        "verus",
        || {
            <RustStdStandard<std::future::Pending<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_READY_MODEL_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/future_carrier.rs");

impl VerusWitness for RustStdStandard<std::future::Ready<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ready_model_resolves_immediately_with_its_value".to_owned(),
            VERIFY_READY_MODEL_RESOLVES_IMMEDIATELY_WITH_ITS_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::future::Ready<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ready<i32>>",
        "verus",
        || {
            <RustStdStandard<std::future::Ready<i32>> as VerusWitness>::proof().to_string()
        },
    )
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
        VerusCheckedProof::new(
            "verify_poll_fn_model_dispatches_through_to_its_closure".to_owned(),
            VERIFY_POLL_FN_MODEL_DISPATCHES_THROUGH_TO_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::future::PollFn<fn(&mut std::task::Context<'_>) -> std::task::Poll<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

const VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_model_indexing_and_length".to_owned(),
            VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        "verus",
        || { <RustStdStandard<[i32; 3]> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_slice_model_indexing_and_length".to_owned(),
            VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32]>",
        "verus",
        || { <RustStdStandard<[i32]> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_model_byte_length_and_content".to_owned(),
            VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<str>);

// The real Verus proof sites across the `str`, `path`, `process`, `env`,
// and panic carriers all call the shared `text_view_matches_expected`
// spec fn directly.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<str>,
    "amenable_std::rust_std::RustStdStandard<str>",
    "text_view_matches_expected"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "verus",
        || { <RustStdStandard<str> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tuple_model_field_access".to_owned(),
            VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "verus",
        || { <RustStdStandard<(i32, i32)> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fn_pointer_model_calls_the_underlying_function".to_owned(),
            VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "verus",
        || { <RustStdStandard<fn(i32) -> i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_const_pointer_model_cast_is_reproducible".to_owned(),
            VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*const i32>",
        "verus",
        || { <RustStdStandard<*const i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_mut_pointer_model_cast_is_reproducible".to_owned(),
            VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*mut i32>",
        "verus",
        || { <RustStdStandard<*mut i32> as VerusWitness>::proof().to_string() },
    )
}

impl VerusWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_shared_reference_model_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "verus",
        || { <RustStdStandard<&'static i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_mutable_reference_model_dereferences_to_and_updates_the_referent".to_owned(),
            VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "verus",
        || { <RustStdStandard<&'static mut i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_default_hasher_model_is_deterministic_across_fresh_instances".to_owned(),
            VERIFY_DEFAULT_HASHER_MODEL_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::DefaultHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
        "verus",
        || {
            <RustStdStandard<std::hash::DefaultHasher> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_RANDOM_STATE_MODEL_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_hash_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::RandomState> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_random_state_model_gives_the_same_hasher_seed_across_calls".to_owned(),
            VERIFY_RANDOM_STATE_MODEL_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::RandomState>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RandomState>",
        "verus",
        || {
            <RustStdStandard<std::hash::RandomState> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_OS_STR_MODEL_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_os_str_model_valid_utf8_content_round_trips_through_to_str".to_owned(),
            VERIFY_OS_STR_MODEL_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsStr>",
        "verus",
        || { <RustStdStandard<std::ffi::OsStr> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_OS_STRING_MODEL_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_os_string_model_push_appends_to_the_existing_content".to_owned(),
            VERIFY_OS_STRING_MODEL_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsString>",
        "verus",
        || {
            <RustStdStandard<std::ffi::OsString> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_OS_STR_DISPLAY_MODEL_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_ffi_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::os_str::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_os_str_display_model_renders_valid_utf8_content_unchanged".to_owned(),
            VERIFY_OS_STR_DISPLAY_MODEL_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::os_str::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::os_str::Display<'static>>",
        "verus",
        || {
            <RustStdStandard<std::ffi::os_str::Display<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_HASH_MAP_MODEL_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_collections_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::HashMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_hash_map_model_insert_then_get_recovers_the_value".to_owned(),
            VERIFY_HASH_MAP_MODEL_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::HashMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::HashMap<i32, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_HASH_SET_MODEL_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/std_collections_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::HashSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_hash_set_model_insert_then_contains_reports_membership".to_owned(),
            VERIFY_HASH_SET_MODEL_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::HashSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::HashSet<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PIN_MODEL_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::pin::Pin<Box<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_pin_model_derefs_and_get_mut_round_trip".to_owned(),
            VERIFY_PIN_MODEL_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::pin::Pin<Box<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Pin<Box<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::pin::Pin<Box<i32>>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_NON_NULL_MODEL_REJECTS_THE_NULL_POINTER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::ptr::NonNull<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_non_null_model_rejects_the_null_pointer".to_owned(),
            VERIFY_NON_NULL_MODEL_REJECTS_THE_NULL_POINTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ptr::NonNull<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonNull<i32>>",
        "verus",
        || {
            <RustStdStandard<std::ptr::NonNull<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_MODEL_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::alloc::System> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_model_allocates_and_deallocates_a_layout".to_owned(),
            VERIFY_SYSTEM_MODEL_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::alloc::System>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<System>",
        "verus",
        || { <RustStdStandard<std::alloc::System> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::backtrace::Backtrace> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_backtrace_model_force_capture_always_actually_captures".to_owned(),
            VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::backtrace::Backtrace>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Backtrace>",
        "verus",
        || {
            <RustStdStandard<std::backtrace::Backtrace> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::backtrace::BacktraceStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_backtrace_model_force_capture_always_actually_captures".to_owned(),
            VERIFY_BACKTRACE_MODEL_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::backtrace::BacktraceStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BacktraceStatus>",
        "verus",
        || {
            <RustStdStandard<std::backtrace::BacktraceStatus> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PANIC_HOOK_INFO_MODEL_REPORTS_THE_PANICS_OWN_MESSAGE_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::panic::PanicHookInfo<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_panic_hook_info_model_reports_the_panics_own_message".to_owned(),
            VERIFY_PANIC_HOOK_INFO_MODEL_REPORTS_THE_PANICS_OWN_MESSAGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::panic::PanicHookInfo<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PanicHookInfo<'static>>",
        "verus",
        || {
            <RustStdStandard<std::panic::PanicHookInfo<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_VEC_DEQUE_DRAIN_MODEL_REMOVES_AND_YIELDS_IN_ORDER_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/misc_singletons_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_deque_drain_model_removes_and_yields_in_order".to_owned(),
            VERIFY_VEC_DEQUE_DRAIN_MODEL_REMOVES_AND_YIELDS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
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
        VerusCheckedProof::new(
            "<EncodeWide<'_> as Iterator>::next".to_owned(),
            VERIFY_ENCODE_WIDE_AXIOM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<EncodeWide<'static>>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "verus",
        || { <RustStdStandard<EncodeWide<'static>> as VerusWitness>::proof().to_string() },
    )
}

#[cfg(windows)]
const VERIFY_BORROWED_HANDLE_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<BorrowedHandle<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "<BorrowedHandle<'_> as AsRawHandle>::as_raw_handle".to_owned(),
            VERIFY_BORROWED_HANDLE_AXIOM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<BorrowedHandle<'static>>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "verus",
        || { <RustStdStandard<BorrowedHandle<'static>> as VerusWitness>::proof().to_string() },
    )
}

#[cfg(windows)]
const VERIFY_BORROWED_SOCKET_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<BorrowedSocket<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "<BorrowedSocket<'_> as AsRawSocket>::as_raw_socket".to_owned(),
            VERIFY_BORROWED_SOCKET_AXIOM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<BorrowedSocket<'static>>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "verus",
        || { <RustStdStandard<BorrowedSocket<'static>> as VerusWitness>::proof().to_string() },
    )
}

#[cfg(windows)]
const VERIFY_HANDLE_OR_INVALID_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<HandleOrInvalid> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "<OwnedHandle as TryFrom<HandleOrInvalid>>::try_from".to_owned(),
            VERIFY_HANDLE_OR_INVALID_AXIOM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<HandleOrInvalid>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "verus",
        || { <RustStdStandard<HandleOrInvalid> as VerusWitness>::proof().to_string() },
    )
}

#[cfg(windows)]
const VERIFY_OWNED_HANDLE_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<OwnedHandle> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "<OwnedHandle as AsRawHandle>::as_raw_handle".to_owned(),
            VERIFY_OWNED_HANDLE_AXIOM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<OwnedHandle>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "verus",
        || { <RustStdStandard<OwnedHandle> as VerusWitness>::proof().to_string() },
    )
}

#[cfg(windows)]
const VERIFY_OWNED_SOCKET_AXIOM_SRC: &str =
    include_str!("../../amenable_verus/src/rust_std/os_windows_carrier.rs");

#[cfg(windows)]
impl VerusWitness for RustStdStandard<OwnedSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "<OwnedSocket as AsRawSocket>::as_raw_socket".to_owned(),
            VERIFY_OWNED_SOCKET_AXIOM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

#[cfg(windows)]
bridge_verus_witness!(RustStdStandard<OwnedSocket>);

#[cfg(windows)]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "verus",
        || { <RustStdStandard<OwnedSocket> as VerusWitness>::proof().to_string() },
    )
}
