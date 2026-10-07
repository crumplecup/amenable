//! A real chrono-backed `amenable_time` temporal backend.
//!
//! Lives in `amenable_ext`, beside the jiff backend, for the same reason: the
//! `Exchange` impls are `impl ForeignTrait for LocalType`, which the orphan rule
//! allows here. See `docs/CHRONO_SUPPORT_PLAN.md` for the phased plan this
//! module follows.
//!
//! One file per family:
//!
//! - **identity:** `ChronoVerifier`, the verifier marker, and `ChronoTimeBackend`,
//!   the struct every family's `Exchange` impls target.
//! - **civil (Phase 2):** the `TemporalCivilProps` carriers and the
//!   `LocalDateTime` realize and reflect bridge.

mod civil;
mod identity;
mod trusted_witness;
mod value_types;

pub use civil::{
    ChronoDate, ChronoDateTime, ChronoReducedCalendarDate, ChronoReducedLocalTime, ChronoTime,
};
pub use identity::{ChronoTimeBackend, ChronoVerifier, ChronoVerifierMetadata};
