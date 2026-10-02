//! The `Into<i64>`-stand-in machinery shared by every `Span` unit
//! setter's `extern_spec!` in `accessors.rs`.
//!
//! Every setter (`years`/`months`/…) is generic — `fn years<I:
//! Into<i64>>(self, years: I) -> Span` — a real toolchain finding
//! worth recording: Creusot's `extern_spec!` genuinely requires the
//! declared signature's generics to match the real function's,
//! confirmed by first trying a concrete (non-generic) signature and
//! getting a real "extern spec generics don't match" error; the fix is
//! writing the identical `<I: Into<i64>>` clause. But `.into()` itself
//! cannot appear inside `#[ensures]` — confirmed by a second real
//! error, "unbound function or predicate symbol `into_i16`", once the
//! generics matched but the ensures clause called `years.into()`
//! directly. The fix used here: an opaque `span_i64_of<I>` logic
//! function stands in for "whatever `Into<i64>::into` would produce"
//! (never calling it for real, since it's `#[logic(opaque)]`), plus
//! one small `#[trusted]` *lemma* function per concrete `I` this crate
//! actually instantiates the setters at (`i16`/`i32`/`i64` — the three
//! native widths jiff's own doc comments assign across the ten
//! fields), each just asserting `span_i64_of::<I>(x) == x as i64` and
//! called for its postcondition's side effect at each harness call
//! site. Three lemmas cover all ten fields, since the generic-over-`I`
//! opaqueness is shared.

pub(crate) mod logic;
#[cfg(creusot)]
use creusot_std::macros::logic;

amenable_derive::harness! {
    creusot, SPAN_I64_OF_I16_MATCHES_CAST_HOLDS_SRC, {
        /// The opaque `span_i64_of::<i16>` stand-in agrees with the
        /// real `as i64` cast it stands in for — named so `span_i64_of_
        /// i16_lemma`'s own postcondition points at a real predicate
        /// instead of restating the comparison inline.
        #[logic(open)]
        fn span_i64_of_i16_matches_cast(x: i16) -> bool {
            pearlite! { logic::span_i64_of::<i16>(x) == x as i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_i64_of_i16_matches_cast",
        "creusot",
        "ensures",
        || SPAN_I64_OF_I16_MATCHES_CAST_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_I64_OF_I32_MATCHES_CAST_HOLDS_SRC, {
        /// The opaque `span_i64_of::<i32>` stand-in agrees with the
        /// real `as i64` cast it stands in for.
        #[logic(open)]
        fn span_i64_of_i32_matches_cast(x: i32) -> bool {
            pearlite! { logic::span_i64_of::<i32>(x) == x as i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_i64_of_i32_matches_cast",
        "creusot",
        "ensures",
        || SPAN_I64_OF_I32_MATCHES_CAST_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, SPAN_I64_OF_I64_MATCHES_CAST_HOLDS_SRC, {
        /// The opaque `span_i64_of::<i64>` stand-in agrees with the
        /// real `as i64` cast it stands in for (the identity cast, but
        /// checked the same way as the narrower widths for uniformity).
        #[logic(open)]
        fn span_i64_of_i64_matches_cast(x: i64) -> bool {
            pearlite! { logic::span_i64_of::<i64>(x) == x as i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::span::span_i64_of_i64_matches_cast",
        "creusot",
        "ensures",
        || SPAN_I64_OF_I64_MATCHES_CAST_HOLDS_SRC,
    )
}
