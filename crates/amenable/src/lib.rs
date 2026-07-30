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
//! `amenable_creusot`, `amenable_verus`, `amenable_code`, and `amenable_std`
//! itself) depend on `amenable_core` directly, never on this facade, to
//! avoid a circular dependency.
//!
//! See `AMENABLE_PLAN.md` and `amenable.md` in the repository root for the
//! full design rationale.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
pub mod paths;

pub mod assessment;
pub mod gallery;
pub mod kani;

pub use error::{
    AmenableError, AmenableErrorKind, AmenableResult, IoSource, SerdeSource, SystemTimeSource,
    TimeComponentRangeSource, TimeFormatDescriptionSource, TimeFormatSource, TimeParseSource,
};

pub use amenable_core::{
    Amenable, AsStandard, Calculation, CarriesToken, Certificate, ChainError, ChainGap, ChainNode,
    Establish, Evidence, EvidenceLink, Exchange, MetadataEntry, OwnedProvenanceReport,
    ProofChainReport, ProofRecord, ProofToken, Provenance, ProvenanceReport, Registry,
    RegistryReport, Sidecar, Standard, StateMachine, Verifier, Witness, proof_chain,
    proof_chain_for_verifiers,
};
pub use amenable_creusot::{
    CheckedProof as CreusotCheckedProof, CreusotVerifier, CreusotVerifierMetadata, CreusotWitness,
};
pub use amenable_kani::{
    AddEvidence, AddToken, CalculationProof, CheckedProof as KaniCheckedProof, Color, Credit,
    Debit, Green, GreenToken, KaniCompose, KaniGalleryCase, KaniGalleryDisposition,
    KaniGalleryExpectation, KaniGalleryRegistration, KaniProof, KaniProofRegistration,
    KaniVerifier, KaniVerifierMetadata, KaniWitness, Red, RedToken, SequentialCycle, Stoplight,
    Sum, Yellow, YellowToken, add,
};
pub use amenable_std::{
    CertId, CertRegistry, ProvenanceCertificate, RustLanguageProvenance, RustStdProvenance,
    RustStdStandard, RustStdType, write_rust_std_certificate_artifacts,
};
pub use amenable_verus::{
    CheckedProof as VerusCheckedProof, VerusVerifier, VerusVerifierMetadata, VerusWitness,
};
