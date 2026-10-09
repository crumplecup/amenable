//! A statically-registered fact: this evidence type rests on that basis
//! type.

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
    /// Register a link from `name` to `basis`, at fan-out position
    /// `index`.
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
