//! Formatter output sidecars for the calendar/ordinal/week-date family.
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    CalendarDateBasicFormattedToken, CalendarDateExtendedFormattedToken,
    OrdinalDateBasicFormattedToken, OrdinalDateExtendedFormattedToken,
    ReducedCalendarDateBasicFormattedToken, ReducedCalendarDateExtendedFormattedToken,
    WeekDateBasicFormattedToken, WeekDateExtendedFormattedToken,
};

/// Output sidecar for the `format_calendar_date_extended` exchange: the emitted text plus
/// a token for [`CalendarDateExtendedFormatted`](crate::CalendarDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::CalendarDateExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedCalendarDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: CalendarDateExtendedFormattedToken,
}

impl FormattedCalendarDateExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_calendar_date_basic` exchange: the emitted text plus
/// a token for [`CalendarDateBasicFormatted`](crate::CalendarDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CalendarDateBasicFormatted", constructor = "pub")]
pub struct FormattedCalendarDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: CalendarDateBasicFormattedToken,
}

impl FormattedCalendarDateBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_calendar_date_extended` exchange: the emitted text plus
/// a token for [`ReducedCalendarDateExtendedFormatted`](crate::ReducedCalendarDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedCalendarDateExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedCalendarDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedCalendarDateExtendedFormattedToken,
}

impl FormattedReducedCalendarDateExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_calendar_date_basic` exchange: the emitted text plus
/// a token for [`ReducedCalendarDateBasicFormatted`](crate::ReducedCalendarDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedCalendarDateBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedCalendarDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedCalendarDateBasicFormattedToken,
}

impl FormattedReducedCalendarDateBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ordinal_date_extended` exchange: the emitted text plus
/// a token for [`OrdinalDateExtendedFormatted`](crate::OrdinalDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OrdinalDateExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedOrdinalDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OrdinalDateExtendedFormattedToken,
}

impl FormattedOrdinalDateExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ordinal_date_basic` exchange: the emitted text plus
/// a token for [`OrdinalDateBasicFormatted`](crate::OrdinalDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::OrdinalDateBasicFormatted", constructor = "pub")]
pub struct FormattedOrdinalDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OrdinalDateBasicFormattedToken,
}

impl FormattedOrdinalDateBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_week_date_extended` exchange: the emitted text plus
/// a token for [`WeekDateExtendedFormatted`](crate::WeekDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::WeekDateExtendedFormatted", constructor = "pub")]
pub struct FormattedWeekDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: WeekDateExtendedFormattedToken,
}

impl FormattedWeekDateExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_week_date_basic` exchange: the emitted text plus
/// a token for [`WeekDateBasicFormatted`](crate::WeekDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::WeekDateBasicFormatted", constructor = "pub")]
pub struct FormattedWeekDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: WeekDateBasicFormattedToken,
}

impl FormattedWeekDateBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
