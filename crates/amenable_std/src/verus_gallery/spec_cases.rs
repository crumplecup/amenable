//! Gallery findings about `assume_specification` mechanics: `#[cfg(verus)]`
//! is never set, a duplicate `assume_specification` ICE, `Layout::new`'s
//! opaque size/align, and `Cell`'s hidden interior state.

use crate::{
    VerusGalleryCase, VerusGalleryDisposition, VerusGalleryExpectation, VerusGalleryRegistration,
};

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::cfg_verus_is_never_actually_set".to_owned(),
            "#[cfg(verus)] is a declared check-cfg name, not a cfg the real verus binary ever sets — gating proof content behind it silently strips it".to_owned(),
            VerusGalleryDisposition::FalseTrail,
            VerusGalleryExpectation::Unproved,
            r#"
// Hypothesis (by analogy with amenable_kani's #[cfg(kani)] and
// amenable_creusot's #[cfg(creusot)], both of which really are set by
// their respective toolchains): gate Option/Result proof content behind
// #[cfg(verus)] so it's invisible to plain, non-Verus rustc/clippy —
// amenable_verus/Cargo.toml's own `check-cfg` list even declares
// 'cfg(verus)' as a possible name, which reads as confirmation.
#![cfg(verus)]
verus! {
    pub fn verify_option_unwrap_returns_the_wrapped_value(/* ... */) { /* ... */ }
}

// Observed: this compiles with no error under `verus --crate-type=lib`
// (declaring a cfg name via check-cfg only silences the "unexpected cfg"
// lint — it never implies the cfg is ever set to true by anything).
// `just verify-verus`'s own reported proof count DROPPED (5 -> 4) with
// the gate in place versus without it: the real verus binary does not
// set cfg(verus) either, so the gated block compiles out under real
// verus compilation too, not just under plain rustc — the content was
// being silently skipped, not silently protected.

// Fix: no cfg gate at all. amenable_verus has no plain-rustc build to
// protect content from in the first place (it is never a dependency of
// anything — see amenable_verus::lib's own module doc comment) — the
// clippy-visibility problem this hypothesis was trying to solve
// (clippy::unnecessary_literal_unwrap) was fixed instead by restructuring
// the proof itself (see option_carrier.rs/result_carrier.rs: take the
// Option/Result as a `requires`-constrained parameter, not a literal
// constructed inline).
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::try_from_int_error_occurs_via_duplicate_assume_specification_ice".to_owned(),
            "declaring assume_specification for a trait method vstd already specifies crashes verus outright, not a diagnosed conflict".to_owned(),
            VerusGalleryDisposition::FalseTrail,
            VerusGalleryExpectation::Ice,
            r#"
// Attempt: axiomatize u8::try_from(i32), the same claim amenable_kani's
// verify_try_from_int_error_occurs_exactly_when_out_of_range harness
// checks over every possible i32.
pub assume_specification [<u8 as std::convert::TryFrom<i32>>::try_from] (value: i32) -> (result: Result<u8, <u8 as std::convert::TryFrom<i32>>::Error>)
    ensures
        (0 <= value && value <= u8::MAX as i32) ==> (result is Ok && result->Ok_0 == value as u8),
        (value < 0 || value > u8::MAX as i32) ==> result is Err,
;

// Observed under `verus --crate-type=lib` — NOT a diagnosed error:
//   thread 'rustc' panicked at vir/src/traits.rs:511:13:
//   assertion failed: !method_impls.contains(&p)
// Confirmed this is a genuine internal crash, not a syntax problem: the
// exact same panic reproduces regardless of surface form (fully
// qualified `<u8 as TryFrom<i32>>::try_from` vs. the short `u8::try_from`
// path, with or without an explicit `<u8 as TryFrom<i32>>::Error`
// associated-type return — every variant that reaches signature-match
// crashes identically).

// Root cause, found by reading vstd's own source
// (vstd/std_specs/convert.rs), not guessed: vstd ALREADY declares an
// assume_specification for this exact trait-method instantiation, via
// its impl_int_try_from_spec! macro (`impl_int_try_from_spec! { i32 =>
// [u8 u16 u32 u64 u128 i8 i16 usize isize] }`), with real, matching
// semantics (`if Self::MIN <= v <= Self::MAX { Ok(v as Self) } else {
// Err(arbitrary()) }`, `obeys_try_from_spec()` unconditionally true for
// this pair). A second, local assume_specification for the identical
// (Self, T) instantiation doesn't produce a diagnosed "already declared"
// error the way redeclaring an ordinary Rust item would — verus's
// internal trait-impl bookkeeping (`vir::traits`) asserts the impl slot
// is unclaimed and panics when it finds it already is.

// Fix: don't declare a local assume_specification for a trait method
// vstd already specifies at all — just call it. amenable_verus::rust_std
// ::try_from_int_error_carrier's real, working proof relies on vstd's
// own spec directly and states the same postcondition as its own
// function-level ensures clause instead, with no local
// assume_specification for try_from whatsoever.

// General lesson: before writing a new assume_specification for any std
// trait method, check vstd's own std_specs/*.rs for an existing one
// first (as amenable_std::creusot_gallery's own findings already
// establish for Creusot's extern_spec! equivalent) — not just to avoid
// duplicate effort, but because here a duplicate isn't merely wasted
// work, it crashes the toolchain.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::layout_new_size_and_align_are_opaque_even_for_primitives".to_owned(),
            "Layout::new::<i32>()'s size/align values are unprovable: vstd deliberately treats size_of/align_of as fully opaque, even for i32".to_owned(),
            VerusGalleryDisposition::FalseTrail,
            VerusGalleryExpectation::Unproved,
            r#"
