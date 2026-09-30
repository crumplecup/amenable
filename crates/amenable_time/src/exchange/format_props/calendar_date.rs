//! Emission-proof composites for the calendar/ordinal/week-date family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{Iso8601BasicFormUsesCompactRepresentation, Iso8601ExtendedFormUsesSeparators};

/// Emission proof for [`FormattedCalendarDateExtended`](crate::FormattedCalendarDateExtended) — the `format_calendar_date_extended` output proof(s), folded.
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
pub struct CalendarDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedCalendarDateBasic`](crate::FormattedCalendarDateBasic) — the `format_calendar_date_basic` output proof(s), folded.
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
pub struct CalendarDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedReducedCalendarDateExtended`](crate::FormattedReducedCalendarDateExtended) — the `format_reduced_calendar_date_extended` output proof(s), folded.
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
pub struct ReducedCalendarDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedReducedCalendarDateBasic`](crate::FormattedReducedCalendarDateBasic) — the `format_reduced_calendar_date_basic` output proof(s), folded.
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
pub struct ReducedCalendarDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedOrdinalDateExtended`](crate::FormattedOrdinalDateExtended) — the `format_ordinal_date_extended` output proof(s), folded.
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
pub struct OrdinalDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedOrdinalDateBasic`](crate::FormattedOrdinalDateBasic) — the `format_ordinal_date_basic` output proof(s), folded.
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
pub struct OrdinalDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedWeekDateExtended`](crate::FormattedWeekDateExtended) — the `format_week_date_extended` output proof(s), folded.
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
pub struct WeekDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedWeekDateBasic`](crate::FormattedWeekDateBasic) — the `format_week_date_basic` output proof(s), folded.
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
pub struct WeekDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}
