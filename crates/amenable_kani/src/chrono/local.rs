//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::Local>`.
//!
//! Trusted, not proven, and this is a real boundary rather than a tool
//! limitation: `Local`'s offset rules come from the host's own timezone
//! database at runtime (`iana-time-zone`/`winapi`, confirmed in chrono's
//! own `clock` feature), so there is no universal claim like `Utc`'s own
//! "local offset is always Single" to state — depending on which zone the
//! host is configured for, the identical local wall-clock time can be
//! ambiguous or fall in a DST gap. Kani has no way to range over "every
//! possible host timezone database" the way it ranges over every `i32`
//! year. The approved boundary wording (docs/CHRONO_SUPPORT_PLAN.md,
//! decision 13): "`Local` takes its offset rules from the host's timezone
//! database, not from a fixed zone. Our claims hold for the rules the host
//! supplies. Where the host's rules are wrong or absent, our claims don't
//! apply."

use crate::ext_macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(chrono::Local);
