//! The `std::time` canary backend — see [`super`] for what "canary" means.

use std::time::{Duration, SystemTime};

use amenable_core::{
    ClassifiedWitness, Establish, Exchange, Metadata, OwnedEntry, Provenance, Sidecar, Standard,
    Verifier, Witness, WitnessSupportSummary,
};

use crate::{
    OrderOffsetEndpointsEstablished, OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput,
    OrderOffsetEndpointsPreconditionsToken, ParsedDuration, ParsedRecurringInterval,
    ParsedTimeInterval, RawInput, SerializationProfile, TemporalDurationProps, TemporalError,
    TemporalErrorKind, TemporalInstantProps, TemporalProvenance, TemporalReporter,
};

// ── CanaryVerifier ──────────────────────────────────────────────────

/// Reporting surface for [`CanaryVerifier`] — states plainly that it
/// checks nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CanaryVerifierMetadata;

impl Metadata for CanaryVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "canary"),
            OwnedEntry::new("checks", "nothing — interface exercise only"),
        ]
    }
}

impl Provenance for CanaryVerifierMetadata {}

/// A [`Verifier`] marker that proves nothing.
///
/// It exists so the [`std::time`](std::time) backend can name a concrete
/// `V` and have the temporal trait surface type-check end to end. An
/// `Exchange` written against `CanaryVerifier` carries no proof
/// obligation beyond what the type system already enforces (the
/// `Witness<V>` / `ClassifiedWitness<V>` bounds on every proposition,
/// which `amenable_time::structural_witness` and the `proof_composition`
/// derives satisfy for every `V`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CanaryVerifier;

impl Verifier for CanaryVerifier {
    type Metadata = CanaryVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "canary"
    }
}

// ── Trusted witnesses for the machine-checkable contracts ───────────
//
// The 23 range / ordering / arithmetic contracts carry real
// `Witness<KaniVerifier>` / `<CreusotVerifier>` / `<VerusVerifier>` proofs
// (`amenable_kani::time` etc.). `CanaryVerifier` is not one of those
// backends — it checks nothing — so from its point of view these are
// trusted citations exactly like the structural contracts. Without this
// block a `proof_composition` aggregate that reaches one of them (e.g.
// `IntervalEndpointsOrdered` → `IntervalStartPrecedesEnd`) would not be
// `Witness<CanaryVerifier>`, and the interval exchange surface would not
// type-check.

/// One `Witness<CanaryVerifier>` + `ClassifiedWitness<CanaryVerifier>` per
/// named contract, resting on the contract's own citation (the canary
/// verifies nothing).
macro_rules! canary_trusts {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Witness<CanaryVerifier> for $ty {
                type SupportingEvidence = Self;
                type ProofArtifact = TemporalProvenance;

                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn proof() -> Self::ProofArtifact {
                    <Self as Standard>::provenance(
                        &<Self as ::std::default::Default>::default(),
                    )
                }

                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn support() -> WitnessSupportSummary {
                    WitnessSupportSummary::trusted_leaf()
                }
            }

            impl ClassifiedWitness<CanaryVerifier> for $ty {}
        )+
    };
}

canary_trusts! {
    crate::CalendarMonthInRangeOneToTwelve,
    crate::HourInRangeZeroToTwentyFour,
    crate::MinuteInRangeZeroToFiftyNine,
    crate::SecondInRangeZeroToSixty,
    crate::UtcOffsetHourInRangeZeroToTwentyThree,
    crate::UtcOffsetMinuteInRangeZeroToFiftyNine,
    crate::WeekdayInRangeOneToSeven,
    crate::WeekNumberInRangeOneToFiftyThree,
    crate::OrdinalDayInRangeOneToThreeHundredSixtySix,
    crate::CenturyOrdinalInRangeZeroToNinetyNine,
    crate::DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    crate::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    crate::IntervalStartPrecedesEnd,
    crate::IntervalDurationIsNonNegative,
    crate::UtcTimelineOrderingAppliesToFixedInstants,
    crate::GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    crate::CentennialYearDivisibleByOneHundred,
    crate::LeapYearHasThreeHundredSixtySixCalendarDays,
    crate::CommonYearHasThreeHundredSixtyFiveCalendarDays,
    crate::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    crate::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    crate::CalendarDayWithinMonthBounds,
    crate::LeapDayOccursOnlyInLeapYear,
}

