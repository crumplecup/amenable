//! Reporter trait: backend capability inspection, minting no proofs.

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
}
