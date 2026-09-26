//! `types` — spot-checks the ported descriptor vocabulary: private
//! fields + `derive_getters` accessors, builder / `derive_new`
//! construction, and the closed-enum `strum::EnumIter` + `Display`
//! surface. Descriptors carry no construction-time validation (a
//! degenerate value is rejected by a proof, not a constructor guard).

use amenable_time::{
    CalendarDateDescriptor, DurationDescriptor, DurationDescriptorBuilder,
    LocalTimeDescriptorBuilder, PrecisionDescriptorBuilder, SerializationProfile,
    TemporalComponent, UtcOffsetRelationship, UtcOffsetSign,
};
use miette::{IntoDiagnostic, WrapErr};
use strum::IntoEnumIterator;

#[test]
fn derive_new_struct_exposes_copy_getters() {
    amenable_core::init_tracing();

    let date = CalendarDateDescriptor::new(2026, 9, 8);

    assert_eq!(date.year(), 2026);
    assert_eq!(date.month(), 9);
    assert_eq!(date.day(), 8);
}

#[test]
fn builder_defaults_the_optional_fields() -> miette::Result<()> {
    amenable_core::init_tracing();

    let time = LocalTimeDescriptorBuilder::default()
        .hour(23u8)
        .minute(59u8)
        .second(60u8)
        .build()
        .into_diagnostic()
        .wrap_err("every required field is set")?;

    assert_eq!(time.hour(), 23);
    assert!(time.fractional_second().is_none());
    Ok(())
}

#[test]
fn duration_builder_is_all_optional() -> miette::Result<()> {
    amenable_core::init_tracing();

    let empty = DurationDescriptorBuilder::default()
        .build()
        .into_diagnostic()
        .wrap_err("all fields default")?;
    assert_eq!(empty, DurationDescriptor::default());

    let a_week = DurationDescriptorBuilder::default()
        .weeks(1u32)
        .build()
        .into_diagnostic()
        .wrap_err("weeks is the only set field")?;
    assert_eq!(a_week.weeks(), 1);
    assert_eq!(a_week.days(), 0);
    Ok(())
}

#[test]
fn precision_builder_carries_the_smallest_component() -> miette::Result<()> {
    amenable_core::init_tracing();

    let precision = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(9u8)
        .build()
        .into_diagnostic()
        .wrap_err("component set, rounding mode defaulted")?;

    assert_eq!(precision.smallest_component(), TemporalComponent::Second);
    assert_eq!(precision.fractional_digits(), Some(9));
    assert!(precision.rounding_mode().is_none());
    Ok(())
}

#[test]
fn closed_enums_iterate_and_display() {
    amenable_core::init_tracing();

    assert_eq!(TemporalComponent::iter().count(), 8);
    assert_eq!(TemporalComponent::Year.to_string(), "year");

    assert_eq!(SerializationProfile::iter().count(), 4);
    assert_eq!(SerializationProfile::Rfc3339.to_string(), "RFC 3339");

    assert_eq!(UtcOffsetSign::default(), UtcOffsetSign::Positive);
    assert_eq!(UtcOffsetSign::Negative.to_string(), "-");
    assert_eq!(
        UtcOffsetRelationship::default(),
        UtcOffsetRelationship::Known
    );
}