/// `TemporalInputReceived` — "a caller handed us this text" — is a root
/// claim asserted by construction (see `exchange::markers`); its
/// `Witness<V>` is trivial and backend-provided, so the canary provides
/// the `CanaryVerifier` one here.
impl Witness<CanaryVerifier> for crate::TemporalInputReceived {
    type SupportingEvidence = Self;
    type ProofArtifact = &'static str;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        "raw text received at the exchange boundary — a root claim, asserted by construction"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trivial_leaf()
    }
}

impl ClassifiedWitness<CanaryVerifier> for crate::TemporalInputReceived {}

// ── Native carriers ─────────────────────────────────────────────────

/// A [`std::time::Duration`] as a temporal duration carrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct StdDuration(
    /// The wrapped span.
    pub Duration,
);

/// A [`std::time::SystemTime`] as a fixed-instant carrier.
///
/// `SystemTime` is a point on the system clock's timeline measured from
/// the Unix epoch — an absolute instant with no civil-calendar or
/// time-zone structure of its own, which is exactly the slice of the
/// interface this backend claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct StdSystemTime(
    /// The wrapped instant.
    pub SystemTime,
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

// ── Interval exchanges ──────────────────────────────────────────────
//
// `TemporalIntervalFactory<CanaryVerifier>` is a blanket-impl'd supertrait
// bundle over these four `Exchange`s. The three text-parse edges have no
// honest `std::time` implementation (there is no ISO 8601 parser in
// `std`), so they are lawful `Exchange` impls that always report the
// operation unsupported. `order_offset_endpoints` is real: it exercises
// the full sidecar loop — read the input's proof token, mint the output's
// via `Establish`, and pair it with the re-issued proposition.

/// Report the missing ISO 8601 parser for a text-parse edge.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn unsupported_parse(edge: &'static str) -> TemporalError {
    TemporalError::new(TemporalErrorKind::Unsupported(format!(
        "std::time cannot parse {edge}: no ISO 8601 / RFC 3339 parser in the standard library"
    )))
}

impl Exchange<RawInput, ParsedDuration, CanaryVerifier> for StdTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedDuration, TemporalError> {
        Err(unsupported_parse("an ISO 8601 duration"))
    }
}

impl Exchange<RawInput, ParsedRecurringInterval, CanaryVerifier> for StdTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedRecurringInterval, TemporalError> {
        Err(unsupported_parse("a recurring interval"))
    }
}

impl Exchange<RawInput, ParsedTimeInterval, CanaryVerifier> for StdTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedTimeInterval, TemporalError> {
        Err(unsupported_parse("a time interval"))
    }
}

impl Exchange<OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput, CanaryVerifier>
    for StdTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: OrderOffsetEndpointsInput,
    ) -> Result<OrderOffsetEndpointsOutput, TemporalError> {
        // The endpoint descriptors are structurally ordered by the input's
        // own proven precondition; the canary just carries that proof
        // forward. Consume the input token, mint the output token from it.
        let credential: OrderOffsetEndpointsPreconditionsToken =
            <OrderOffsetEndpointsInput as Sidecar<CanaryVerifier>>::sidecar(&input);
        let token = <OrderOffsetEndpointsEstablished as Establish<
            OrderOffsetEndpointsPreconditionsToken,
            CanaryVerifier,
        >>::establish(credential);

        Ok(OrderOffsetEndpointsOutput::new(
            OrderOffsetEndpointsEstablished::default(),
            token,
        ))
    }
}
