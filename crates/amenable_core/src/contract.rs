//! Explicit backend-facing contract fragments for named proof tokens.

use crate::{Verifier, Witness};

/// Names the postcondition a piece of evidence guarantees, in backend `V`'s
/// own contract syntax.
///
/// Bounding on [`Witness<V>`] is the same compile-time obligation
/// [`Establish`](crate::Establish) already rests on: a type cannot claim to
/// ensure something in backend `V`'s syntax unless a matching `Witness<V>`
/// impl — naming which proof backs it — exists alongside it. The fragment
/// itself is a `&'static str`, not a `TokenStream`: this matches how proof
/// content already travels through this codebase (`CheckedProof::claim`,
/// `amenable_derive::harness!`'s captured source), so composing fragments
/// later is plain string work, not a second representation to reconcile.
pub trait Ensures<V: Verifier>: Witness<V> {
    /// The postcondition fragment, written in backend `V`'s native syntax.
    fn ensures() -> &'static str;
}

/// Names the precondition a piece of evidence demands, in backend `V`'s own
/// contract syntax. See [`Ensures`] for the shape and rationale; this is its
/// precondition-facing counterpart.
pub trait Requires<V: Verifier>: Witness<V> {
    /// The precondition fragment, written in backend `V`'s native syntax.
    fn requires() -> &'static str;
}
