//! Constitutional trait family for lawful proof-carrying software structure.
//!
//! `amenable_core` defines the roles and admissibility criteria governing a
//! proof economy: which types are permitted to serve as trusted roots, which
//! types may count as derived evidence, which exchanges are lawful, and
//! which workflows are closed under those exchanges. It is dependency-light
//! by design and does not depend on any downstream proof-carrying framework.
//!
//! This crate holds only the core roles. Traits that must be implemented
//! directly on foreign standard-library types (which Rust's orphan rules
//! require to live in the crate that defines the trait) live in dedicated
//! sibling crates instead — `amenable_std` for `RustStdType`, `amenable_code`
//! for `Code`. Users should generally depend on the top-level `amenable`
//! facade crate, which re-exports this crate's family alongside its
//! siblings, rather than depending on `amenable_core` directly.
//!
//! See `AMENABLE_PLAN.md` and `amenable.md` in the repository root for the
//! full design rationale.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cert;
mod evidence;
mod exchange;
mod provenance;
mod roles;
mod state_machine;
mod verifier;
mod witness;

pub use cert::{Certificate, Registry, RegistryReport};
pub use evidence::Evidence;
pub use exchange::{Establish, Exchange, ProofToken, Sidecar};
pub use provenance::{MetadataEntry, OwnedProvenanceReport, Provenance, ProvenanceReport};
pub use roles::{AsStandard, Standard};
pub use state_machine::{Amenable, StateMachine};
pub use verifier::{
    CreusotVerifier, CreusotVerifierMetadata, KaniVerifier, KaniVerifierMetadata, Verifier,
    VerusVerifier, VerusVerifierMetadata,
};
pub use witness::{Witness, WitnessSource, Witnessed};
