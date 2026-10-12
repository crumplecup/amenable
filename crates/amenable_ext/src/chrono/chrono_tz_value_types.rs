//! `ExtType` registration and `ExtStandard<T>` evidence for `chrono_tz`'s
//! own value types (decision 4/6 in `docs/CHRONO_SUPPORT_PLAN.md`).
//!
//! `GapInfo` and `chrono_tz::ParseError` still need their claim kind
//! decided from their real role (decision 6) before a witness is written
//! for either -- not done yet, deliberately: `GapInfo` in particular has
//! no derives at all in chrono-tz's own source (confirmed by reading
//! it), which needs real thought about what, if anything, a wrapper's
//! own derive stack and claim should look like, not a rushed trusted
//! stub.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    chrono_tz::Tz,
    "chrono-tz contributors",
    "chrono-tz",
    "chrono_tz",
    "https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html",
    "An IANA time zone, one of about 600 compiled-in variants, implementing chrono's TimeZone."
);

impl_ext_type!(
    chrono_tz::TzOffset,
    "chrono-tz contributors",
    "chrono-tz",
    "chrono_tz",
    "https://docs.rs/chrono-tz/latest/chrono_tz/struct.TzOffset.html",
    "A resolved offset for a specific Tz at a specific instant, carrying the base/DST split."
);

register_ext_standard_evidence!(chrono_tz::Tz, chrono_tz::TzOffset);
