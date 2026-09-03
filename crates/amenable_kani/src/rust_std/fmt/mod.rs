//! `KaniWitness` impls for `core::fmt`.
//!
//! `Error` is an opaque unit-like marker signaling that a formatting trait
//! impl failed to write to its `Formatter` — no accessors beyond `Debug`/
//! `Display`, nothing to build and check. It stays at the trusted
//! disposition.
//!
//! The direct rendering paths for `Arguments`, `FromFn`, and the `Debug*`
//! builders time out under Kani's formatting machinery. Production proofs
//! for those shapes therefore use an Amenable-owned formatter model
//! instead; `Alignment` and `Formatter` remain on the direct observable
//! std path.

mod builders;
mod direct;

pub use builders::RenderedKindMatchesTheBuildingOperation;
