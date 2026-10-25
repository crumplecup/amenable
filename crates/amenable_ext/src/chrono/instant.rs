use crate::chrono::civil::{
    local_date_time_descriptor_to_naive, naive_date_time_to_local_date_time_descriptor,
};
use crate::chrono::identity::{ChronoTimeBackend, ChronoVerifier};
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    ComponentOutOfRangeSource, FixedOffsetOutOfRangeSource, NoUniqueLocalMappingSource,
    OffsetDateTimeBuilderFailedSource, OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder,
    ProvenOffsetDateTimeCarrier, ReflectedOffsetDateTime, TemporalComponent, TemporalError,
    TemporalErrorKind, TemporalInstantProps, UnsupportedCase, UnsupportedSource,
    UtcOffsetBuilderFailedSource, UtcOffsetDescriptor, UtcOffsetDescriptorBuilder,
    UtcOffsetRelationship, UtcOffsetSign,
};

// ── Carriers ─────────────────────────────────────────────────────────

/// A [`chrono::FixedOffset`] as a UTC-offset carrier.
///
/// `chrono::FixedOffset` derives only `PartialEq, Eq, Hash, Copy, Clone`
/// (confirmed by reading chrono's real source) — no `Default`. Same
/// `basis_ctor` need as the jiff backend's `JiffOffset`: this wrapper
/// supplies `FixedOffset::east_opt(0).unwrap()` (a real, concrete zero
/// offset) as its root basis instead.
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
#[evidence(
    basis = "Self",
    basis_ctor = "Self(chrono::FixedOffset::east_opt(0).expect(\"0 is in FixedOffset's representable range\"))"
)]
pub struct ChronoOffset(
    /// The wrapped offset.
    chrono::FixedOffset,
);

/// A [`chrono::DateTime<chrono::FixedOffset>`] as an offset-aware
/// date-time carrier.
///
/// Unlike jiff (which has no first-class "offset date-time" type and
/// needs a small composite carrier), chrono's `DateTime<FixedOffset>` is
/// already exactly this: a local date-time paired with the fixed offset
/// it was recorded at. `DateTime<FixedOffset>` has a real `Default`
/// (confirmed in source: `FixedOffset::west_opt(0).unwrap().from_utc_
/// datetime(&NaiveDateTime::default())`), so no `basis_ctor` is needed.
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
pub struct ChronoOffsetDateTime(
    /// The wrapped offset date-time.
    chrono::DateTime<chrono::FixedOffset>,
);

/// A [`chrono::DateTime<chrono::Utc>`] as a fixed-instant carrier.
///
/// `DateTime<Utc>` has a real `Default` (confirmed in source), so no
/// `basis_ctor` is needed, the same as `ChronoOffsetDateTime`.
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
pub struct ChronoInstant(
    /// The wrapped instant.
    chrono::DateTime<chrono::Utc>,
);

// ── Props ────────────────────────────────────────────────────────────

impl TemporalInstantProps for ChronoTimeBackend {
    type UtcOffset = ChronoOffset;
    type OffsetDateTime = ChronoOffsetDateTime;
    type Instant = ChronoInstant;
}

// ── Descriptor conversions ───────────────────────────────────────────

