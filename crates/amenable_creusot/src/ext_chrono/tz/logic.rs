#![cfg(creusot)]
//! `chrono_tz::Tz`'s `extern_spec!` bridge: `name()` and `FromStr`, each
//! stated as its own independent axiom (not the whole round trip stated
//! wholesale as one fact) -- the wrapper in the parent module's own
//! `#[ensures]` is what checks these two axioms are consistent with each
//! other.

use crate::ext_chrono::shared_trusted_accessors::tz_name_logic;
use creusot_std::macros::{check, ensures, extern_spec};

extern_spec! {
    impl chrono_tz::Tz {
        #[check(ghost)]
        #[ensures(result == tz_name_logic(&self))]
        fn name(self) -> &'static str;
    }

    impl std::str::FromStr for chrono_tz::Tz {
        #[check(ghost)]
        #[ensures(match result {
            Ok(ref t) => s == tz_name_logic(t),
            Err(_) => forall<t: chrono_tz::Tz> s != tz_name_logic(&t),
        })]
        fn from_str(s: &str) -> Result<chrono_tz::Tz, chrono_tz::ParseError>;
    }
}
