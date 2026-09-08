//! The shared provenance vocabulary: canonically-keyed [`Entry`] types the
//! standard-library provenance records and the three verifier-descriptor
//! records are built from.
//!
//! Each type fixes its own key, so the keys cannot drift across the records
//! that use them (before this, `KaniVerifierMetadata` /
//! `CreusotVerifierMetadata` / `VerusVerifierMetadata` each hand-maintained a
//! byte-for-byte-identical `const FACTS: &[(&str, &str)]` slice in a different
//! crate). `Authority` and `SourceUrl` are shared by both vocabularies.
//!
//! `amenable_core` cannot depend on `amenable_derive` (the proc-macro crate
//! optionally depends back on this one), so these are macro-generated here
//! rather than `#[derive(Entry)]`-generated — the same reason `stoplight.rs`
//! hand-writes its `Standard` impls.

use crate::{Entry, Metadata, OwnedEntry};

/// Define a `String`-valued vocabulary [`Entry`] type: a newtype that is also a
/// one-entry [`Metadata`] record (keyed by `$key`, holding the value by clone).
macro_rules! string_vocab_entry {
    ($(#[$doc:meta])* $name:ident, $key:literal) => {
        $(#[$doc])*
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            derive_more::Display,
            derive_more::From,
        )]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Create a `", stringify!($name), "` fact.")]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl Entry for $name {
            const KEY: &'static str = $key;
            type Value = str;

            fn value(&self) -> &str {
                &self.0
            }
        }

        impl Metadata for $name {
            fn snapshot(&self) -> ::std::vec::Vec<OwnedEntry> {
                ::std::vec![OwnedEntry::new($key, self.clone())]
            }
        }
    };
}

string_vocab_entry! {
    /// The authorizing body for a documented semantics or trust decision.
    Authority, "authority"
}
string_vocab_entry! {
    /// The Rust crate that normatively defines a standard-library carrier.
    SourceCrate, "source_crate"
}
string_vocab_entry! {
    /// The Rust module path that normatively defines a standard-library carrier.
    SourceModule, "source_module"
}
string_vocab_entry! {
    /// The canonical documentation URL for a carrier or a verifier backend.
    SourceUrl, "source_url"
}
string_vocab_entry! {
    /// The fully-qualified Rust type name being certified.
    TypeName, "type_name"
}
string_vocab_entry! {
    /// Concise summary of a semantic promise made by the standard library.
    SemanticSummary, "semantic_summary"
}
string_vocab_entry! {
    /// The verifier backend family a descriptor record describes.
    VerifierFamily, "verifier_family"
}
string_vocab_entry! {
    /// The proof artifact a verifier backend emits.
    ProofArtifact, "proof_artifact"
}
string_vocab_entry! {
    /// How a verifier backend is configured (CLI args, environment variables).
    ConfigurationChannel, "configuration_channel"
}
string_vocab_entry! {
    /// What a verifier backend's configuration surface covers.
    ConfigurationSurface, "configuration_surface"
}

/// The class of authority a provenance record represents — a closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
pub enum AuthorityKind {
    /// A third-party standard the program cites and upholds (the common case:
    /// Rust's own documented std-library semantics).
    #[display("external_standard")]
    ExternalStandard,
    /// A local design decision with no external authority behind it.
    #[display("local_design")]
    LocalDesign,
}

impl Entry for AuthorityKind {
    const KEY: &'static str = "authority_kind";
    type Value = AuthorityKind;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn value(&self) -> &AuthorityKind {
        self
    }
}

impl Metadata for AuthorityKind {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![OwnedEntry::new("authority_kind", *self)]
    }
}
