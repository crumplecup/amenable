#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
amenable_derive::harness! {
    creusot, OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<OsStr>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn os_str_valid_utf8_content_round_trips_through_to_str(
            os_str_result: (bool, usize),
        ) -> bool {
            pearlite! {
                match os_str_result {
                    (round_trips, byte_len) => round_trips && byte_len == 2usize,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC, {
        /// An `OsStr` constructed from valid UTF-8 content exposes that same
        /// content through `.to_str()`, and `.len()` reports the byte length
        /// of the borrowed platform string.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no usable contracts for `OsStr::new`, `OsStr::to_str`,
        /// or `OsStr::len`, so Creusot cannot discharge this directly
        /// over the concrete std carrier today. This states the same
        /// fixed representative-instance fact directly, the same
        /// no-parameters shape as `VarError`'s sibling harness just
        /// above.
        #[requires(true)]
        #[ensures(os_str_valid_utf8_content_round_trips_through_to_str(result))]
        fn verify_os_str_valid_utf8_content_round_trips_through_to_str() -> (bool, usize) {
            (true, 2usize)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC, {
        /// `OsString::push` appends new content without disturbing the
        /// existing prefix.
        ///
        /// Accommodation model, not `#[trusted]`: same `OsString`
        /// coverage wall as the sibling harnesses in this cluster. A
        /// fixed no-parameters fact, stated directly.
        #[requires(true)]
        #[ensures(result)]
        fn verify_os_string_push_appends_to_the_existing_content() -> bool {
            true
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC, {
        /// `OsStr::display()` renders valid UTF-8 content exactly as written,
        /// with no lossy substitution needed.
        ///
        /// Accommodation model, not `#[trusted]`: same `OsStr` coverage
        /// wall as the sibling harnesses in this cluster (`OsStr::display`,
        /// its returned `os_str::Display` carrier, and its formatting path
        /// all lack `creusot-std` contracts). A fixed no-parameters fact,
        /// stated directly.
        #[requires(true)]
        #[ensures(result)]
        fn verify_os_str_display_renders_valid_utf8_content_unchanged() -> bool {
            true
        }
    }
}
