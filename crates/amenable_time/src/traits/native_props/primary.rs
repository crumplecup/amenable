//! Civil/instant/zone native-carrier trait families.
//!
//! Associated native-carrier trait families for temporal backends --
//! ported from `elicit_temporal::traits::native_props`, one-to-one.
//!
//! Each backend exposes its real upstream carrier types here instead of
//! the framework inventing replacement runtime values. The amenable-flavored
//! change: every associated type is bound `: Evidence`, so a native
//! carrier can ride as the [`Sidecar::Primary`](amenable_core::Sidecar)
//! of a [`ProvenTemporalCarrier`](crate::ProvenTemporalCarrier) -- a thin
//! `#[derive(Evidence)]` newtype over the upstream type, not a synthetic
//! replacement.

use amenable_core::Evidence;

/// Civil temporal carriers that do not by themselves identify a fixed instant.
///
/// These associated types cover date and local-time families whose semantics
/// remain civil or local until additional zone or offset law is applied.
pub trait TemporalCivilProps {
    /// Backend-native calendar date carrier.
    type CalendarDate: Evidence;

    /// Backend-native reduced-precision calendar date carrier.
    type ReducedCalendarDate: Evidence;

    /// Backend-native ordinal date carrier.
    type OrdinalDate: Evidence;

    /// Backend-native week date carrier.
    type WeekDate: Evidence;

    /// Backend-native local time-of-day carrier.
    type LocalTime: Evidence;

    /// Backend-native reduced local time-of-day carrier.
    type ReducedLocalTime: Evidence;

    /// Backend-native combined local date-time carrier.
    type LocalDateTime: Evidence;
}
/// Instant-bearing temporal carriers.
///
/// `Instant` is kept distinct from `OffsetDateTime` because some upstream
/// libraries expose an absolute timestamp type that is not identical to an
/// offset-bearing civil representation.
pub trait TemporalInstantProps {
    /// Backend-native UTC offset carrier.
    type UtcOffset: Evidence;

    /// Backend-native offset date-time carrier.
    type OffsetDateTime: Evidence;

    /// Backend-native fixed-instant carrier.
    type Instant: Evidence;
}
/// Named-zone and zone-attached temporal carriers.
///
/// This family remains separate from the civil and instant families because
/// some backends lawfully support offsets but do not have an upstream named-
/// zone carrier.
pub trait TemporalZoneProps {
    /// Backend-native named time zone carrier.
    type NamedTimeZone: Evidence;

    /// Backend-native named-zone-attached date-time carrier.
    type ZonedDateTime: Evidence;
}
