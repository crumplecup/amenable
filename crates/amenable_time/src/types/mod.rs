//! Neutral temporal descriptors — the accord vocabulary that crosses
//! trait boundaries, ported from `elicit_temporal::types`.
//!
//! These types are pure data: a descriptor carries **no**
//! construction-time validation. A `month = 13` or a `year` that is a
//! degenerate `Default` is rejected by a proof, not a constructor guard
//! (`docs/AMENABLE_TIME_PLAN.md`, decision 5). Structs use private fields
//! with `derive_getters::Getters` accessors and a builder or
//! `derive_new::new` constructor; closed enums carry `strum::EnumIter`
//! and `derive_more::Display` per house policy.
//!
//! The `#[derive(Evidence)]` pass (making the descriptors that become a
//! `Sidecar<V>::Primary` into [`Evidence`](amenable_core::Evidence)) is
//! deferred to Phase 3/4, where the `Sidecar` wiring shows exactly which
//! descriptors need it — rather than forcing a meaningless `Default` onto
//! every data-only enum here (`docs/AMENABLE_TIME_PLAN.md`, Phase 2).

mod component;
mod date;
mod datetime;
mod duration;
mod interval;
mod recurrence;
mod season;
mod serialization;
mod set;
mod time;
mod unspecified;
mod value;
mod zone;

pub use component::{
    DurationFractionDescriptor, FractionalSecondDescriptor, QualificationPlacementDescriptor,
    QualificationScopeDescriptor, QualifiedTemporalExpressionDescriptor, TemporalComponent,
    TimeScaleUnitDescriptor, TimeScaleUnitFractionDescriptor, TimeScaleUnitValueDescriptor,
};
pub use date::{
    CalendarDateDescriptor, CenturyDescriptor, CenturyDescriptorBuilder, CompleteDateDescriptor,
    DateDescriptor, DecadeDescriptor, DecadeDescriptorBuilder, ExtendedYearBaseDescriptor,
    ExtendedYearDescriptor, ExtendedYearDescriptorBuilder, OrdinalDateDescriptor,
    ReducedCalendarDateDescriptor, WeekDateDescriptor,
};
pub use datetime::{
    DateWithShiftDescriptor, DateWithShiftDescriptorBuilder, ExplicitDateTimeDescriptor,
    ExplicitDateTimeDescriptorBuilder, ExplicitDateTimeWithShiftDescriptor,
    ExplicitDateTimeWithShiftDescriptorBuilder, LocalDateTimeDescriptor,
    LocalDateTimeDescriptorBuilder, OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder,
    TimeOfDayWithShiftDescriptor, TimeOfDayWithShiftDescriptorBuilder,
};
pub use duration::{
    DurationDescriptor, DurationDescriptorBuilder, ExplicitDurationDescriptor,
    ExplicitDurationDescriptorBuilder, ExplicitDurationRepresentationKindDescriptor,
    ExplicitDurationSemanticKindDescriptor, ExplicitDurationSignDescriptor,
};
pub use interval::{
    ExplicitTimeIntervalDescriptor, ExplicitTimeIntervalEndpointDescriptor,
    ExplicitTimeIntervalRepresentation, RecurringIntervalDescriptor,
    RecurringIntervalDescriptorBuilder, TimeIntervalDescriptor, TimeIntervalEndpoint,
    TimeIntervalRepresentation,
};
pub use recurrence::{
    DateTimeFormulaDescriptor, DateTimeFormulaEvaluationKindDescriptor,
    EligibleTimeIntervalsDescriptor, EligibleTimeIntervalsDescriptorBuilder,
    RecurringIntervalWithRepeatRuleDescriptor, RecurringIntervalWithRepeatRuleDescriptorBuilder,
    RecurringIntervalWithRepeatRuleIntervalDescriptor, RepeatRuleDescriptor,
    SelectionExpressionDescriptor, SelectionExpressionDescriptorBuilder, SelectionRuleDescriptor,
};
pub use season::{
    NamedSeasonDescriptor, QuadrimesterOfYearDescriptor, QuarterOfYearDescriptor,
    SeasonScopeDescriptor, SeasonalTemporalExpressionDescriptor,
    SeasonalTemporalExpressionDescriptorBuilder, SemestralHalfDescriptor,
    SubYearGroupingDescriptor, SubYearGroupingExpressionDescriptor,
    SubYearGroupingExpressionDescriptorBuilder,
};
pub use serialization::{
    IxdtfTimestampDescriptor, IxdtfTimestampDescriptorBuilder, PrecisionDescriptor,
    PrecisionDescriptorBuilder, RoundingModeDescriptor, SerializationProfile,
    TemporalSerializationDescriptor, TemporalSerializationDescriptorBuilder,
};
pub use set::{
    TemporalSetDescriptor, TemporalSetDescriptorBuilder, TemporalSetMemberDescriptor,
    TemporalSetSemanticsDescriptor,
};
pub use time::{
    ExplicitTimeOfDayDescriptor, ExplicitTimeOfDayDescriptorBuilder, ExplicitTimeShiftDescriptor,
    ExplicitTimeShiftDescriptorBuilder, LocalTimeDescriptor, LocalTimeDescriptorBuilder,
    LocalTimeScaleDescriptor, ReducedLocalTimeDescriptor, StandardTimeDescriptor,
    StandardTimeOfDayDescriptor, TimeDescriptor, UtcOfDayDescriptor, UtcOffsetDescriptor,
    UtcOffsetDescriptorBuilder, UtcOffsetRelationship, UtcOffsetSign, UtcTimeScaleDescriptor,
};
pub use unspecified::{
    MaskedNumericComponentDescriptor, MaskedNumericComponentDescriptorBuilder,
    UnspecifiedComponentExpressionDescriptor, UnspecifiedComponentExpressionDescriptorBuilder,
    UnspecifiedDigitDescriptor, UnspecifiedPrecisionProfileDescriptor,
    UnspecifiedTemporalShapeDescriptor,
};
pub use value::{
    ExplicitTemporalFormDescriptor, ExplicitTemporalFormDescriptorBuilder,
    ExplicitTemporalValueDescriptor, GroupedTimeScaleUnitDescriptor,
    GroupedTimeScaleUnitDescriptorBuilder, QualifiedOrBareTemporalValueDescriptor,
    QualifiedTemporalValueDescriptor, TemporalValueDescriptor,
};
pub use zone::{
    IxdtfAnnotationDescriptor, IxdtfAnnotationDescriptorBuilder, IxdtfCalendarAnnotationDescriptor,
    IxdtfCalendarAnnotationDescriptorBuilder, IxdtfTimeZoneAnnotationDescriptor,
    LocalTimeZoneResolutionAuthorityDescriptor, NamedTimeZoneDescriptor,
    NamedTimeZoneDescriptorBuilder, ZoneAmbiguityResolutionDescriptor, ZoneGapResolutionDescriptor,
    ZonedDateTimeDescriptor, ZonedDateTimeDescriptorBuilder,
};
