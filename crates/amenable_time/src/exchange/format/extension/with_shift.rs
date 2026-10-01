//! Explicit-form date/time-of-day-with-shift formatter output sidecars.
//!
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{DateWithShiftFormattedToken, TimeOfDayWithShiftFormattedToken};

/// Output sidecar for the `format_date_with_shift` exchange: the emitted text plus
/// a token for [`DateWithShiftFormatted`](crate::DateWithShiftFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DateWithShiftFormatted", constructor = "pub")]
pub struct FormattedDateWithShift {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DateWithShiftFormattedToken,
}

impl FormattedDateWithShift {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_time_of_day_with_shift` exchange: the emitted text plus
/// a token for [`TimeOfDayWithShiftFormatted`](crate::TimeOfDayWithShiftFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::TimeOfDayWithShiftFormatted",
    constructor = "pub"
)]
pub struct FormattedTimeOfDayWithShift {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: TimeOfDayWithShiftFormattedToken,
}

impl FormattedTimeOfDayWithShift {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
