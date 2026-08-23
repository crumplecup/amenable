//! Verus spec for `std::hash::SipHasher`.
//!
//! Same shape as `hash_carrier.rs`'s `BuildHasherDefault<DefaultHasher>`
//! proof, but with no `vstd::std_specs::hash` support to build on for
//! `SipHasher` itself (only `DefaultHasher` has one) — this file gives
//! `SipHasher` its own opaque write-history view and ties `new`/`write`/
//! `finish` to it directly, the same modeling shape `vstd` uses for
//! `DefaultHasher`.
//!
//! `SipHasher` itself is stable, only deprecated as a recommendation to
//! use `DefaultHasher`; covering it is a coverage-completeness question,
//! not a call to use it — the same reasoning
//! `amenable_kani::rust_std::hash`'s own `SipHasherAlias` uses. A type
//! alias alone doesn't suppress the deprecation warning at each use site
//! (confirmed: `SipHasherAlias::new()` still warned even through the
//! alias), so this file expects the lint at the whole-file level instead
//! of scattering an attribute over every reference.
#![expect(
    deprecated,
    reason = "SipHasher itself is stable, only deprecated as a recommendation to use DefaultHasher; covering it is a coverage-completeness question, not a call to use it"
)]

use std::hash::Hasher;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

type SipHasherAlias = std::hash::SipHasher;

verus! {

#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExSipHasher(SipHasherAlias);

/// The sequence of byte slices written to a `SipHasher` so far —
/// opaque (uninterpreted), the same "write-history view" shape `vstd`
/// gives `DefaultHasher`.
pub uninterp spec fn sip_hasher_view(h: SipHasherAlias) -> Seq<Seq<u8>>;

/// `SipHasher::finish()`'s result as a pure function of its write
/// history — opaque (uninterpreted, `vstd` also leaves
/// `DefaultHasher::spec_finish` uninterpreted): its determinism (same
/// history, same output) is an inherent property of being a `spec fn`
/// at all, not an axiom stated separately.
pub uninterp spec fn sip_hasher_spec_finish(history: Seq<Seq<u8>>) -> u64;

pub assume_specification [SipHasherAlias::new] () -> (result: SipHasherAlias)
    ensures
        sip_hasher_view(result) == Seq::<Seq<u8>>::empty(),
;

pub assume_specification [<SipHasherAlias as Hasher>::write] (h: &mut SipHasherAlias, bytes: &[u8])
    ensures
        sip_hasher_view(*final(h)) == sip_hasher_view(*old(h)).push(bytes@),
;

pub assume_specification [<SipHasherAlias as Hasher>::finish] (h: &SipHasherAlias) -> (result: u64)
    ensures
        result == sip_hasher_spec_finish(sip_hasher_view(*h)),
;

/// Same determinism contract as `BuildHasherDefault`: two freshly
/// constructed `SipHasher`s hash the same input to the same output —
/// the same claim the Kani harness checks.
pub fn verify_sip_hasher_produces_consistent_hashes(value: u8) -> (result: bool)
    ensures
        result,
{
    let mut h1 = SipHasherAlias::new();
    let mut h2 = SipHasherAlias::new();
    h1.write(&[value]);
    h2.write(&[value]);
    h1.finish() == h2.finish()
}

} // verus!
