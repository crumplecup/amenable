//! Duration/interval parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    DurationDescriptor, DurationFormValidToken, RecurringIntervalDescriptor,
    RecurringIntervalFormValidToken, TimeIntervalDescriptor, TimeIntervalProofToken,
};

/// Output sidecar for the `parse_time_interval` exchange: [`TimeIntervalDescriptor`](crate::TimeIntervalDescriptor)
/// plus a token for [`TimeIntervalProof`](crate::TimeIntervalProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TimeIntervalProof", constructor = "pub")]
pub struct ParsedTimeInterval {
    #[sidecar(primary)]
    descriptor: TimeIntervalDescriptor,
    #[sidecar(token)]
    token: TimeIntervalProofToken,
}
/// Proven-descriptor input sidecar for the matching formatter method(s).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::DurationFormValid", constructor = "pub")]
pub struct ParsedDuration {
    #[sidecar(primary)]
    descriptor: DurationDescriptor,
    #[sidecar(token)]
    token: DurationFormValidToken,
}
/// Proven-descriptor input sidecar for the matching formatter method(s).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::RecurringIntervalFormValid", constructor = "pub")]
pub struct ParsedRecurringInterval {
    #[sidecar(primary)]
    descriptor: RecurringIntervalDescriptor,
    #[sidecar(token)]
    token: RecurringIntervalFormValidToken,
}