/// Resolve a UTC-offset descriptor to a real [`chrono::FixedOffset`].
///
/// Real, honest scoping: the RFC 9557 "unknown local offset" relationship
/// (`-00:00`) has no fixed-offset representation at all — `chrono::
/// FixedOffset` is always a concrete, known value — so that case is a real
/// `Unsupported` error, never silently coerced to a zero offset or any
/// other default. Mirrors the jiff backend's own `utc_offset_descriptor_
/// to_jiff_offset` exactly; the two upstream offset types share the same
/// signed-seconds-east representation.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub fn utc_offset_descriptor_to_fixed_offset(
    descriptor: UtcOffsetDescriptor,
) -> Result<chrono::FixedOffset, TemporalError> {
    if descriptor.relationship() == UtcOffsetRelationship::UnknownLocalOffset {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::closed(UnsupportedCase::ChronoUnknownLocalOffset),
        )));
    }

    let magnitude = i32::from(descriptor.hours()) * 3600
        + i32::from(descriptor.minutes().unwrap_or(0)) * 60
        + i32::from(descriptor.seconds().unwrap_or(0));
    let seconds = match descriptor.sign() {
        UtcOffsetSign::Positive => magnitude,
        UtcOffsetSign::Negative => -magnitude,
    };

    // `FixedOffsetOutOfRangeSource` carries the real `seconds`
    // magnitude as a getter field but deliberately excludes it from
    // `Display` -- see that type's own doc comment for why
    // interpolating a symbolic signed value here was confirmed
    // Kani-expensive (`Formatter::padding` on a variable-width value)
    // at this exact, real-Kani-harness-reachable call site.
    chrono::FixedOffset::east_opt(seconds).ok_or_else(|| {
        TemporalError::new(TemporalErrorKind::FixedOffsetOutOfRange(
            FixedOffsetOutOfRangeSource::new(seconds),
        ))
    })
}

/// Decompose a real `chrono::FixedOffset`'s sign and magnitude in
/// seconds. Infallible -- `chrono::FixedOffset` is always within
/// +/-23:59:59 (confirmed in its own source), so this never needs the
/// narrowing fallibility the caller's `u8::try_from` calls still carry.
///
/// Split out from [`fixed_offset_to_utc_offset_descriptor`] so a Kani
/// proof of that function's own contract can `#[kani::stub_verified]`
/// this step instead of inlining it -- confirmed the fix for a real
/// timeout: the combined claim (sign *and* magnitude, four facts about
/// one symbolic call) exceeded the 3-minute budget as one unstubbed
/// body, but passes once this arithmetic half is proven and stubbed on
/// its own, leaving only the builder assembly to analyze directly (see
/// `gallery::chrono_offset_date_time_round_trip`'s staged isolation
/// cases for the full decomposition).
// `#[cfg_attr(kani, ...)]`, not a Cargo dependency on `kani`: the
// attribute is stripped entirely outside the Kani compiler, the same
// way `#[cfg_attr(not(kani), tracing::instrument(...))]` already does
// the reverse throughout this module -- `kani` is only ever resolved
// as a path when the cfg is actually active.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset)))]
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &(UtcOffsetSign, u32)| {
        let seconds = offset.local_minus_utc();
        let sign_matches = match result.0 {
            UtcOffsetSign::Positive => seconds >= 0,
            UtcOffsetSign::Negative => seconds < 0,
        };
        // The explicit `<= 86_399` isn't redundant with `==
        // seconds.unsigned_abs()`: it's what lets a downstream
        // `requires` (`narrow_to_u8`'s) be checked directly against
        // this stubbed value, without needing to re-derive chrono's
        // own `FixedOffset` range from its real (unstubbed) internals.
        sign_matches && result.1 == seconds.unsigned_abs() && result.1 <= 86_399
    })
)]
pub fn fixed_offset_sign_and_magnitude(offset: chrono::FixedOffset) -> (UtcOffsetSign, u32) {
    let seconds = offset.local_minus_utc();
    let sign = if seconds < 0 {
        UtcOffsetSign::Negative
    } else {
        UtcOffsetSign::Positive
    };
    (sign, seconds.unsigned_abs())
}

