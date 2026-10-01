//! Qualified-value/seasonal/sub-year-grouping/unspecified-component parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    QualifiedTemporalValueDescriptor, QualifiedTemporalValueProofToken,
    SeasonalTemporalExpressionDescriptor, SeasonalTemporalExpressionProofToken,
    SubYearGroupingExpressionDescriptor, SubYearGroupingExpressionProofToken,
    UnspecifiedComponentExpressionDescriptor, UnspecifiedComponentExpressionProofToken,
};

/// Output sidecar for the `parse_qualified_temporal_value` exchange: [`QualifiedTemporalValueDescriptor`](crate::QualifiedTemporalValueDescriptor)
/// plus a token for [`QualifiedTemporalValueProof`](crate::QualifiedTemporalValueProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::QualifiedTemporalValueProof",
    constructor = "pub"
)]
pub struct ParsedQualifiedTemporalValue {
    #[sidecar(primary)]
    descriptor: QualifiedTemporalValueDescriptor,
    #[sidecar(token)]
    token: QualifiedTemporalValueProofToken,
}
/// Output sidecar for the `parse_seasonal_temporal_expression` exchange: [`SeasonalTemporalExpressionDescriptor`](crate::SeasonalTemporalExpressionDescriptor)
/// plus a token for [`SeasonalTemporalExpressionProof`](crate::SeasonalTemporalExpressionProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::SeasonalTemporalExpressionProof",
    constructor = "pub"
)]
pub struct ParsedSeasonalTemporalExpression {
    #[sidecar(primary)]
    descriptor: SeasonalTemporalExpressionDescriptor,
    #[sidecar(token)]
    token: SeasonalTemporalExpressionProofToken,
}
/// Output sidecar for the `parse_sub_year_grouping_expression` exchange: [`SubYearGroupingExpressionDescriptor`](crate::SubYearGroupingExpressionDescriptor)
/// plus a token for [`SubYearGroupingExpressionProof`](crate::SubYearGroupingExpressionProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::SubYearGroupingExpressionProof",
    constructor = "pub"
)]
pub struct ParsedSubYearGroupingExpression {
    #[sidecar(primary)]
    descriptor: SubYearGroupingExpressionDescriptor,
    #[sidecar(token)]
    token: SubYearGroupingExpressionProofToken,
}
/// Output sidecar for the `parse_unspecified_component_expression` exchange: [`UnspecifiedComponentExpressionDescriptor`](crate::UnspecifiedComponentExpressionDescriptor)
/// plus a token for [`UnspecifiedComponentExpressionProof`](crate::UnspecifiedComponentExpressionProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::UnspecifiedComponentExpressionProof",
    constructor = "pub"
)]
pub struct ParsedUnspecifiedComponentExpression {
    #[sidecar(primary)]
    descriptor: UnspecifiedComponentExpressionDescriptor,
    #[sidecar(token)]
    token: UnspecifiedComponentExpressionProofToken,
}
