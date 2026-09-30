//! Emission-proof composites for the local-time/UTC-offset family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{Iso8601BasicFormUsesCompactRepresentation, Iso8601ExtendedFormUsesSeparators};

/// Emission proof for [`FormattedLocalTimeExtended`](crate::FormattedLocalTimeExtended) — the `format_local_time_extended` output proof(s), folded.
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
pub struct LocalTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedLocalTimeBasic`](crate::FormattedLocalTimeBasic) — the `format_local_time_basic` output proof(s), folded.
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
pub struct LocalTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedReducedLocalTimeExtended`](crate::FormattedReducedLocalTimeExtended) — the `format_reduced_local_time_extended` output proof(s), folded.
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
pub struct ReducedLocalTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedReducedLocalTimeBasic`](crate::FormattedReducedLocalTimeBasic) — the `format_reduced_local_time_basic` output proof(s), folded.
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
pub struct ReducedLocalTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedUtcOffsetExtended`](crate::FormattedUtcOffsetExtended) — the `format_utc_offset_extended` output proof(s), folded.
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
pub struct UtcOffsetExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedUtcOffsetBasic`](crate::FormattedUtcOffsetBasic) — the `format_utc_offset_basic` output proof(s), folded.
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
pub struct UtcOffsetBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}
