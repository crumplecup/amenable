//! Verifier backends recognized by the constitutional proof surface.

use crate::{MetadataEntry, Provenance};

/// Marker trait for a verifier backend.
pub trait Verifier: 'static {
    /// Structured reporting surface for backend-specific provenance and
    /// configuration disclosure.
    type Metadata: Provenance + Default;

    /// Canonical backend name for audit and display purposes.
    fn name() -> &'static str;

    /// Provenance metadata describing the verifier backend and its reporting
    /// surface. Query the result through the [`Provenance`] interface
    /// (`iter`, `get`, `contains_key`, ...) rather than expecting a
    /// particular collection back.
    fn metadata() -> Self::Metadata {
        Self::Metadata::default()
    }
}

/// The Kani verifier backend.
pub struct KaniVerifier;

/// Provenance surface for the Kani verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct KaniVerifierMetadata;

impl Provenance for KaniVerifierMetadata {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "kani"),
            ("authority", "Kani Rust Verifier"),
            ("source_url", "https://model-checking.github.io/kani/"),
            ("proof_artifact", "Rust proof harness token stream"),
            (
                "configuration_channel",
                "CLI arguments and KANI_* or PROVE_* environment variables",
            ),
            (
                "configuration_surface",
                "package selection, flags, timeout, and report output",
            ),
        ];
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
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
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
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
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
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
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "verus"),
            ("authority", "Verus project"),
            ("source_url", "https://verus-lang.github.io/verus/"),
            ("proof_artifact", "Verus proof module token stream"),
            (
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            (
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ];
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}
