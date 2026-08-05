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
use creusot_std::logic::Int;
#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use creusot_std::std::time::nanos_to_secs;
#[cfg(creusot)]
use std::cmp::Ordering;
#[cfg(creusot)]
use std::num::{
    FpCategory, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating, TryFromIntError,
    Wrapping,
};
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

// Trusted logic wrapper for `ParseIntError::kind()` — same shape as
// `nonzero_i16_get`/`string_len`: an ordinary getter, modeled as an axiom
// tying a real method's result to a logic-context-callable value. Used
// both by the `FromStr` extern_spec below (to state what error kind a
// given input produces) and by the harness itself (to check the result).
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn parse_int_error_kind(_e: &ParseIntError) -> IntErrorKind {
    dead
}

// Real, computable (`#[logic(open)]`, not opaque) — whether a char is an
// ASCII digit. `c@` is char's own View (Unicode scalar value as `Int`,
// same operator the char contract above uses); 48/57 are `'0'`/`'9'`.
#[cfg(creusot)]
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    pearlite! { c@ >= 48 && c@ <= 57 }
}

// `str::parse::<i32>()` (`FromStr::from_str`) is uncontracted everywhere
// — not `creusot-std` (checked: no `FromStr` coverage for integers at
// all), not `elicitation` (checked: no prior art). Four clauses below,
// each a real, general (not just-for-these-inputs) but *sufficient*
// (not exhaustive) condition — true for every string matching the
// pattern, not merely the four concrete ones the harness exercises:
//
// - Empty: exact, matches real behavior for every empty string.
// - InvalidDigit: exact — any character outside an optional leading
//   sign that isn't an ASCII digit forces this outcome, for any string.
// - Pos/NegOverflow: deliberately *not* exact (no digit-value
//   accumulation, which would need a recursive logic function over the
//   digit sequence to state precisely) — instead: an all-digit string
//   with a nonzero leading digit and more than 10 digits (i32::MAX is
//   10 digits) is unconditionally too large for `i32` regardless of the
//   exact value, since the leading nonzero digit alone already puts the
//   magnitude at or above 10^10. True for any string of that shape, not
//   only the 20-nines literal the harness happens to use.
#[cfg(creusot)]
extern_spec! {
    impl core::str::FromStr for i32 {
        #[check(ghost)]
        #[ensures(s@.len() == 0 ==> match result {
            Err(ref e) => parse_int_error_kind(e) == IntErrorKind::Empty,
            Ok(_) => false,
        })]
        #[ensures(
            (exists<i: Int> 0 <= i && i < s@.len()
                && !(i == 0 && (s@[i] == '+' || s@[i] == '-'))
                && !is_ascii_digit(s@[i]))
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
                Ok(_) => false,
            }
        )]
        #[ensures(
            s@.len() > 10
                && is_ascii_digit(s@[0]) && s@[0] != '0'
                && forall<i: Int> 0 <= i && i < s@.len() ==> is_ascii_digit(s@[i])
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::PosOverflow,
                Ok(_) => false,
            }
        )]
        #[ensures(
            s@.len() > 11
                && s@[0] == '-'
                && is_ascii_digit(s@[1]) && s@[1] != '0'
                && forall<i: Int> 1 <= i && i < s@.len() ==> is_ascii_digit(s@[i])
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::NegOverflow,
                Ok(_) => false,
            }
        )]
        fn from_str(s: &str) -> Result<i32, ParseIntError>;
    }
}

