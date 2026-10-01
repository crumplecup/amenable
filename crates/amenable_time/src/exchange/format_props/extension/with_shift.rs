//! Explicit-form date/time-of-day-with-shift emission-proof composites.
//!
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{DateWithShiftValid, TimeOfDayWithShiftValid};

/// Emission proof for [`FormattedDateWithShift`](crate::FormattedDateWithShift) — the `format_date_with_shift` output proof(s), folded.
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
pub struct DateWithShiftFormatted {
    /// The `date_with_shift_valid` sub-claim.
    date_with_shift_valid: DateWithShiftValid,
}
/// Emission proof for [`FormattedTimeOfDayWithShift`](crate::FormattedTimeOfDayWithShift) — the `format_time_of_day_with_shift` output proof(s), folded.
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
pub struct TimeOfDayWithShiftFormatted {
    /// The `time_of_day_with_shift_valid` sub-claim.
    time_of_day_with_shift_valid: TimeOfDayWithShiftValid,
}
