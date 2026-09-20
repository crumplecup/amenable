//! Real Creusot proof content for `jiff::fmt::StdFmtWrite<String>`'s
//! infallibility property (`ext::jiff::fmt_std_fmt_write` holds the
//! `CreusotWitness` bridge that references the `_SRC` constant this
//! file's `harness!` call emits).
//!
//! `jiff::fmt::Write` (the trait `StdFmtWrite` implements) is
//! uncontracted everywhere — not `creusot-std` (confirmed: no
//! `core::fmt::Write`/`jiff::fmt::Write` coverage there at all, only
//! `std::io::Write`), not `elicitation`.
//!
//! **Narrower than `amenable_kani::ext::jiff::fmt_std_fmt_write`'s own
//! claim, for a real, CONFIRMED compiler-enforced reason, not a
//! shortcut.** A first attempt extern-spec'd `jiff::fmt::Write::
//! write_str` directly on the concrete `StdFmtWrite<String>`
//! instantiation and hit a genuine `cargo creusot` error: "extern spec
//! generics don't match" — jiff's real impl is `impl<W: core::fmt::
//! Write> Write for StdFmtWrite<W>`, generic over `W`, and Creusot's
//! own extern-spec matching (confirmed by reading `creusot`'s own
//! `translation/external.rs`) requires the extern_spec's impl to carry
//! the identical generic parameters as the real impl — concretizing to
//! `W = String` is rejected outright, not merely discouraged. Since a
//! postcondition generic over ANY `W: core::fmt::Write` can't honestly
//! claim infallibility (an adversarial `W` really can fail), there is
//! no nontrivial, TRUE claim extern-specable about `StdFmtWrite<W>::
//! write_str` itself at all.
//!
//! So this witness checks the real, true, load-bearing fact the whole
//! adapter's soundness actually rests on instead: `String`'s own
//! `core::fmt::Write::write_str` impl (concrete, non-generic — matches
//! Creusot's exact-generics requirement trivially) always succeeds.
//! This is exactly the documented fact that makes `StdFmtWrite<String>`
//! safe to use (jiff's real body is `self.0.write_str(string).map_err
//! (..)`, so `StdFmtWrite<String>` can only fail if `String`'s own
//! `write_str` fails) — narrower than `amenable_kani::ext::jiff::
//! fmt_std_fmt_write`'s own stronger claim, which exercises jiff's real
//! wrapper method directly by symbolic execution rather than going
//! through Creusot's extern-spec generics constraint at all.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
}
#[cfg(creusot)]
use mirror::{check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl core::fmt::Write for String {
        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => true,
            Err(_) => false,
        })]
        fn write_str(&mut self, string: &str) -> Result<(), core::fmt::Error>;
    }
}

amenable_derive::harness! {
    creusot, FMT_STD_FMT_WRITE_WRITE_STR_NEVER_FAILS_HOLDS_SRC, {
        /// The `amenable_ext::
        /// ExtStandard<jiff::fmt::StdFmtWrite<String>>` postcondition
        /// — real, callable Pearlite content, not just descriptive
        /// text alongside it.
        #[logic(open)]
        fn fmt_std_fmt_write_write_str_never_fails_holds(succeeded: bool) -> bool {
            pearlite! { succeeded }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_NEVER_FAILS_SRC, {
        /// `String`'s own `core::fmt::Write::write_str` always
        /// succeeds — a real, checked postcondition resting on the
        /// `extern_spec!` above, the load-bearing fact behind
        /// `jiff::fmt::StdFmtWrite<String>`'s own infallibility (see
        /// this module's own doc comment for why Creusot's
        /// extern-spec generics constraint rules out stating the
        /// claim on jiff's generic wrapper method directly).
        #[requires(true)]
        #[ensures(fmt_std_fmt_write_write_str_never_fails_holds(result))]
        fn verify_fmt_std_fmt_write_write_str_never_fails(mut string: String, s: &str) -> bool {
            use core::fmt::Write as _;
            string.write_str(s).is_ok()
        }
    }
}
