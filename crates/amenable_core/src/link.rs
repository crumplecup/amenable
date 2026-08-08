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
