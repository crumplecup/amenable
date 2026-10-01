//! Extended-year/decade/century formatter output sidecars.
//!
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{CenturyFormattedToken, DecadeFormattedToken, ExtendedYearFormattedToken};

/// Output sidecar for the `format_extended_year` exchange: the emitted text plus
/// a token for [`ExtendedYearFormatted`](crate::ExtendedYearFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExtendedYearFormatted", constructor = "pub")]
pub struct FormattedExtendedYear {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ExtendedYearFormattedToken,
}

impl FormattedExtendedYear {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_decade` exchange: the emitted text plus
/// a token for [`DecadeFormatted`](crate::DecadeFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DecadeFormatted", constructor = "pub")]
pub struct FormattedDecade {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DecadeFormattedToken,
}

impl FormattedDecade {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_century` exchange: the emitted text plus
/// a token for [`CenturyFormatted`](crate::CenturyFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CenturyFormatted", constructor = "pub")]
pub struct FormattedCentury {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: CenturyFormattedToken,
}

impl FormattedCentury {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
