//! Emission-proof composites for the local/offset date-time family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{
    Iso8601BasicFormUsesCompactRepresentation, Iso8601ExtendedFormUsesSeparators,
    SerializationCarriesExplicitUtcRelationship,
};

/// Emission proof for [`FormattedLocalDateTimeExtended`](crate::FormattedLocalDateTimeExtended) — the `format_local_date_time_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalDateTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedLocalDateTimeBasic`](crate::FormattedLocalDateTimeBasic) — the `format_local_date_time_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalDateTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedOffsetDateTimeExtended`](crate::FormattedOffsetDateTimeExtended) — the `format_offset_date_time_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetDateTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
    /// The `serialization_carries_explicit_utc_relationship` sub-claim.
    serialization_carries_explicit_utc_relationship: SerializationCarriesExplicitUtcRelationship,
}

/// Emission proof for [`FormattedOffsetDateTimeBasic`](crate::FormattedOffsetDateTimeBasic) — the `format_offset_date_time_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetDateTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
    /// The `serialization_carries_explicit_utc_relationship` sub-claim.
    serialization_carries_explicit_utc_relationship: SerializationCarriesExplicitUtcRelationship,
}
