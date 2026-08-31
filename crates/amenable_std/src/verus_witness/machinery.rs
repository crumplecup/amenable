//! The Verus verifier, the `VerusWitness` trait, the
//! `bridge_verus_witness!`/`impl_verus_witness_trusted!` macros every
//! other file in this module invokes, and the `VerusCallShape` family
//! (structural call shapes a compositional renderer uses to emit a
//! literal call to, or citation of, a real Verus harness instead of
//! assuming its conclusion).

use amenable_core::{
    Evidence, MetadataEntry, Provenance, Verifier, WitnessArtifact, WitnessArtifactNode,
    WitnessSupportKind, WitnessSupportSummary,
};

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

pub(crate) trait VerusProofArtifactSupport {
    fn support() -> WitnessSupportSummary;
}

// Every path below is fully qualified, not a bare name relying on
// hygienic definition-site resolution: this macro is invoked from every
// other file in this module (not just this one, the way it was before
// the split into a directory), and a plain `macro_rules!` macro
// re-exported via `pub(..) use` does not carry its defining module's
// unqualified names to the call site the way `#[macro_export]`'s own
// `$crate`-qualified convention already assumes -- confirmed empirically
// while doing this split: bare `VerusVerifier`/`Witness`/`ClassifiedWitness`/
// `WitnessSupportSummary`/`VerusProofArtifactSupport` all failed to
// resolve from any call site outside this file.
macro_rules! bridge_verus_witness {
    ($ty:ty) => {
        impl ::amenable_core::Witness<$crate::VerusVerifier> for $ty {
            type SupportingEvidence = <$ty as $crate::VerusWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as $crate::VerusWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as $crate::VerusWitness>::proof()
            }

            fn support() -> ::amenable_core::WitnessSupportSummary {
                <<$ty as $crate::VerusWitness>::ProofArtifact as $crate::verus_witness::machinery::VerusProofArtifactSupport>::support()
            }
        }

        // Every VerusWitness-bridged type resolves to a real VerusCheckedProof
        // or RustStdProvenance artifact -- both genuinely classified (Checked
        // or Trusted respectively, see VerusProofArtifactSupport below), never
        // the Witness::support() default (Opaque) -- so this impl is always
        // sound to add unconditionally alongside the bridge.
        impl ::amenable_core::ClassifiedWitness<$crate::VerusVerifier> for $ty {}
    };
}
// Every other file in this module invokes this macro on its own leaf
// types, so it needs to be path-addressable outside its own textual
// scope -- the same `pub(..) use` mechanism every other cross-file macro
// in this crate already uses for the identical reason.
pub(super) use bridge_verus_witness;

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
