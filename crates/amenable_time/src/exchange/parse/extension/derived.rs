//! Temporal-set/grouped-time-scale-unit/date-time-formula parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaProofToken, GroupedTimeScaleUnitDescriptor,
    GroupedTimeScaleUnitProofToken, TemporalSetDescriptor, TemporalSetProofToken,
};

/// Output sidecar for the `parse_temporal_set` exchange: [`TemporalSetDescriptor`](crate::TemporalSetDescriptor)
/// plus a token for [`TemporalSetProof`](crate::TemporalSetProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalSetProof", constructor = "pub")]
pub struct ParsedTemporalSet {
    #[sidecar(primary)]
    descriptor: TemporalSetDescriptor,
    #[sidecar(token)]
    token: TemporalSetProofToken,
}
/// Output sidecar for the `parse_grouped_time_scale_unit` exchange: [`GroupedTimeScaleUnitDescriptor`](crate::GroupedTimeScaleUnitDescriptor)
/// plus a token for [`GroupedTimeScaleUnitProof`](crate::GroupedTimeScaleUnitProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::GroupedTimeScaleUnitProof", constructor = "pub")]
pub struct ParsedGroupedTimeScaleUnit {
    #[sidecar(primary)]
    descriptor: GroupedTimeScaleUnitDescriptor,
    #[sidecar(token)]
    token: GroupedTimeScaleUnitProofToken,
}
/// Output sidecar for the `parse_date_time_formula` exchange: [`DateTimeFormulaDescriptor`](crate::DateTimeFormulaDescriptor)
/// plus a token for [`DateTimeFormulaProof`](crate::DateTimeFormulaProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::DateTimeFormulaProof", constructor = "pub")]
pub struct ParsedDateTimeFormula {
    #[sidecar(primary)]
    descriptor: DateTimeFormulaDescriptor,
    #[sidecar(token)]
    token: DateTimeFormulaProofToken,
}
