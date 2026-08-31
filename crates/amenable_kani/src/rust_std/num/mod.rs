//! `KaniWitness` impls for `core::num`.
//!
//! Split into [`nonzero`] (all twelve `NonZero<T>` instantiations, plus the
//! shared `NonZeroGetRoundTrips` marker) and [`wrapping_and_parse_errors`]
//! (`Wrapping`, `Saturating`, `TryFromIntError`, `IntErrorKind`,
//! `ParseIntError`, `ParseFloatError`, `FpCategory`).
//!
//! Each `NonZero<T>` instantiation is written out literally rather than
//! generated through a wrapping `macro_rules!`: `amenable_derive::harness!`
//! captures a harness's verbatim source via the group's span, and a span
//! produced by a `macro_rules!` expansion resolves back to the *defining*
//! macro's on-disk text — so a generator macro would capture its own
//! `$ty`/`$harness_fn` placeholders, unsubstituted, instead of each type's
//! real harness. Twelve literal blocks is the price of an honest `claim`.

mod nonzero;
mod wrapping_and_parse_errors;

pub use nonzero::NonZeroGetRoundTrips;
