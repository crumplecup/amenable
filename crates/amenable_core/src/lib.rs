//! Constitutional trait family for lawful proof-carrying software structure.
//!
//! `amenable_core` defines the roles and admissibility criteria governing a
//! proof economy: which types are permitted to serve as trusted roots, which
//! types may count as derived evidence, which exchanges are lawful, and
//! which workflows are closed under those exchanges. It is dependency-light
//! by design and does not depend on any downstream proof-carrying framework.
//!
//! This crate holds the core roles, plus one exception: the canonical
//! `Stoplight` worked example's bare evidence markers (`Green`/`Yellow`/
//! `Red`, see `stoplight.rs`'s own doc comment for why they live here
//! rather than in a verifier backend crate). Traits that must be
//! implemented directly on foreign standard-library types (which Rust's
//! orphan rules require to live in the crate that defines the trait) live
//! in dedicated sibling crates instead — `amenable_std` for `RustStdType`,
//! `amenable_code` for `Code`. Users should generally depend on the
//! top-level `amenable` facade crate, which re-exports this crate's
//! family alongside its siblings, rather than depending on `amenable_core`
//! directly.
//!
//! See `docs/AMENABLE_PLAN.md` and `amenable.md` (repository root) for
//! the full design rationale.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod calculation;
mod cert;
mod chain;
mod contract;
mod evidence;
mod exchange;
mod link;
mod provenance;
mod roles;
mod state;
mod state_machine;
mod stoplight;
mod verifier;
#[cfg(feature = "verus")]
mod verus_carrier;
mod witness;

pub use calculation::{Calculation, CarriesToken};
pub use cert::{Certificate, Registry, RegistryReport};
pub use chain::{
    ChainError, ChainGap, ChainNode, ProofChainReport, proof_chain, proof_chain_for_verifiers,
};
pub use contract::{Ensures, Requires};
pub use evidence::Evidence;
pub use exchange::{Establish, Exchange, ProofToken, Sidecar};
pub use link::{
    ContractRecord, EvidenceLink, ExchangeEdgeRecord, ProofRecord, ProofTokenMintRecord,
};
pub use provenance::{MetadataEntry, OwnedProvenanceReport, Provenance, ProvenanceReport};
pub use roles::{AsStandard, Standard};
pub use state::State;
pub use state_machine::{StateMachine, Transition, TransitionAudit};
pub use stoplight::{Green, Red, Yellow};
pub use verifier::Verifier;
#[cfg(feature = "verus")]
pub use verus_carrier::predicate_body as verus_predicate_body;
#[cfg(feature = "verus")]
pub use verus_carrier::{find_fn as verus_find_fn, literal_clauses as verus_literal_clauses};
#[cfg(feature = "verus")]
pub use verus_carrier::{param_name as verus_param_name, walk_tokens as verus_walk_tokens};
pub use witness::{
    ClassifiedWitness, Witness, WitnessArtifact, WitnessArtifactMember, WitnessArtifactNode,
    WitnessArtifactShape, WitnessArtifactVariant, WitnessExportRecord, WitnessExportSnapshot,
    WitnessModulePath, WitnessSupportKind, WitnessSupportSummary, witness_exports,
};

#[doc(hidden)]
pub use inventory as __inventory;
