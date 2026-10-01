//! Calendar/ordinal/week-date parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    CalendarDateDescriptor, CalendarDateValidToken, OrdinalDateDescriptor, OrdinalDateValidToken,
    ReducedCalendarDateDescriptor, ReducedCalendarDateValidToken, WeekDateDescriptor,
    WeekDateValidToken,
};

/// Output sidecar for the `parse_calendar_date` exchange: [`CalendarDateDescriptor`](crate::CalendarDateDescriptor)
/// plus a token for [`CalendarDateValid`](crate::CalendarDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::CalendarDateValid", constructor = "pub")]
pub struct ParsedCalendarDate {
    #[sidecar(primary)]
    descriptor: CalendarDateDescriptor,
    #[sidecar(token)]
    token: CalendarDateValidToken,
}
/// Output sidecar for the `parse_reduced_calendar_date` exchange: [`ReducedCalendarDateDescriptor`](crate::ReducedCalendarDateDescriptor)
/// plus a token for [`ReducedCalendarDateValid`](crate::ReducedCalendarDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::ReducedCalendarDateValid", constructor = "pub")]
pub struct ParsedReducedCalendarDate {
    #[sidecar(primary)]
    descriptor: ReducedCalendarDateDescriptor,
    #[sidecar(token)]
    token: ReducedCalendarDateValidToken,
}
/// Output sidecar for the `parse_ordinal_date` exchange: [`OrdinalDateDescriptor`](crate::OrdinalDateDescriptor)
/// plus a token for [`OrdinalDateValid`](crate::OrdinalDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::OrdinalDateValid", constructor = "pub")]
pub struct ParsedOrdinalDate {
    #[sidecar(primary)]
    descriptor: OrdinalDateDescriptor,
    #[sidecar(token)]
    token: OrdinalDateValidToken,
}
/// Output sidecar for the `parse_week_date` exchange: [`WeekDateDescriptor`](crate::WeekDateDescriptor)
/// plus a token for [`WeekDateValid`](crate::WeekDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::WeekDateValid", constructor = "pub")]
pub struct ParsedWeekDate {
    #[sidecar(primary)]
    descriptor: WeekDateDescriptor,
    #[sidecar(token)]
    token: WeekDateValidToken,
}
