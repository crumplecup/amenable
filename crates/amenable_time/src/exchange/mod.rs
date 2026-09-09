//! The temporal exchange surface — trait methods re-expressed as
//! [`Exchange`](amenable_core::Exchange)s.
//!
//! - [`markers`] / [`tokens`] — the boundary `Evidence` markers and proof
//!   tokens.
//! - [`sidecars`] — [`RawInput`] (every method's input) and
//!   [`Proven<D, P>`](Proven) (every method's output).
//!
//! Per-method `Exchange<RawInput, Proven<D, P>, V>` impls +
//! `ExchangeEdgeRecord`s land alongside per trait (`TemporalParser`,
//! `TemporalFormatter`, …) in later Phase 4 steps — no `Witness` /
//! `Ensures` proof there, only the wiring (`docs/AMENABLE_TIME_PLAN.md`,
//! Phase 4).

mod markers;
mod sidecars;
mod tokens;

pub use markers::{RawTemporalText, TemporalInputReceived};
pub use sidecars::{Proven, RawInput};
pub use tokens::{ProvenToken, TemporalInputToken};
