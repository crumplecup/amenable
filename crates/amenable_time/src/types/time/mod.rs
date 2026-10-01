//! Time-of-day, UTC-offset, and time-scale descriptors: the wall-clock
//! half of the temporal accord, below the date/time-zone composites.
//!
//! Split by real domain: `local` (local/UTC-of-day/the unifying
//! `TimeDescriptor` form-selection enum), `standard_and_shift`
//! (standard-time and explicit-form time-of-day/time-shift), `zone`
//! (UTC-offset descriptors) — matching the `clock`/`zone` domain split
//! already used by `proof_composition::composites`.

mod local;
mod standard_and_shift;
mod zone;

pub use local::{
    LocalTimeDescriptor, LocalTimeDescriptorBuilder, ReducedLocalTimeDescriptor, TimeDescriptor,
};
pub use standard_and_shift::{
    ExplicitTimeOfDayDescriptor, ExplicitTimeOfDayDescriptorBuilder, ExplicitTimeShiftDescriptor,
    ExplicitTimeShiftDescriptorBuilder, LocalTimeScaleDescriptor, StandardTimeDescriptor,
    StandardTimeOfDayDescriptor,
};
pub use zone::{
    UtcOfDayDescriptor, UtcOffsetDescriptor, UtcOffsetDescriptorBuilder, UtcOffsetRelationship,
    UtcOffsetSign, UtcTimeScaleDescriptor,
};
