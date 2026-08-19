//! Object-safe reporting facade over a state's real evidence chain.

use crate::{Evidence, Verifier, Witness};

/// A state within a closed, proof-bearing state machine.
///
/// Deliberately thin: `Evidence` and `Witness<V>` are not object-safe
/// (`Witness::proof()` has no `self` receiver at all, and both traits
/// return associated types from their methods), so `State<V>` cannot be a
/// literal `trait State<V>: Evidence + Witness<V>` — that would make `dyn
/// State<V>` itself uncompilable. Instead, `State<V>` is a narrow,
/// `self`-receiver, owned-return facade with a blanket impl bounded on
/// the real traits, giving `Vec<Box<dyn State<V>>>` heterogeneous
/// collections over otherwise-unrelated state types while the compiler
/// still enforces that every implementor is a real, checked state — the
/// blanket impl is the enforcement, not a separate assertion.
///
/// The bound is `Evidence + Witness<V>`, not `Evidence + Witness<V> +
/// Provenance` — confirmed precise, not `Provenance`'s own metadata-
/// reporting facet. `Sidecar<V>::Proposition` is already bounded exactly
/// `Evidence + Witness<V>`, so every proposition flowing through a real
/// `Exchange` today already satisfies `State<V>`, with no new impl work
/// anywhere: `Green`/`Yellow`/`Red`, `Pending`/`Validated`/`Committed`/
/// `Rejected<..>`, and every `amenable_gaap` contract type all qualify
/// the moment this blanket impl exists.
pub trait State<V: Verifier> {
    /// This state's own type name, for reporting.
    fn type_name(&self) -> &'static str;

    /// Whether this state is a root of its own evidence chain.
    fn is_root(&self) -> bool;
}

impl<V, T> State<V> for T
where
    V: Verifier,
    T: Evidence + Witness<V>,
{
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn is_root(&self) -> bool {
        T::is_root()
    }
}
