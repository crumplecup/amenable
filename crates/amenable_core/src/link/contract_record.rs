//! A statically-registered fact: a verifier backend checks a named
//! requires/ensures bound, in its own native syntax, for a given evidence
//! type.

/// A statically-registered fact: a verifier backend checks a named
/// requires/ensures bound, in its own native syntax, for a given evidence
/// type. Registered once per `(evidence, verifier, kind)` triple by each
/// `Ensures`/`Requires` impl, alongside its own definition.
///
/// Unlike [`crate::ProofRecord::describe`], `fragment` is not merely a
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
/// reason [`crate::EvidenceLink`]/[`crate::ProofRecord`] are:
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
    /// convention as [`crate::EvidenceLink::name`].
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
