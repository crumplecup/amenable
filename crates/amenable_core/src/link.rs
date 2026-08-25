//! Static self-registration of evidence chain links.
//!
//! `Evidence::basis()` is a compile-time fact about a type, but Rust has no
//! reflection: nothing lets code discover every type implementing `Evidence`
//! across a compiled binary just by asking the type system. `inventory`
//! bridges that gap — each concrete evidence type submits a small
//! descriptor of itself once, and tooling can later walk the whole
//! collection to reconstruct an arbitrary chain without enumerating every
//! type by hand.
//!
//! `inventory::submit!` requires a `const`-evaluable value, which rules out
//! `std::any::type_name` (not yet stable as `const fn`). Build `name` and
//! `basis` from `concat!(module_path!(), "::", stringify!(Type))` instead —
//! both are compile-time text macros, not runtime calls:
//!
//! ```
//! # use amenable_core::EvidenceLink;
//! struct MyStandard;
//!
//! inventory::submit! {
//!     EvidenceLink::new(
//!         concat!(module_path!(), "::", stringify!(MyStandard)),
//!         concat!(module_path!(), "::", stringify!(MyStandard)),
//!         0,
//!     )
//! }
//! ```

/// A statically-registered fact: this evidence type rests on that basis
/// type. A root `Standard` registers a link to itself, since it is its own
/// basis.
///
/// A calculation over more than one argument registers several links that
/// share the same `name` — one per argument, fanning out rather than
/// chaining. `inventory`'s iteration order across those links is not
/// guaranteed to match registration order, so `index` (the argument's
/// position) is what lets a reconstructed chain show `add(a, b)`'s
/// branches as `a` then `b`, not whatever order the linker happened to
/// place them in.
///
/// Hand-written `const fn new`/getters, not derived: `inventory::submit!`
/// requires a `const`-evaluable value (`derive_new::new` can't generate
/// `const fn`), and every real reader gets an `EvidenceLink` back as
/// `&'static` (from [`inventory::iter`]) -- `derive_getters::Getters`
/// binds a `&'static`-typed field's getter to a `&'static self` receiver
/// instead of a plain `&self` (confirmed the hard way on
/// `KaniCallerLocationObservation::file`), which happens to be harmless
/// here given that `'static` receiver, but writing it by hand keeps the
/// whole type uniformly `const` instead of mixing a hand-written
/// constructor with a derived, non-const accessor shape.
pub struct EvidenceLink {
    name: &'static str,
    basis: &'static str,
    index: usize,
}

impl EvidenceLink {
    /// Register a link from `name` to `basis`, at fan-out position `index`.
    #[must_use]
    pub const fn new(name: &'static str, basis: &'static str, index: usize) -> Self {
        Self { name, basis, index }
    }

    /// This evidence type's name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// This evidence type's basis type name.
    #[must_use]
    pub const fn basis(&self) -> &'static str {
        self.basis
    }

    /// This link's position among other links sharing the same `name`
    /// (e.g. a calculation argument's index). `0` for a single-basis link.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
}

inventory::collect!(EvidenceLink);

/// A statically-registered fact: a verifier backend names a proof for a
/// given evidence type. Registered once per `(evidence, verifier)` pair by
/// each verifier crate's bridge macro, alongside its `Witness` impl.
///
/// `describe` is a plain function pointer, not a closure captured over
/// runtime state — `inventory::submit!` requires a `const`-evaluable
/// value, and a (possibly monomorphized) function item's address qualifies
/// where a call to it would not. Calling it reads the registered `Witness`
/// impl's `proof()` and renders it for audit; it never runs a verifier.
///
/// Hand-written `const fn new`/getters, not derived, for the same real
/// reason [`EvidenceLink`] is: `inventory::submit!` requires a
/// `const`-evaluable value, and every real reader gets a `ProofRecord`
/// back as `&'static` (from [`inventory::iter`]), so plain `&self`
/// getters are both sufficient and strictly more general than
/// `derive_getters::Getters`' `&'static self` receiver quirk for
/// reference-typed fields.
pub struct ProofRecord {
    evidence: &'static str,
    verifier: &'static str,
    describe: fn() -> String,
}

impl ProofRecord {
    /// Register a proof for `evidence`, written for `verifier`.
    #[must_use]
    pub const fn new(
        evidence: &'static str,
        verifier: &'static str,
        describe: fn() -> String,
    ) -> Self {
        Self {
            evidence,
            verifier,
            describe,
        }
    }

    /// The evidence type this proof backs, in the same naming convention
    /// as [`EvidenceLink::name`].
    #[must_use]
    pub const fn evidence(&self) -> &'static str {
        self.evidence
    }

    /// The verifier backend this proof is written for (e.g. `"kani"`).
    #[must_use]
    pub const fn verifier(&self) -> &'static str {
        self.verifier
    }

    /// Render the registered proof artifact for audit, without running it.
    #[must_use]
    pub const fn describe(&self) -> fn() -> String {
        self.describe
    }
}

inventory::collect!(ProofRecord);

