//! The `confirm_named_zone_revision` exchange.
//!
//!
//! `TemporalZoneFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.
//!
//! Each `*Request` type gets a real `derive_new::new` constructor (a
//! genuine gap fixed while building the first real backend against
//! this trait, `amenable_ext::jiff`'s own Phase 4b): these fields are
//! private with only `derive_getters::Getters` read access, and none of
//! them previously had any public constructor at all, `Default`
//! included since some fields -- the plain-value case -- round-trip
//! fine, but a real caller needs to set genuinely different, specific
//! field values (an actual IANA identifier, a real resolution
//! authority), not the all-defaulted case.

use derive_new::new;

use crate::{
    ConfirmNamedZoneRevisionEstablishedToken, ConfirmNamedZoneRevisionPreconditionsToken,
    NamedTimeZoneInterpretationTracksTzdbRevision, ZonedDateTimeDescriptor,
    ZonedDateTimeHasNamedZone,
};

/// Descriptors the `confirm_named_zone_revision` exchange consumes.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_getters::Getters,
    new,
)]
#[evidence(basis = "Self")]
pub struct ConfirmNamedZoneRevisionRequest {
    /// The `timestamp` descriptor.
    timestamp: ZonedDateTimeDescriptor,
}
/// Preconditions the `confirm_named_zone_revision` exchange requires — the
/// caller's `Established<_>` sidecars, folded into one proposition.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ConfirmNamedZoneRevisionPreconditions {
    /// The established `zoned_date_time_has_named_zone` sub-claim.
    zoned_date_time_has_named_zone: ZonedDateTimeHasNamedZone,
}
/// Proofs the `confirm_named_zone_revision` exchange re-issues, folded into one
/// proposition (the `proof_composition` structural closure).
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ConfirmNamedZoneRevisionEstablished {
    /// The re-issued `named_time_zone_interpretation_tracks_tzdb_revision` sub-claim.
    named_time_zone_interpretation_tracks_tzdb_revision:
        NamedTimeZoneInterpretationTracksTzdbRevision,
}
/// Input sidecar for the `confirm_named_zone_revision` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::ConfirmNamedZoneRevisionPreconditions",
    constructor = "pub"
)]
pub struct ConfirmNamedZoneRevisionInput {
    #[sidecar(primary)]
    request: ConfirmNamedZoneRevisionRequest,
    #[sidecar(token)]
    token: ConfirmNamedZoneRevisionPreconditionsToken,
}
/// Output sidecar for the `confirm_named_zone_revision` exchange — no descriptor,
/// so the folded [`ConfirmNamedZoneRevisionEstablished`](crate::ConfirmNamedZoneRevisionEstablished) proposition
/// is itself the primary payload.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(constructor = "pub")]
pub struct ConfirmNamedZoneRevisionOutput {
    #[sidecar(primary)]
    established: ConfirmNamedZoneRevisionEstablished,
    #[sidecar(token)]
    token: ConfirmNamedZoneRevisionEstablishedToken,
}
