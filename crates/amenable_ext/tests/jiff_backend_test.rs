//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 1 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalDurationProps` +
//! `TemporalDurationNativeBridge` over `jiff::Span`.
//!
//! Unlike `amenable_std`'s `std_backend_test.rs` (a runtime oracle over
//! a deliberately narrow canary), this exercises real jiff calendar
//! arithmetic: `jiff::Span`'s own `try_*` setters and `get_*` getters,
//! not a hand-rolled stand-in.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffSpan, JiffTimeBackend, JiffVerifier};
use amenable_time::{
    DurationDescriptorBuilder, DurationFormValid, DurationFormValidToken,
    DurationFractionDescriptor, DurationSemanticBundle, DurationSemanticBundleToken,
    ProvenDurationCarrier, ReflectedDuration, TemporalComponent, TemporalDurationNativeBridge,
    TemporalError, TemporalErrorKind, TemporalInputToken,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalDurationNativeBridge<JiffVerifier>`.
fn _assert_duration_bridge<T: TemporalDurationNativeBridge<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_duration_bridge::<JiffTimeBackend>;
};

fn duration_bundle_token() -> DurationSemanticBundleToken {
    let form: DurationFormValidToken = <DurationFormValid as Establish<
        TemporalInputToken,
        JiffVerifier,
    >>::establish(TemporalInputToken::new());
    <DurationSemanticBundle as Establish<DurationFormValidToken, JiffVerifier>>::establish(form)
}

#[test]
fn realize_duration_round_trips_every_whole_unit() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = DurationDescriptorBuilder::default()
        .years(1u32)
        .months(2u32)
        .weeks(3u32)
        .days(4u32)
        .hours(5u32)
        .minutes(6u32)
        .seconds(7u32)
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;

    let carrier: ProvenDurationCarrier<JiffSpan> = backend
        .exchange(ReflectedDuration::new(descriptor, duration_bundle_token()))
        .into_diagnostic()
        .wrap_err("all-whole-unit descriptor realizes to a real jiff::Span")?;
    let span = carrier.carrier().0;

    assert_eq!(span.get_years(), 1);
    assert_eq!(span.get_months(), 2);
    assert_eq!(span.get_weeks(), 3);
    assert_eq!(span.get_days(), 4);
    assert_eq!(span.get_hours(), 5);
    assert_eq!(span.get_minutes(), 6);
    assert_eq!(span.get_seconds(), 7);
    assert_eq!(span.get_nanoseconds(), 0);
    Ok(())
}

#[test]
fn realize_duration_converts_a_fractional_second() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = DurationDescriptorBuilder::default()
        .seconds(4u32)
        .fractional_component(DurationFractionDescriptor::new(
            TemporalComponent::Second,
            "5",
        ))
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;

    let carrier: ProvenDurationCarrier<JiffSpan> = backend
        .exchange(ReflectedDuration::new(descriptor, duration_bundle_token()))
        .into_diagnostic()
        .wrap_err("a fractional-second descriptor realizes to a real jiff::Span")?;
    let span = carrier.carrier().0;

    assert_eq!(span.get_seconds(), 4);
    assert_eq!(span.get_nanoseconds(), 500_000_000);
    Ok(())
}

#[test]
fn realize_duration_rejects_a_fraction_on_a_coarser_unit() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = DurationDescriptorBuilder::default()
        .years(1u32)
        .fractional_component(DurationFractionDescriptor::new(
            TemporalComponent::Year,
            "5",
        ))
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;

    let err = backend
        .exchange(ReflectedDuration::new(descriptor, duration_bundle_token()))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!(
                "jiff::Span has no fractional representation for a coarser-than-seconds unit"
            )
        })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn realize_duration_rejects_a_component_beyond_jiffs_representable_range() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    // jiff::Span::try_years's own documented max is 19,998.
    let descriptor = DurationDescriptorBuilder::default()
        .years(20_000u32)
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;

    let err: TemporalError = backend
        .exchange(ReflectedDuration::new(descriptor, duration_bundle_token()))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!("20,000 years exceeds jiff::Span's own representable range")
        })?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn duration_round_trips_through_a_real_jiff_span() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let span = jiff::Span::new()
        .days(1)
        .hours(2)
        .minutes(3)
        .seconds(4)
        .nanoseconds(250_000_000);
    let carrier = ProvenDurationCarrier::<JiffSpan>::new(JiffSpan(span), duration_bundle_token());

    let reflected: ReflectedDuration = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real jiff::Span reflects to a descriptor")?;
    let round_tripped: ProvenDurationCarrier<JiffSpan> = backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to an equivalent span")?;
    let round_tripped_span = round_tripped.carrier().0;

    assert_eq!(round_tripped_span.get_days(), span.get_days());
    assert_eq!(round_tripped_span.get_hours(), span.get_hours());
    assert_eq!(round_tripped_span.get_minutes(), span.get_minutes());
    assert_eq!(round_tripped_span.get_seconds(), span.get_seconds());
    assert_eq!(round_tripped_span.get_nanoseconds(), span.get_nanoseconds());
    Ok(())
}
