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
//!     EvidenceLink {
//!         name: concat!(module_path!(), "::", stringify!(MyStandard)),
//!         basis: concat!(module_path!(), "::", stringify!(MyStandard)),
//!         index: 0,
//!     }
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
pub struct EvidenceLink {
    /// This evidence type's name.
    pub name: &'static str,
    /// This evidence type's basis type name.
    pub basis: &'static str,
    /// This link's position among other links sharing the same `name`
    /// (e.g. a calculation argument's index). `0` for a single-basis link.
    pub index: usize,
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
pub struct ProofRecord {
    /// The evidence type this proof backs, in the same naming convention
    /// as [`EvidenceLink::name`].
    pub evidence: &'static str,
    /// The verifier backend this proof is written for (e.g. `"kani"`).
    pub verifier: &'static str,
    /// Render the registered proof artifact for audit, without running it.
    pub describe: fn() -> String,
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
pub struct ContractRecord {
    /// The evidence type this contract names, in the same naming
    /// convention as [`EvidenceLink::name`].
    pub evidence: &'static str,
    /// The verifier backend this fragment is written for (e.g. `"kani"`).
    pub verifier: &'static str,
    /// Which half of the contract this is: `"ensures"` or `"requires"`.
    pub kind: &'static str,
    /// The bound's fragment, in the verifier's own native syntax.
    pub fragment: fn() -> &'static str,
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
}

inventory::collect!(ExchangeEdgeRecord);
