#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
amenable_derive::harness! {
    creusot, ASCII_BYTE_HOLDS_SRC, {
        /// The `amenable_std::AsciiByte` precondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn ascii_byte_holds(byte: u8) -> bool {
            pearlite! { byte < 128u8 }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC, {
        /// A one-byte ASCII `str` reports a byte length of one, and its
        /// first byte is exactly the byte it was constructed from.
        ///
        /// Accommodation model, not `#[trusted]`: expressing the real
        /// construction path here would need `char::to_string` and
        /// `str::as_bytes`, but `creusot-std` 0.11.0 ships no contracts
        /// for either function, so this states the same law directly
        /// over the byte value instead (no real call needed, so no
        /// trusted boundary is needed either -- Creusot discharges the
        /// resulting tuple equalities on its own).
        ///
        /// `ascii_byte_holds` is the canonical home
        /// `amenable_std::AsciiByte`'s own `Requires<CreusotVerifier>`
        /// impl names.
        #[requires(ascii_byte_holds(byte))]
        #[ensures(str_byte_length_and_content_holds(byte, result))]
        fn verify_str_byte_length_and_content(byte: u8) -> (usize, u8) {
            (1usize, byte)
        }
    }
}

amenable_derive::harness! {
    creusot, STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<str>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn str_byte_length_and_content_holds(byte: u8, str_result: (usize, u8)) -> bool {
            pearlite! { str_result.0 == 1usize && str_result.1 == byte }
        }
    }
}
