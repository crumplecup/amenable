//! UTC-offset/named-zone/zone-transition family aggregate proof propositions.

mod identity;
mod suffix;
mod transition;
mod zoned_date;

pub use identity::{
    NamedTimeZoneIdentityValid, NamedTimeZoneInterpretationTracksTzdbRevision,
    OffsetConsistentWithNamedZone, OffsetOnlyZoneSemanticsLimited, UtcOffsetKnown, UtcOffsetValid,
    UtcTimeScaleValid,
};
pub use suffix::{
    CriticalTimeZoneInconsistencyHandlingValid, ElectiveTimeZoneInconsistencyHandlingValid,
    OffsetTimeZoneAnnotationConsistentWithTimestamp, ZuluTimeZoneInconsistencyAvoidanceValid,
};
pub use transition::{
    LocalDateTimeDoesNotIdentifyFixedInstant, ZoneTransitionAmbiguitySemanticsValid,
    ZoneTransitionGapSemanticsValid, ZoneTransitionResolutionAuthorityValid,
    ZonedDateTimeHasNamedZone,
};
pub use zoned_date::ZonedDateValid;
