//! Constitutional trait family for lawful proof-carrying software structure.
//!
//! `amenable` is the top-level facade over the `amenable_*` crate family: it
//! re-exports the core constitutional roles from `amenable_core` alongside
//! sibling crates such as `amenable_std`, so most users depend on this one
//! crate rather than assembling the family themselves. This is the single
//! sanctioned exception to the workspace's "no re-exports between crates"
//! rule — see `CLAUDE.md`'s Workspace Organization section.
//!
//! Crates that are themselves part of the family (`amenable_kani`,
//! `amenable_creusot`, `amenable_code`, and `amenable_std` itself) depend
//! on `amenable_core` directly, never on this facade, to avoid a circular
//! dependency. `amenable_verus` is the one exception: Verus never resolves
//! `Cargo.toml`, so it depends on nothing from this workspace at all (not
//! even `amenable_core`) — see `amenable_std::verus_witness`'s doc comment
//! for the full split rationale. This facade re-exports Verus's witness
//! types from `amenable_std` (where they now live) instead.
//!
//! See `docs/AMENABLE_PLAN.md` and `amenable.md` (repository root) for
//! the full design rationale.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "creusot")]
mod creusot_export;
mod error;
pub mod paths;
#[cfg(feature = "verus")]
mod verus_exchange_export;
#[cfg(feature = "verus")]
mod verus_export;
#[cfg(feature = "verus")]
mod verus_gaap_tokens_export;

pub mod assessment;
pub mod gallery;
pub mod kani;

#[cfg(feature = "creusot")]
pub use creusot_export::write_creusot_exchange_companions;
pub use error::{
    AmenableError, AmenableErrorKind, AmenableResult, IoSource, SerdeSource, SystemTimeSource,
    TimeComponentRangeSource, TimeFormatDescriptionSource, TimeFormatSource, TimeParseSource,
};
#[cfg(feature = "verus")]
pub use verus_exchange_export::write_verus_exchange_companions;
#[cfg(feature = "verus")]
pub use verus_export::write_verus_witness_modules;
#[cfg(feature = "verus")]
pub use verus_gaap_tokens_export::write_verus_gaap_token_companion;

pub use amenable_core::{
    AsStandard, Calculation, CarriesToken, Certificate, ChainError, ChainGap, ChainNode,
    ClassifiedWitness, ContractRecord, Ensures, Establish, Evidence, EvidenceLink, Exchange,
    ExchangeEdgeRecord, Green, MetadataEntry, OwnedProvenanceReport, ProofChainReport, ProofRecord,
    ProofToken, ProofTokenMintRecord, Provenance, ProvenanceReport, Red, Registry, RegistryReport,
    Requires, Sidecar, Standard, State, StateMachine, Transition, TransitionAudit, Verifier,
    Witness, WitnessArtifact, WitnessArtifactMember, WitnessArtifactNode, WitnessArtifactShape,
    WitnessArtifactVariant, WitnessExportRecord, WitnessExportSnapshot, WitnessModulePath,
    WitnessSupportKind, WitnessSupportSummary, Yellow, proof_chain, proof_chain_for_verifiers,
    witness_exports,
};
#[cfg(feature = "creusot")]
pub use amenable_creusot::{
    CheckedProof as CreusotCheckedProof, CreusotVerifier, CreusotVerifierMetadata, CreusotWitness,
};
pub use amenable_kani::{
    AddEvidence, AddToken, CalculationProof, CheckedProof as KaniCheckedProof, Credit, Debit,
    GreenToken, KaniCompose, KaniGalleryCase, KaniGalleryDisposition, KaniGalleryExpectation,
    KaniGalleryRegistration, KaniProof, KaniProofRegistration, KaniVerifier, KaniVerifierMetadata,
    KaniWitness, NonNegativeFd, RedToken, Stoplight, Sum, YellowToken, add,
};
pub use amenable_std::{
    CertId, CertRegistry, ProvenanceCertificate, RustLanguageProvenance, RustStdProvenance,
    RustStdStandard, RustStdType, ValidUnicodeScalar, write_rust_std_certificate_artifacts,
};
#[cfg(feature = "verus")]
pub use amenable_std::{VerusCheckedProof, VerusVerifier, VerusVerifierMetadata, VerusWitness};
