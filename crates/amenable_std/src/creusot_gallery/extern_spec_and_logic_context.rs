//! Findings about the boundary between Pearlite logic context and ordinary
//! program code: `#[check(ghost)]` extern_spec methods, `NonZero::new`'s
//! sealed trait bound, uncontracted calls poisoning a whole goal, the `@` view
//! operator outside attribute position, and `str::parse`'s uncontracted
//! `FromStr` wrapper.

use super::model::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::check_ghost_extern_spec_methods_are_still_program_functions".to_owned(),
            "creusot-std's own #[check(ghost)] extern_spec methods can't be called inside #[ensures] either".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact shape was in amenable_creusot::rust_std's
// Duration contract before the fix — calling creusot-std's own trusted
// extern_spec methods for Duration::as_secs/subsec_nanos directly inside
// #[ensures]):
#[ensures(result.as_secs()@ == secs@ + (nanos@ / 1_000_000_000))]
#[ensures(result.subsec_nanos()@ == nanos@ % 1_000_000_000)]
fn verify_duration_new_normalizes_nanos_and_carries_into_secs(secs: u64, nanos: u32) -> Duration {
    Duration::new(secs, nanos)
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: called program function `std::time::Duration::as_secs` in
//   logic context
//   error: called program function `std::time::Duration::subsec_nanos`
//   in logic context
// `creusot_std::std::time`'s extern_spec! block marks Duration::as_secs/
// subsec_nanos/etc. with #[check(ghost)], which gives them a real
// postcondition creusot-rustc trusts — but #[check(ghost)] only makes a
// function callable from GHOST program context (ghost! blocks and the
// like), not from Pearlite LOGIC context (#[requires]/#[ensures]). Same
// underlying restriction as the plain String::len() case, just easy to
// miss here because the method genuinely does have a stated
// postcondition, unlike String::len().

// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today) — expressed via `result@` (Duration's
// View, its total nanosecond count as Pearlite's Int) and creusot-std's
// own PUBLIC #[logic(open)] helper functions (nanos_to_secs et al, from
// creusot_std::std::time — plain logic functions, not extern_spec
// methods, so freely callable) instead of calling the methods at all:
#[ensures(nanos_to_secs(result@) == secs@ + (nanos@ / 1_000_000_000))]
#[ensures(result@ % 1_000_000_000 == nanos@ % 1_000_000_000)]
fn verify_duration_new_normalizes_nanos_and_carries_into_secs(secs: u64, nanos: u32) -> Duration {
    Duration::new(secs, nanos)
}
// These are the exact terms as_secs/subsec_nanos's own postconditions are
// stated in, so this proves the same underlying fact their (untouchable)
// contracts would give, without ever invoking the methods themselves.
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::nonzero_new_extern_spec_needs_a_sealed_unstable_trait_bound".to_owned(),
            "extern_spec!-ing NonZero<T>::new isn't practical: it needs the sealed, unstable ZeroablePrimitive bound".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact shape was in amenable_creusot::rust_std before
// the fix — an extern_spec targeting the concrete NonZero<i16> alone,
// bypassing the generic std::num::NonZero<T>::new):
extern_spec! {
    impl NonZero<i16> {
        #[check(ghost)]
        #[ensures(value != 0i16 ==> match result { Some(_) => true, None => false })]
        #[ensures(value == 0i16 ==> match result { Some(_) => false, None => true })]
        fn new(value: i16) -> Option<NonZero<i16>>;
    }
}

// Observed under `cargo creusot -- -p amenable_creusot`:
//   error: extern spec generics don't match
//           rust_std::extern_spec_... []
//           std::num::NonZero::<T>::new [i16]
// extern_spec! requires the declared signature to match the REAL one
// structurally, generics included — `new` is defined once, generically,
// on `impl<T: ZeroablePrimitive> NonZero<T>`, not per-instantiation, so a
// non-generic `impl NonZero<i16>` extern_spec doesn't match no matter what
// the body says. Writing the generic version faithfully would need
// `impl<T: ZeroablePrimitive> NonZero<T> { fn new(value: T) -> ... }` —
// but `core::num::ZeroablePrimitive` is `pub unsafe trait ... : ... +
// private::Sealed`, and its own doc comment calls it "currently
// permanently unstable": not nameable as a bound from outside `std` on
// stable Rust at all, confirmed by reading the real std source
// (library/core/src/num/nonzero.rs), not assumed.
//
// Unlike every other case in this gallery, there's no "working form" that
// stays a real, why3find-discharged proof — `NonZero::new` genuinely
// cannot be given a contract from outside `std` under these constraints.
// The honest fallback (this is the real content, in
// amenable_creusot::rust_std today): state the same claim Kani checks by
// symbolic execution, but mark the whole harness #[trusted] rather than
// silently dropping the postconditions or pretending they're verified:
#[trusted]
#[ensures(match result { Some(_) => value != 0i16, None => value == 0i16 })]
#[ensures(match result { Some(nz) => nonzero_i16_get(&nz) == value, None => true })]
fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
    NonZero::new(value)
}
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::uncontracted_calls_poison_the_whole_goal_not_just_logic_context".to_owned(),
            "an uncontracted external call anywhere in a harness body blocks the goal, not just calls inside #[ensures]".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::Unproved,
            r#"
