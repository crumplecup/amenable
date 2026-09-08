//! Temporal contract types, grouped by normative source. The module
//! layout mirrors `elicit_temporal/src/contracts/` one-to-one so bulk
//! registration stays orderly (`docs/AMENABLE_TIME_PLAN.md`,
//! "Registration strategy").
//!
//! Citation-only structural contracts are
//! [`Standard`](amenable_core::Standard)s (this module). The composed /
//! provable `*Valid` aggregates become
//! [`Evidence`](amenable_core::Evidence) in `contracts::proof_composition`
//! (plan Phase 3).

mod precision;

pub use precision::{
    FractionalSecondDigitsAreContiguous, FractionalSecondPrecisionDeclared,
    PrecisionReductionDeclared, RoundingModeDeclared, SubsecondDigitsPreserved,
};
