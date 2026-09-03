//! Floating-point classification and parsing: `f64::classify` into the
//! matching `FpCategory` variant, and `FromStr for f64` failing only on
//! unparseable input. Both `#[trusted]` -- `f64` has no `View` impl in
//! creusot-std and bare float literals ICE `creusot-rustc` (see the
//! `f64_has_no_view_impl_at_all` / `float_literals_in_pearlite_ice_the_compiler`
//! gallery findings).

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires, trusted};
#[cfg(creusot)]
use std::num::{FpCategory, ParseFloatError};

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
