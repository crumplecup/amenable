//! Verus spec for `std::hash::BuildHasherDefault<DefaultHasher>`.
//!
//! `vstd` already gives `DefaultHasher` a real spec module
//! (`std_specs/hash.rs`): `new()`, `write(&mut self, &[u8])`, and
//! `finish(&self) -> u64 == DefaultHasher::spec_finish(state@)`, where
//! `state@: Seq<Seq<u8>>` is the sequence of byte slices written so
//! far. Two hashers built identically and fed the identical write
//! therefore have identical views, and — since `spec_finish` is a pure
//! function, an inherent property of being a `spec fn` at all — provably
//! identical `finish()` results. Only `BuildHasherDefault::build_hasher`
//! itself needs a local axiom (`vstd` has none): its real implementation
//! is exactly `H::default()`, so it's specified here relative to
//! `DefaultHasher::new()`'s already-`vstd`-specified empty view.
//!
//! Uses `.write(&[value])` rather than `Hasher::write_u8`, matching what
//! `vstd` actually specifies (`write_u8` has no spec of its own) — not a
//! weaker claim, the same real one-byte write `amenable_kani`'s own
//! harness narrows to (see that harness's own doc comment: `write_u64`
//! times out under Kani, so it already checks `write_u8` specifically,
//! not a symbolic multi-byte write).

use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `#[verifier::external_type_specification]` marker binding
/// `BuildHasherDefault<H>` to Verus.
#[cfg(verus_keep_ghost)]
#[verifier::reject_recursive_types(H)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExBuildHasherDefault<H>(BuildHasherDefault<H>);

pub assume_specification<H> [<BuildHasherDefault<H> as core::default::Default>::default] () -> (result: BuildHasherDefault<H>);

// `H::default.ensures((), result)` is Verus's own builtin function-item
// contract-inspection syntax (specifying `build_hasher`'s result in
// terms of whatever `H::default`'s own real `ensures` resolves to for
// the concrete `H` a caller instantiates) -- not a call to any
// `amenable_core::Ensures` predicate, so there is no named fn to point
// this clause at; deliberately left as the literal builtin form.
pub assume_specification<H: core::default::Default + Hasher> [<BuildHasherDefault<H> as BuildHasher>::build_hasher] (builder: &BuildHasherDefault<H>) -> (result: H)
    ensures
        H::default.ensures((), result),
;

/// `DefaultHasher::new`'s whole postcondition: starts with an empty
/// write history.
pub open spec fn default_hasher_new_view_is_empty(result: DefaultHasher) -> bool {
    result@ == Seq::<Seq<u8>>::empty()
}

pub assume_specification [<DefaultHasher as core::default::Default>::default] () -> (result: DefaultHasher)
    ensures
        default_hasher_new_view_is_empty(result),
;

/// `BuildHasherDefault` fulfils `BuildHasher`'s core contract: two
/// hashers it constructs (via `H::default()` each time) hash the same
/// input to the same output — the same claim the Kani harness checks.
pub fn verify_build_hasher_default_produces_consistent_hashers(value: u8) -> (result: bool)
    ensures
        result,
{
    let builder = BuildHasherDefault::<DefaultHasher>::default();
    let mut h1 = builder.build_hasher();
    let mut h2 = builder.build_hasher();
    h1.write(&[value]);
    h2.write(&[value]);
    h1.finish() == h2.finish()
}

} // verus!
