//! Local/offset date-time parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    LocalDateTimeDescriptor, LocalDateTimeProofToken, OffsetDateTimeDescriptor,
    OffsetDateTimeProofToken,
};

/// Output sidecar for the `parse_local_date_time` exchange: [`LocalDateTimeDescriptor`](crate::LocalDateTimeDescriptor)
/// plus a token for [`LocalDateTimeProof`](crate::LocalDateTimeProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::LocalDateTimeProof", constructor = "pub")]
pub struct ParsedLocalDateTime {
    #[sidecar(primary)]
    descriptor: LocalDateTimeDescriptor,
    #[sidecar(token)]
    token: LocalDateTimeProofToken,
}
/// Output sidecar for the `parse_offset_date_time` exchange: [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor)
/// plus a token for [`OffsetDateTimeProof`](crate::OffsetDateTimeProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::OffsetDateTimeProof", constructor = "pub")]
pub struct ParsedOffsetDateTime {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: OffsetDateTimeProofToken,
}
