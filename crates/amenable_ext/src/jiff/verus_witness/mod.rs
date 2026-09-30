//! `Witness<VerusVerifier>` for `ExtStandard<T>` over jiff's registered
//! carriers. Lives here, not in a backend crate: `VerusVerifier` moved
//! to `amenable_core` in the Phase 8 dependency reversal (see
//! `docs/AMENABLE_TIME_PLAN.md`), which this crate already depends on
//! unconditionally, so the bridge lives with the type registrations —
//! mirroring `amenable_std::verus_witness`'s own placement (see
//! `docs/AMENABLE_EXT_PLAN.md`'s Architecture section).
//!
//! Split by concern: `bridge` (the trusted/checked macros and the
//! `ExtCheckedProof` proof artifact), `trusted` (the types with
//! nothing beyond `Evidence::basis().audit()` to rest on), and
//! `checked_{top_level,civil,fmt,tz}` (types backed by a real,
//! hand-verified Verus accommodation model in `amenable_verus::jiff`,
//! grouped by jiff sub-namespace). See `trusted.rs`'s own doc comment
//! for the trusted/checked split rationale in full.

mod bridge;
mod checked_civil;
mod checked_fmt;
mod checked_top_level;
mod checked_tz;
mod trusted;
