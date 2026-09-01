/// The `#[cfg(creusot)]` import and trusted logic-wrapper function this
/// file needs, consolidated into one gate on this `mod` instead of one per
/// item -- see `stoplight::mirror`'s own doc comment for the general
/// rationale. Both are re-exported: the `harness! { .. }` blocks below
/// (macro invocations, invisible to the cfg-scatter scanner) reference
/// them, unqualified, from this file's own top level.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires, trusted};

    // `String::len` is a program function, not callable from `#[ensures]`
    // (Pearlite logic context) directly — confirmed by a real translation
    // error, not a guess: `error: called program function 'std::string::String
    // ::len' in logic context`. `elicitation`'s own `logic_fns.rs` solves this
    // with exactly this shape: a `#[trusted] #[logic(opaque)]` wrapper whose
    // body is the Pearlite `dead` placeholder (an axiom — the relationship to
    // the real method is asserted, not proven) so the length claim can appear
    // in a postcondition at all.
    #[trusted]
    #[logic(opaque)]
    pub(super) fn string_len(_s: &String) -> usize {
        dead
    }
}
#[cfg(creusot)]
use mirror::{ensures, logic, requires, string_len};

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
        #[ensures(string_roundtrips_and_preserves_length(s, result))]
        fn verify_string_roundtrip(s: String) -> String {
            s
        }
    }
}

amenable_derive::harness! {
    creusot, STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<String>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it. Not `open`: it calls the
        /// opaque `string_len`, and an `open` wrapper around an opaque
        /// callee would leak that opacity boundary (a real
        /// `creusot-rustc` "less-visible item" error, not a guess).
        #[logic]
        fn string_roundtrips_and_preserves_length(s: String, string_result: String) -> bool {
            pearlite! { string_result == s && string_len(&string_result) == string_len(&s) }
        }
    }
}
