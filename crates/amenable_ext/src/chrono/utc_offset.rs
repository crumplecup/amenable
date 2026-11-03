//! UTC-offset descriptor conversions, realize direction: an
//! [`amenable_time::UtcOffsetDescriptor`] to a real
//! [`chrono::FixedOffset`], plus the staged, stub-friendly
//! decomposition this direction's own Kani contract composes from.
//! Split out of `instant.rs` -- this is a single, cohesive concern
//! (descriptor <-> `FixedOffset`) with its own proof-architecture
//! layer, large enough on its own to deserve a dedicated module
//! rather than sharing a file with the carriers/props/instant-bridge
//! code that builds on top of it. The reflect direction (`FixedOffset`
//! to descriptor) split further into `utc_offset_reflect.rs` once
//! this file also grew too large.

use amenable_time::{
    FixedOffsetOutOfRangeSource, TemporalError, TemporalErrorKind, UnsupportedCase,
    UnsupportedSource, UtcOffsetDescriptor, UtcOffsetRelationship, UtcOffsetSign,
};


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
///
/// Never proven standalone until now -- only ever exercised inlined,
/// inside the composed round-trip claim. `requires` states exactly the
/// bound every real `fixed_offset_to_utc_offset_descriptor` output
/// satisfies (hours `<= 23`, minutes/seconds `<= 59` when present):
/// under that bound the maximum possible magnitude, `23*3600 + 59*60 +
/// 59 = 86399`, is always within `chrono::FixedOffset`'s own
/// `+/-86399s` range, so the `Err` arm is provably unreachable here --
/// the same "infallible for a real value, fallible API kept for
/// robustness" shape as `narrow_to_u8`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
#[cfg_attr(
    kani,
    kani::requires(
        descriptor.relationship() == UtcOffsetRelationship::Known
            && descriptor.hours() <= 23
            && descriptor.minutes().unwrap_or(0) <= 59
            && descriptor.seconds().unwrap_or(0) <= 59
    )
)]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Result<chrono::FixedOffset, TemporalError>| match result {
        Ok(offset) => {
            let magnitude = i32::from(descriptor.hours()) * 3600
                + i32::from(descriptor.minutes().unwrap_or(0)) * 60
                + i32::from(descriptor.seconds().unwrap_or(0));
            let expected = match descriptor.sign() {
                UtcOffsetSign::Positive => magnitude,
                UtcOffsetSign::Negative => -magnitude,
            };
            offset.local_minus_utc() == expected
        }
        Err(_) => false,
    })
)]
pub fn utc_offset_descriptor_to_fixed_offset(
    descriptor: UtcOffsetDescriptor,
) -> Result<chrono::FixedOffset, TemporalError> {
    if descriptor.relationship() == UtcOffsetRelationship::UnknownLocalOffset {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::closed(UnsupportedCase::ChronoUnknownLocalOffset),
        )));
    }

    let seconds = signed_seconds_parts_4(
        descriptor.sign(),
        descriptor.hours(),
        descriptor.minutes().unwrap_or(0),
        descriptor.seconds().unwrap_or(0),
    );

    // Kani-only: this function's own `requires` above (hours <= 23,
    // minutes/seconds <= 59) combined with `signed_seconds_parts_4`'s
    // own proven range guarantees `seconds.unsigned_abs() <= 86_399`
    // unconditionally here -- the general wrapper's own bound check
    // below is always true for this call site under Kani, making it
    // pure real, unstubbed code sitting between this caller and the
    // stub for no benefit. Confirmed as the actual remaining fix: a
    // gallery reconstruction calling the stubbable core directly
    // passed, while one going through the wrapper (matching this
    // function's real shape before this change) still timed out.
    #[cfg(kani)]
    {
        Ok(build_refined_fixed_offset_in_range(seconds).into_inner())
    }
    #[cfg(not(kani))]
    {
        build_fixed_offset_from_signed_seconds(seconds)
    }
}