/// Narrow a `u32` to `u8`, reporting which [`TemporalComponent`]
/// overflowed on failure. Split out so a Kani proof can
/// `#[kani::stub_verified]` each of
/// `fixed_offset_to_utc_offset_descriptor`'s three narrowing call sites
/// instead of inlining `u8::try_from`'s own fallibility three times --
/// each is individually "genuinely unreachable defensive code for a
/// real offset" (see that function's own doc comment), but left
/// unstubbed, their combined cost under DFCC was confirmed to push the
/// whole contract back over the 3-minute budget even with the sign/
/// magnitude arithmetic above already stubbed.
///
/// `requires(value <= 255)` isn't just a tighter bound for its own
/// sake: proving this contract over the *full* `u32` domain means
/// actually reaching the `Err` arm, which calls `invalid` ->
/// `TemporalError::new`, which is `#[track_caller]` -- and Kani does
/// not support `caller_location` at all (a hard failure, confirmed
/// directly, not a timeout: "caller_location is not currently
/// supported by Kani"). Restricting to this function's only real
/// domain of use (every call site passes `magnitude / 3600`,
/// `remainder_minutes`, or `remainder_seconds`, each well under 255
/// once `fixed_offset_sign_and_magnitude`'s own ensures bounds
/// `magnitude`) keeps `Err` provably unreachable, matching what the
/// caller's doc comment already claims in prose.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(component)))]
pub fn narrow_to_u8(value: u32, component: TemporalComponent) -> Result<u8, TemporalError> {
    u8::try_from(value).map_err(|_err| {
        TemporalError::new(TemporalErrorKind::ComponentOutOfRange(
            ComponentOutOfRangeSource::new(component, value),
        ))
    })
}

/// Decompose a real [`chrono::FixedOffset`] back into a UTC-offset
/// descriptor.
///
/// Canonicalizes to the integral-hour form (`minutes: None`) whenever the
/// offset is an exact whole number of hours, the same convention
/// `UtcOffsetDescriptor`'s own doc comment states. Mirrors the jiff
/// backend's `jiff_offset_to_utc_offset_descriptor`.
///
/// Captures a sub-minute remainder losslessly via `UtcOffsetDescriptor`'s
/// own `seconds` field, rather than truncating it: a real
/// `chrono::FixedOffset` can carry sub-minute precision, confirmed by
/// scanning chrono-tz's own real historical data, not assumed -- 1952
/// real `(zone, year)` combinations before timezone standardization
/// have a genuine non-zero seconds remainder (e.g. `Africa/Abidjan`,
/// 1800: `-968s` = `-0:16:08`, a real Local Mean Time offset). See
/// `UtcOffsetDescriptor::seconds`'s own doc comment for why this is
/// still ISO-8601-compliant (the spec states a floor on the required
/// shape, not a ceiling on what the type may also carry).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset)))]
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Result<UtcOffsetDescriptor, TemporalError>| {
        match result {
            Ok(descriptor) => {
                let seconds = offset.local_minus_utc();
                let sign_matches = match descriptor.sign() {
                    UtcOffsetSign::Positive => seconds >= 0,
                    UtcOffsetSign::Negative => seconds < 0,
                };
                let magnitude = i64::from(descriptor.hours()) * 3600
                    + i64::from(descriptor.minutes().unwrap_or(0)) * 60
                    + i64::from(descriptor.seconds().unwrap_or(0));
                sign_matches && magnitude == i64::from(seconds.unsigned_abs())
            }
            Err(_) => true,
        }
    })
)]
pub fn fixed_offset_to_utc_offset_descriptor(
    offset: chrono::FixedOffset,
) -> Result<UtcOffsetDescriptor, TemporalError> {
    let (sign, magnitude) = fixed_offset_sign_and_magnitude(offset);
    // `magnitude / 3600 <= 23` and the two `% 60` remainders below
    // always fit `u8`, and the builder's two required fields are always
    // set -- every `narrow_to_u8` call below is genuinely unreachable
    // defensive code for a real offset. `narrow_to_u8` doesn't
    // interpolate the discarded `TryFromIntError` itself (confirmed,
    // by reading its `Display` impl, to carry no runtime-varying
    // information beyond a fixed description -- doing so was the real
    // cost behind a CBMC timeout, a `Debug`-driven `PadAdapter`
    // recursion), but it does carry the real overflowing `value`
    // through `ComponentOutOfRangeSource`, unlike the old free-text
    // `detail: String` this replaced.
    let hours = narrow_to_u8(magnitude / 3600, TemporalComponent::Hour)?;
    let remainder_minutes = (magnitude % 3600) / 60;
    let remainder_seconds = magnitude % 60;

    let mut builder = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(hours);
    if remainder_minutes != 0 {
        let minutes = narrow_to_u8(remainder_minutes, TemporalComponent::Minute)?;
        builder = builder.minutes(minutes);
    }
    if remainder_seconds != 0 {
        let seconds = narrow_to_u8(remainder_seconds, TemporalComponent::Second)?;
        builder = builder.seconds(seconds);
    }

    builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::UtcOffsetBuilderFailed(
            UtcOffsetBuilderFailedSource::new(err),
        ))
    })
}

