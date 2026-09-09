//! The temporal exchange surface — every `elicit_temporal` trait method
//! re-expressed as an [`Exchange`](amenable_core::Exchange): a proven
//! [`Sidecar`](amenable_core::Sidecar) in, a proven `Sidecar` out.
//!
//! `amenable_time` owns the *shape* — the sidecar types, the boundary
//! tokens, the [`Establish`](amenable_core::Establish) edges, and the
//! [`TemporalParser<V>`](crate::TemporalParser) bundle (whose contract
//! *is* "be all 24 parse exchanges"). The `Exchange` **impls** live in
//! the backend crate: a downstream `Jiff` writes one inherent
//! `fn parse_calendar_date(&self, RawInput) -> Result<ParsedCalendarDate,
//! TemporalError>` per method and `#[amenable_derive::capture_exchange_body]`
//! generates `impl<V> Exchange<..> for Jiff`. `amenable_time` cannot
//! provide those — the orphan rule forbids `impl<T: …, V> Exchange<
//! RawInput, …, V> for T` (foreign trait, uncovered `Self`).
//!
//! - [`markers`] / [`tokens`] — boundary `Evidence` markers, root input token.
//! - [`establish`] — the output tokens and their `Establish` edges.
//! - [`sidecars`] — [`RawInput`], the shared input sidecar.
//! - [`parse`] — the per-method output sidecars.

mod establish;
mod markers;
mod parse;
mod sidecars;
mod tokens;

pub use establish::CalendarDateValidToken;
pub use markers::{RawTemporalText, TemporalInputReceived};
pub use parse::ParsedCalendarDate;
pub use sidecars::RawInput;
pub use tokens::TemporalInputToken;
