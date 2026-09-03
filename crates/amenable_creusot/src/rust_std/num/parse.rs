//! Integer parsing and conversion failures: `FromStr for i32` /
//! `FromStr for NonZero<i32>` (and the `IntErrorKind` / `ParseIntError`
//! claims resting on them), plus `TryFrom<i32> for u8`'s `TryFromIntError`.

/// The `#[cfg(creusot)]` imports and trusted logic wrappers this file needs,
/// consolidated onto one `mod` gate -- see `stoplight::mirror` for the
/// rationale. `extern_spec! { .. }` / `harness! { .. }` blocks reference all
/// of it unqualified.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::logic::Int;
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
    pub(super) use std::num::{IntErrorKind, NonZero, ParseIntError, TryFromIntError};

    // Trusted logic wrapper for `ParseIntError::kind()` -- an ordinary
    // getter modeled as an axiom tying a real method's result to a
    // logic-context-callable value.
    #[trusted]
    #[logic(opaque)]
    pub(super) fn parse_int_error_kind(_e: &ParseIntError) -> IntErrorKind {
        dead
    }

    // Real, computable (`#[logic(open)]`) -- whether a char is an ASCII
    // digit. `c@` is char's own View; 48/57 are `'0'`/`'9'`.
    #[logic(open)]
    pub(super) fn is_ascii_digit(c: char) -> bool {
        pearlite! { c@ >= 48 && c@ <= 57 }
    }
}
#[cfg(creusot)]
use mirror::{
    Int, IntErrorKind, NonZero, ParseIntError, TryFromIntError, check, ensures, extern_spec,
    is_ascii_digit, logic, parse_int_error_kind, requires,
};

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
    creusot, INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// core::num::IntErrorKind>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        /// Not `open`: it calls the opaque `parse_int_error_kind`, and
        /// an `open` wrapper around an opaque callee would leak that
        /// opacity boundary (same real `creusot-rustc` "less-visible
        /// item" error `string_roundtrips_and_preserves_length` hit
        /// earlier).
        #[logic]
        fn int_error_kind_classifies_parse_failures_holds(
            parse_result: (
                Result<i32, ParseIntError>,
                Result<i32, ParseIntError>,
                Result<i32, ParseIntError>,
                Result<i32, ParseIntError>,
                Result<NonZero<i32>, ParseIntError>,
            ),
        ) -> bool {
            pearlite! {
                match parse_result {
                    (Err(ref e1), Err(ref e2), Err(ref e3), Err(ref e4), Err(ref e5)) => {
                        parse_int_error_kind(e1) == IntErrorKind::Empty
                            && parse_int_error_kind(e2) == IntErrorKind::InvalidDigit
                            && parse_int_error_kind(e3) == IntErrorKind::PosOverflow
                            && parse_int_error_kind(e4) == IntErrorKind::NegOverflow
                            && parse_int_error_kind(e5) == IntErrorKind::Zero
                    }
                    _ => false,
                }
            }
        }
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
        #[ensures(int_error_kind_classifies_parse_failures_holds(result))]
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
    creusot, TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// TryFromIntError>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn try_from_int_error_occurs_exactly_when_out_of_range_holds(
            value: i32,
            try_from_result: Result<u8, TryFromIntError>,
        ) -> bool {
            pearlite! {
                match try_from_result {
                    Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
                    Err(_) => value@ < 0 || value@ > 255,
                }
            }
        }
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
        #[ensures(try_from_int_error_occurs_exactly_when_out_of_range_holds(value, result))]
        fn verify_try_from_int_error_occurs_exactly_when_out_of_range(
            value: i32,
        ) -> Result<u8, TryFromIntError> {
            u8::try_from(value)
        }
    }
}

amenable_derive::harness! {
    creusot, PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<ParseIntError>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it. Not `open`: it calls the
        /// opaque `parse_int_error_kind`, and an `open` wrapper around
        /// an opaque callee would leak that opacity boundary (same
        /// real `creusot-rustc` "less-visible item" error
        /// `string_roundtrips_and_preserves_length` hit earlier).
        #[logic]
        fn parse_int_error_reports_the_kind_of_the_failure_holds(
            parse_result: &Result<i32, ParseIntError>,
        ) -> bool {
            pearlite! {
                match parse_result {
                    Err(e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
                    Ok(_) => false,
                }
            }
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
        #[ensures(parse_int_error_reports_the_kind_of_the_failure_holds(&result))]
        fn verify_parse_int_error_reports_the_kind_of_the_failure() -> Result<i32, ParseIntError>
        {
            <i32 as std::str::FromStr>::from_str("not a number")
        }
    }
}
