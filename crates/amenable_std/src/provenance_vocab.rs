//! The shared provenance vocabulary: canonically-keyed [`Entry`](amenable_core::
//! Entry) types the standard-library provenance records and the three
//! verifier-descriptor records are built from.
//!
//! Each type fixes its own key, so the keys cannot drift across the records
//! that use them (before this, `KaniVerifierMetadata` /
//! `CreusotVerifierMetadata` / `VerusVerifierMetadata` each hand-maintained a
//! byte-for-byte-identical `const FACTS: &[(&str, &str)]` slice in a different
//! crate). `Authority` is shared by both vocabularies.
//!
//! Lives in `amenable_std`, not `amenable_core`: nothing in core uses the
//! vocabulary, and here it can be `#[derive(Entry)]`-generated rather than
//! hand-rolled. `SourceUrl` and `SemanticSummary` are the exception —
//! `amenable_core::provenance_vocab` hand-rolls those two so `amenable_std`
//! can depend on `amenable_time` without a cycle; see that module's own
//! doc comment.

use amenable_derive::Entry;

/// Define a `String`-valued vocabulary [`Entry`](amenable_core::Entry) type: a
/// `#[derive(Entry)]` newtype with the ergonomic `new` / `From<&str>` wrappers.
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
            Entry,
            derive_more::Display,
            derive_more::From,
        )]
        #[entry(key = $key, crate = "amenable_core")]
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
    /// The fully-qualified Rust type name being certified.
    TypeName, "type_name"
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Entry, derive_more::Display)]
#[entry(key = "authority_kind", crate = "amenable_core")]
pub enum AuthorityKind {
    /// A third-party standard the program cites and upholds (the common case:
    /// Rust's own documented std-library semantics).
    #[display("external_standard")]
    ExternalStandard,
    /// A local design decision with no external authority behind it.
    #[display("local_design")]
    LocalDesign,
}
