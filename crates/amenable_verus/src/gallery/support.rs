//! Shared fixture types every gallery case that needs one imports,
//! rather than redefining. `GalleryVerifier`/`GalleryVerifierMetadata`
//! (a local marker standing in for what would be `crate::VerusVerifier`
//! in a real design, mirroring `amenable_kani::KaniVerifier`/
//! `amenable_creusot::CreusotVerifier`) are identical, load-bearing
//! plumbing for every case (`Verifier::Metadata` needs *some*
//! `Provenance` impl to exist) with no per-case variation possible --
//! confirmed real, not assumed: every one of the 7 cases that used to
//! define its own copy had a byte-for-byte identical definition, diffed
//! directly against each other, not merely eyeballed.
//!
//! `Green`/`Yellow`/their tokens are deliberately **not** here, despite
//! looking like the identical story at first: `evidence_self_
//! referential_root`, `proof_token_external_trait_bound`, and
//! `stoplight_exchange` each define byte-identical `Green`/`Yellow`
//! *structs*, but each case then attaches its *own*, different further
//! trait impls to them (`Witness<GalleryVerifier>`, `Establish<..>`,
//! or a generated `verus_exchange!` companion's own `Witness` impl) --
//! confirmed the hard way: sharing one `Green`/`Yellow` type across
//! cases produces real `E0119: conflicting implementations` the moment
//! two cases each try to attach their own `Witness`/`Establish` impl to
//! what coherence then sees as the exact same type. Each case needs its
//! *own* `Green`/`Yellow` for exactly the reason the file-level doc
//! comments already say: independent investigations, not a shared
//! library -- that reasoning holds for these two, even though it turned
//! out not to hold for `GalleryVerifier` itself.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::Verifier;

verus! {

/// Local marker, standing in for what would be `crate::VerusVerifier` in
/// a real design (mirroring `amenable_kani::KaniVerifier`/
/// `amenable_creusot::CreusotVerifier`).
pub struct GalleryVerifier;

impl Verifier for GalleryVerifier {
    type Metadata = GalleryVerifierMetadata;

    fn name() -> &'static str {
        "gallery"
    }
}

#[derive(Default)]
pub struct GalleryVerifierMetadata;

// Incidental plumbing (`Verifier::Metadata` needs *some* `Provenance`
// impl to exist, but nothing any gallery case's own claim calls into
// it).
#[verifier::external]
impl crate::Provenance for GalleryVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<crate::MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

} // verus!
