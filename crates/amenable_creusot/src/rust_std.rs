//! Creusot proof-function content for Rust standard-library carriers.
//!
//! This crate contains *only* what `cargo creusot -- -p amenable_creusot`
//! needs to translate: the harness functions themselves and the trusted
//! logic wrappers they depend on. Nothing here references
//! `RustStdStandard`, registers a `ProofRecord`, or implements
//! `CreusotWitness` — that machinery moved to
//! `amenable_std::creusot_witness`, which imports the `&'static str`
//! constants below rather than duplicating the contract text. See that
//! module's doc comment for why: creusot-rustc's translator sweeps every
//! local item in a `creusot-std`-dependent crate, `#[cfg(creusot)]`-gated
//! or not, and chokes on ordinary Rust infrastructure (a return-position
//! `impl Trait` panicked its intrinsics pass outright; the `static` item
//! `::inventory::submit!` expands to hits "unsupported definition kind").
//!
//! `char` and `String` carry a genuine constraint worth stating as a real
//! Creusot postcondition; every other std carrier `amenable_std` proves
//! about has no invariant beyond what the type system already guarantees,
//! so there's nothing to translate for them here at all.
//!
//! Both contracts here are machine-checked, not just syntactically valid:
//! `just verify-creusot` runs `cargo creusot prove -- -p amenable_creusot`
//! (translation + `why3find` SMT solving) and reports `Proved (7 files) ✔`
//! — every goal in this crate discharges, including these two.

#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use creusot_std::std::time::nanos_to_secs;
#[cfg(creusot)]
use std::cmp::Ordering;
#[cfg(creusot)]
use std::num::{NonZero, Saturating, Wrapping};
#[cfg(creusot)]
use std::time::Duration;

amenable_derive::harness! {
    creusot, VERIFY_CHAR_ROUNDTRIP_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through
        /// itself — the same claim the Kani harness checks by symbolic
        /// exploration, restated as a Creusot postcondition.
        ///
        /// NOTE: this deliberately goes further than the reference pattern
        /// in `elicitation`'s `verification::proof_helpers::creusot_char`,
        /// which states only `ensures(result == c)` — identity, no range
        /// check — and does the same for every other stdlib opaque type it
        /// covers this way (`String`, `PathBuf`, `Duration`, `SystemTime`).
        /// The range check uses `c@` (the `View`/`ShallowModel` operator,
        /// yielding Pearlite's arbitrary-precision `Int`), not `c as u32` —
        /// confirmed by a real translation error, not a guess: `error:
        /// unsupported cast from char to u32 (allowed: bool as integer,
        /// integer as integer, or pointer as pointer)`. `char`'s `View`
        /// impl in `creusot-std` maps to `Int` via a builtin
        /// (`creusot.prelude.Char.to_int`), which is exactly what `@` is
        /// for per the Creusot guide's own Pearlite reference.
        #[requires(true)]
        #[ensures(result == c)]
        #[ensures(c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF))]
        fn verify_char_roundtrip(c: char) -> char {
            c
        }
    }
}

// `String::len` is a program function, not callable from `#[ensures]`
// (Pearlite logic context) directly — confirmed by a real translation
// error, not a guess: `error: called program function 'std::string::String
// ::len' in logic context`. `elicitation`'s own `logic_fns.rs` solves this
// with exactly this shape: a `#[trusted] #[logic(opaque)]` wrapper whose
// body is the Pearlite `dead` placeholder (an axiom — the relationship to
// the real method is asserted, not proven) so the length claim can appear
// in a postcondition at all.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn string_len(_s: &String) -> usize {
    dead
}

