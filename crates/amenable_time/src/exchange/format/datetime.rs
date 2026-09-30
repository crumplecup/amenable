//! Formatter output sidecars for the local/offset date-time family.
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    LocalDateTimeBasicFormattedToken, LocalDateTimeExtendedFormattedToken,
    OffsetDateTimeBasicFormattedToken, OffsetDateTimeExtendedFormattedToken,
};

/// Output sidecar for the `format_local_date_time_extended` exchange: the emitted text plus
/// a token for [`LocalDateTimeExtendedFormatted`](crate::LocalDateTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::LocalDateTimeExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedLocalDateTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalDateTimeExtendedFormattedToken,
}

impl FormattedLocalDateTimeExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_local_date_time_basic` exchange: the emitted text plus
/// a token for [`LocalDateTimeBasicFormatted`](crate::LocalDateTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::LocalDateTimeBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedLocalDateTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalDateTimeBasicFormattedToken,
}

impl FormattedLocalDateTimeBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_offset_date_time_extended` exchange: the emitted text plus
/// a token for [`OffsetDateTimeExtendedFormatted`](crate::OffsetDateTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OffsetDateTimeExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedOffsetDateTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OffsetDateTimeExtendedFormattedToken,
}

impl FormattedOffsetDateTimeExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_offset_date_time_basic` exchange: the emitted text plus
/// a token for [`OffsetDateTimeBasicFormatted`](crate::OffsetDateTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OffsetDateTimeBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedOffsetDateTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OffsetDateTimeBasicFormattedToken,
}

impl FormattedOffsetDateTimeBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
