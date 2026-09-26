//! `ExtType` registrations for [`jiff`], split into one module per real
//! `jiff` area, mirroring `amenable_std::rust_std`'s own per-source-area
//! split.
//!
//! Nothing in the registration submodules is `pub use`d: `impl_ext_type!`
//! implements `ExtType` directly on the foreign `jiff` type, so there is
//! no new local type to export — the same shape `amenable_std::rust_std`'s
//! own per-type registration files follow. `backend` is the one
//! exception (see its own doc comment): it defines real new local types
//! (`JiffVerifier`, `JiffTimeBackend`, …), re-exported here so `lib.rs`
//! can reach them despite this module's own privacy.

mod backend;
mod civil;
mod fmt;
mod misc;
mod span;
mod timestamp;
mod tz;
#[cfg(feature = "verus")]
mod verus_witness;
mod zoned;

pub use backend::{
    JiffDate, JiffDateTime, JiffISOWeekDate, JiffOffset, JiffOffsetDateTime, JiffRecurringInterval,
    JiffReducedCalendarDate, JiffReducedLocalTime, JiffSpan, JiffTime, JiffTimeBackend,
    JiffTimeInterval, JiffTimeIntervalEndpoint, JiffTimeIntervalRepresentation, JiffTimeZone,
    JiffTimestamp, JiffVerifier, JiffVerifierMetadata, JiffZoned,
};
