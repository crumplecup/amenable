//! The Verus [`Verifier`](crate::Verifier) marker.
//!
//! Available with the `verus` feature. Previously lived in
//! `amenable_std`; moved here so `amenable_std` can depend on
//! `amenable_time` (for its own `std::time` backend support) without a
//! cycle — `amenable_time::verus_witness` needs this marker too, and
//! after the reversal `amenable_time` no longer depends on
//! `amenable_std` at all (see `docs/AMENABLE_EXT_PLAN.md`). Its crate of
//! origin was never load-bearing for the orphan rule at either call
//! site: `amenable_std`'s per-type bridges (`bridge_verus_witness!`) are
//! always invoked on a fully concrete, already-local
//! `RustStdStandard<ConcreteType>`, and `amenable_time`'s own impls are
//! written for its own local contract types — in both cases the `Self`
//! type is already local, so `Witness`'s other type parameter (this
//! marker) is free to live wherever is convenient.
//!
//! There is still only one verifier Verus works with — Verus.

use crate::{Entry, Metadata, OwnedEntry, Provenance, SourceUrl, Verifier};

/// The Verus verifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifier;

/// Provenance surface for the Verus verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifierMetadata;

impl Metadata for VerusVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "verus"),
            OwnedEntry::new("authority", "Verus project"),
            SourceUrl::new("https://verus-lang.github.io/verus/").into_entry(),
            OwnedEntry::new("proof_artifact", "Verus proof module token stream"),
            OwnedEntry::new(
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            OwnedEntry::new(
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ]
    }
}

impl Provenance for VerusVerifierMetadata {}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "verus"
    }
}
