//! Parser exchange output sidecars — one `#[derive(Sidecar)]` struct per
//! [`TemporalParser`](crate::TemporalParser) method (the `elicit_temporal`
//! return tuple, named and given the sidecar shape).
//!
//! The `Exchange<RawInput, ParsedX, V>` impls themselves live **in the
//! backend crate** — a downstream `Jiff` writes one inherent
//! `fn parse_calendar_date(&self, RawInput) -> Result<ParsedCalendarDate,
//! TemporalError>` and `#[amenable_derive::capture_exchange_body]`
//! generates its `impl<V> Exchange<..> for Jiff`. `amenable_time` can't
//! provide those (orphan rule: `Exchange` is foreign, the backend `Self`
//! is uncovered). What it provides is the sidecar types, the `Establish`
//! edges, and the [`TemporalParser<V>`](crate::TemporalParser) bundle.
//!
//! Phase 4 Step 1 wires only `parse_calendar_date`; Step 2 adds the rest.

use crate::{CalendarDateDescriptor, CalendarDateValidToken};

/// Output sidecar for the calendar-date parse exchange: the neutral
/// [`CalendarDateDescriptor`] plus a token for
/// [`CalendarDateValid`](crate::CalendarDateValid). `#[sidecar(primary)]`
/// is the data, `#[sidecar(token)]` is the proof — the `elicit_temporal`
/// `(descriptor, Established<CalendarDateValid>)` tuple, named.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CalendarDateValid", constructor = "pub")]
pub struct ParsedCalendarDate {
    #[sidecar(primary)]
    descriptor: CalendarDateDescriptor,
    #[sidecar(token)]
    token: CalendarDateValidToken,
}

impl ParsedCalendarDate {
    /// Borrow the parsed calendar-date descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &CalendarDateDescriptor {
        &self.descriptor
    }
}