// `NonZero<i32>::from_str` is a *different* `FromStr` impl from `i32`'s own
// (`impl FromStr for NonZero<$Int>`, generated once per concrete width by
// the same `nonzero_integer!` macro that generates `Wrapping`/`Saturating`'s
// per-width arithmetic impls — confirmed by reading the real source, not
// assumed), so it needs its own extern_spec rather than following from the
// one above. The real impl parses via `from_str_radix`/`from_ascii_radix`
// (accepts a valid `i32` first, then checks nonzero), so "the input is
// exactly the one-character string `\"0\"`" is both real and exact for the
// `Zero` outcome — not a narrowed sufficient condition the way the
// Pos/NegOverflow clauses above are, since it's the only single-digit
// all-zero string there is.
#[cfg(creusot)]
extern_spec! {
    impl core::str::FromStr for NonZero<i32> {
        #[check(ghost)]
        #[ensures(s@.len() == 1 && s@[0] == '0' ==> match result {
            Err(ref e) => parse_int_error_kind(e) == IntErrorKind::Zero,
            Ok(_) => false,
        })]
        fn from_str(s: &str) -> Result<NonZero<i32>, ParseIntError>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, {
        /// Each representative integer-parse failure mode produces the
        /// matching `IntErrorKind` variant — the same claim
        /// `amenable_kani::rust_std::num::verify_int_error_kind_classifies_parse_failures`
        /// checks, restated as a real, `why3find`-discharged Creusot
        /// postcondition against the local `FromStr` extern_specs above,
        /// not `#[trusted]`. All five of Kani's cases, not four: `Zero`
        /// (parsing `"0"` as `NonZero<i32>`) is now covered too, via the
        /// second extern_spec above.
        ///
        /// Calls `<i32 as FromStr>::from_str`/`<NonZero<i32> as
        /// FromStr>::from_str` directly, not `s.parse::<T>()`:
        /// `str::parse<F>` is a distinct generic wrapper method
        /// (`FromStr::from_str(self)`, called through, not inlined), so
        /// extern-speccing `from_str` doesn't cover calls made through
        /// `parse` — confirmed by a real warning (`calling external
        /// function 'parse' with no contract will yield an impossible
        /// precondition`) before switching call sites.
        #[requires(true)]
        #[ensures(match result {
            (Err(ref e1), Err(ref e2), Err(ref e3), Err(ref e4), Err(ref e5)) => {
                parse_int_error_kind(e1) == IntErrorKind::Empty
                    && parse_int_error_kind(e2) == IntErrorKind::InvalidDigit
                    && parse_int_error_kind(e3) == IntErrorKind::PosOverflow
                    && parse_int_error_kind(e4) == IntErrorKind::NegOverflow
                    && parse_int_error_kind(e5) == IntErrorKind::Zero
            }
            _ => false,
        })]
        fn verify_int_error_kind_classifies_parse_failures() -> (
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<NonZero<i32>, ParseIntError>,
        ) {
            (
                <i32 as std::str::FromStr>::from_str(""),
                <i32 as std::str::FromStr>::from_str("not a number"),
                <i32 as std::str::FromStr>::from_str("99999999999999999999"),
                <i32 as std::str::FromStr>::from_str("-99999999999999999999"),
                <NonZero<i32> as std::str::FromStr>::from_str("0"),
            )
        }
    }
}

