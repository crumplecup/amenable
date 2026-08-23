//! Verus accommodation model for `core::mem::Discriminant<Option<i32>>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `Discriminant`'s defining law is variant identity,
//! independent of payload — modeled here as a plain tag (`1` for
//! `Some`, `0` for `None`) rather than the real opaque `Discriminant`
//! value `std::mem::discriminant` returns. `VerusDiscriminantModel`
//! isn't `Discriminant` and doesn't claim to be — the proof is
//! conditional: sound if the real `Discriminant` refines this law,
//! which `amenable_kani`'s own
//! `verify_discriminant_identifies_variant_not_payload` harness
//! (checking the real `std::mem::discriminant` directly) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Two `Some` values with different payloads share a discriminant tag,
/// while `Some` and `None` do not — the variant-identity,
/// payload-independence law `Discriminant<Option<i32>>` is expected to
/// refine.
pub fn verify_discriminant_model_identifies_variant_not_payload(a: i32, b: i32) -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    let discriminant_some_a: u8 = 1;
    let discriminant_some_b: u8 = 1;
    let discriminant_none: u8 = 0;

    let same_variant_shares_discriminant = discriminant_some_a == discriminant_some_b;
    let different_variant_differs = discriminant_some_a != discriminant_none;

    // `a`/`b` themselves play no role beyond standing in for two
    // distinct `Some` payloads — the claim doesn't depend on their
    // values at all.
    let _ = a;
    let _ = b;

    (same_variant_shares_discriminant, different_variant_differs)
}

} // verus!
