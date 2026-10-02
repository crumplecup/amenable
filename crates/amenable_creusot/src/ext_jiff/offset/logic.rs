#![cfg(creusot)]
//! `jiff::tz::Offset`'s `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]` (cordial's
//! CFG-SCATTER finding; same fix as every sibling `ext_jiff` module).
//! The opaque `offset_seconds_value` accessor itself now lives in
//! `ext_jiff::shared_trusted_accessors` (the single most cross-file-
//! reused accessor in this crate — nine consumers — so a module too
//! thin on its own to clear cordial's `VIS-MOD-THIN-001` floor would
//! have had to exist here otherwise; see that module's own doc
//! comment).

use crate::ext_jiff::shared_trusted_accessors::offset_seconds_value;
use creusot_std::macros::{check, ensures, extern_spec};

// jiff's own documented valid range for `Offset::from_seconds`
// (`-25:59:59..=25:59:59`, in seconds) — the same constant the Kani
// harness for this type independently confirms, restated here as the
// trusted axiom's own failure-side claim.
extern_spec! {
    impl jiff::tz::Offset {
        #[check(ghost)]
        #[ensures(match result {
            Ok(ref offset) => offset_seconds_value(offset) == seconds,
            Err(_) => seconds < -93_599i32 || seconds > 93_599i32,
        })]
        fn from_seconds(seconds: i32) -> Result<jiff::tz::Offset, jiff::Error>;

        #[check(ghost)]
        #[ensures(result == offset_seconds_value(&self))]
        fn seconds(self) -> i32;
    }
}
