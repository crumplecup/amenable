//! Formatter output sidecars for the duration/interval family.
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{DurationFormattedToken, RecurringIntervalFormattedToken, TimeIntervalFormattedToken};

/// Output sidecar for the `format_duration` exchange: the emitted text plus
/// a token for [`DurationFormatted`](crate::DurationFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DurationFormatted", constructor = "pub")]
pub struct FormattedDuration {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DurationFormattedToken,
}

impl FormattedDuration {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_recurring_interval` exchange: the emitted text plus
/// a token for [`RecurringIntervalFormatted`](crate::RecurringIntervalFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::RecurringIntervalFormatted", constructor = "pub")]
pub struct FormattedRecurringInterval {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: RecurringIntervalFormattedToken,
}

impl FormattedRecurringInterval {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_time_interval` exchange: the emitted text plus
/// a token for [`TimeIntervalFormatted`](crate::TimeIntervalFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TimeIntervalFormatted", constructor = "pub")]
pub struct FormattedTimeInterval {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: TimeIntervalFormattedToken,
}

impl FormattedTimeInterval {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
