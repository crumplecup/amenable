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

mod error;
pub mod paths;
mod verus_export;

pub mod assessment;
pub mod gallery;
pub mod kani;

pub use error::{
    AmenableError, AmenableErrorKind, AmenableResult, IoSource, SerdeSource, SystemTimeSource,
    TimeComponentRangeSource, TimeFormatDescriptionSource, TimeFormatSource, TimeParseSource,
};
pub use verus_export::write_verus_witness_modules;

pub use amenable_core::{
    Amenable, AsStandard, Calculation, CarriesToken, Certificate, ChainError, ChainGap, ChainNode,
    ClassifiedWitness, ContractRecord, Ensures, Establish, Evidence, EvidenceLink, Exchange,
    MetadataEntry, OwnedProvenanceReport, ProofChainReport, ProofRecord, ProofToken, Provenance,
    ProvenanceReport, Registry, RegistryReport, Requires, Sidecar, Standard, StateMachine,
    Verifier, Witness, WitnessArtifact, WitnessArtifactMember, WitnessArtifactNode,
    WitnessArtifactShape, WitnessArtifactVariant, WitnessExportRecord, WitnessExportSnapshot,
    WitnessModulePath, WitnessSupportKind, WitnessSupportSummary, proof_chain,
    proof_chain_for_verifiers, witness_exports,
};
#[cfg(feature = "creusot")]
pub use amenable_creusot::{CreusotVerifier, CreusotVerifierMetadata, CreusotWitness};
pub use amenable_kani::{
    AddEvidence, AddToken, CalculationProof, CheckedProof as KaniCheckedProof, Color, Credit,
    Debit, Green, GreenToken, KaniCompose, KaniGalleryCase, KaniGalleryDisposition,
    KaniGalleryExpectation, KaniGalleryRegistration, KaniProof, KaniProofRegistration,
    KaniVerifier, KaniVerifierMetadata, KaniWitness, NonNegativeFd, Red, RedToken, SequentialCycle,
    Stoplight, Sum, Yellow, YellowToken, add,
};
#[cfg(feature = "creusot")]
pub use amenable_std::CheckedProof as CreusotCheckedProof;
pub use amenable_std::{
    CertId, CertRegistry, ProvenanceCertificate, RustLanguageProvenance, RustStdProvenance,
    RustStdStandard, RustStdType, ValidUnicodeScalar, write_rust_std_certificate_artifacts,
};
#[cfg(feature = "verus")]
pub use amenable_std::{VerusCheckedProof, VerusVerifier, VerusVerifierMetadata, VerusWitness};
