//! UTC-offset conversions, reflect direction: a real
//! [`chrono::FixedOffset`] to [`amenable_time::UtcOffsetDescriptor`],
//! plus the staged, stub-friendly decomposition this direction's own
//! Kani contract composes from. Split out of `utc_offset.rs` once that
//! file grew large enough to be unnavigable as one unit.

use amenable_time::{
    ComponentOutOfRangeSource, TemporalComponent, TemporalError, TemporalErrorKind,
    UtcOffsetBuilderFailedSource, UtcOffsetDescriptor, UtcOffsetDescriptorBuilder, UtcOffsetSign,
};


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

/// Narrow a `u32` already known to fit in a `u8`, infallibly.
///
/// Split out of `narrow_to_u8` specifically so `#[kani::stub_verified]`
/// can target it directly: `narrow_to_u8`'s own `Result<u8,
/// TemporalError>` return type can't be stub-verified at all
/// (`TemporalError` has no `kani::Arbitrary` impl, and deliberately
/// doesn't need one -- see its own module doc), but a plain `u8`
/// return needs nothing beyond what it already has.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
#[cfg_attr(kani, kani::requires(value <= 255))]
#[cfg_attr(kani, kani::ensures(|result: &u8| u32::from(*result) == value))]
pub fn narrow_to_u8_in_range(value: u32) -> u8 {
    u8::try_from(value).expect("bounded by requires above")
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
#[cfg_attr(kani, kani::requires(value <= 255))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Result<u8, TemporalError>| match result {
        Ok(v) => u32::from(*v) == value,
        Err(_) => value > u32::from(u8::MAX),
    })
)]
pub fn narrow_to_u8(value: u32, component: TemporalComponent) -> Result<u8, TemporalError> {
    u8::try_from(value).map_err(|_err| {
        TemporalError::new(TemporalErrorKind::ComponentOutOfRange(
            ComponentOutOfRangeSource::new(component, value),
        ))
    })
}

/// Stage 2 of the offset-descriptor decomposition: sign and hours
/// only, stubbing `fixed_offset_sign_and_magnitude`. Each stage below
/// adds exactly one more fact and stubs the *previous stage's own
/// proven conjunction* rather than re-deriving everything inline --
/// the real function's own unstubbed, all-at-once contract states
/// four correlated facts (sign, hours, minutes, seconds) about one
/// symbolic `FixedOffset` at once, which DFCC cannot jointly
/// discharge within budget (confirmed directly, not assumed); staging
/// never asks it to relate more than two values at a time. Mirrors
/// `gallery::chrono_offset_date_time_round_trip_offset_staged_fix`'s
/// already-proven `offset_parts_2`/`_3`/`_4` exactly, now as the real
/// production path rather than a gallery-only reconstruction.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset)))]
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &(UtcOffsetSign, u8)| {
        let seconds = offset.local_minus_utc();
        let sign_matches = match result.0 {
            UtcOffsetSign::Positive => seconds >= 0,
            UtcOffsetSign::Negative => seconds < 0,
        };
        sign_matches && i64::from(result.1) == i64::from(seconds.unsigned_abs() / 3600)
    })
)]
pub fn offset_descriptor_parts_2(offset: chrono::FixedOffset) -> (UtcOffsetSign, u8) {
    let (sign, magnitude) = fixed_offset_sign_and_magnitude(offset);
    (sign, narrow_to_u8_in_range(magnitude / 3600))
}

/// Stage 3: stubs stage 2's proven two-fact conjunction, adds
/// `minutes` as the one new real computation.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset)))]
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &(UtcOffsetSign, u8, u8)| {
        let seconds = offset.local_minus_utc();
        let sign_matches = match result.0 {
            UtcOffsetSign::Positive => seconds >= 0,
            UtcOffsetSign::Negative => seconds < 0,
        };
        let magnitude = seconds.unsigned_abs();
        sign_matches
            && i64::from(result.1) == i64::from(magnitude / 3600)
            && i64::from(result.2) == i64::from((magnitude % 3600) / 60)
    })
)]
pub fn offset_descriptor_parts_3(offset: chrono::FixedOffset) -> (UtcOffsetSign, u8, u8) {
    let (sign, hours) = offset_descriptor_parts_2(offset);
    let minutes = narrow_to_u8_in_range((offset.local_minus_utc().unsigned_abs() % 3600) / 60);
    (sign, hours, minutes)
}

/// Stage 4: stubs stage 3's proven three-fact conjunction, adds
/// `seconds` -- the full four-fact relation, staged.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset)))]
#[cfg_attr(kani, kani::requires(true))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &(UtcOffsetSign, u8, u8, u8)| {
        let seconds_offset = offset.local_minus_utc();
        let sign_matches = match result.0 {
            UtcOffsetSign::Positive => seconds_offset >= 0,
            UtcOffsetSign::Negative => seconds_offset < 0,
        };
        let magnitude = seconds_offset.unsigned_abs();
        sign_matches
            && i64::from(result.1) == i64::from(magnitude / 3600)
            && i64::from(result.2) == i64::from((magnitude % 3600) / 60)
            && i64::from(result.3) == i64::from(magnitude % 60)
    })
)]
pub fn offset_descriptor_parts_4(offset: chrono::FixedOffset) -> (UtcOffsetSign, u8, u8, u8) {
    let (sign, hours, minutes) = offset_descriptor_parts_3(offset);
    let seconds = narrow_to_u8_in_range(offset.local_minus_utc().unsigned_abs() % 60);
    (sign, hours, minutes, seconds)
}

