//! Constitutional trait family for lawful proof-carrying software structure.
//!
//! `amenable` defines the roles and admissibility criteria governing a proof
//! economy: which types are permitted to serve as trusted roots, which types
//! may count as derived evidence, which exchanges are lawful, and which
//! workflows are closed under those exchanges. It is dependency-light by
//! design and does not depend on any downstream proof-carrying framework.
//!
//! See `AMENABLE_PLAN.md` and `amenable.md` in the repository root for the
//! full design rationale.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod evidence;
mod exchange;
mod provenance;
mod roles;
mod rust_std;
mod state_machine;
mod verifier;
mod witness;

pub use evidence::Evidence;
pub use exchange::{Establish, Exchange, ProofToken, Sidecar};
pub use provenance::{MetadataEntry, Provenance};
pub use roles::{AsObjective, AsStandard, Objective, Standard};
pub use rust_std::RustStdType;
pub use state_machine::{Amenable, StateMachine};
pub use verifier::{
    CreusotVerifier, CreusotVerifierMetadata, KaniVerifier, KaniVerifierMetadata, Verifier,
    VerusVerifier, VerusVerifierMetadata,
};
pub use witness::{Witness, WitnessSource, Witnessed};