/// Stage 2 of the realize-direction decomposition: sign and hours'
/// contribution alone, as a signed seconds quantity. Mirrors the
/// reflect direction's `offset_descriptor_parts_2/3/4`, just running
/// the arithmetic forward (recombination) instead of backward
/// (decomposition) -- confirmed via a standalone probe that the
/// realize direction has the exact same "multiple correlated facts in
/// one proof obligation" shape the reflect direction did, never
/// proven at all before that probe.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(sign)))]
#[cfg_attr(kani, kani::requires(hours <= 23))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &i32| {
        let magnitude = i32::from(hours) * 3600;
        let expected = match sign {
            UtcOffsetSign::Positive => magnitude,
            UtcOffsetSign::Negative => -magnitude,
        };
        *result == expected && result.unsigned_abs() <= 82_800
    })
)]
pub fn signed_seconds_parts_2(sign: UtcOffsetSign, hours: u8) -> i32 {
    let magnitude = i32::from(hours) * 3600;
    match sign {
        UtcOffsetSign::Positive => magnitude,
        UtcOffsetSign::Negative => -magnitude,
    }
}

/// Stage 3: stubs stage 2, adds the minutes contribution.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(sign)))]
#[cfg_attr(kani, kani::requires(hours <= 23 && minutes <= 59))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &i32| {
        let magnitude = i32::from(hours) * 3600 + i32::from(minutes) * 60;
        let expected = match sign {
            UtcOffsetSign::Positive => magnitude,
            UtcOffsetSign::Negative => -magnitude,
        };
        *result == expected && result.unsigned_abs() <= 86_340
    })
)]
pub fn signed_seconds_parts_3(sign: UtcOffsetSign, hours: u8, minutes: u8) -> i32 {
    let hours_part = signed_seconds_parts_2(sign, hours);
    let minutes_magnitude = i32::from(minutes) * 60;
    match sign {
        UtcOffsetSign::Positive => hours_part + minutes_magnitude,
        UtcOffsetSign::Negative => hours_part - minutes_magnitude,
    }
}

/// Stage 4: stubs stage 3, adds the seconds contribution -- the full
/// signed-seconds total. States the explicit `<= 86_399` bound (not
/// just the equality) for the same reason `fixed_offset_sign_and_
/// magnitude`'s own `ensures` does: it lets `build_fixed_offset_from_
/// signed_seconds`'s `requires` be checked directly against this
/// stubbed value, without re-deriving chrono's own range from the
/// combination arithmetic.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(sign)))]
#[cfg_attr(kani, kani::requires(hours <= 23 && minutes <= 59 && seconds <= 59))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &i32| {
        let magnitude =
            i32::from(hours) * 3600 + i32::from(minutes) * 60 + i32::from(seconds);
        let expected = match sign {
            UtcOffsetSign::Positive => magnitude,
            UtcOffsetSign::Negative => -magnitude,
        };
        *result == expected && result.unsigned_abs() <= 86_399
    })
)]
pub fn signed_seconds_parts_4(sign: UtcOffsetSign, hours: u8, minutes: u8, seconds: u8) -> i32 {
    let hours_minutes_part = signed_seconds_parts_3(sign, hours, minutes);
    let seconds_magnitude = i32::from(seconds);
    match sign {
        UtcOffsetSign::Positive => hours_minutes_part + seconds_magnitude,
        UtcOffsetSign::Negative => hours_minutes_part - seconds_magnitude,
    }
}

mod kani_only {
    #![cfg(kani)]

    use super::{UtcOffsetDescriptor, UtcOffsetRelationship, UtcOffsetSign, signed_seconds_parts_4};

    /// A [`chrono::FixedOffset`] already known to be within its own real
    /// domain -- `-86_400 < seconds && seconds < 86_400`, confirmed via
    /// chrono's own source (`FixedOffset::east_opt`/`west_opt` are its
    /// only constructors, both enforcing exactly this bound, and
    /// `local_minus_utc` is a private field with no other way in).
    ///
    /// Exists only so a Kani proof can `#[kani::stub_verified]` a function
    /// returning a real `FixedOffset`: the real type can't get a manual
    /// `kani::Arbitrary` impl directly here (both the trait and the type
    /// are foreign -- the orphan rule blocks it), but a local newtype
    /// wrapping it isn't foreign. `any()` composes from `signed_seconds_
    /// parts_4`'s own already-proven-cheap chain (a mixed-radix
    /// decomposition that's a bijection onto `0..=86399`) rather than a
    /// wide `kani::any::<i32>()` plus `assume`, so it ranges over exactly
    /// chrono's real domain -- every real offset reachable, nothing
    /// invalid included -- without reintroducing the wide-space-then-
    /// filter cost this investigation has been fighting throughout.
    /// Confirmed in the gallery before this type ever touched production
    /// code (`gallery::chrono_offset_date_time_round_trip_realize_staged_
    /// fix::realize_staged_stubbed_resolves_the_wall`).
    pub struct RefinedFixedOffset(chrono::FixedOffset);

