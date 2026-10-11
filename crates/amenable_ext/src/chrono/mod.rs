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
//! - **instant (Phase 3):** the `TemporalInstantProps` carriers and the
//!   `OffsetDateTime` realize and reflect bridge.
//! - **zone (Phase 3):** the `TemporalZoneProps` carriers and the
//!   `NamedTimeZone`/`ZonedDateTime` realize and reflect bridge, over
//!   chrono-tz.

#[cfg(feature = "chrono-tz")]
mod bounded_step_walk;
#[cfg(feature = "chrono-tz")]
mod chrono_tz_value_types;
mod civil;
mod identity;
mod instant;
mod trusted_witness;
mod value_types;
#[cfg(feature = "verus")]
mod verus_witness;
#[cfg(feature = "chrono-tz")]
mod zone;

#[cfg(feature = "chrono-tz")]
pub use bounded_step_walk::BoundedStepWalk;
pub use civil::{
    ChronoDate, ChronoDateTime, ChronoReducedCalendarDate, ChronoReducedLocalTime, ChronoTime,
};
pub use identity::{ChronoTimeBackend, ChronoVerifier, ChronoVerifierMetadata};
pub use instant::{ChronoInstant, ChronoOffset, ChronoOffsetDateTime};
#[cfg(feature = "chrono-tz")]
pub use zone::{ChronoTz, ChronoZoned};
