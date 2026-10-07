#![cfg(creusot)]
//! `chrono::FixedOffset`'s `extern_spec!` bridge.
//!
//! Self-gated via this file's own `#![cfg(creusot)]`, same as every sibling
//! `ext_chrono` module.

use crate::ext_chrono::shared_trusted_accessors::fixed_offset_local_minus_utc_value;
use creusot_std::macros::{check, ensures, extern_spec};

// chrono's own documented valid range for `FixedOffset::east_opt`/`west_opt`:
// `-23:59:59..=23:59:59`, in seconds (`fixed.rs`'s own `-86_400 < secs && secs <
// 86_400`) — the same bound the Kani harness for this type independently confirms.
extern_spec! {
    impl chrono::FixedOffset {
        #[check(ghost)]
        #[ensures(match result {
            Some(ref offset) => fixed_offset_local_minus_utc_value(offset) == secs,
            None => secs <= -86_400i32 || secs >= 86_400i32,
        })]
        fn east_opt(secs: i32) -> Option<chrono::FixedOffset>;

        #[check(ghost)]
        #[ensures(match result {
            Some(ref offset) => fixed_offset_local_minus_utc_value(offset) == -secs,
            None => secs <= -86_400i32 || secs >= 86_400i32,
        })]
        fn west_opt(secs: i32) -> Option<chrono::FixedOffset>;

        #[check(ghost)]
        #[ensures(result == fixed_offset_local_minus_utc_value(self))]
        fn local_minus_utc(&self) -> i32;
    }
}
