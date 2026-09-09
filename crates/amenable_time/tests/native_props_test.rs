//! `native_props` — a fake backend implements the leaf carrier families
//! and the aggregate blanket traits (`TemporalSpanProps`,
//! `TemporalExtensionProps`, `TemporalNativeProps`) fall out for free.
//! Every associated type is `: Evidence`, so a carrier can ride a
//! `ProvenTemporalCarrier` sidecar.

use amenable_core::Evidence;
use amenable_time::{
    TemporalCivilProps, TemporalDateTimeFormulaProps, TemporalDurationProps,
    TemporalExplicitDurationProps, TemporalExplicitTemporalFormProps,
    TemporalExplicitTimeIntervalProps, TemporalGroupedTimeScaleUnitProps, TemporalInstantProps,
    TemporalNativeProps, TemporalQualifiedTemporalValueProps, TemporalRecurringIntervalProps,
    TemporalSetProps, TemporalSpanProps, TemporalTimeIntervalProps, TemporalZoneProps,
};

/// One thin `#[derive(Evidence)]` newtype stands in for every native
/// carrier type in this fake backend.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
struct FakeCarrier;

struct FakeBackend;

macro_rules! civil {
    ($($assoc:ident),+) => { $(type $assoc = FakeCarrier;)+ };
}

impl TemporalCivilProps for FakeBackend {
    civil!(
        CalendarDate,
        ReducedCalendarDate,
        OrdinalDate,
        WeekDate,
        LocalTime,
        ReducedLocalTime,
        LocalDateTime
    );
}
impl TemporalInstantProps for FakeBackend {
    type UtcOffset = FakeCarrier;
    type OffsetDateTime = FakeCarrier;
    type Instant = FakeCarrier;
}
impl TemporalZoneProps for FakeBackend {
    type NamedTimeZone = FakeCarrier;
    type ZonedDateTime = FakeCarrier;
}
impl TemporalDurationProps for FakeBackend {
    type Duration = FakeCarrier;
}
impl TemporalTimeIntervalProps for FakeBackend {
    type TimeInterval = FakeCarrier;
}
impl TemporalRecurringIntervalProps for FakeBackend {
    type RecurringInterval = FakeCarrier;
}
impl TemporalQualifiedTemporalValueProps for FakeBackend {
    type QualifiedTemporalValue = FakeCarrier;
}
impl TemporalExplicitTemporalFormProps for FakeBackend {
    type ExplicitTemporalForm = FakeCarrier;
}
impl TemporalExplicitDurationProps for FakeBackend {
    type ExplicitDuration = FakeCarrier;
}
impl TemporalExplicitTimeIntervalProps for FakeBackend {
    type ExplicitTimeInterval = FakeCarrier;
}
impl TemporalGroupedTimeScaleUnitProps for FakeBackend {
    type GroupedTimeScaleUnit = FakeCarrier;
}
impl TemporalSetProps for FakeBackend {
    type TemporalSet = FakeCarrier;
}
impl TemporalDateTimeFormulaProps for FakeBackend {
    type DateTimeFormula = FakeCarrier;
}

#[test]
fn the_aggregate_blankets_compose_from_the_leaf_families() {
    amenable_core::init_tracing();

    fn assert_span<T: TemporalSpanProps>() {}
    fn assert_native<T: TemporalNativeProps>() {}
    assert_span::<FakeBackend>();
    assert_native::<FakeBackend>();

    // and the associated types really are `Evidence`
    fn assert_carrier<T>()
    where
        T: TemporalCivilProps,
        T::LocalDateTime: Evidence,
    {
    }
    assert_carrier::<FakeBackend>();
    assert!(<FakeCarrier as Evidence>::is_root());
}