/// Resolve a full offset date-time descriptor to a real
/// `chrono::DateTime<chrono::FixedOffset>`.
///
/// Reuses the civil bridge's own local-date-time conversion for the local
/// half. `NaiveDateTime::and_local_timezone` is the real, non-deprecated
/// way to attach an offset (`DateTime::from_local` was deprecated in
/// 0.4.27) — confirmed via chrono's real source that `FixedOffset`'s own
/// `TimeZone::offset_from_local_datetime` always returns `MappedLocalTime::
/// Single`, since a fixed offset has no ambiguity or gap to report, so the
/// `None` arm below is unreachable in practice but still a real `Err`
/// rather than an unwrap.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub fn offset_date_time_descriptor_to_chrono(
    descriptor: &OffsetDateTimeDescriptor,
) -> Result<chrono::DateTime<chrono::FixedOffset>, TemporalError> {
    let local = local_date_time_descriptor_to_naive(descriptor.local())?;
    let offset = utc_offset_descriptor_to_fixed_offset(descriptor.offset())?;
    // `NoUniqueLocalMappingSource` carries no payload beyond
    // provenance -- see that type's own doc comment: this branch is
    // genuinely unreachable for `FixedOffset` (confirmed above: a
    // fixed offset has no ambiguity or gap to report), so there's no
    // real value to add, and formatting either `local`/`offset`
    // symbolically was separately confirmed Kani-expensive for the
    // same `Formatter::padding` reason.
    local
        .and_local_timezone(offset)
        .single()
        .ok_or_else(|| TemporalError::new(TemporalErrorKind::NoUniqueLocalMapping(
            NoUniqueLocalMappingSource::new(),
        )))
}

/// The inverse of [`offset_date_time_descriptor_to_chrono`].
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(datetime)))]
pub fn chrono_to_offset_date_time_descriptor(
    datetime: chrono::DateTime<chrono::FixedOffset>,
) -> Result<OffsetDateTimeDescriptor, TemporalError> {
    let local = naive_date_time_to_local_date_time_descriptor(datetime.naive_local())?;
    let offset = fixed_offset_to_utc_offset_descriptor(*datetime.offset())?;
    // Both required fields (`local`/`offset`) are always set above, so
    // this can't genuinely fail -- a static message, not
    // `format!("...{err}")`, for the same reason given in
    // `fixed_offset_to_utc_offset_descriptor`.
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::OffsetDateTimeBuilderFailed(
                OffsetDateTimeBuilderFailedSource::new(err),
            ))
        })
}

// ── Instant native bridge ────────────────────────────────────────────
//
// `TemporalInstantNativeBridge<ChronoVerifier>` is the `realize_offset_
// date_time` / `reflect_offset_date_time` inverse pair. `UtcOffset` and
// `Instant` are plain associated types with no Exchange obligation of
// their own in this trait (see its definition in amenable_time).

impl
    Exchange<
        ReflectedOffsetDateTime,
        ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>,
        ChronoVerifier,
    > for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedOffsetDateTime,
    ) -> Result<ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>, TemporalError> {
        let datetime = offset_date_time_descriptor_to_chrono(input.descriptor())?;
        let token = <ReflectedOffsetDateTime as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
            ChronoOffsetDateTime::new(datetime),
            token,
        ))
    }
}

impl
    Exchange<
        ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>,
        ReflectedOffsetDateTime,
        ChronoVerifier,
    > for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>,
    ) -> Result<ReflectedOffsetDateTime, TemporalError> {
        let descriptor = chrono_to_offset_date_time_descriptor(**input.carrier())?;
        let token = <ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime> as Sidecar<
            ChronoVerifier,
        >>::sidecar(&input);
        Ok(ReflectedOffsetDateTime::new(descriptor, token))
    }
}
