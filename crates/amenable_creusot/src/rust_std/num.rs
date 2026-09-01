/// The `#[cfg(creusot)]` imports and trusted logic-wrapper functions this
/// file needs, consolidated into one gate on this `mod` instead of one per
/// item -- see `stoplight::mirror`'s own doc comment for the general
/// rationale. Every import and every function is re-exported: the
/// `extern_spec! { .. }`/`harness! { .. }` blocks below (both macro
/// invocations, invisible to the cfg-scatter scanner the same way
/// `include!`'s own content is, so there's no consolidation benefit to
/// moving them in too) reference all of it, unqualified, from this file's
/// own top level.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::logic::Int;
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
    pub(super) use std::num::{
        FpCategory, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating,
        TryFromIntError, Wrapping,
    };

    // `NonZero::get` is a plain program function too — same restriction as
    // `String::len`, no `#[check(ghost)]` contract to trip over this time
    // since creusot-std has no extern_spec for `NonZero<T>` at all. Trusted
    // wrapper, same shape as `string_len`.
    #[trusted]
    #[logic(opaque)]
    pub(super) fn nonzero_i16_get(_nz: &NonZero<i16>) -> i16 {
        dead
    }

    // Trusted logic wrapper for `ParseIntError::kind()` — same shape as
    // `nonzero_i16_get`/`string_len`: an ordinary getter, modeled as an
    // axiom tying a real method's result to a logic-context-callable
    // value. Used both by the `FromStr` extern_spec below (to state what
    // error kind a given input produces) and by the harness itself (to
    // check the result).
    #[trusted]
    #[logic(opaque)]
    pub(super) fn parse_int_error_kind(_e: &ParseIntError) -> IntErrorKind {
        dead
    }

    // Real, computable (`#[logic(open)]`, not opaque) — whether a char is
    // an ASCII digit. `c@` is char's own View (Unicode scalar value as
    // `Int`, same operator the char contract above uses); 48/57 are
    // `'0'`/`'9'`.
    #[logic(open)]
    pub(super) fn is_ascii_digit(c: char) -> bool {
        pearlite! { c@ >= 48 && c@ <= 57 }
    }
}
#[cfg(creusot)]
use mirror::{
    FpCategory, Int, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating,
    TryFromIntError, Wrapping, check, ensures, extern_spec, is_ascii_digit, logic, nonzero_i16_get,
    parse_int_error_kind, requires, trusted,
};

amenable_derive::harness! {
    creusot, NONZERO_I16_NEW_SUCCEEDS_EXACTLY_WHEN_NONZERO_SRC, {
        /// The first `amenable_std::rust_std::RustStdStandard<NonZero<i16>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn nonzero_i16_new_succeeds_exactly_when_nonzero(
            value: i16,
            new_result: Option<NonZero<i16>>,
        ) -> bool {
            pearlite! {
                match new_result {
                    Some(_) => value != 0i16,
                    None => value == 0i16,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, NONZERO_I16_GET_ROUND_TRIPS_SRC, {
        /// The second `amenable_std::rust_std::RustStdStandard<NonZero<i16>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        ///
        /// `opaque`, not `open`: it calls the module-private
        /// `nonzero_i16_get` (itself `#[trusted] #[logic(opaque)]`), and
        /// Creusot rejects an `open` (transparent) definition that would
        /// expose a less-visible item to its own callers.
        #[logic(opaque)]
        fn nonzero_i16_get_round_trips(value: i16, new_result: Option<NonZero<i16>>) -> bool {
            pearlite! {
                match new_result {
                    Some(nz) => nonzero_i16_get(&nz) == value,
                    None => true,
                }
            }
        }
    }
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
        ///
        /// Both `#[ensures]` clauses below are the canonical home
        /// `RustStdStandard<NonZero<i16>>`'s own `Ensures<CreusotVerifier>`
        /// impl (`amenable_std::creusot_witness`) names.
        #[trusted]
        #[requires(true)]
        #[ensures(nonzero_i16_new_succeeds_exactly_when_nonzero(value, result))]
        #[ensures(nonzero_i16_get_round_trips(value, result))]
        fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
            NonZero::new(value)
        }
    }
}

amenable_derive::harness! {
    creusot, WRAPPING_I32_ADD_WRAPS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// Wrapping<i32>>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn wrapping_i32_add_wraps_holds(
            a: Wrapping<i32>,
            b: Wrapping<i32>,
            add_result: Wrapping<i32>,
        ) -> bool {
            pearlite! { add_result.0 == a.0 + b.0 }
        }
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
        #[ensures(wrapping_i32_add_wraps_holds(a, b, result))]
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
    creusot, SATURATING_I32_ADD_CLAMPS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// Saturating<i32>>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn saturating_i32_add_clamps_holds(
            a: Saturating<i32>,
            b: Saturating<i32>,
            add_result: Saturating<i32>,
        ) -> bool {
            pearlite! {
                ((a.0@ + b.0@) >= i32::MIN@ && (a.0@ + b.0@) <= i32::MAX@
                    ==> add_result.0@ == (a.0@ + b.0@))
                    && ((a.0@ + b.0@) < i32::MIN@ ==> add_result.0@ == i32::MIN@)
                    && ((a.0@ + b.0@) > i32::MAX@ ==> add_result.0@ == i32::MAX@)
            }
        }
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
        #[ensures(saturating_i32_add_clamps_holds(a, b, result))]
        fn verify_saturating_i32_add_clamps(a: Saturating<i32>, b: Saturating<i32>) -> Saturating<i32> {
            a + b
        }
    }
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

amenable_derive::harness! {
    creusot, FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<FpCategory>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn fp_category_matches_the_value_it_classifies_holds(
            fp_category_result: (FpCategory, FpCategory, FpCategory, FpCategory, FpCategory),
        ) -> bool {
            pearlite! {
                match fp_category_result {
                    (FpCategory::Nan, FpCategory::Infinite, FpCategory::Zero, FpCategory::Normal, FpCategory::Subnormal) => true,
                    _ => false,
                }
            }
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
        #[ensures(fp_category_matches_the_value_it_classifies_holds(result))]
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
    creusot, PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// ParseFloatError>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn parse_float_error_occurs_only_for_unparseable_input_holds(
            parse_result: (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>),
        ) -> bool {
            pearlite! {
                match parse_result {
                    (Err(_), Ok(_)) => true,
                    _ => false,
                }
            }
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
        #[ensures(parse_float_error_occurs_only_for_unparseable_input_holds(result))]
        fn verify_parse_float_error_occurs_only_for_unparseable_input()
        -> (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>) {
            (
                <f64 as std::str::FromStr>::from_str("not a float"),
                <f64 as std::str::FromStr>::from_str("3.14"),
            )
        }
    }
}