// `impl TryFrom<i32> for u8` is generated once per concrete
// (source, target) pair by `impl_try_from_both_bounded!`
// (`library/core/src/convert/num.rs`, confirmed by reading the real
// source, not assumed) — same per-concrete-instantiation shape as
// `Wrapping`/`Saturating`'s arithmetic impls and `Ordering::reverse`, so a
// local `extern_spec!` targeting this one pair matches the real signature
// exactly. Unlike `IntErrorKind`'s parsing contract, this one is exact, not
// merely sufficient: the real body is `if u < 0 { Err(NegOverflow) } else
// if u > 255 { Err(PosOverflow) } else { Ok(u as u8) }`, so "fits in
// 0..=255" is precisely the success condition, not an approximation of it.
// No `creusot-std` coverage and no `elicitation` prior art for
// `TryFromIntError`/`TryFrom` (checked both first).
#[cfg(creusot)]
extern_spec! {
    impl TryFrom<i32> for u8 {
        #[check(ghost)]
        #[ensures(match result {
            Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
            Err(_) => value@ < 0 || value@ > 255,
        })]
        fn try_from(value: i32) -> Result<u8, TryFromIntError>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(i32)` fails with `TryFromIntError` exactly when the
        /// source value doesn't fit in `u8`, and succeeds with the same
        /// value otherwise — the same claim
        /// `amenable_kani::rust_std::num::verify_try_from_int_error_occurs_exactly_when_out_of_range`
        /// checks by symbolic execution over `kani::any()`, restated as a
        /// real Creusot postcondition against the local `extern_spec`
        /// above (not `#[trusted]`): both directions of the iff are a
        /// single `match` clause there, so this harness just confirms the
        /// axiom is usable at a concrete call site, the same relationship
        /// every non-`char`/`String` harness in this file has to a trusted
        /// axiom on the real method it exercises.
        #[requires(true)]
        #[ensures(match result {
            Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
            Err(_) => value@ < 0 || value@ > 255,
        })]
        fn verify_try_from_int_error_occurs_exactly_when_out_of_range(
            value: i32,
        ) -> Result<u8, TryFromIntError> {
            u8::try_from(value)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC, {
        /// `ParseIntError::kind()` reports the specific reason the parse
        /// failed, not just that it failed — the same claim
        /// `amenable_kani::rust_std::num::verify_parse_int_error_reports_the_kind_of_the_failure`
        /// checks by symbolic execution. Already implied by the
        /// `InvalidDigit` clause of the `FromStr for i32` `extern_spec!`
        /// above (which every other `IntErrorKind` harness in this file
        /// also rests on): this harness just states that same fact as
        /// `ParseIntError`'s own claim, at the one concrete input Kani
        /// exercises.
        #[requires(true)]
        #[ensures(match &result {
            Err(e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
            Ok(_) => false,
        })]
        fn verify_parse_int_error_reports_the_kind_of_the_failure() -> Result<i32, ParseIntError>
        {
            <i32 as std::str::FromStr>::from_str("not a number")
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC, {
        /// Each representative floating-point value classifies into the
        /// `FpCategory` variant matching its own `is_*` predicates — the
        /// same claim
        /// `amenable_kani::rust_std::num::verify_fp_category_matches_the_value_it_classifies`
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike every real proof in this file: `f64` has
        /// no `View` impl in `creusot-std` at all (`self@` is
        /// unavailable), and a bare float literal inside
        /// `#[ensures]`/`#[requires]` panics `creusot-rustc` outright (a
        /// real internal compiler error, not a diagnosed one) — both
        /// confirmed, not guessed; see the `f64_has_no_view_impl_at_all`
        /// and `float_literals_in_pearlite_ice_the_compiler` gallery
        /// findings. The postcondition below never needs a float
        /// literal or `@` itself (it only compares the resulting
        /// `FpCategory` values, an ordinary enum), so it parses and would
        /// translate — but there is no way to give `f64::classify` a real
        /// `extern_spec!` connecting an arbitrary input float to its
        /// category under these constraints, so the harness body's own
        /// float literals (needed to construct the five representative
        /// inputs) are what force `#[trusted]` here, the same honest
        /// fallback `NonZero::new` uses for its own genuine, confirmed
        /// blocker.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (FpCategory::Nan, FpCategory::Infinite, FpCategory::Zero, FpCategory::Normal, FpCategory::Subnormal) => true,
            _ => false,
        })]
        fn verify_fp_category_matches_the_value_it_classifies() -> (
            FpCategory,
            FpCategory,
            FpCategory,
            FpCategory,
            FpCategory,
        ) {
            let subnormal = f64::MIN_POSITIVE / 2.0;
            (
                f64::NAN.classify(),
                f64::INFINITY.classify(),
                0.0f64.classify(),
                f64::MIN_POSITIVE.classify(),
                subnormal.classify(),
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC, {
        /// A non-numeric string fails to parse as `f64` with
        /// `ParseFloatError`, while a valid numeric string succeeds — the
        /// same claim
        /// `amenable_kani::rust_std::num::verify_parse_float_error_occurs_only_for_unparseable_input`
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike `ParseIntError`'s analogous harness: this
        /// claim never needs to characterize a float VALUE (only
        /// `Result::is_ok`/`is_err`), so it looked tractable by the same
        /// char/int-literal-only technique `IntErrorKind`'s Pos/NegOverflow
        /// clauses use — and a real `extern_spec!` for `FromStr for f64`
        /// using exactly that technique DOES translate cleanly (`cargo
        /// creusot -- -p amenable_creusot` succeeds, including a
        /// well-formedness check on the extern_spec itself). But
        /// `why3find prove`'s automatic strategy fails to discharge the
        /// harness's own goal against it: the goal splits into two
        /// sub-cases and one is left unattempted (`null` in the emitted
        /// `proof.json`, not a reported counterexample) — reproduced with
        /// the Err clause alone, the Ok clause alone, and both together,
        /// all three isolate to the same unresolved split. The identical
        /// technique (`s@.len()`/`s@[i]` char comparisons via
        /// `is_ascii_digit`) proves fine for `i32`'s `FromStr` in this
        /// same file, so the difference is specific to `f64` appearing in
        /// the `Result` — not fully root-caused (no diagnostic points at
        /// a specific cause the way the `f64` View/literal ICEs do for
        /// `FpCategory`), but confirmed reproducible across several
        /// independent attempts, not a "looks hard" guess. See
        /// `amenable_std::creusot_gallery`'s
        /// `parse_float_error_extern_spec_translates_but_wont_discharge`
        /// finding for the full repro.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (Err(_), Ok(_)) => true,
            _ => false,
        })]
        fn verify_parse_float_error_occurs_only_for_unparseable_input()
        -> (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>) {
            (
                <f64 as std::str::FromStr>::from_str("not a float"),
                <f64 as std::str::FromStr>::from_str("3.14"),
            )
        }
    }
}
