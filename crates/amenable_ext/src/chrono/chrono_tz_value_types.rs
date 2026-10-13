//! `ExtType` registration and `ExtStandard<T>` evidence for `chrono_tz`'s
//! own value types (decision 4/6 in `docs/CHRONO_SUPPORT_PLAN.md`).
//!
//! `chrono_tz::ParseError`'s claim kind (decision 6): proven on Kani,
//! an accommodation model on Creusot. Its real source (`pub struct
//! ParseError(());`, `impl FromStr for Tz { fn from_str(s) {
//! TIMEZONES.get(s).cloned().ok_or(ParseError(())) } }`) carries zero
//! information -- the only possible claim about it is *when* it
//! occurs, which is exactly the negative direction of `Tz`'s own
//! name/`FromStr` round trip (`Tz`'s own witnesses only exercise the
//! positive direction). Kani checks this for real, by symbolic
//! execution over three concrete witness strings. Creusot cannot:
//! `creusot-std`'s own `str::view()` is fully opaque for every string,
//! including a literal (confirmed by reading its real source, after
//! five genuine attempts at a mechanical proof, each hitting a
//! newly-confirmed wall) -- a real gap in Creusot's string model
//! itself. The Creusot witness is `#[trusted]`, backed by a named,
//! audited `Ensures<CreusotVerifier>` bound (`amenable_creusot::
//! chrono::parse_error`), the accommodation-model shape this project
//! uses whenever a bound genuinely has no exec representation to
//! mechanically check.
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

impl_ext_type!(
    chrono_tz::ParseError,
    "chrono-tz contributors",
    "chrono-tz",
    "chrono_tz",
    "https://docs.rs/chrono-tz/latest/chrono_tz/struct.ParseError.html",
    "The error returned when a string does not name any real IANA time zone recognized by Tz's FromStr."
);

register_ext_standard_evidence!(
    chrono_tz::Tz,
    chrono_tz::TzOffset,
    chrono_tz::GapInfo,
    chrono_tz::ParseError
);
