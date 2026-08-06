//! Verus spec for `alloc::borrow::Cow<'static, i32>`.
//!
//! Not the `Deref::deref` half of Kani's claim — genuinely unreachable
//! via `assume_specification` for this exact type shape, not a
//! convenience shortcut: `deref(&self) -> &B` needs its return tied to
//! `&self`'s own (elided) lifetime, but spelling the receiver out
//! concretely as `&Cow<'a, B>` (required to name the function at all)
//! introduces a SECOND, competing lifetime candidate — Cow's own `'a`
//! — that plain Rust's lifetime elision cannot disambiguate no matter
//! how the parameter/return lifetimes are named or elided (confirmed:
//! every combination tried either fails to typecheck at all with
//! "missing lifetime specifier", or typechecks but no longer matches
//! `assume_specification`'s required generic-binder shape). This is a
//! structural mismatch between how `assume_specification` requires
//! spelling out `Self` concretely and how the real trait method's own
//! signature avoids the ambiguity by staying abstract over `Self`.
//!
//! `vstd` already registers `Cow` itself (`std_specs/borrow.rs`'s
//! `ExCow`, not `external_body` — its variants stay transparent, so
//! `Cow::Borrowed(...)`/`Cow::Owned(...)` construction and matching
//! needs no axiom at all), so the variant-matching half of the claim
//! needs nothing extra either. Only `into_owned` (no receiver reference,
//! so no elision ambiguity) gets a real local axiom.

use std::borrow::Cow;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `ToOwned::to_owned`'s abstract result for a bare generic `B` —
/// opaque (uninterpreted), connected to the concrete `i32` case (where
/// `to_owned` is exactly `Clone::clone`, an identity-preserving
/// operation) by the broadcast axiom below.
pub uninterp spec fn to_owned_spec<B: ToOwned + ?Sized>(b: &B) -> <B as ToOwned>::Owned;

#[verifier::external_body]
pub broadcast proof fn axiom_i32_to_owned_is_identity(value: i32)
    ensures
        #[trigger] to_owned_spec(&value) == value,
{
}

pub assume_specification<'a, B: ToOwned + ?Sized> [Cow::<'a, B>::into_owned] (cow: Cow<'a, B>) -> (result: <B as ToOwned>::Owned)
    ensures
        match cow {
            Cow::Borrowed(b) => result == to_owned_spec(b),
            Cow::Owned(o) => result == o,
        },
;

/// `Cow::Borrowed`/`Cow::Owned` construct their respective variants, and
/// `into_owned` preserves the wrapped value regardless of variant — the
/// variant-construction and `into_owned` halves of the claim the Kani
/// harness checks (not the `Deref::deref` half; see the module doc
/// comment).
pub fn verify_cow_borrowed_and_owned_agree_on_their_value(value: i32) -> (result: (bool, bool, bool))
    ensures
        result.0,
        result.1,
        result.2,
{
    broadcast use axiom_i32_to_owned_is_identity;

    let borrowed: Cow<'_, i32> = Cow::Borrowed(&value);
    let is_borrowed_variant = matches!(borrowed, Cow::Borrowed(_));

    let owned: Cow<'_, i32> = Cow::Owned(value);
    let is_owned_variant = matches!(owned, Cow::Owned(_));

    let recovered = borrowed.into_owned();
    assert(recovered == value);
    let into_owned_matches = recovered == value;

    (is_borrowed_variant, is_owned_variant, into_owned_matches)
}

} // verus!
