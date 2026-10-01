//! RFC 3339 / IXDTF timestamp parse-output sidecars.
//!
//!
//! Parser exchange output sidecars -- one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    IxdtfTimestampDescriptor, IxdtfTimestampProofToken, IxdtfZonedTimestampProofToken,
    OffsetDateTimeDescriptor, Rfc3339TimestampProofToken, ZonedDateTimeDescriptor,
};

/// Output sidecar for the `parse_rfc3339_timestamp` exchange: [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor)
/// plus a token for [`Rfc3339TimestampProof`](crate::Rfc3339TimestampProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::Rfc3339TimestampProof", constructor = "pub")]
pub struct ParsedRfc3339Timestamp {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: Rfc3339TimestampProofToken,
}
/// Output sidecar for the `parse_ixdtf_timestamp` exchange: [`IxdtfTimestampDescriptor`](crate::IxdtfTimestampDescriptor)
/// plus a token for [`IxdtfTimestampProof`](crate::IxdtfTimestampProof).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::IxdtfTimestampProof", constructor = "pub")]
pub struct ParsedIxdtfTimestamp {
    #[sidecar(primary)]
    descriptor: IxdtfTimestampDescriptor,
    #[sidecar(token)]
    token: IxdtfTimestampProofToken,
}
/// Proven-descriptor input sidecar for the matching formatter method(s).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::IxdtfZonedTimestampProof", constructor = "pub")]
pub struct ParsedIxdtfZonedTimestamp {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: IxdtfZonedTimestampProofToken,
}
