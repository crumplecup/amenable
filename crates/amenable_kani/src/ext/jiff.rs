//! `Witness<KaniVerifier>` registrations for `amenable_ext::ExtStandard<T>`
//! over jiff's registered carriers (`jiff::{Timestamp, Zoned,
//! civil::DateTime}` — see `docs/AMENABLE_EXT_PLAN.md`'s Phase 1 for why
//! that set and not a wider one). All trusted for now: jiff is opaque to
//! Kani, the same way most `RustStdStandard<T>` registrations are —
//! `checked` gets added per type only where a real harness adds value
//! over trusting jiff's own correctness.

use crate::ext::macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(jiff::Timestamp, jiff::Zoned, jiff::civil::DateTime);
