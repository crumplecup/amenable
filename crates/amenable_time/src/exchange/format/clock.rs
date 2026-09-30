//! Formatter output sidecars for the local-time/UTC-offset family.
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    LocalTimeBasicFormattedToken, LocalTimeExtendedFormattedToken,
    ReducedLocalTimeBasicFormattedToken, ReducedLocalTimeExtendedFormattedToken,
    UtcOffsetBasicFormattedToken, UtcOffsetExtendedFormattedToken,
};

/// Output sidecar for the `format_local_time_extended` exchange: the emitted text plus
/// a token for [`LocalTimeExtendedFormatted`](crate::LocalTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::LocalTimeExtendedFormatted", constructor = "pub")]
pub struct FormattedLocalTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalTimeExtendedFormattedToken,
}

impl FormattedLocalTimeExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_local_time_basic` exchange: the emitted text plus
/// a token for [`LocalTimeBasicFormatted`](crate::LocalTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::LocalTimeBasicFormatted", constructor = "pub")]
pub struct FormattedLocalTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalTimeBasicFormattedToken,
}

impl FormattedLocalTimeBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_local_time_extended` exchange: the emitted text plus
/// a token for [`ReducedLocalTimeExtendedFormatted`](crate::ReducedLocalTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedLocalTimeExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedLocalTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedLocalTimeExtendedFormattedToken,
}

impl FormattedReducedLocalTimeExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_local_time_basic` exchange: the emitted text plus
/// a token for [`ReducedLocalTimeBasicFormatted`](crate::ReducedLocalTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedLocalTimeBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedLocalTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedLocalTimeBasicFormattedToken,
}

impl FormattedReducedLocalTimeBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_utc_offset_extended` exchange: the emitted text plus
/// a token for [`UtcOffsetExtendedFormatted`](crate::UtcOffsetExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::UtcOffsetExtendedFormatted", constructor = "pub")]
pub struct FormattedUtcOffsetExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: UtcOffsetExtendedFormattedToken,
}

impl FormattedUtcOffsetExtended {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_utc_offset_basic` exchange: the emitted text plus
/// a token for [`UtcOffsetBasicFormatted`](crate::UtcOffsetBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::UtcOffsetBasicFormatted", constructor = "pub")]
pub struct FormattedUtcOffsetBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: UtcOffsetBasicFormattedToken,
}

impl FormattedUtcOffsetBasic {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
