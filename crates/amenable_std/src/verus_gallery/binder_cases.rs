//! Gallery findings about lifetime/binder and match-ergonomics friction:
//! `TryFrom<&[T]>`'s phantom lifetime binder, `Cow`'s deref lifetime
//! elision, and cross-file spec-fn reuse getting real proof credit.

use crate::{
    VerusGalleryCase, VerusGalleryDisposition, VerusGalleryExpectation, VerusGalleryRegistration,
};

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::try_from_slice_phantom_lifetime_binder_and_match_ergonomics".to_owned(),
            "<[T; N]>::try_from(&[T]) needed a phantom outer lifetime binder to match, and a match expression on the result doesn't see the call's own postcondition -- both solved, full claim proved".to_owned(),
            VerusGalleryDisposition::BestPractice,
            VerusGalleryExpectation::Proved,
            r#"
// Real claim, now proved in full in amenable_verus::rust_std::
// try_from_slice_carrier: <[T; N]>::try_from(&[T]) succeeds exactly
// when the slice's length matches N, round-tripping the elements
// otherwise fails -- the same claim amenable_kani's own harness checks,
// with no case dropped or weakened.

// Lesson 1 (signature matching): verus prints this kind of generic
// TryFrom impl as TWO separate binder groups (for<'_0, T, N> for<'_>),
// and only the SECOND governs the argument's actual lifetime. Declaring
// the lifetime tied to the argument (&'a [T]) as a single combined
// for<'a, T, N> group does NOT match. What matches: put the lifetime in
// the TRAIT REFERENCE only (TryFrom<&'a [T]>) and leave the argument
// itself elided (bare &[T], not &'a [T]):
pub assume_specification<'a, T: Copy, const N: usize> [<[T; N] as core::convert::TryFrom<&'a [T]>>::try_from] (slice: &[T]) -> (result: Result<[T; N], TryFromSliceError>)
    ensures
        slice@.len() == N ==> (result is Ok && result->Ok_0@ == slice@),
        slice@.len() != N ==> result is Err,
;

// Lesson 2 (match ergonomics): once the axiom above compiled, a plain
// match on the call's result didn't let its postcondition reach the Ok
// arm at all -- both facts about the returned array read as completely
// unknown:
match <[i32; 2]>::try_from(matching) {
    Ok(arr) => arr[0] == matching[0] && arr[1] == matching[1],  // "postcondition not satisfied"
    Err(_) => false,
}
// Fix: bind the call's result to a `let` first, assert its shape, then
// `.unwrap()` it -- the SAME real call, but broken into steps verus's
// own reasoning can follow:
let converted = <[i32; 2]>::try_from(matching);
assert(converted is Ok);
let arr = converted.unwrap();
arr[0] == matching[0] && arr[1] == matching[1]  // verifies cleanly

// Both lessons generalize beyond this one type: any future const-
// generic-array TryFrom axiom needs the same phantom-lifetime shape,
// and any proof consuming an assume_specification'd Result should
// prefer let+assert+unwrap over a bare match when the postcondition
// needs to be visible inside the branch.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::cow_deref_lifetime_elision_ambiguity".to_owned(),
            "Cow<'a, B>::deref can't be axiomatized: spelling the receiver out concretely creates a lifetime ambiguity plain Rust elision can't resolve".to_owned(),
            VerusGalleryDisposition::FalseTrail,
            VerusGalleryExpectation::CompileError,
            r#"
