//! `Witness<VerusVerifier>` for `ExtStandard<T>` over chrono's registered
//! carriers. Lives here, not in a backend crate, mirroring
//! `jiff::verus_witness`'s own placement and rationale exactly.
//!
//! Split by concern, the same shape as `jiff::verus_witness`:
//! `crate::ext_verus_witness` (the trusted/checked macros and the
//! `ExtCheckedProof` proof artifact, shared with every other target's own
//! `verus_witness` module) and `checked_civil` (types backed by a real,
//! hand-verified Verus accommodation model in `amenable_verus::chrono`).

mod checked_civil;
mod trusted;
