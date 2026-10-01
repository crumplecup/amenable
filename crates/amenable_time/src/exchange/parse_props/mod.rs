//! Per-method composite proof propositions — for the parse methods
//! whose `elicit_temporal` return tuple carries 2+ proof sidecars, the
//! sidecars folded into one `#[derive(Evidence, Witness)]` struct (the
//! same folding as `proof_composition`) so the output `Sidecar` keeps
//! its single-`token` shape.
//!
//! Split by the same real domains as `exchange::parse`/`::establish`:
//! `datetime`, `timestamp`, `extension`, `interval`.

mod datetime;
mod extension;
mod interval;
mod timestamp;

pub use datetime::{LocalDateTimeProof, OffsetDateTimeProof};
pub use extension::{
    DateTimeFormulaProof, GroupedTimeScaleUnitProof, QualifiedTemporalValueProof,
    SeasonalTemporalExpressionProof, SubYearGroupingExpressionProof, TemporalSetProof,
    UnspecifiedComponentExpressionProof,
};
pub use interval::TimeIntervalProof;
pub use timestamp::{IxdtfTimestampProof, IxdtfZonedTimestampProof, Rfc3339TimestampProof};
