//! A statically-registered fact: a real `Exchange` edge exists, with real
//! type names and its real transition body's verbatim source.

/// A statically-registered fact: a real `Exchange` edge exists, with real
/// type names and its real transition body's verbatim source. Registered
/// once per edge by `#[amenable_derive::exchange(..)]`, from whichever
/// crate owns the real inherent method (currently always `amenable_kani`,
/// the only backend whose own toolchain can run this macro's generated
/// code safely — see that macro's own doc comment).
///
/// Exists so a *different* verifier backend's own companion can be
/// generated from the real body without a Cargo dependency on the crate
/// that owns it (verifier backend crates never depend on each other) and
/// without embedding an `inventory` call inside a crate a translator-based
/// backend (Creusot) has to sweep whole — the registering crate
/// (`amenable_kani`) is ordinary Cargo-built and never translated by
/// anything, so registering here is safe regardless of what any other
/// backend's own toolchain can tolerate. A codegen tool (an ordinary,
/// safely-built binary, e.g. `amenable`'s own CLI) queries this registry
/// and *writes* a real, checked-in, `inventory`-free companion file for
/// the target backend — the same shape `amenable::verus_export`'s
/// `emit-verus-witnesses` already uses for the witness-composition system,
/// applied to `Exchange` edges specifically.
///
/// Hand-written `const fn new`/getters, not derived, for the same real
/// reason [`crate::EvidenceLink`]/[`crate::ProofRecord`]/[`crate::ContractRecord`] are:
/// `inventory::submit!` requires a `const`-evaluable value, and every
/// real reader gets an `ExchangeEdgeRecord` back as `&'static` (from
/// [`inventory::iter`]), so plain `&self` getters are both sufficient
/// and strictly more general than `derive_getters::Getters`' `&'static
/// self` receiver quirk for reference-typed fields.
pub struct ExchangeEdgeRecord {
    /// The type the `Exchange` impl is for (`Self`), as written.
    self_ty: &'static str,
    /// The edge's `Input` type, as written.
    input_ty: &'static str,
    /// The edge's `Output` type, as written.
    output_ty: &'static str,
    /// The edge's `Error` type, as written.
    error_ty: &'static str,
    /// The evidence type this edge establishes, in the same naming
    /// convention as [`crate::EvidenceLink::name`].
    evidence: &'static str,
    /// The real inherent method's own name (e.g. `green_to_yellow`).
    method_name: &'static str,
    /// The real inherent method's own body, verbatim — whitespace and
    /// all, captured the same way `amenable_derive::harness!` captures a
    /// harness's own source, so it can never drift from the real logic.
    body: &'static str,
    /// The real Pearlite predicate expression a generated Creusot
    /// companion's own `#[ensures(..)]` clause should call through,
    /// referencing `result` (the generated function's own implicit
    /// binding, matching every hand-written `#[ensures(..)]` clause
    /// elsewhere in `amenable_creusot`). Defaults to the literal `"true"`
    /// for edges with no meaningful postcondition beyond type safety
    /// (every `Stoplight` edge today) — Creusot's own `Ensures<V>` impls
    /// carry `Bound = &'static str` (descriptive text, not a checkable
    /// predicate: Pearlite has no trait-dispatch mechanism at all), so
    /// unlike Kani's/Verus's `bool`-checked impls, there is no way to
    /// route a generated companion's `#[ensures(..)]` clause through
    /// `Ensures<V>` mechanically — the real predicate has to be named
    /// explicitly, the same way `kani_ensures!`/`verus_ensures!` are
    /// still hand-authored rather than derived.
    creusot_ensures: &'static str,
    /// Extra generic type parameters the real method declares beyond
    /// `Self`'s own (e.g. `"V"`), as a bare comma-separated list with no
    /// angle brackets, or `""` for none. `GAAP_LEDGER_PLAN.md`'s Step 7:
    /// `Ledger::validate`'s real body now calls `Self::check_amount_
    /// positive::<V>(..)`/`self.check_sufficient_funds::<V>(..)` with an
    /// explicit turbofish, naming a generic parameter the real method
    /// itself declares -- a generated companion's own function has to
    /// declare the identical parameter for that captured text to resolve
    /// at all, even though the companion never gives it any bound
    /// (unconstrained is enough: nothing in the captured body needs `V`
    /// to satisfy any real trait, only to name *some* type the turbofish
    /// can point at). `""` for every edge with no such call (every
    /// `Stoplight` edge, `Ledger::commit`) — a codegen consumer splices
    /// this verbatim between `fn method_name<` and `>` only when
    /// non-empty, adding no generic parameter list at all otherwise.
    method_generics: &'static str,
}

