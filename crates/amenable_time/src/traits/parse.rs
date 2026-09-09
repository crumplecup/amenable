//! [`TemporalParser`] — raw parsing of standards-governed temporal forms
//! into neutral descriptors.
//!
//! Ported from `elicit_temporal::traits::TemporalParser`. Each method
//! takes a raw `&str` and returns just the descriptor; the proof sidecar
//! is synthesised by the blanket [`TemporalExchange`](crate::TemporalExchange)
//! impl over this trait (`src/exchange/parse.rs`), which mints the
//! output token through [`Establish`](amenable_core::Establish). A
//! backend implements the raw methods; it gets the exchange surface for
//! free.
//!
//! Phase 4 Step 1 wires only `parse_calendar_date`; the remaining 23
//! methods land in Step 2.

use crate::{CalendarDateDescriptor, TemporalResult};

/// Parse standards-governed temporal forms into neutral descriptors.
pub trait TemporalParser: Send + Sync {
    /// Parse a complete ISO 8601 calendar date (`YYYY-MM-DD` or
    /// equivalent).
    ///
    /// Normative source: ISO 8601-1:2019, 5.2.2.
    fn parse_calendar_date(&self, input: &str) -> TemporalResult<CalendarDateDescriptor>;
}
