//! Local witness trait bridging into `amenable_core::Witness`.

use amenable_core::{Evidence, MetadataEntry, Provenance, Verifier};
use creusot_std::macros::trusted;

/// The Creusot verifier, local to this crate: there is only one verifier
/// Creusot works with — Creusot. `CreusotVerifier` belongs here rather
/// than `amenable_core`: it's what makes each per-type `Witness<
/// CreusotVerifier>` impl elsewhere in this crate (`rust_std_witness.rs`,
/// `ledger.rs`, `stoplight.rs`) a bridge to Creusot specifically, not a
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

impl Provenance for CreusotVerifierMetadata {
    // `Vec::IntoIter`, not `Box<dyn Iterator<...>>` like every other
    // verifier's own metadata (see `amenable_kani`/`amenable_verus`'s own
    // `witness.rs`) — this is the one `Provenance` impl in this workspace
    // that's actually swept up by `creusot-rustc`'s translation of
    // `amenable_creusot` (it's local, unlike `amenable_std`'s registrations,
    // which only ever compile in ordinary mode as a dependency). `dyn` is
    // confirmed unsupported there: `error: forbidden dyn type: dyn
    // std::iter::Iterator<...> (dyn support is currently minimal)`.
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    // Ordinary reporting code, not a proof claim — nothing here states or
    // needs a Pearlite postcondition, but creusot-rustc still generates a
    // real verification condition for every translated function unless
    // told not to, and that VC is unprovable as stated: `MetadataEntry::
    // new` has no contract, so calling it "yields an impossible
    // precondition" (its own warning, confirmed by why3find actually
    // failing this exact goal — `Coma.vc_metadata_CreusotVerifierMetadata`
    // — before this attribute was added).
    #[trusted]
    fn metadata(&self) -> Self::MetadataIter {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "creusot"),
            ("authority", "Creusot project"),
            ("source_url", "https://creusot-rs.github.io/creusot/"),
            ("proof_artifact", "Why3-oriented proof token stream"),
            (
                "configuration_channel",
                "CLI arguments and CREUSOT_* or PROVE_* environment variables",
            ),
            (
                "configuration_surface",
                "package selection, flags, binary path, timeout, and report output",
            ),
        ];
        FACTS
            .iter()
            .map(|&(k, v)| MetadataEntry::new(k, v))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl Verifier for CreusotVerifier {
    type Metadata = CreusotVerifierMetadata;

    fn name() -> &'static str {
        "creusot"
    }
}

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

/// Proof artifact for a claim that rests on one or more real Creusot
/// contract functions — used wherever a `Witness<CreusotVerifier>` impl
/// targets a *real* evidence type directly (no accommodation-model
/// mirror), naming each backing harness and its own verbatim source.
/// Owned strings, not `&'static str` — matching `amenable_kani::
/// CalculationProof`'s own precedent, this crate's proof-artifact
/// convention, not `rust_std_witness::CheckedProof`'s (which carries an
/// extra chain-derived `provenance` field specific to `RustStdStandard<T>`
/// carriers).
///
/// `#[cfg(not(creusot))]` on the type itself, not just at each call
/// site: the `Vec<(String, String)>`/`Display` machinery is exactly the
/// kind of ordinary Rust infrastructure `creusot-rustc`'s translator
/// chokes on when it's *local* to the crate being translated — confirmed
/// the hard way, twice. First building `ledger.rs`'s own `Witness<
/// CreusotVerifier>` bridge (an ungated `impl Display` there caused a
/// real internal compiler panic); then again here, the *second* time
/// around, gating only this struct's re-export in `lib.rs` and not the
/// struct's own definition — the derived `Clone` for the `Vec<(String,
/// String)>` field still got swept and panicked (`Cannot handle builtin
/// implementation of std::clone::Clone for (String, String)`) even
/// though nothing under `#[cfg(creusot)]` ever constructs one. The gate
/// has to live at the definition site itself, not downstream of it.
/// Every caller keeps its `#[cfg(creusot)]`-gated `Witness` impl trivial
/// (`ProofArtifact = ()`) and only builds one of these on the
/// `#[cfg(not(creusot))]` side.
#[cfg(not(creusot))]
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct MultiCheckProof {
    /// Each real Creusot contract function backing this claim, and its
    /// own verbatim source.
    checks: Vec<(String, String)>,
}

#[cfg(not(creusot))]
impl std::fmt::Display for MultiCheckProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (harness, claim) in &self.checks {
            writeln!(f, "harness: {harness}")?;
            writeln!(f, "claim: {claim}")?;
        }
        Ok(())
    }
}