    impl RefinedFixedOffset {
        /// Unwrap to the real `chrono::FixedOffset` this refines.
        pub fn into_inner(self) -> chrono::FixedOffset {
            self.0
        }
    }

    impl kani::Arbitrary for RefinedFixedOffset {
        fn any() -> Self {
            let sign: UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let seconds: u8 = kani::any();
            kani::assume(seconds <= 59);
            let signed_seconds = signed_seconds_parts_4(sign, hours, minutes, seconds);
            Self(
                chrono::FixedOffset::east_opt(signed_seconds)
                    .expect("bounded by signed_seconds_parts_4's own proven range"),
            )
        }
    }

    /// The infallible core behind `build_fixed_offset_from_signed_seconds`'s
    /// in-range path -- returns the newtype directly (no `Result`, so no
    /// `TemporalError`-`Arbitrary` problem either), making it directly
    /// `stub_verified`-able.
    #[kani::requires(seconds.unsigned_abs() <= 86_399)]
    #[kani::ensures(|result: &RefinedFixedOffset| result.0.local_minus_utc() == seconds)]
    pub fn build_refined_fixed_offset_in_range(seconds: i32) -> RefinedFixedOffset {
        RefinedFixedOffset(
            chrono::FixedOffset::east_opt(seconds).expect("bounded by requires above"),
        )
    }

    /// `utc_offset_descriptor_to_fixed_offset`'s infallible core, for Kani
    /// only -- takes the relationship bound as a `requires` precondition
    /// instead of a runtime branch, and returns `RefinedFixedOffset`
    /// directly (no `Result`, so no `TemporalError`-`Arbitrary` problem),
    /// making it directly `stub_verified`-able as a whole function. Mirrors
    /// the already-confirmed-fast internal structure of the real function's
    /// own Kani fast path above, just without the `Result` wrapper.
    #[kani::requires(
        descriptor.relationship() == UtcOffsetRelationship::Known
            && descriptor.hours() <= 23
            && descriptor.minutes().unwrap_or(0) <= 59
            && descriptor.seconds().unwrap_or(0) <= 59
    )]
    #[kani::ensures(|result: &RefinedFixedOffset| {
        let magnitude = i32::from(descriptor.hours()) * 3600
            + i32::from(descriptor.minutes().unwrap_or(0)) * 60
            + i32::from(descriptor.seconds().unwrap_or(0));
        let expected = match descriptor.sign() {
            UtcOffsetSign::Positive => magnitude,
            UtcOffsetSign::Negative => -magnitude,
        };
        result.0.local_minus_utc() == expected
    })]
    pub fn utc_offset_descriptor_to_fixed_offset_infallible(
        descriptor: UtcOffsetDescriptor,
    ) -> RefinedFixedOffset {
        let seconds = signed_seconds_parts_4(
            descriptor.sign(),
            descriptor.hours(),
            descriptor.minutes().unwrap_or(0),
            descriptor.seconds().unwrap_or(0),
        );
        build_refined_fixed_offset_in_range(seconds)
    }
}

#[cfg(kani)]
pub use kani_only::{
    RefinedFixedOffset, build_refined_fixed_offset_in_range,
    utc_offset_descriptor_to_fixed_offset_infallible,
};

/// The final, cheap step: just the one `FixedOffset::east_opt` call
/// and its error mapping, isolated from the signed-seconds arithmetic
/// `signed_seconds_parts_4` already proves.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
#[cfg_attr(kani, kani::requires(seconds.unsigned_abs() <= 86_399))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &Result<chrono::FixedOffset, TemporalError>| match result {
        Ok(offset) => offset.local_minus_utc() == seconds,
        Err(_) => false,
    })
)]
pub fn build_fixed_offset_from_signed_seconds(seconds: i32) -> Result<chrono::FixedOffset, TemporalError> {
    // Kani-only fast path: when a Kani proof stubs `build_refined_
    // fixed_offset_in_range`, this early return is what lets that
    // stub actually apply here instead of CBMC reasoning through the
    // real `east_opt` call below under already-stubbed symbolic
    // state -- confirmed in the gallery as the fix for exactly that
    // timeout. Changes nothing about real (non-Kani) behavior: the
    // bound check is the same one `east_opt` already performs
    // internally, just surfaced so this early return can exist.
    #[cfg(kani)]
    if seconds.unsigned_abs() <= 86_399 {
        return Ok(build_refined_fixed_offset_in_range(seconds).into_inner());
    }

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
