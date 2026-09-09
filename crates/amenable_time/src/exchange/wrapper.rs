//! [`TemporalExchange`] — the exchange interface for the temporal seam.
//!
//! Structurally identical to [`Exchange`](amenable_core::Exchange): a
//! proven [`Sidecar`](amenable_core::Sidecar) in, a proven `Sidecar` out,
//! one `exchange` method. It is a *separate* trait, not `: Exchange`,
//! only because the orphan rule forbids `amenable_time` from writing the
//! `impl<T: TemporalParser, V> Exchange<RawInput, …, V> for T` blanket
//! (`T` is an uncovered parameter before the first local type). A
//! concrete backend type still gets a real `amenable_core::Exchange`
//! impl — that one is legal — but the reusable, backend-neutral wiring
//! lives here, blanket-implemented over the parser/formatter traits.

use amenable_core::{Sidecar, Verifier};

/// A lawful proof-bearing exchange over the temporal seam: consume a
/// proven input sidecar, produce a proven output sidecar.
pub trait TemporalExchange<Input, Output, V>
where
    Input: Sidecar<V>,
    Output: Sidecar<V>,
    V: Verifier,
{
    /// Error surface for a failed exchange.
    type Error;

    /// Perform the exchange, consuming a proven input sidecar and
    /// producing a proven output sidecar.
    fn exchange(&self, input: Input) -> Result<Output, Self::Error>;
}
