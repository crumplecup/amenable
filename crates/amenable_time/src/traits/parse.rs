//! [`TemporalParser`] — the parse half of the temporal seam, ported from
//! `elicit_temporal::traits::TemporalParser`.
//!
//! The contract *is* the exchanges: `TemporalParser<V>` is a bundle of
//! `Exchange<RawInput, ParsedX, V>` supertraits, so a type implements it
//! exactly when it is all 24 parse exchanges for verifier `V`. The
//! `where <Valid>: Witness<V>` bounds are the honest precondition —
//! `TemporalParser<KaniVerifier>` only makes sense once Kani has a proof
//! for each temporal validity proposition (each output sidecar's
//! `Proposition`). A backend (`Jiff`, …) writes the inherent parse
//! methods and lets `#[amenable_derive::capture_exchange_body]` generate
//! each `impl<V> Exchange<RawInput, ParsedX, V> for Jiff`; the blanket
//! below then gives it `TemporalParser<V>` for free.
//!
//! Phase 4 Step 1 bundles only the `parse_calendar_date` edge; Step 2
//! adds the other 23.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{CalendarDateValid, ParsedCalendarDate, RawInput, TemporalError};

/// A backend that provides every standards-governed temporal parse as an
/// [`Exchange`], for verifier `V`.
pub trait TemporalParser<V: Verifier>:
    Send + Sync + Exchange<RawInput, ParsedCalendarDate, V, Error = TemporalError>
where
    CalendarDateValid: Witness<V>,
{
}

impl<T, V> TemporalParser<V> for T
where
    V: Verifier,
    CalendarDateValid: Witness<V>,
    T: Send + Sync + Exchange<RawInput, ParsedCalendarDate, V, Error = TemporalError>,
{
}
