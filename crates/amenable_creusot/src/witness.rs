//! Local witness trait bridging into `amenable_core::Witness`.

use amenable_core::{Entry, Evidence, Metadata, OwnedEntry, Provenance, SourceUrl, Verifier};
use amenable_std::{
    Authority, ConfigurationChannel, ConfigurationSurface, ProofArtifact, VerifierFamily,
};
use creusot_std::macros::trusted;

/// Creusot-specific witness: identifies the Creusot contract (if any) behind
/// a piece of evidence, without ever running it.
pub trait CreusotWitness {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the Creusot proof relevant to this evidence.
    type ProofArtifact;

    /// Identify the Creusot proof artifact for this evidence.
    fn proof() -> Self::ProofArtifact;
}

/// The Creusot verifier, local to this crate: there is only one verifier
/// Creusot works with — Creusot. `CreusotVerifier` belongs here rather
/// than `amenable_core`: it's what makes each per-type `Witness<
/// CreusotVerifier>` impl elsewhere in this crate (`rust_std_witness/`,
/// `ledger/`, `stoplight.rs`) a bridge to Creusot specifically, not a
/// generic one no other verifier could also legally claim.
pub struct CreusotVerifier;

/// Provenance surface for the Creusot verifier backend.
///
/// No `PartialEq`/`Eq`/`PartialOrd`/`Ord` here, unlike every sibling
/// verifier's own metadata marker (Kani, Verus) — `Verifier::Metadata`
/// only requires `Provenance + Default`, nothing in this workspace
/// compares two `CreusotVerifierMetadata` values, and deriving `PartialEq`
/// specifically (which `Eq`, `PartialOrd`, and `Ord` all require in turn)
/// breaks under real translation: `creusot-rustc` requires a `DeepModel`
/// impl for any type deriving it, confirmed by a real error (`the trait
/// bound witness::CreusotVerifierMetadata: creusot_std::model::DeepModel
/// is not satisfied`), and this crate is the one sibling whose code is
/// actually swept up by that translator. `Hash` alone doesn't trigger it.
#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct CreusotVerifierMetadata;

impl Metadata for CreusotVerifierMetadata {
    // This is the one `Metadata` impl in this workspace that's actually
    // swept up by `creusot-rustc`'s translation of `amenable_creusot`
    // (it's local, unlike `amenable_std`'s registrations, which only ever
    // compile in ordinary mode as a dependency).
    //
    // Ordinary reporting code, not a proof claim — nothing here states or
    // needs a Pearlite postcondition, but creusot-rustc still generates a
    // real verification condition for every translated function unless
    // told not to, and that VC is unprovable as stated: neither
    // `OwnedEntry::new` nor its inner `Arc::new` has a contract, so
    // calling one "yields an impossible precondition" (creusot's own
    // warning — the same goal, `Coma.vc_snapshot_CreusotVerifierMetadata`,
    // that failed under `why3find` before this attribute was added).
    #[trusted]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            VerifierFamily::new("creusot").into_entry(),
            Authority::new("Creusot project").into_entry(),
            SourceUrl::new("https://creusot-rs.github.io/creusot/").into_entry(),
            ProofArtifact::new("Why3-oriented proof token stream").into_entry(),
            ConfigurationChannel::new(
                "CLI arguments and CREUSOT_* or PROVE_* environment variables",
            )
            .into_entry(),
            ConfigurationSurface::new(
                "package selection, flags, binary path, timeout, and report output",
            )
            .into_entry(),
        ]
    }
}

impl Provenance for CreusotVerifierMetadata {}

impl Verifier for CreusotVerifier {
    type Metadata = CreusotVerifierMetadata;

    fn name() -> &'static str {
        "creusot"
    }
}

/// `MultiCheckProof` and its `Display` impl, consolidated into one gate on
/// this `mod` instead of one per item -- see `stoplight::mirror`'s own doc
/// comment for the general rationale. `pub`, not `pub(super)`: `lib.rs`
/// re-exports `MultiCheckProof` at the crate's own public boundary (`pub
/// use witness::MultiCheckProof;`), and a re-export can never be more
/// visible than what it re-exports -- the same real constraint
/// `stoplight::mirror`'s own split re-export hit first.
///
/// `#[cfg(not(creusot))]` on the module, not just at each item inside it:
/// the `Vec<(String, String)>`/`Display` machinery is exactly the kind of
/// ordinary Rust infrastructure `creusot-rustc`'s translator chokes on
/// when it's *local* to the crate being translated — confirmed the hard
/// way, twice. First building `ledger/`'s own `Witness<CreusotVerifier>`
/// bridge (an ungated `impl Display` there caused a real internal
/// compiler panic); then again here, the *second* time around, gating
/// only this struct's re-export in `lib.rs` and not the struct's own
/// definition — the derived `Clone` for the `Vec<(String, String)>` field
/// still got swept and panicked (`Cannot handle builtin implementation of
/// std::clone::Clone for (String, String)`) even though nothing under
/// `#[cfg(creusot)]` ever constructs one. The gate has to live at the
/// definition site itself, not downstream of it. Every caller keeps its
/// `#[cfg(creusot)]`-gated `Witness` impl trivial (`ProofArtifact = ()`)
/// and only builds one of these on the `#[cfg(not(creusot))]` side.
#[cfg(not(creusot))]
mod mirror {
    /// Proof artifact for a claim that rests on one or more real Creusot
    /// contract functions — used wherever a `Witness<CreusotVerifier>` impl
    /// targets a *real* evidence type directly (no accommodation-model
    /// mirror), naming each backing harness and its own verbatim source.
    /// Owned strings, not `&'static str` — matching `amenable_kani::
    /// CalculationProof`'s own precedent, this crate's proof-artifact
    /// convention, not `rust_std_witness::CheckedProof`'s (which carries an
    /// extra chain-derived `provenance` field specific to `RustStdStandard<T>`
    /// carriers).
    #[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
    pub struct MultiCheckProof {
        /// Each real Creusot contract function backing this claim, and its
        /// own verbatim source.
        checks: Vec<(String, String)>,
    }

    impl std::fmt::Display for MultiCheckProof {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (harness, claim) in &self.checks {
                writeln!(f, "harness: {harness}")?;
                writeln!(f, "claim: {claim}")?;
            }
            Ok(())
        }
    }
}
#[cfg(not(creusot))]
pub use mirror::MultiCheckProof;
