//! The temporal runtime seams. The module layout mirrors
//! `elicit_temporal/src/traits/` one-to-one.
//!
//! Three roles (`elicit_temporal`'s own taxonomy, carried over):
//!
//! - **descriptor factories** (parse / format / resolve) — each method
//!   becomes an [`Exchange`](amenable_core::Exchange): a `Sidecar` in, a
//!   `Sidecar` out (plan Phase 4).
//! - **native carrier families** — backend-owned associated types (Phase 5).
//! - **reporters** — capability queries that mint no proofs. [`TemporalReporter`]
//!   is the one seam that is *not* an `Exchange` (`docs/AMENABLE_TIME_PLAN.md`,
//!   "Every trait method is an Exchange").

mod report;

pub use report::TemporalReporter;