impl ExchangeEdgeRecord {
    /// Register a real `Exchange` edge's identifying fields.
    ///
    /// Split from `creusot_ensures`/`method_generics` (set via
    /// [`Self::with_creusot_ensures`]/[`Self::with_method_generics`])
    /// rather than one flat constructor, so this stays under 8
    /// parameters -- `#[allow(clippy::too_many_arguments)]` is never
    /// used in this codebase; this is the real fix.
    #[must_use]
    pub const fn new(
        self_ty: &'static str,
        input_ty: &'static str,
        output_ty: &'static str,
        error_ty: &'static str,
        evidence: &'static str,
        method_name: &'static str,
        body: &'static str,
    ) -> Self {
        Self {
            self_ty,
            input_ty,
            output_ty,
            error_ty,
            evidence,
            method_name,
            body,
            creusot_ensures: "true",
            method_generics: "",
        }
    }

    /// Set the real Pearlite predicate a generated Creusot companion's
    /// `#[ensures(..)]` clause should call through. Defaults to `"true"`.
    #[must_use]
    pub const fn with_creusot_ensures(mut self, creusot_ensures: &'static str) -> Self {
        self.creusot_ensures = creusot_ensures;
        self
    }

    /// Set the extra generic type parameters the real method declares
    /// beyond `Self`'s own. Defaults to `""` (none).
    #[must_use]
    pub const fn with_method_generics(mut self, method_generics: &'static str) -> Self {
        self.method_generics = method_generics;
        self
    }

    /// The type the `Exchange` impl is for (`Self`), as written.
    #[must_use]
    pub const fn self_ty(&self) -> &'static str {
        self.self_ty
    }

    /// The edge's `Input` type, as written.
    #[must_use]
    pub const fn input_ty(&self) -> &'static str {
        self.input_ty
    }

    /// The edge's `Output` type, as written.
    #[must_use]
    pub const fn output_ty(&self) -> &'static str {
        self.output_ty
    }

    /// The edge's `Error` type, as written.
    #[must_use]
    pub const fn error_ty(&self) -> &'static str {
        self.error_ty
    }

    /// The evidence type this edge establishes, in the same naming
    /// convention as [`crate::EvidenceLink::name`].
    #[must_use]
    pub const fn evidence(&self) -> &'static str {
        self.evidence
    }

    /// The real inherent method's own name (e.g. `green_to_yellow`).
    #[must_use]
    pub const fn method_name(&self) -> &'static str {
        self.method_name
    }

    /// The real inherent method's own body, verbatim.
    #[must_use]
    pub const fn body(&self) -> &'static str {
        self.body
    }

    /// The real Pearlite predicate expression a generated Creusot
    /// companion's own `#[ensures(..)]` clause should call through.
    #[must_use]
    pub const fn creusot_ensures(&self) -> &'static str {
        self.creusot_ensures
    }

    /// Extra generic type parameters the real method declares beyond
    /// `Self`'s own.
    #[must_use]
    pub const fn method_generics(&self) -> &'static str {
        self.method_generics
    }
}

inventory::collect!(ExchangeEdgeRecord);
