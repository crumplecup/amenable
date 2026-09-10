//! Reporter trait: backend capability inspection, minting no proofs.

use std::fmt::{self, Display, Formatter};

use crate::SerializationProfile;

/// Report a temporal backend's capabilities without minting or consuming
/// proof tokens.
///
/// The one runtime seam that is deliberately *not* an
/// [`Exchange`](amenable_core::Exchange): a capability query carries no
/// precondition and produces no proven postcondition, so wrapping it in a
/// `Sidecar` pair would be ceremony over nothing. It feeds an
/// `Amenable`-style capability surface instead (plan Phase 7).
///
/// Ported from `elicit_temporal::traits::TemporalReporter`.
pub trait TemporalReporter: Send + Sync {
    /// Serialization profiles this backend can emit directly.
    fn supported_serialization_profiles(&self) -> Vec<SerializationProfile>;

    /// Maximum fractional-second precision this backend preserves, in
    /// decimal digits, if it has a fixed limit.
    fn max_fractional_second_digits(&self) -> Option<u8>;

    /// Whether this backend can represent leap-second inputs distinctly.
    fn supports_leap_seconds(&self) -> bool;

    /// Whether this backend preserves RFC 3339 unknown-local-offset
    /// semantics (the `-00:00` convention).
    fn supports_unknown_local_offset(&self) -> bool;

    /// Whether this backend can round-trip named-zone identity without
    /// flattening it to a numeric offset.
    fn supports_named_zone_round_trip(&self) -> bool;

    /// Whether this backend supports the end-of-day `24:00:00` form.
    fn supports_end_of_day_twenty_four(&self) -> bool;

    /// The current TZDB revision visible to this backend, when applicable.
    fn current_tzdb_revision(&self) -> Option<String>;

    /// A borrowed, [`Display`]-able view of this backend's whole
    /// capability declaration — the capability "surface" the plan calls
    /// for, built from the queries above.
    fn capabilities(&self) -> TemporalCapabilities<'_>
    where
        Self: Sized,
    {
        TemporalCapabilities { reporter: self }
    }
}

/// A borrowed capability declaration, rendered as a readable block by its
/// [`Display`] impl. Obtain one from [`TemporalReporter::capabilities`].
#[derive(Clone, Copy)]
pub struct TemporalCapabilities<'a> {
    reporter: &'a dyn TemporalReporter,
}

impl<'a> TemporalCapabilities<'a> {
    /// Wrap any `&dyn TemporalReporter` for display.
    #[must_use]
    pub fn new(reporter: &'a dyn TemporalReporter) -> Self {
        Self { reporter }
    }
}

impl Display for TemporalCapabilities<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let yes_no = |flag: bool| if flag { "yes" } else { "no" };

        let profiles = self.reporter.supported_serialization_profiles();
        let profiles = if profiles.is_empty() {
            "(none)".to_owned()
        } else {
            profiles
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        };

        writeln!(f, "serialization profiles     : {profiles}")?;
        writeln!(
            f,
            "max fractional-sec digits  : {}",
            self.reporter
                .max_fractional_second_digits()
                .map_or_else(|| "unbounded".to_owned(), |digits| digits.to_string()),
        )?;
        writeln!(
            f,
            "leap seconds               : {}",
            yes_no(self.reporter.supports_leap_seconds()),
        )?;
        writeln!(
            f,
            "unknown local offset (-00:00): {}",
            yes_no(self.reporter.supports_unknown_local_offset()),
        )?;
        writeln!(
            f,
            "named-zone round-trip      : {}",
            yes_no(self.reporter.supports_named_zone_round_trip()),
        )?;
        writeln!(
            f,
            "end-of-day 24:00:00        : {}",
            yes_no(self.reporter.supports_end_of_day_twenty_four()),
        )?;
        write!(
            f,
            "TZDB revision              : {}",
            self.reporter
                .current_tzdb_revision()
                .unwrap_or_else(|| "(not tracked)".to_owned()),
        )
    }
}
