//! Explicit backend-facing contract fragments for named proof tokens.

use crate::{Verifier, Witness};

/// Names the postcondition a piece of evidence guarantees, checked in
/// backend `V`'s own way.
///
/// `Bound` and `Input` are chosen per impl, not fixed by the trait — this
/// is what lets the guarantee be as strong as each backend actually
/// allows, instead of flattening every backend down to the weakest common
/// representation. A backend whose bound is plain executable Rust (Kani)
/// sets `Bound = bool` and `Input` to the value being checked: `ensures()`
/// *is* the real check, called directly at the proof site, not text a
/// human promises matches it. A backend whose bound can only be expressed
/// in its own non-Rust spec syntax (Creusot's Pearlite, Verus's DSL) sets
/// `Bound = &'static str` and `Input = ()` — the best available
/// representation there, not a second-class case of the same shape.
///
/// Bounding on [`Witness<V>`] is the same compile-time obligation
/// [`Establish`](crate::Establish) already rests on: a type cannot claim to
/// ensure something in backend `V`'s syntax unless a matching `Witness<V>`
/// impl — naming which proof backs it — exists alongside it.
pub trait Ensures<V: Verifier>: Witness<V> {
    /// What `ensures` checks against.
    type Input;
    /// The verifier's own representation of the bound.
    type Bound;

    /// Evaluate the postcondition, in backend `V`'s own way.
    fn ensures(input: Self::Input) -> Self::Bound;
}

/// Names the precondition a piece of evidence demands, checked in backend
/// `V`'s own way. See [`Ensures`] for the shape and rationale; this is its
/// precondition-facing counterpart.
pub trait Requires<V: Verifier>: Witness<V> {
    /// What `requires` checks against.
    type Input;
    /// The verifier's own representation of the bound.
    type Bound;

    /// Evaluate the precondition, in backend `V`'s own way.
    fn requires(input: Self::Input) -> Self::Bound;
}
