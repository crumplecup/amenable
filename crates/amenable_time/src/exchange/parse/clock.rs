//! Local-time/UTC-offset parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    LocalTimeDescriptor, LocalTimeValidToken, ReducedLocalTimeDescriptor,
    ReducedLocalTimeValidToken, UtcOffsetDescriptor, UtcOffsetValidToken,
};

/// Output sidecar for the `parse_local_time` exchange: [`LocalTimeDescriptor`](crate::LocalTimeDescriptor)
/// plus a token for [`LocalTimeValid`](crate::LocalTimeValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::LocalTimeValid", constructor = "pub")]
pub struct ParsedLocalTime {
    #[sidecar(primary)]
    descriptor: LocalTimeDescriptor,
    #[sidecar(token)]
    token: LocalTimeValidToken,
}
/// Output sidecar for the `parse_reduced_local_time` exchange: [`ReducedLocalTimeDescriptor`](crate::ReducedLocalTimeDescriptor)
/// plus a token for [`ReducedLocalTimeValid`](crate::ReducedLocalTimeValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::ReducedLocalTimeValid", constructor = "pub")]
pub struct ParsedReducedLocalTime {
    #[sidecar(primary)]
    descriptor: ReducedLocalTimeDescriptor,
    #[sidecar(token)]
    token: ReducedLocalTimeValidToken,
}
/// Output sidecar for the `parse_utc_offset` exchange: [`UtcOffsetDescriptor`](crate::UtcOffsetDescriptor)
/// plus a token for [`UtcOffsetValid`](crate::UtcOffsetValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::UtcOffsetValid", constructor = "pub")]
pub struct ParsedUtcOffset {
    #[sidecar(primary)]
    descriptor: UtcOffsetDescriptor,
    #[sidecar(token)]
    token: UtcOffsetValidToken,
}
