//! The temporal runtime seams. The module layout mirrors
//! `elicit_temporal/src/traits/` one-to-one.
//!
//! Three roles (`elicit_temporal`'s own taxonomy, carried over):
//!
//! - **descriptor factories** (parse / format / resolve) — [`TemporalParser<V>`]
//!   and [`TemporalFormatter<V>`] are traits whose supertrait bundle *is*
//!   a set of `Exchange<Sidecar, Sidecar, V>` obligations (a proven
//!   `Sidecar` in, a proven `Sidecar` out). A backend writes the inherent
//!   methods and `#[capture_exchange_body]` generates the `Exchange`
//!   impls there — the orphan rule keeps them out of `amenable_time`.
//! - **native carrier families** — backend-owned associated types (Phase 5).
//! - **reporters** — capability queries that mint no proofs. [`TemporalReporter`]
//!   is the one seam that stays a plain trait (`docs/AMENABLE_TIME_PLAN.md`).

mod format;
mod parse;
mod report;

pub use format::TemporalFormatter;
pub use parse::TemporalParser;
pub use report::TemporalReporter;
