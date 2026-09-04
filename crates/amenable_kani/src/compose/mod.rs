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

#[cfg(kani)]
mod model;
#[cfg(kani)]
mod proofs;
#[cfg(kani)]
pub use model::KaniCompose;
#[cfg(kani)]
pub(crate) use model::{kani_assume, symbolic_any};

// Registered unconditionally (not inside `#[cfg(kani)] mod proofs` below)
// so `amenable dump-registry`'s ordinary, non-Kani build still sees these
// `ContractRecord`s -- the self-test proof bodies that call them are
// Kani-only (the whole module is `#[cfg(kani)]`-gated, since the derived
// `KaniCompose` impls it exercises call `kani::any()`/`kani::assume`
// directly, and this crate has no unconditional `kani` dependency), but
// the named claim they call into is not. Each carries the "trusted"
// disposition (chain-derived provenance only, no dedicated harness of its
// own) rather than pointing at one of the three self-test harnesses that
// use it, since those harnesses aren't nameable from unconditional code.
use crate::rust_std::{bridge_kani_witness, kani_ensures};

macro_rules! impl_compose_claim_witness {
    ($ty:ty) => {
        impl crate::KaniWitness for $ty {
            type SupportingEvidence = Self;
            type ProofArtifact = amenable_std::RustStdProvenance;

            fn proof() -> Self::ProofArtifact {
                amenable_core::Evidence::audit(
                    &<Self::SupportingEvidence as amenable_core::Evidence>::basis(),
                )
            }
        }

        bridge_kani_witness!($ty);
    };
}

impl_compose_claim_witness!(amenable_std::ComposeDepthZeroIsEmpty);
impl_compose_claim_witness!(amenable_std::ComposeFieldPresenceTracksDepth);
impl_compose_claim_witness!(amenable_std::ComposeAnyLengthIsBounded);
impl_compose_claim_witness!(amenable_std::ComposeArrayLengthIsFixed);

kani_ensures!(
    amenable_std::ComposeDepthZeroIsEmpty,
    "amenable_std::ComposeDepthZeroIsEmpty",
    usize,
    |len| len == 0
);

kani_ensures!(
    amenable_std::ComposeFieldPresenceTracksDepth,
    "amenable_std::ComposeFieldPresenceTracksDepth",
    (bool, usize),
    |(is_some, depth)| is_some == (depth > 0)
);

kani_ensures!(
    amenable_std::ComposeAnyLengthIsBounded,
    "amenable_std::ComposeAnyLengthIsBounded",
    (usize, usize),
    |(actual, bound)| actual <= bound
);

kani_ensures!(
    amenable_std::ComposeArrayLengthIsFixed,
    "amenable_std::ComposeArrayLengthIsFixed",
    (usize, usize),
    |(actual, expected)| actual == expected
);