// Attempt: the deref half of the claim amenable_kani's
// verify_cow_borrowed_and_owned_agree_on_their_value harness checks --
// Cow::Borrowed and Cow::Owned both deref to the wrapped value.
pub assume_specification<'a, B: ToOwned + ?Sized> [<Cow<'a, B> as core::ops::Deref>::deref] (cow: &Cow<'a, B>) -> (result: &B);

// Observed under `verus --crate-type=lib`:
//   error[E0106]: missing lifetime specifier
//   this function's return type contains a borrowed value, but the
//   signature does not say which one of `cow`'s 2 lifetimes it is
//   borrowed from
// Real std::ops::Deref::deref is `fn deref(&self) -> &Self::Target` --
// its return elides to `&self`'s own lifetime with no ambiguity,
// because `Self` stays abstract in the trait definition. Spelling the
// receiver out concretely as `&Cow<'a, B>` (required to name the
// function for assume_specification at all) introduces a SECOND,
// competing candidate lifetime -- Cow's own `'a` -- that Rust's plain
// elision rules cannot disambiguate between.

// Tried every combination of naming/eliding both lifetimes:
//   (result: &'a B)                          -- typechecks, but then
//     doesn't match assume_specification's required generic-binder
//     shape (`for<'_0, B> for<'_> (&Cow<'_0, B>) -> &B`, a BARE `&B`
//     return with no name)
//   cow: &Cow<'_, B> ... -> &B                -- same "missing lifetime
//     specifier" error as the fully-named version; anonymizing Cow's
//     own lifetime with `_` doesn't remove the ambiguity, since there
//     are still two candidate sources
//   cow: &'b Cow<'a, B> ... -> &'b B           -- typechecks (like
//     TryFromSliceError's phantom-lifetime fix), but produces a single
//     combined `for<'a, 'b>` binder group, not the required TWO
//     separate groups
// Every variant either fails to typecheck as ordinary Rust at all, or
// typechecks into a shape assume_specification's exact-match
// requirement rejects. This is a different KIND of blocker than
// TryFromSliceError's (that one was about binder ORDER once the
// underlying signature was unambiguous; this one is a genuine
// unresolvable ambiguity in the concrete spelling itself).

// Real, narrower coverage lands instead (amenable_verus::rust_std::
// cow_carrier's actual, live proof): the variant-construction facts
// (Cow::Borrowed(_)/Cow::Owned(_) pattern matching needs no axiom,
// vstd's own ExCow registration keeps Cow's variants transparent) plus
// the full into_owned claim (no receiver reference, so no elision
// ambiguity at all) -- covering two of the claim's three original
// facts in full, with only the deref half left uncovered.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::cross_file_spec_fn_reuse_gets_real_proof_credit".to_owned(),
            "a pub open spec fn defined in one carrier file, called from a requires clause in a sibling carrier file, verifies for real -- not opaque the way Creusot's cross-module #[logic] calls are".to_owned(),
            VerusGalleryDisposition::BestPractice,
            VerusGalleryExpectation::Proved,
            r#"
// Investigated while designing a "single source living with the
// contract type" mechanism for amenable_core::Requires<VerusVerifier>/
// Ensures<VerusVerifier> (Kani already reached this: its Bound = bool,
// so Requires::requires() IS the real check, called directly at the
// proof site instead of restating the expression -- see
// amenable_kani::rust_std::primitives's AsciiByte::requires). Verus
// spec content isn't executable Rust, so it can never be `Bound`
// itself, but the question was whether the DUPLICATION across many
// carrier files' own requires/ensures clauses could still collapse to
// one real, verified definition, the way `amenable_creusot::rust_std`'s
// existing `is_ascii_digit` #[logic(open)] fn already does for Creusot
// (five real call sites, same file, genuinely proven -- see
// amenable_std::creusot_gallery's view_operator_needs_pearlite_macro_
// outside_attributes case for that fn's own history).
//
// elicitation_verus's gallery::level12 imports lemmas/spec fns from
// gallery::level11 (a DIFFERENT file) and its header claims this
// proves -- unlike elicitation_creusot's gallery::level12, which
// documents the identical cross-MODULE #[logic] call going opaque
// under Creusot. Confirmed for real against this repo's own
// IncrementHeadroom precondition (recurs across iter_sequence_carrier,
// iter_stateful_carrier, iter_transform_carrier, primitive_shapes_
// carrier, slice_chunks_carrier -- with the real proof sites now
// calling shared named spec fns rather than restating raw arithmetic):
pub open spec fn increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX - 1
}
// ... in iter_sequence_carrier.rs, called from a requires clause in
// THAT same file, AND (via `use crate::rust_std::iter::
// iter_sequence_carrier::increment_headroom_holds;`) from
// iter_stateful_carrier.rs, a different file:
pub fn verify_cycle_model_repeats_its_sequence_forever(a: i32) -> (result: (i32, i32, i32, i32))
    requires
        increment_headroom_holds(a),
    ...

// Observed under `verus --crate-type=lib crates/amenable_verus/src/
// lib.rs`: verification results:: 332 verified, 0 errors -- no
// regression, both call sites get real credit for the shared
// definition, exactly like Creusot's is_ascii_digit.
//
// The real remaining gap, on both backends, isn't "can the proof sites
// share a real definition" (yes, within one crate) -- it's that
// amenable_std (where Requires<VerusVerifier>::requires()'s &'static
// str fragment is registered, for cross-backend enumeration) can never
// see amenable_verus at all: verus is invoked as a bare compiler over
// a single file tree, never reads Cargo.toml, so it cannot resolve
// amenable_core/amenable_std/inventory/any proc-macro crate (see
// amenable_verus::lib's own doc comment, confirmed empirically there
// already). Creusot's version of the same gap is a real Cargo
// dependency cycle: amenable_std optionally depends on
// amenable_creusot (to include_str! its real proof source), so
// amenable_creusot structurally cannot depend back on amenable_std.
// Neither gap is a macro-expansion-order problem -- nesting either
// side's macro inside the other's attribute/DSL position wouldn't
// route around a Cargo dependency cycle or a file-based, Cargo-blind
// invocation model. The fragment text registered in amenable_std stays
// a manually-kept-in-sync transcription of the shared spec fn/logic
// fn's real body, verified only by cordial's contract-bound
// scanner -- which is the correct layer for this specific boundary,
// not a fallback: this boundary structurally cannot carry an
// executable, provable connection across it, unlike the boundary
// between a Kani contract type and its own crate's proof sites.
"#.to_owned(),
        ),
    )
}
