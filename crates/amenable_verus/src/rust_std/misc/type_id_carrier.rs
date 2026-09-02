//! Verus spec for `core::any::TypeId`.

use std::any::TypeId;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `#[verifier::external_type_specification]` marker binding `TypeId` to
/// Verus.
#[cfg(verus_keep_ghost)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExTypeId(TypeId);

/// The type-level identity `TypeId::of::<T>()` carries — opaque
/// (uninterpreted): its only content is what the axioms below assert
/// about it, never a real hash or bit pattern.
pub uninterp spec fn type_id_spec<T: 'static + ?Sized>() -> nat;

/// The per-value identity a constructed `TypeId` carries, connecting
/// `TypeId::of`'s result to `eq`'s comparison without either needing to
/// know the other's generic parameter `T` (erased by the time two
/// `TypeId`s are compared).
pub uninterp spec fn type_id_identity_spec(id: TypeId) -> nat;

/// `TypeId::of::<T>()`'s whole postcondition: carries `T`'s own identity.
pub open spec fn type_id_of_matches_spec<T: 'static + ?Sized>(result: TypeId) -> bool {
    type_id_identity_spec(result) == type_id_spec::<T>()
}

pub assume_specification<T: 'static + ?Sized> [TypeId::of::<T>] () -> (result: TypeId)
    ensures
        type_id_of_matches_spec::<T>(result),
;

/// `TypeId::eq`'s whole postcondition: compares the two carried
/// identities.
pub open spec fn type_id_eq_matches_identity(a: TypeId, b: TypeId, result: bool) -> bool {
    result == (type_id_identity_spec(a) == type_id_identity_spec(b))
}

pub assume_specification [<TypeId as core::cmp::PartialEq>::eq] (a: &TypeId, b: &TypeId) -> (result: bool)
    ensures
        type_id_eq_matches_identity(*a, *b, result),
;

/// Distinct concrete types get distinct identities — asserted for this
/// one representative pair (the same pair Kani's harness checks), not a
/// general injectivity claim over every possible `T`.
pub open spec fn i32_and_bool_type_ids_differ() -> bool {
    type_id_spec::<i32>() != type_id_spec::<bool>()
}

#[verifier::external_body]
pub broadcast proof fn axiom_i32_and_bool_type_ids_differ()
    ensures
        // Verus's own automatic broadcast/trigger instantiation needs
        // the literal comparison present as its own `#[trigger]`ed
        // clause -- a call wrapping it inside a named predicate gives
        // the solver nothing to pattern-match on (same idiom as
        // `cstring_carrier.rs`'s `axiom_vec_u8_into_vec_u8_is_identity`
        // and `cow_carrier.rs`'s `axiom_i32_to_owned_is_identity`). The
        // clause right below states the identical fact through the real
        // named predicate above, so the claim itself is still named
        // once; this raw clause exists only to give the solver
        // something to pattern-match on.
        #[trigger] type_id_spec::<i32>() != #[trigger] type_id_spec::<bool>(),
        i32_and_bool_type_ids_differ(),
{
}

/// `TypeId::of::<T>()` is the same value across calls for the same `T`,
/// and differs between distinct `T`s — the same claim the Kani harness
/// checks.
pub fn verify_type_id_is_reflexive_and_distinguishes_distinct_types() -> (result: (bool, bool))
    ensures
        result.0,
        !result.1,
{
    broadcast use axiom_i32_and_bool_type_ids_differ;

    let reflexive = TypeId::of::<i32>() == TypeId::of::<i32>();
    let distinguishes = TypeId::of::<i32>() == TypeId::of::<bool>();
    (reflexive, distinguishes)
}

} // verus!
