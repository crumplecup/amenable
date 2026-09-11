//! The two provenance-vocabulary entries `amenable_core` itself needs.
//!
//! The full vocabulary (`Authority`, `SourceCrate`, `VerifierFamily`, …)
//! lives in `amenable_std::provenance_vocab`, on purpose — nothing in
//! this crate used to need it, and there it can be `#[derive(Entry)]`-
//! generated rather than hand-rolled. `SourceUrl` and `SemanticSummary`
//! are the exception: [`verus_verifier::VerusVerifierMetadata`](crate::VerusVerifierMetadata)
//! (this crate, `verus` feature) needs a `SourceUrl` entry, and
//! `amenable_time::TemporalProvenance` needs both — moving just these
//! two here (not the rest) is what lets `amenable_std` depend on
//! `amenable_time` without a cycle (see `docs/AMENABLE_EXT_PLAN.md`).
//!
//! Hand-rolled rather than `#[derive(Entry)]`, deliberately:
//! `amenable_derive` carries an *optional* dependency back on
//! `amenable_core` (behind its own `verus` feature, for `verus_carrier`
//! parsing) — adding `amenable_derive` as a real dependency of
//! `amenable_core` just to use its derive here would recreate a cycle
//! the moment both crates' `verus` features are active. The generated
//! shape below matches `#[derive(Entry)]`'s own codegen for a `String`
//! newtype exactly (`amenable_derive::entry::expand_entry`).

use std::fmt::{self, Display, Formatter};

use crate::{Entry, Metadata, OwnedEntry};

/// Define a `String`-valued vocabulary [`Entry`] type: a newtype with the
/// same `new` / `From<&str>` / `Entry` / `Metadata` shape
/// `amenable_std::provenance_vocab`'s `#[derive(Entry)]`-based
/// `string_vocab_entry!` produces, written by hand.
macro_rules! core_vocab_entry {
    ($(#[$doc:meta])* $name:ident, $key:literal) => {
        $(#[$doc])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Create a `", stringify!($name), "` fact.")]
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl Entry for $name {
            const KEY: &'static str = $key;
            type Value = str;

            fn value(&self) -> &Self::Value {
                self.0.as_str()
            }
        }

        impl Metadata for $name {
            fn snapshot(&self) -> Vec<OwnedEntry> {
                vec![OwnedEntry::new(Self::KEY, self.clone())]
            }
        }
    };
}

core_vocab_entry! {
    /// The canonical documentation URL for a carrier or a verifier backend.
    SourceUrl, "source_url"
}
core_vocab_entry! {
    /// Concise summary of a semantic promise a carrier or contract makes.
    SemanticSummary, "semantic_summary"
}