/// A statically-registered fact: a verifier backend checks a named
/// requires/ensures bound, in its own native syntax, for a given evidence
/// type. Registered once per `(evidence, verifier, kind)` triple by each
/// `Ensures`/`Requires` impl, alongside its own definition.
///
/// Unlike [`ProofRecord::describe`], `fragment` is not merely a
/// presence/absence signal — external tooling (e.g. a scanner that flags
/// proof sites still writing a bound's expression inline instead of
/// pointing at a named contract type) needs the literal fragment text to
/// compare against real source, not just the fact that some contract
/// exists. It is still a plain function pointer, not a captured closure,
/// for the same `const`-evaluable reason `describe` is.
///
/// A proof site is only recognized as using this contract when it names it
/// by a real call (`Type::ensures(...)`/`Type::requires(...)` for Kani, a
/// bare `name(...)` call for Creusot/Verus) — never by its clause merely
/// normalizing to the same text as `fragment`. Two unrelated types can
/// independently state claims that normalize to identical text (e.g.
/// `"result == value"` for a round-trip claim about completely different
/// types); call-shape recognition means that coincidence can never silence
/// an unnamed site, so no per-site scoping field is needed here.
///
/// Hand-written `const fn new`/getters, not derived, for the same real
/// reason [`EvidenceLink`]/[`ProofRecord`] are:
/// `inventory::submit!` requires a `const`-evaluable value, and every
/// real reader gets a `ContractRecord` back as `&'static` (from
/// [`inventory::iter`]), so plain `&self` getters are both sufficient
/// and strictly more general than `derive_getters::Getters`' `&'static
/// self` receiver quirk for reference-typed fields.
pub struct ContractRecord {
    evidence: &'static str,
    verifier: &'static str,
    kind: &'static str,
    fragment: fn() -> &'static str,
}

impl ContractRecord {
    /// Register a `kind` contract fragment for `evidence`, written for
    /// `verifier`.
    #[must_use]
    pub const fn new(
        evidence: &'static str,
        verifier: &'static str,
        kind: &'static str,
        fragment: fn() -> &'static str,
    ) -> Self {
        Self {
            evidence,
            verifier,
            kind,
            fragment,
        }
    }

    /// The evidence type this contract names, in the same naming
    /// convention as [`EvidenceLink::name`].
    #[must_use]
    pub const fn evidence(&self) -> &'static str {
        self.evidence
    }

    /// The verifier backend this fragment is written for (e.g. `"kani"`).
    #[must_use]
    pub const fn verifier(&self) -> &'static str {
        self.verifier
    }

    /// Which half of the contract this is: `"ensures"` or `"requires"`.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        self.kind
    }

    /// The bound's fragment, in the verifier's own native syntax.
    #[must_use]
    pub const fn fragment(&self) -> fn() -> &'static str {
        self.fragment
    }
}

inventory::collect!(ContractRecord);

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
pub struct ExchangeEdgeRecord {
    /// The type the `Exchange` impl is for (`Self`), as written.
    pub self_ty: &'static str,
    /// The edge's `Input` type, as written.
    pub input_ty: &'static str,
    /// The edge's `Output` type, as written.
    pub output_ty: &'static str,
    /// The edge's `Error` type, as written.
    pub error_ty: &'static str,
    /// The evidence type this edge establishes, in the same naming
    /// convention as [`EvidenceLink::name`].
    pub evidence: &'static str,
    /// The real inherent method's own name (e.g. `green_to_yellow`).
    pub method_name: &'static str,
    /// The real inherent method's own body, verbatim — whitespace and
    /// all, captured the same way `amenable_derive::harness!` captures a
    /// harness's own source, so it can never drift from the real logic.
    pub body: &'static str,
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
    pub creusot_ensures: &'static str,
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
    pub method_generics: &'static str,
}

inventory::collect!(ExchangeEdgeRecord);

/// A statically-registered fact: a real proof-token type exists, minting a
/// named proposition from an optional credential. Registered once per
/// token by `#[derive(amenable_derive::ProofToken)]` (always) and by
/// `#[amenable_derive::establish(credential = .., proposition = ..)]`'s
/// *verifier-less* form (only when present, which supersedes the
/// `ProofToken`-only registration for the same token — see this record's
/// own `credential` field doc).
///
/// Exists for the identical reason [`ExchangeEdgeRecord`] does, applied to
/// a different capture target: `verus --crate-type=lib` can resolve no
/// external crate at all, not even a proc-macro one (unlike `cargo
/// creusot`, which really does resolve ordinary Cargo dependencies) — so
/// neither the real token type nor its real, backend-generic `Establish`
/// impl (living in `amenable_gaap`, `GAAP_LEDGER_PLAN.md`'s Step 7) can
/// ever be named from Verus's own gallery code, and a hand-written local
/// mirror duplicates the same trivial shape `#[amenable_derive::
/// establish]` already exists to eliminate. A codegen tool reads this
/// registry and *writes* a real, checked-in, proc-macro-free companion —
/// the same shape [`ExchangeEdgeRecord`]'s own consumers use, applied to a
/// type/impl shape instead of a captured method body.
pub struct ProofTokenMintRecord {
    /// The token type's own name, as written (e.g. `"ValidatedToken"`).
    pub token: &'static str,
    /// The proposition this token proves, in the same naming convention as
    /// [`EvidenceLink::name`].
    pub proposition: &'static str,
    /// The credential type this token is established from, if any —
    /// `None` for a root token (no `#[establish(..)]` at all, e.g.
    /// `PendingToken`: asserted, not derived from a prior credential).
    /// `Some` registrations (from `#[establish(..)]`) are strictly richer
    /// than a bare `ProofToken`-only registration for the same `token`
    /// name — a codegen consumer reading both should keep the `Some` one.
    pub credential: Option<&'static str>,
}

inventory::collect!(ProofTokenMintRecord);
