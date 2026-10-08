//! A statically-registered fact: a verifier backend names a proof for a
//! given evidence type.

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
/// reason [`crate::EvidenceLink`] is:
/// `inventory::submit!` requires a `const`-evaluable value, and every
/// real reader gets a `ProofRecord` back as `&'static` (from
/// [`inventory::iter`]), so plain `&self` getters are both sufficient
/// and strictly more general than `derive_getters::Getters`' `&'static
/// self` receiver quirk for reference-typed fields.
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
    /// as [`crate::EvidenceLink::name`].
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
