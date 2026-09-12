//! The provenance vocabulary [`ExtType`](crate::ExtType) registrations
//! are built from: canonically-keyed [`Entry`](amenable_core::Entry)
//! types naming the third-party crate/module that defines a carrier and
//! the authority behind it.
//!
//! A near-identical mirror of `amenable_std::provenance_vocab`'s own
//! `string_vocab_entry!` — deliberately not a dependency on
//! `amenable_std` for these few types: `amenable_ext` is architecturally
//! a sibling to `amenable_std` (this crate's own "one crate per foreign
//! type surface, orphan rule forces it" reasoning is the same doc
//! comment, restated for third-party crates instead of the standard
//! library), not a consumer layered on top of it, and duplicating this
//! small vocabulary keeps it that way. `SourceUrl`/`SemanticSummary`
//! are the exception — reused directly from `amenable_core`, where they
//! already live for the same reason `amenable_time` reuses them (see
//! `amenable_core::provenance_vocab`'s own doc comment).

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
    /// The authorizing body for a documented semantics or trust decision
    /// (the third-party crate's own maintaining project, e.g. "jiff
    /// contributors").
    Authority, "authority"
}
string_vocab_entry! {
    /// The third-party crate that normatively defines a carrier.
    SourceCrate, "source_crate"
}
string_vocab_entry! {
    /// The module path within the third-party crate that normatively
    /// defines a carrier.
    SourceModule, "source_module"
}
string_vocab_entry! {
    /// The fully-qualified Rust type name being certified.
    TypeName, "type_name"
}

/// The class of authority a provenance record represents — a closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Entry, derive_more::Display)]
#[entry(key = "authority_kind", crate = "amenable_core")]
pub enum AuthorityKind {
    /// A third-party standard the program cites and upholds (the common
    /// case here: a target crate's own documented semantics).
    #[display("external_standard")]
    ExternalStandard,
    /// A local design decision with no external authority behind it.
    #[display("local_design")]
    LocalDesign,
}
