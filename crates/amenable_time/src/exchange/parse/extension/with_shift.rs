//! Explicit-form date/time-of-day-with-shift parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    DateWithShiftDescriptor, DateWithShiftValidToken, TimeOfDayWithShiftDescriptor,
    TimeOfDayWithShiftValidToken,
};

/// Output sidecar for the `parse_date_with_shift` exchange: [`DateWithShiftDescriptor`](crate::DateWithShiftDescriptor)
/// plus a token for [`DateWithShiftValid`](crate::DateWithShiftValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::DateWithShiftValid", constructor = "pub")]
pub struct ParsedDateWithShift {
    #[sidecar(primary)]
    descriptor: DateWithShiftDescriptor,
    #[sidecar(token)]
    token: DateWithShiftValidToken,
}
/// Output sidecar for the `parse_time_of_day_with_shift` exchange: [`TimeOfDayWithShiftDescriptor`](crate::TimeOfDayWithShiftDescriptor)
/// plus a token for [`TimeOfDayWithShiftValid`](crate::TimeOfDayWithShiftValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TimeOfDayWithShiftValid", constructor = "pub")]
pub struct ParsedTimeOfDayWithShift {
    #[sidecar(primary)]
    descriptor: TimeOfDayWithShiftDescriptor,
    #[sidecar(token)]
    token: TimeOfDayWithShiftValidToken,
}
