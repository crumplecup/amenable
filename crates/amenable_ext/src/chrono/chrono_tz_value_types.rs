//! `ExtType` registration and `ExtStandard<T>` evidence for `chrono_tz`'s
//! own value types (decision 4/6 in `docs/CHRONO_SUPPORT_PLAN.md`).
//!
//! `chrono_tz::ParseError` still needs its claim kind decided from its
//! real role (decision 6) before a witness is written for it -- not
//! done yet, deliberately.
//!
//! `GapInfo` has no derives at all in chrono-tz's own source (confirmed
//! by reading it) -- irrelevant to this registration, since
//! `ExtStandard<T>` is a bare `PhantomData<*const T>` marker that never
//! stores a real `T` value, so `T`'s own derive stack (or lack of one)
//! places no requirement on it.

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

impl_ext_type!(
    chrono_tz::GapInfo,
    "chrono-tz contributors",
    "chrono-tz",
    "chrono_tz",
    "https://docs.rs/chrono-tz/latest/chrono_tz/struct.GapInfo.html",
    "Information about a local-time gap (a DST forward jump): the real offset just before it and the real zoned instant just after it."
);

register_ext_standard_evidence!(chrono_tz::Tz, chrono_tz::TzOffset, chrono_tz::GapInfo);
