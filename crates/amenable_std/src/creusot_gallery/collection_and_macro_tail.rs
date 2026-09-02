//! Findings about bare integer literal comparison, `BinaryHeap` having no
//! local fix, `TryReserveError` not being publicly constructible, and a macro
//! invocation nested inside attribute position never getting pre-expanded.

use super::model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::bare_integer_literal_comparison_is_unsupported"
                .to_owned(),
            "Comparing an exec-typed integer to a bare literal in a contract needs the `@` View operator on both sides"
                .to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact shape was in amenable_creusot::rust_std's argv
// accommodation model, and separately in its std::os::windows Handle/
// EncodeWide models, before the fix -- same root cause, three sites):
#[ensures(result.0 >= 1)]
fn verify_args_reports_at_least_the_program_path(extra: usize) -> (usize, usize) {
    (1 + extra, extra)
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error[E0308]: mismatched types
//   expected `usize`, found `Int`
//   note: method defined here --> creusot-std-0.11.0/src/logic/ord.rs
//   fn ge_log(self, o: Self) -> bool

// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today):
#[ensures(result.0@ >= 1)]
fn verify_args_reports_at_least_the_program_path(extra: usize) -> (usize, usize) {
    (1 + extra, extra)
}
// A bare integer literal inside a `#[requires]`/`#[ensures]` clause is
// inferred as Pearlite's arbitrary-precision `Int`, not the concrete
// exec type (`usize` here) -- so `result.0 >= 1` tries to compare a
// `usize` against an `Int` and fails to typecheck, the same "wrong
// logic-level type" shape `char_as_u32_cast_is_unsupported` documents
// for casts rather than comparisons. `@` on the exec-typed side (or on
// both, for `==`/`<`/`<=` chains against other exec values, e.g.
// `code_point@ < 0x10000`) puts everything in `Int` uniformly, which is
// what the literal was already inferred as. This is not specific to
// `usize`: the same fix applies to `isize`/`u64`/`u32`/any integer type
// compared against a bare literal in logic context.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::binary_heap_has_no_local_fix_either".to_owned(),
            "creusot-std ships no BinaryHeap contracts, and a downstream crate can't add a View for it either -- resolved with an accommodation model instead of extending the real type".to_owned(),
            CreusotGalleryDisposition::BestPractice,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Attempted fix for amenable_creusot::rust_std's five #[trusted]
// BinaryHeap<i32> proofs (verify_binary_heap_{pop,drain,into_iter,iter,
// peek_mut}_...): give BinaryHeap<i32> a real View (the `@` operator's
// backing trait) so a local extern_spec! could state real, checked
// push/pop/len contracts in terms of Seq<i32> -- the same "local
// extern_spec! fills a creusot-std gap" move that already worked for
// Ordering::reverse, Wrapping<i32>, and ManuallyDrop<T> elsewhere in
// this file.

// Attempt 1: implement View directly.
impl creusot_std::model::View for std::collections::BinaryHeap<i32> {
    type ViewTy = creusot_std::logic::Seq<i32>;
    #[trusted]
    #[logic(opaque)]
    fn view(self) -> creusot_std::logic::Seq<i32> { dead }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error[E0117]: only traits defined in the current crate can be
//   implemented for types defined outside of the crate
//
// A genuine orphan-rule violation, not a Creusot-specific restriction:
// `View` is defined in creusot_std, `BinaryHeap` in std -- neither is
// local to amenable_creusot, so plain rustc rejects this exactly the way
// it would reject `impl serde::Serialize for std::vec::Vec<T>` from a
// crate that owns neither. `Ordering`/`Ord::reverse` and
// `Wrapping<i32>`'s Add impl worked around the *analogous* problem for
// extern_spec! itself only because extern_spec! is a macro that doesn't
// go through a real trait impl at all -- View has no such macro-based
// escape hatch available here.

// Attempt 2 (hypothetical, not even worth trying given attempt 1): even
// with a View somehow in place, calling the real, still-uncontracted
// BinaryHeap::pop/push *inside* the same function as the checked
// #[ensures] would hit the identical false trail this gallery's own
// `get_disjoint_mut_...` case already documents: "a contractless
// external call can let a harness 'prove' for the wrong reason" (an
// impossible precondition makes the goal vacuously true). Any real fix
// would need *every* BinaryHeap method the harness calls extern_spec'd
// too, not just the ones referenced from #[ensures] -- and attempt 1
// already blocks doing that for even one method.

// Resolution (implemented, not hypothetical): replaced all five
// #[trusted] real-API harnesses with a genuine synthetic accommodation
// model that doesn't name BinaryHeap at all -- the same move
// amenable_creusot::rust_std's argv proofs already made, and the same
// move amenable_kani::btree_model.rs describes for BTreeMap/BTreeSet.
// Each harness states the max-first ordering law directly over an
// explicit two-element pair instead of calling the real type:
//
// fn verify_binary_heap_pop_yields_the_maximum_first(a: i32, b: i32) -> (Option<i32>, Option<i32>) {
//     let first = Some(if a >= b { a } else { b });
//     let second = Some(if a >= b { b } else { a });
//     (first, second)
// }
// #[ensures(match result {
//     (first, second) =>
//         first == Some(if a >= b { a } else { b })
//             && second == Some(if a >= b { b } else { a }),
// })]
//
// This is not `#[trusted]` -- it's a real, checked postcondition over
// arithmetic Creusot can actually discharge (confirmed: `cargo creusot
// prove -- -p amenable_creusot` went from 54 to 59 proved files after
// converting all five proofs this way). The trade-off named in the
// conclusion below is real and was accepted deliberately: the model
// never touches BinaryHeap, so it proves the *law* the real type is
// expected to refine, not the real type's own behavior directly --
// exactly `amenable_kani::btree_model`'s own documented relationship
// between its modeled proofs and the real B-tree collections. The
// original proofs' drop-count observations (does pop/drain/into_iter
// transfer ownership without dropping, does the remainder get dropped
// the right number of times on scope exit) were dropped from the model
// entirely rather than kept unchecked: Creusot has no way to reason
// about `Drop::drop` call counts for any container, real or modeled, so
// there was nothing for the model to state there either. Kani's own
// proofs for this cluster keep that half of the claim, since Kani's
// bounded symbolic execution has no analogous restriction.
//
// Conclusion: unlike the bare-integer-literal and char-as-u32 cases
// above, this was never a "use the right idiom" fix -- extending the
// real type locally is a genuine dead end (two independent blockers,
// confirmed above), and an accommodation model was the actual way
// through, not just the fallback if nothing better turned up.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::try_reserve_error_is_not_publicly_constructible"
                .to_owned(),
            "TryReserveError has no public constructor, so it blocks even the accommodation-model escape hatch that fixed HashMap/BinaryHeap/LinkedList"
                .to_owned(),
            CreusotGalleryDisposition::BestPractice,
            CreusotGalleryExpectation::TranslationError,
            r#"
// amenable_creusot::rust_std::verify_try_reserve_rejects_an_impossible_capacity
// stayed #[trusted] after the HashMap/HashSet/LinkedList/VecDeque/Result-iterator
// accommodation-model sweep that resolved every other "creusot-std has no
// contracts for this carrier" harness in this file. The difference: those
// carriers' claims could be restated over an `Option<i32>`/`bool`/`usize`
// return value built entirely from ordinary, publicly-constructible types.
// This claim's return value includes `Option<TryReserveError>`, and
// `TryReserveError` (std::collections::TryReserveError) has no public
// constructor at all -- every field is private, and its only stable public
// surface is `.kind()` (via TryReserveErrorKind, itself unstable) plus
// Display/Debug/Error. There is no `TryReserveError::new(..)` or public
// variant to write `Some(TryReserveError { .. })` the way `Some(byte)` or
// `Some(0u8)` could stand in for CString's terminator byte.
//
// So the accommodation-model move used everywhere else in this cluster --
// "state the law directly over the return value instead of calling the
// real, uncontracted API" -- has no way to produce a real value of the
// return type without calling the real, uncontracted
// `Vec::try_reserve(usize::MAX)` that produces it. This is a different
// shape of wall than BinaryHeap's orphan-rule block (a real trait-impl
// restriction) or NonZero::new's sealed ZeroablePrimitive bound (a real
// generic-bound restriction): here the blocker is that the return type
// itself is opaque and non-constructible from outside std, which rules out
// the "avoid the real type, state the law" idiom structurally, not just
// practically.
//
// Conclusion: kept #[trusted], the same category as NonZero::new and
// f64::classify above -- a genuinely confirmed wall, not a "didn't try hard
// enough" gap. Worth naming so the next reviewer doesn't re-attempt the
// same accommodation-model conversion and re-discover the same
// non-constructible-type blocker.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::macro_invocation_inside_an_attribute_is_never_pre_expanded".to_owned(),
            "a macro_rules! invocation inside #[requires(...)]'s argument position is not expanded before Creusot's own Pearlite parser sees it".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Investigated as part of the same "single source living with the
// contract type" effort as amenable_std::verus_gallery's
// cross_file_spec_fn_reuse_gets_real_proof_credit case: if a bound's
// literal expression could be written once as a macro_rules! invocation
// and used both inside a real #[requires(...)]/#[ensures(...)] clause AND
// wherever amenable_std::Requires/Ensures needs it, that would be a real
// single source spanning the amenable_std/amenable_creusot crate
// boundary -- worth checking before concluding the boundary can only ever
// be scanner-verified.

#[cfg(creusot)]
macro_rules! probe_bound {
    () => {
        value@ < 10
    };
}

#[requires(probe_bound!())]
fn probe_macro_nesting_in_attribute_position(value: i32) -> i32 {
    value
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: Unsupported expression: macros other than `pearlite!`,
//   `proof_assert!` or `seq!` are unsupported in Pearlite code.
//     --> #[requires(probe_bound!())]
// Creusot's `#[requires]`/`#[ensures]` attribute proc-macro receives its
// argument tokens and parses them itself, via its own Pearlite grammar --
// it does NOT first hand the tokens to rustc's ordinary macro-expansion
// pass the way a ordinary expression position would. A nested
// macro_rules! invocation reaches Creusot's parser completely
// unexpanded, as the literal tokens `probe_bound ! ( )`, which its
// Pearlite grammar explicitly rejects (only `pearlite!`/`proof_assert!`/
// `seq!` are recognized as legal nested macros -- an explicit allowlist,
// not a general macro-expansion mechanism).
//
// This settles the whole "nest a shared macro inside their attribute"
// family of approaches, not just the declarative-macro case tested here:
// attribute macros process raw, unexpanded input by design (this is how
// Rust's macro system works generally, not a Creusot-specific quirk), so
// a proc-macro invocation nested the same way would hit the identical
// wall -- Creusot's parser has no mechanism to recursively expand or
// delegate to an arbitrary external macro mid-parse, only to its own
// fixed allowlist.
//
// Real path forward for a genuine single source across this boundary:
// generate the real attribute's literal tokens (and amenable_std's
// registered fragment) from one shared spec, as a build-time text/codegen
// step (before rustc ever parses either file), not as a macro nested
// inside Creusot's own attribute-position parsing.
"#.to_owned(),
        ),
    )
}
