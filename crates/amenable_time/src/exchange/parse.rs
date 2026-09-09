//! Parser exchange surface — one `#[derive(Sidecar)]` output struct per
//! [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named and given the sidecar shape),
//! and the blanket [`TemporalExchange`](crate::TemporalExchange) impl
//! over `TemporalParser` that produces it.
//!
//! Phase 4 Step 1 wires only `parse_calendar_date`; Step 2 adds the rest
//! (and folds the multi-proof methods' 2–4 proofs into one per-method
//! composite proposition).

use amenable_core::{Establish, Sidecar, Verifier, Witness};

use crate::{
    CalendarDateDescriptor, CalendarDateValid, CalendarDateValidToken, RawInput, TemporalError,
    TemporalExchange, TemporalInputToken, TemporalParser,
};

/// Output sidecar for [`TemporalParser::parse_calendar_date`]: the neutral
/// [`CalendarDateDescriptor`] plus a token for [`CalendarDateValid`].
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CalendarDateValid", constructor = "pub(crate)")]
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

impl<T, V> TemporalExchange<RawInput, ParsedCalendarDate, V> for T
where
    T: TemporalParser,
    V: Verifier,
    CalendarDateValid: Witness<V>,
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedCalendarDate, TemporalError> {
        let descriptor = self.parse_calendar_date(input.as_str())?;
        let credential: TemporalInputToken = <RawInput as Sidecar<V>>::sidecar(&input);
        let token = <CalendarDateValid as Establish<TemporalInputToken, V>>::establish(credential);
        Ok(ParsedCalendarDate::new(descriptor, token))
    }
}