// Failing form (this exact shape was in amenable_creusot::rust_std before
// the fix — the postcondition matches the (o, result) PAIR structurally,
// never calling .reverse() inside #[ensures], which was expected to route
// around "called program function in logic context" the same way it did
// for char/Duration's range/decomposition claims):
#[ensures(match (o, result) {
    (Ordering::Less, Ordering::Greater) => true,
    (Ordering::Equal, Ordering::Equal) => true,
    (Ordering::Greater, Ordering::Less) => true,
    _ => false,
})]
fn verify_ordering_reverse_swaps_less_and_greater(o: Ordering) -> Ordering {
    o.reverse()
}

// Observed under `cargo creusot prove -- -p amenable_creusot` (translates
// clean, no compile error — the difference from every earlier finding
// here — but fails at the SMT stage):
//   warning: calling external function `reverse` with no contract will
//   yield an impossible precondition
//   Goal Coma.vc_verify_ordering_reverse_swaps_less_and_greater: ✘
// The earlier String::len()/Duration::as_secs() cases were about what's
// callABLE inside #[ensures] specifically. This is a different, wider
// restriction: `.reverse()` is called in the harness BODY, not inside any
// ensures clause, yet the goal still fails — an uncontracted external
// call anywhere in the function poisons the whole verification condition,
// because WP has no idea what the call actually does and assumes the
// worst (an impossible precondition) for everything downstream of it,
// logic context or not.
//
// Working form (this is the real, proven contract, in
// amenable_creusot::rust_std today) — give `Ordering::reverse` a local
// extern_spec! instead. Unlike NonZero::new, this one is actually
// practical: `reverse` has no generics and no sealed trait bound
// (`pub const fn reverse(self) -> Ordering`), so it's a real,
// why3find-discharged proof, not a #[trusted] fallback:
extern_spec! {
    impl Ordering {
        #[check(ghost)]
        #[ensures(match (self, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn reverse(self) -> Ordering;
    }
}
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::view_operator_needs_pearlite_macro_outside_attributes".to_owned(),
            "the @ View operator only parses inside #[requires]/#[ensures]; a #[logic] function body needs pearlite! {} to use it".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact shape was attempted while writing
// amenable_creusot::rust_std's IntErrorKind contract — a plain #[logic]
// helper function using `@` directly in its body, the same operator
// every #[ensures]/#[requires] clause in this crate already uses freely):
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    c@ >= 48 && c@ <= 57
}

// Observed under plain `cargo check -p amenable_creusot` (not even real
// translation — a hard parse error, before creusot-rustc is involved at
// all):
//   error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an
//   operator, found `@`
// `#[requires(...)]`/`#[ensures(...)]` attribute *arguments* are consumed
// as an opaque token stream by their own proc-macro (parsed internally
// via pearlite-syn, never by rustc's ordinary expression grammar) — `@`
// is legal there regardless of context. An ordinary function BODY,
// even one annotated `#[logic]`, is parsed by rustc's normal expression
// grammar first; `@` isn't a valid operator there at all, so this fails
// to even parse, unconditionally, `#[cfg(creusot)]` gating or not (cfg
// stripping happens after parsing, not before).

// Working form (this is the real, proven helper, in
// amenable_creusot::rust_std today) — wrap the body in creusot-std's own
// `pearlite! {}` macro, which (like `requires!`/`ensures!`) receives its
// contents as an opaque token stream too, making `@` legal inside it
// even in an ordinary expression position:
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    pearlite! { c@ >= 48 && c@ <= 57 }
}
"#.to_owned(),
        ),
    )
}

::inventory::submit! {
    CreusotGalleryRegistration::new(
        || CreusotGalleryCase::new(
            "amenable_std::creusot_gallery::str_parse_is_a_distinct_uncontracted_wrapper_around_from_str".to_owned(),
            "extern_spec-ing FromStr::from_str doesn't cover calls through str::parse<F>, a separate generic wrapper method".to_owned(),
            CreusotGalleryDisposition::FalseTrail,
            CreusotGalleryExpectation::TranslationError,
            r#"
// Failing form (this exact call shape was in amenable_creusot::rust_std's
// IntErrorKind harness before the fix — calling through the ordinary,
// idiomatic `.parse()` method, after already giving `<i32 as
// FromStr>::from_str` a real local extern_spec!):
fn verify_int_error_kind_classifies_parse_failures() -> ... {
    (
        "".parse::<i32>(),
        "not a number".parse::<i32>(),
        // ...
    )
}

// Observed under `cargo creusot -- -p amenable_creusot` (translates, but
// with a warning that predicts the same "impossible precondition" class
// of goal failure the Ordering::reverse finding hit):
//   warning: calling external function `parse` with no contract will
//   yield an impossible precondition
// `str::parse<F>(&self) -> Result<F, F::Err>` is its own distinct,
// generic method (`{ FromStr::from_str(self) }`, forwarding at runtime)
// — extern-speccing the trait method `FromStr::from_str` doesn't
// automatically cover calls made through this separate wrapper; Creusot
// reasons about exactly the function actually called, not what it
// happens to delegate to internally.

// Working form (this is the real, proven harness, in
// amenable_creusot::rust_std today) — call the contracted trait method
// directly; semantically identical at runtime, since `parse` just
// forwards to it anyway:
fn verify_int_error_kind_classifies_parse_failures() -> ... {
    (
        <i32 as std::str::FromStr>::from_str(""),
        <i32 as std::str::FromStr>::from_str("not a number"),
        // ...
    )
}
"#.to_owned(),
        ),
    )
}