// Attempt: the same claim amenable_kani's verify_layout_new_reports_
// the_types_size_and_alignment harness checks — Layout::new::<i32>()
// reports size 4, align 4.
pub assume_specification [Layout::new::<i32>] () -> (result: Layout)
    ensures
        result.size() == 4,
        result.align() == 4,
;

// Observed under `verus --crate-type=lib`: signature mismatch first —
// the real Layout::new is generic over T, so a concrete i32
// instantiation is rejected outright (same shape as the Reverse::cmp
// and TryFromIntError findings above):
//   error: assume_specification requires function type signature to
//   match ... exactly ... expected: `for<T> () -> Layout`

// Root cause, found by reading vstd's own source (vstd/layout.rs), not
// guessed: vstd already gives real, working specs for
// core::mem::size_of::<V>()/align_of::<V>() — but as `uninterp spec fn
// size_of<V>() -> nat` / `align_of<V>() -> nat`, deliberately left
// UNCONSTRAINED for every V, primitives included. The file's own
// comment explains why: "we are NOT creating an axiom that size_of fits
// in usize" (soundness concern about reasoning over arbitrarily large,
// possibly-unmonomorphized generic types in ghost code). So even
// switching to the correct generic form
// (`pub assume_specification<T> [Layout::new::<T>] () -> (result:
// Layout) ensures result.size() == size_of::<T>() as usize, ...`) only
// relates the result to size_of::<T>()'s ABSTRACT value — never to a
// concrete number like 4, for ANY T, not just i32. There is no path
// from this crate to the concrete fact "size_of::<i32>() == 4": the
// opacity is deliberate upstream design in vstd itself, not a gap we
// could close with our own assume_specification (declaring one that
// pins size_of::<i32>() to 4 would itself be a second, conflicting
// assume_specification for a function vstd already specifies — see the
// duplicate-assume_specification ICE finding above; the same crash
// would very likely recur here too).

// Real coverage lands on the independent half of the claim instead
// (amenable_verus::rust_std::layout_carrier's actual, live proof):
// Layout::from_size_align rejects a non-power-of-two alignment — a pure
// fact about the constructor's own validation logic, provable without
// ever touching size_of/align_of's opacity.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    VerusGalleryRegistration::new(
        || VerusGalleryCase::new(
            "amenable_std::verus_gallery::cell_hidden_state_unreachable_via_plain_assume_specification".to_owned(),
            "Cell<T>'s get/set/replace/take can't be chained: assume_specification only relates one call's own inputs/outputs, never a prior call's effect".to_owned(),
            VerusGalleryDisposition::FalseTrail,
            VerusGalleryExpectation::Unproved,
            r#"
// Attempt: the same claim amenable_kani's verify_cell_get_set_replace_
// take_round_trip harness checks — new stores the initial value, set
// overwrites it, replace overwrites it and hands back the old value,
// take does the same against T::default().
#[verifier::reject_recursive_types(T)]
#[verifier::external_type_specification]
#[verifier::external_body]
pub struct ExCell<T: core::marker::MetaSized>(std::cell::Cell<T>);

pub assume_specification<T> [std::cell::Cell::<T>::new] (value: T) -> (result: std::cell::Cell<T>);
pub assume_specification<T: Copy> [std::cell::Cell::<T>::get] (cell: &std::cell::Cell<T>) -> (result: T);
pub assume_specification<T> [std::cell::Cell::<T>::set] (cell: &std::cell::Cell<T>, value: T);
pub assume_specification<T> [std::cell::Cell::<T>::replace] (cell: &std::cell::Cell<T>, value: T) -> (result: T);
pub assume_specification<T: Default + Default> [std::cell::Cell::<T>::take] (cell: &std::cell::Cell<T>) -> (result: T);

pub fn verify_cell_round_trip(initial: i32) -> (result: i32)
    ensures
        result == initial,
{
    let cell = std::cell::Cell::new(initial);
    cell.get()
}

// Getting the bounds to even reach the signature-match stage needed two
// real fixes along the way, not guesses: ExCell's real bound is
// `T: core::marker::MetaSized` (a newer nightly supertrait of `Sized`
// this toolchain's std uses — Verus compares bound lists structurally,
// not by trait implication, so `T: Sized` alone doesn't satisfy it even
// though Sized: MetaSized); Cell::take's real where-clause lists
// `T: Default` TWICE (an upstream quirk), which the proxy has to match
// literally.

// Observed under `verus --crate-type=lib`, once past both bound issues:
//   error: postcondition not satisfied
//     result == initial
// Root cause: none of the assume_specification declarations above have
// an ensures clause connecting them to each other — and none CAN,
// because assume_specification only states a fact about ONE function's
// own arguments and return value. Cell's whole contract is inherently
// relational across calls (what get() returns depends on what a PRIOR
// set()/new() call did through the SAME shared reference) — the same
// class of "hidden state behind a shared reference" problem vstd's own
// answer for Cell-like types (pcell::PCell) solves with an entirely
// different API shape: explicit Tracked<PermissionToken> objects
// threaded through every call, not std::cell::Cell's plain &self
// methods. There is no way to retrofit that onto the REAL, unmodified
// std::cell::Cell from outside vstd — assume_specification has no
// mechanism for "this call's postcondition may reference a previous
// call's effect."

// Not attempted: no known workaround from a downstream crate. Would
// need vstd itself to ship a real spec module for std::cell::Cell
// (as it does, differently, for Cell-like PCell) before this becomes
// provable.
"#.to_owned(),
        ),
    )
}
