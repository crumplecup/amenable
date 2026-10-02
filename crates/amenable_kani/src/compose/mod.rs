//! Bounded symbolic construction for Kani-facing proof models.
//!
//! `kani::any::<T>()` is fine for leaf scalars, but it becomes a poor default
//! for recursive or heap-backed shapes such as `String`, `Vec<T>`, and nested
//! user-defined carriers: Kani then has to reason about unconstrained
//! destructor paths and collection growth, which is exactly where unbounded
//! unwinding and state blow-up start to dominate.
//!
//! `KaniCompose` gives Amenable a verifier-specific modeling surface instead:
//! small depth-indexed constructors plus a bounded `kani_any()`. It belongs in
//! `amenable_kani`, not `amenable_core`, because it is not a constitutional
//! proof role. It is a Kani-only input-construction discipline.

mod claims;
#[cfg(kani)]
mod model;
#[cfg(kani)]
mod proofs;

#[cfg(kani)]
pub use model::KaniCompose;
#[cfg(kani)]
pub(crate) use model::{kani_assume, symbolic_any};