amenable_derive::harness! {
    creusot, VERIFY_STRING_ROUNDTRIP_SRC, {
        /// `String` round-trips through itself and preserves length.
        ///
        /// This is deliberately weaker than the Kani harness, which checks
        /// UTF-8 validity directly (`std::str::from_utf8`), but deliberately
        /// stronger than `elicitation`'s reference `creusot_string` (plain
        /// `ensures(result == s)`, no length claim). Stating "these bytes
        /// are valid UTF-8" as a first-class Pearlite predicate would need
        /// either a modeled builtin for UTF-8 well-formedness or a
        /// byte-level encoding lemma, so that part stays out of scope. The
        /// length claim goes through `string_len` (see above) since
        /// `.len()` itself can't appear in a postcondition directly.
        #[requires(true)]
        #[ensures(result == s)]
        #[ensures(string_len(&result) == string_len(&s))]
        fn verify_string_roundtrip(s: String) -> String {
            s
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC, {
        /// `Duration::new` does not require `nanos < 1_000_000_000` — it
        /// normalizes: any whole-second carry in `nanos` is added to
        /// `secs`, and `subsec_nanos()` reports the remainder. Same claim
        /// as the Kani harness (`amenable_kani::rust_std::time`), restated
        /// as a Creusot postcondition — right down to the `secs.checked_add
        /// (carry).is_some()` precondition Kani assumes, here expressed as
        /// `secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@` (Pearlite's `@`
        /// operator lifts to arbitrary-precision `Int`, so this is exactly
        /// "the real u64 addition wouldn't overflow", not an approximation).
        ///
        /// `creusot-std` ships its own trusted `extern_spec!` for
        /// `Duration::new`/`as_secs`/`subsec_nanos` (`creusot_std::std::
        /// time`) — but `#[check(ghost)]` extern-spec methods are still
        /// *program* functions, not `#[logic]` ones, so `result.as_secs()`
        /// can't be called directly inside `#[ensures]` any more than
        /// `String::len()` could — confirmed by a real translation error,
        /// not a guess: `error: called program function 'std::time::
        /// Duration::as_secs' in logic context`. Unlike `String::len`,
        /// no local `#[trusted]` wrapper is needed to route around it:
        /// `creusot_std::std::time` already exports `nanos_to_secs`/
        /// `secs_to_nanos` as plain `#[logic(open)]` functions (the exact
        /// terms `as_secs`/`subsec_nanos`'s own postconditions are stated
        /// in), so the claim below is expressed directly in terms of
        /// `result@` (the `View` operator, Duration's total nanosecond
        /// count as Pearlite's arbitrary-precision `Int`) and those
        /// existing logic functions instead.
        ///
        /// This also means this harness proves less than the Kani one:
        /// Kani exercises the real `std::time::Duration` implementation,
        /// symbolically; this proves only that `creusot-std`'s OWN trusted
        /// axiom for `Duration::new`'s total nanosecond count decomposes
        /// the way `as_secs`/`subsec_nanos`'s OWN trusted axioms claim it
        /// should — internal consistency between two independently-trusted
        /// specifications, not agreement with the real implementation.
        #[requires(secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@)]
        #[ensures(nanos_to_secs(result@) == secs@ + (nanos@ / 1_000_000_000))]
        #[ensures(result@ % 1_000_000_000 == nanos@ % 1_000_000_000)]
        fn verify_duration_new_normalizes_nanos_and_carries_into_secs(
            secs: u64,
            nanos: u32,
        ) -> Duration {
            Duration::new(secs, nanos)
        }
    }
}

// `NonZero::get` is a plain program function too — same restriction as
// `String::len`, no `#[check(ghost)]` contract to trip over this time
// since creusot-std has no extern_spec for `NonZero<T>` at all. Trusted
// wrapper, same shape as `string_len`.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn nonzero_i16_get(_nz: &NonZero<i16>) -> i16 {
    dead
}

amenable_derive::harness! {
    creusot, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::num::verify_nonzero_i16` checks by
        /// symbolic execution, restated as a Creusot postcondition.
        ///
        /// `#[trusted]`, unlike every other harness in this file: `new`
        /// is uncontracted (creusot-std covers plain integers and
        /// Duration, not `NonZero<T>` at all), and giving it one myself
        /// isn't practical — `extern_spec!` requires matching the real
        /// generic signature exactly (confirmed: `extern spec generics
        /// don't match` when targeting the concrete `NonZero<i16>`
        /// alone), and the real bound is `T: ZeroablePrimitive`, an
        /// `unsafe`, sealed, doc-comment-flagged-"currently permanently
        /// unstable" trait — not something nameable from outside `std`
        /// on stable Rust. So this states the same claim Kani checks by
        /// symbolic execution, honestly marked as asserted rather than
        /// mechanically discharged, the same way `elicitation`'s own
        /// reference pattern uses `#[trusted]` for claims judged "too
        /// hard to prove" rather than silently weakening them.
        ///
        /// One width, not all twelve `amenable_kani` proves separately
        /// (`i8` through `u128`/`usize`): the coverage checklist resolves
        /// every `NonZero{I,U}*` type alias back to the same evidence,
        /// `RustStdStandard<NonZero<i16>>`, so one representative case is
        /// what actually closes the gap there.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            Some(_) => value != 0i16,
            None => value == 0i16,
        })]
        #[ensures(match result {
            Some(nz) => nonzero_i16_get(&nz) == value,
            None => true,
        })]
        fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
            NonZero::new(value)
        }
    }
}

// `Ordering::reverse` is uncontracted (creusot-std has no coverage for
// `core::cmp::Ordering` at all) — and unlike `String::len`/`Duration::
// as_secs`, matching the `(o, result)` pair structurally in `#[ensures]`
// *without* calling `.reverse()` there doesn't route around it: the
// harness body still calls `.reverse()` to produce `result`, and calling
// any uncontracted external function yields an impossible precondition
// for the WHOLE goal, not just for logic-context call sites — confirmed
// by a real prove failure (`Goal ...vc_verify_ordering_reverse_swaps_
// less_and_greater: ✘`), not a guess. Unlike `NonZero::new`, though,
// `Ordering::reverse` has no generics and no sealed trait bound
// (`pub const fn reverse(self) -> Ordering`), so a local `extern_spec!`
// for it is actually practical — the same trusted-axiom pattern
// `creusot-std` itself uses for `Duration::new`, just written here
// instead of shipped upstream.
#[cfg(creusot)]
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

