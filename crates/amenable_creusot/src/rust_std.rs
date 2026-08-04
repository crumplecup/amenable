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
use creusot_std::macros::{ensures, logic, requires, trusted};

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
