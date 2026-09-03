//! The `VerusCallShape` family: structural, machine-usable call shapes for
//! real Verus harnesses -- enough for a compositional renderer to emit a
//! literal call to (or citation of) a real proof instead of assuming its
//! conclusion. Peeled from `machinery` (which keeps the `VerusWitness`
//! trait, the `VerusVerifier`, and the witness/proof-artifact machinery).

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
/// (`crate::rust_std::cell_and_ref::ref_cell_carrier::observed_value_matches_input`)
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
///     module_path = "crate::rust_std::str_and_char::char_carrier",
///     params = [("c", "char")],
///     returns = "char",
///     requires = [],
///     ensures = [
///         "char_roundtrip_preserves_value($result, $c)",
///         "char_is_valid_unicode_scalar($c)",
///     ],
///     imports = [
///         ("crate::rust_std::str_and_char::char_carrier", "char_roundtrip_preserves_value"),
///         ("crate::rust_std::str_and_char::char_carrier", "char_is_valid_unicode_scalar"),
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
