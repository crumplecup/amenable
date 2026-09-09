//! The temporal runtime seams. The module layout mirrors
//! `elicit_temporal/src/traits/` one-to-one.
//!
//! Three roles (`elicit_temporal`'s own taxonomy, carried over):
//!
//! - **descriptor factories** (parse / format / resolve) — a backend
//!   implements the raw method (`&str` in, descriptor out); the
//!   [`TemporalExchange`](crate::TemporalExchange)-shaped surface (a
//!   proven `Sidecar` in, a proven `Sidecar` out) is blanket-implemented
//!   over the trait in `src/exchange/` (plan Phase 4).
//! - **native carrier families** — backend-owned associated types (Phase 5).
//! - **reporters** — capability queries that mint no proofs. [`TemporalReporter`]
//!   is the one seam that stays a plain trait (`docs/AMENABLE_TIME_PLAN.md`).

mod parse;
mod report;

pub use parse::TemporalParser;
pub use report::TemporalReporter;
