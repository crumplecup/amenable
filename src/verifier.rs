//! Verifier backends recognized by the constitutional proof surface.

use crate::{MetadataEntry, Provenance};

/// Marker trait for a verifier backend.
pub trait Verifier: 'static {
    /// Structured reporting surface for backend-specific provenance and
    /// configuration disclosure.
    type Metadata: Provenance;

    /// Canonical backend name for audit and display purposes.
    fn name() -> &'static str;

    /// Provenance metadata describing the verifier backend and its reporting
    /// surface.
    fn metadata() -> Vec<MetadataEntry> {
        <Self::Metadata as Provenance>::metadata()
    }
}

/// The Kani verifier backend.
pub struct KaniVerifier;

/// Provenance surface for the Kani verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct KaniVerifierMetadata;

impl Provenance for KaniVerifierMetadata {
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("verifier_family", "kani"),
            MetadataEntry::new("authority", "Kani Rust Verifier"),
            MetadataEntry::new("source_url", "https://model-checking.github.io/kani/"),
            MetadataEntry::new("proof_artifact", "Rust proof harness token stream"),
            MetadataEntry::new(
                "configuration_channel",
                "CLI arguments and KANI_* or PROVE_* environment variables",
            ),
            MetadataEntry::new(
                "configuration_surface",
                "package selection, flags, timeout, and report output",
            ),
        ]
    }
}

impl Verifier for KaniVerifier {
    type Metadata = KaniVerifierMetadata;

    fn name() -> &'static str {
        "kani"
    }
}

/// The Creusot verifier backend.
pub struct CreusotVerifier;

/// Provenance surface for the Creusot verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreusotVerifierMetadata;

impl Provenance for CreusotVerifierMetadata {
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("verifier_family", "creusot"),
            MetadataEntry::new("authority", "Creusot project"),
            MetadataEntry::new("source_url", "https://creusot-rs.github.io/creusot/"),
            MetadataEntry::new("proof_artifact", "Why3-oriented proof token stream"),
            MetadataEntry::new(
                "configuration_channel",
                "CLI arguments and CREUSOT_* or PROVE_* environment variables",
            ),
            MetadataEntry::new(
                "configuration_surface",
                "package selection, flags, binary path, timeout, and report output",
            ),
        ]
    }
}

impl Verifier for CreusotVerifier {
    type Metadata = CreusotVerifierMetadata;

    fn name() -> &'static str {
        "creusot"
    }
}

/// The Verus verifier backend.
pub struct VerusVerifier;

/// Provenance surface for the Verus verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    fn metadata() -> Vec<MetadataEntry> {
        vec![
            MetadataEntry::new("verifier_family", "verus"),
            MetadataEntry::new("authority", "Verus project"),
            MetadataEntry::new("source_url", "https://verus-lang.github.io/verus/"),
            MetadataEntry::new("proof_artifact", "Verus proof module token stream"),
            MetadataEntry::new(
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            MetadataEntry::new(
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ]
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}
