//! Explicit root-role wrappers and their governing traits.

use std::marker::PhantomData;

use crate::{Evidence, Provenance};

/// A proposition rooted in an external normative authority.
pub trait Standard: Evidence + Provenance {
    /// The authorizing body for the standard.
    fn authorizing_body() -> &'static str;

    /// A canonical link or citation for the authoritative source text.
    fn authoritative_source() -> &'static str;

    /// The clause, section, or scope represented by this proposition.
    fn source_scope() -> &'static str;

    /// Concise summary of the normative language being encoded.
    fn normative_summary() -> &'static str;

    /// Why the code-level proposition faithfully represents the source text.
    fn fidelity_rationale() -> &'static str;
}

/// A proposition rooted in architectural design authority.
pub trait Objective: Evidence + Provenance {
    /// The design authority, owner, or originating author.
    fn design_authority() -> &'static str;

    /// Where this objective fits within the larger architecture.
    fn architectural_context() -> &'static str;

    /// The intended guarantee or invariant being claimed.
    fn intended_invariant() -> &'static str;

    /// Why this objective is necessary in the design.
    fn rationale() -> &'static str;
}

/// Explicit refinement of a type into a standard-root role.
///
/// This wrapper is purely type-level. It does not carry a value. The point is
/// to make the root-role promotion explicit in the codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsStandard<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> AsStandard<P> {
    /// Promote type `P` into the standard-root role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<P> From<P> for AsStandard<P> {
    fn from(_value: P) -> Self {
        Self::new()
    }
}

/// Explicit refinement of a type into an objective-root role.
///
/// This wrapper is purely type-level. It does not carry a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AsObjective<P> {
    _marker: PhantomData<fn() -> P>,
}

impl<P> AsObjective<P> {
    /// Promote type `P` into the objective-root role.
    pub const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<P> From<P> for AsObjective<P> {
    fn from(_value: P) -> Self {
        Self::new()
    }
}
