use amenable_core::{Metadata, OwnedEntry, Provenance, Verifier};

// ── ChronoVerifier ───────────────────────────────────────────────────

/// Reporting surface for [`ChronoVerifier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChronoVerifierMetadata;

impl Metadata for ChronoVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "chrono"),
            OwnedEntry::new(
                "kind",
                "runtime execution over chrono's real calendar/zone API",
            ),
            OwnedEntry::new("formal_tool", "none"),
        ]
    }
}

impl Provenance for ChronoVerifierMetadata {}

/// A [`Verifier`] marker for the real chrono-backed temporal backend.
///
/// Like the jiff backend's `JiffVerifier`, it runs no formal tool. Its
/// `Exchange` bodies call chrono's own calendar code, so a violation is an
/// `Err` at runtime, not a proof obligation. From this verifier's point of
/// view every machine-checkable contract is a trusted citation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChronoVerifier;

impl Verifier for ChronoVerifier {
    type Metadata = ChronoVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "chrono"
    }
}

// ── The backend ──────────────────────────────────────────────────────

/// The real chrono-backed temporal backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ChronoTimeBackend;
