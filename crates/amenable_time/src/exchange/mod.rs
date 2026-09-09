//! The temporal exchange surface — every `elicit_temporal` trait method
//! re-expressed with the [`Exchange`](amenable_core::Exchange) shape: a
//! proven [`Sidecar`](amenable_core::Sidecar) in, a proven `Sidecar` out.
//!
//! - [`markers`] / [`tokens`] — the boundary `Evidence` markers and the
//!   root input token.
//! - [`establish`] — the output tokens and their
//!   [`Establish`](amenable_core::Establish) edges (`<P as Establish<
//!   TemporalInputToken, V>>::establish` is the one lawful mint path).
//! - [`wrapper`] — [`TemporalExchange`], the backend-neutral exchange
//!   trait (`Exchange`-shaped; a standalone trait only because the orphan
//!   rule forbids the blanket `Exchange` impl here).
//! - [`sidecars`] — [`RawInput`], the shared input sidecar.
//! - [`parse`] — the per-method output sidecars + the `TemporalExchange`
//!   blanket over [`TemporalParser`](crate::TemporalParser).

mod establish;
mod markers;
mod parse;
mod sidecars;
mod tokens;
mod wrapper;

pub use establish::CalendarDateValidToken;
pub use markers::{RawTemporalText, TemporalInputReceived};
pub use parse::ParsedCalendarDate;
pub use sidecars::RawInput;
pub use tokens::TemporalInputToken;
pub use wrapper::TemporalExchange;
