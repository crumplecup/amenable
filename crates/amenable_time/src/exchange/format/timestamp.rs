//! Formatter output sidecars for the RFC 3339 / IXDTF timestamp family.
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    IxdtfTimestampFormattedToken, IxdtfZonedTimestampFormattedToken, Rfc3339TimestampFormattedToken,
};

/// Output sidecar for the `format_rfc3339_timestamp` exchange: the emitted text plus
/// a token for [`Rfc3339TimestampFormatted`](crate::Rfc3339TimestampFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::Rfc3339TimestampFormatted", constructor = "pub")]
pub struct FormattedRfc3339Timestamp {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: Rfc3339TimestampFormattedToken,
}

impl FormattedRfc3339Timestamp {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ixdtf_zoned_timestamp` exchange: the emitted text plus
/// a token for [`IxdtfZonedTimestampFormatted`](crate::IxdtfZonedTimestampFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::IxdtfZonedTimestampFormatted",
    constructor = "pub"
)]
pub struct FormattedIxdtfZonedTimestamp {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: IxdtfZonedTimestampFormattedToken,
}

impl FormattedIxdtfZonedTimestamp {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ixdtf_timestamp` exchange: the emitted text plus
/// a token for [`IxdtfTimestampFormatted`](crate::IxdtfTimestampFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::IxdtfTimestampFormatted", constructor = "pub")]
pub struct FormattedIxdtfTimestamp {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: IxdtfTimestampFormattedToken,
}

impl FormattedIxdtfTimestamp {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