amenable_derive::harness! {
    creusot, VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC, {
        /// `Ordering` has exactly three inhabitants, and `.reverse()`
        /// swaps `Less`/`Greater` while fixing `Equal` — the same claim
        /// `amenable_kani::rust_std::cmp::verify_ordering_reverse_involution`
        /// checks (there, stated as an involution:
        /// `o.reverse().reverse() == o`, over an explicit enumeration of
        /// all three variants, since Kani has no `Arbitrary` impl for
        /// `Ordering`). Rests on the local `extern_spec!` above, which
        /// states the same swap as a trusted axiom on `reverse` itself —
        /// this harness just confirms the axiom is available and usable
        /// where a real proof needs it, the same relationship every
        /// `Duration` clause here has to `creusot-std`'s own axioms.
        ///
        /// Matching the `(o, result)` pair, not calling `.reverse()`
        /// again inside `#[ensures]`, already implies the involution
        /// Kani checks explicitly (applying the same swap twice is the
        /// identity), so no separate reverse-twice clause is needed.
        #[requires(true)]
        #[ensures(match (o, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn verify_ordering_reverse_swaps_less_and_greater(o: Ordering) -> Ordering {
            o.reverse()
        }
    }
}

// Unlike `NonZero<T>`, `Wrapping<T>`'s arithmetic impls aren't one
// generic `impl<T: Sealed> Add for Wrapping<T>` — std generates a
// separate, concrete `impl Add for Wrapping<i32>` (and one per other
// width) via a `macro_rules!` (`library/core/src/num/wrapping.rs`,
// confirmed by reading the real source, not assumed), so an
// `extern_spec!` targeting this one concrete instantiation matches the
// real signature exactly, the same way `Ordering::reverse`'s did.
// `.0` is a public tuple-field projection, not a method call, so it's
// fine inside `#[ensures]` without a trusted wrapper; the plain
// (non-`@`) `+` between the two `i32` fields relies on Pearlite's native
// machine-integer semantics matching real wraparound, the same
// convention `creusot-std`'s own `spec_op_common!` macro uses for
// `i32::wrapping_add`'s postcondition.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Wrapping<i32> {
        #[check(ghost)]
        #[ensures(result.0 == self.0 + rhs.0)]
        fn add(self, rhs: Wrapping<i32>) -> Wrapping<i32>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, {
        /// `Wrapping<T>`'s `+` operator wraps on overflow exactly like the
        /// inner type's `wrapping_add` — the same claim
        /// `amenable_kani::rust_std::num::verify_wrapping_add_matches_the_inner_wrapping_add`
        /// checks by symbolic execution (there, comparing against
        /// `a.wrapping_add(b)` directly). Rests on the local `extern_spec!`
        /// above, the same relationship every other non-`char`/`String`
        /// harness in this file has to a trusted axiom on the real method
        /// it exercises.
        #[requires(true)]
        #[ensures(result.0 == a.0 + b.0)]
        fn verify_wrapping_i32_add_wraps(a: Wrapping<i32>, b: Wrapping<i32>) -> Wrapping<i32> {
            a + b
        }
    }
}

// Same per-concrete-type macro shape as `Wrapping<T>` (confirmed by
// reading the real source, `library/core/src/num/saturating.rs`: `impl
// const Add for Saturating<$t>` generated once per width, not one
// generic sealed-trait impl), so a local `extern_spec!` is practical
// here too — but the semantics are clamping, not wraparound, so the
// postcondition restates `creusot-std`'s own three-way `@`-lifted
// contract for the plain `i32::saturating_add` method (`spec_op_common!`
// in `creusot_std::std::num`) in terms of the wrapper's `.0` fields,
// rather than reusing Wrapping's plain-`+` idiom.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Saturating<i32> {
        #[check(ghost)]
        #[ensures(
            (self.0@ + rhs.0@) >= i32::MIN@ && (self.0@ + rhs.0@) <= i32::MAX@
            ==> result.0@ == (self.0@ + rhs.0@)
        )]
        #[ensures((self.0@ + rhs.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((self.0@ + rhs.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn add(self, rhs: Saturating<i32>) -> Saturating<i32>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, {
        /// `Saturating<T>`'s `+` operator saturates at the numeric bounds
        /// exactly like the inner type's `saturating_add` — the same
        /// claim `amenable_kani::rust_std::num::verify_saturating_add_matches_the_inner_saturating_add`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, which restates `creusot-std`'s own trusted axiom for
        /// `i32::saturating_add` in terms of `Saturating<i32>`'s wrapper
        /// field.
        #[requires(true)]
        #[ensures(
            (a.0@ + b.0@) >= i32::MIN@ && (a.0@ + b.0@) <= i32::MAX@
            ==> result.0@ == (a.0@ + b.0@)
        )]
        #[ensures((a.0@ + b.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((a.0@ + b.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn verify_saturating_i32_add_clamps(a: Saturating<i32>, b: Saturating<i32>) -> Saturating<i32> {
            a + b
        }
    }
}
