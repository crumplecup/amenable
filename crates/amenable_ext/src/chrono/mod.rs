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
#[cfg(feature = "chrono-tz")]
mod discovered_gap_table;
#[cfg(feature = "chrono-tz")]
mod discovered_offset_table;
mod identity;
mod instant;
#[cfg(feature = "chrono-tz")]
mod offset_discovery;
mod trusted_witness;
mod value_types;
#[cfg(feature = "verus")]
mod verus_witness;
#[cfg(feature = "chrono-tz")]
mod zone;

// One combined re-export, not five individually-gated `pub use` lines
// -- cordial's own `cfg-scatter` etiquette flags five or more
// individually-gated items of the same kind in one file as a sign to
// consolidate behind a single gate instead.
#[cfg(feature = "chrono-tz")]
pub use self::{
    bounded_step_walk::BoundedStepWalk,
    discovered_gap_table::{DISCOVERED_GAP_TABLE, DiscoveredGapTableRow},
    discovered_offset_table::{DISCOVERED_OFFSET_TABLE, DiscoveredOffsetTableRow},
    offset_discovery::{
        BoundedStepWalkToken, DiscoveredOffsetEntry, DiscoveredOffsetTable,
        DiscoveredOffsetTableData, OffsetDiscoveryParams, OffsetDiscoveryRequest,
    },
    zone::{ChronoTz, ChronoZoned},
};

pub use civil::{
    ChronoDate, ChronoDateTime, ChronoReducedCalendarDate, ChronoReducedLocalTime, ChronoTime,
    local_date_time_descriptor_to_naive, naive_date_time_to_local_date_time_descriptor,
};
pub use identity::{ChronoTimeBackend, ChronoVerifier, ChronoVerifierMetadata};
pub use instant::{
    ChronoInstant, ChronoOffset, ChronoOffsetDateTime, chrono_to_offset_date_time_descriptor,
    fixed_offset_sign_and_magnitude, fixed_offset_to_utc_offset_descriptor, narrow_to_u8,
    offset_date_time_descriptor_to_chrono, utc_offset_descriptor_to_fixed_offset,
};