/// The builder's own four-fact transcription, isolated from the
/// offset-decomposition arithmetic `offset_descriptor_parts_4`
/// already proves -- no `FixedOffset` arithmetic left to reason about
/// here at all.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(sign)))]
#[cfg_attr(kani, kani::requires(hours <= 23))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Result<UtcOffsetDescriptor, TemporalError>| match result {
        Ok(descriptor) => {
            descriptor.sign() == sign
                && descriptor.hours() == hours
                && descriptor.minutes().unwrap_or(0) == minutes
                && descriptor.seconds().unwrap_or(0) == seconds
        }
        Err(_) => false,
    })
)]
pub fn build_utc_offset_descriptor_from_parts(
    sign: UtcOffsetSign,
    hours: u8,
    minutes: u8,
    seconds: u8,
) -> Result<UtcOffsetDescriptor, TemporalError> {
    let mut builder = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(hours);
    if minutes != 0 {
        builder = builder.minutes(minutes);
    }
    if seconds != 0 {
        builder = builder.seconds(seconds);
    }
    builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::UtcOffsetBuilderFailed(
            UtcOffsetBuilderFailedSource::new(err),
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
    let (sign, hours, minutes, seconds) = offset_descriptor_parts_4(offset);
    build_utc_offset_descriptor_from_parts(sign, hours, minutes, seconds)
}

mod kani_only {
    #![cfg(kani)]

    use super::{
        UtcOffsetDescriptor, UtcOffsetSign, build_utc_offset_descriptor_from_parts,
        offset_descriptor_parts_4,
    };
    use amenable_time::UtcOffsetRelationship;

    /// `fixed_offset_to_utc_offset_descriptor`'s infallible core, for Kani
    /// only -- `build_utc_offset_descriptor_from_parts`'s own `Err` arm is
    /// unreachable here (its builder has no validation at all, confirmed
    /// by reading it, and `sign`/`hours` -- the only required fields --
    /// are always set), the same "infallible for a real value, fallible
    /// API kept for robustness" shape as `narrow_to_u8`.
    ///
    /// Returns `UtcOffsetDescriptor` directly rather than wrapping it in a
    /// refinement newtype: unlike `chrono::FixedOffset`, it isn't foreign
    /// here, and its own `kani::Arbitrary` impl (in `amenable_time`,
    /// covering its whole real domain, not narrowed to this function's
    /// bounded output) already makes it directly `stub_verified`-able as
    /// a return type with nothing further needed.
    #[kani::requires(true)]
    #[kani::ensures(|result: &UtcOffsetDescriptor| {
        let seconds = offset.local_minus_utc();
        let sign_matches = match result.sign() {
            UtcOffsetSign::Positive => seconds >= 0,
            UtcOffsetSign::Negative => seconds < 0,
        };
        let magnitude = i64::from(result.hours()) * 3600
            + i64::from(result.minutes().unwrap_or(0)) * 60
            + i64::from(result.seconds().unwrap_or(0));
        // The explicit bounds aren't redundant with the magnitude equation
        // above: `UtcOffsetDescriptor`'s own `Arbitrary` impl is
        // deliberately unconstrained (that's its whole point), so when
        // this function is stubbed, the equation alone admits field
        // combinations that satisfy it without satisfying these bounds
        // (e.g. `hours: 0, minutes: 200` for a magnitude the equation
        // doesn't otherwise rule out) -- stating them here directly is
        // what lets `utc_offset_descriptor_to_fixed_offset_infallible`'s
        // own `requires` be checked against the stubbed result without
        // re-deriving them, the same lesson `fixed_offset_sign_and_
        // magnitude`'s own `ensures` already teaches.
        result.relationship() == UtcOffsetRelationship::Known
            && result.hours() <= 23
            && result.minutes().unwrap_or(0) <= 59
            && result.seconds().unwrap_or(0) <= 59
            && sign_matches
            && magnitude == i64::from(seconds.unsigned_abs())
    })]
    pub fn fixed_offset_to_utc_offset_descriptor_infallible(
        offset: chrono::FixedOffset,
    ) -> UtcOffsetDescriptor {
        let (sign, hours, minutes, seconds) = offset_descriptor_parts_4(offset);
        build_utc_offset_descriptor_from_parts(sign, hours, minutes, seconds)
            .expect("builder always succeeds with sign/hours set -- no validation on this builder")
    }
}

#[cfg(kani)]
pub use kani_only::fixed_offset_to_utc_offset_descriptor_infallible;
