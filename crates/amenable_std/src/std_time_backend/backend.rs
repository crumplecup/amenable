use std::time::{Duration, SystemTime};

use amenable_time::{
    SerializationProfile, TemporalDurationProps, TemporalInstantProps, TemporalReporter,
};

// ── Native carriers ─────────────────────────────────────────────────

/// A [`std::time::Duration`] as a temporal duration carrier.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct StdDuration(
    /// The wrapped span.
    Duration,
);

/// A [`std::time::SystemTime`] as a fixed-instant carrier.
///
/// `SystemTime` is a point on the system clock's timeline measured from
/// the Unix epoch — an absolute instant with no civil-calendar or
/// time-zone structure of its own, which is exactly the slice of the
/// interface this backend claims.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct StdSystemTime(
    /// The wrapped instant.
    SystemTime,
);

impl Default for StdSystemTime {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self(SystemTime::UNIX_EPOCH)
    }
}

/// The only UTC offset `std::time` represents: `+00:00`.
///
/// `SystemTime` is epoch-relative, so a `std::time` value is always at
/// zero offset from UTC; this ZST records that.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct StdUtcOffset;

// ── The backend ─────────────────────────────────────────────────────

/// The `std::time` canary backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct StdTimeBackend;

impl TemporalDurationProps for StdTimeBackend {
    type Duration = StdDuration;
}

impl TemporalInstantProps for StdTimeBackend {
    type UtcOffset = StdUtcOffset;
    type OffsetDateTime = StdSystemTime;
    type Instant = StdSystemTime;
}

impl TemporalReporter for StdTimeBackend {
    /// None — `std::time` has no serializer.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supported_serialization_profiles(&self) -> Vec<SerializationProfile> {
        Vec::new()
    }

    /// `std::time::Duration` resolves to the nanosecond: nine digits.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn max_fractional_second_digits(&self) -> Option<u8> {
        Some(9)
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_leap_seconds(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_unknown_local_offset(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_named_zone_round_trip(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_end_of_day_twenty_four(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn current_tzdb_revision(&self) -> Option<String> {
        None
    }
}
