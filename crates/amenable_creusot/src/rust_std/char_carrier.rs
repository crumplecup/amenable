#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
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
        ///
        /// `valid_unicode_scalar_holds` is `amenable_std::ValidUnicodeScalar`'s
        /// canonical Creusot postcondition, named rather than restated here —
        /// see that type for the same bound held once, and its
        /// `Ensures<CreusotVerifier>` impl for this exact fragment as a
        /// reusable, backend-checkable claim.
        #[requires(true)]
        #[ensures(char_roundtrips(c, result))]
        #[ensures(valid_unicode_scalar_holds(result))]
        fn verify_char_roundtrip(c: char) -> char {
            c
        }
    }
}

amenable_derive::harness! {
    creusot, CHAR_ROUNDTRIPS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<char>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn char_roundtrips(c: char, char_result: char) -> bool {
            pearlite! { char_result == c }
        }
    }
}

amenable_derive::harness! {
    creusot, VALID_UNICODE_SCALAR_HOLDS_SRC, {
        /// The `amenable_std::ValidUnicodeScalar` postcondition -- real,
        /// callable Pearlite content, not just descriptive text alongside
        /// it.
        #[logic(open)]
        fn valid_unicode_scalar_holds(c: char) -> bool {
            pearlite! { c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF) }
        }
    }
}
