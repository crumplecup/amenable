//! Local/offset date-time parse composite proofs.
//!
//!
//! Per-method composite proof propositions -- for the parse methods
//! whose `elicit_temporal` return tuple carries 2+ proof sidecars, the
//! sidecars folded into one `#[derive(Evidence, Witness)]` struct (the
//! same folding as `proof_composition`) so the output `Sidecar` keeps
//! its single-`token` shape.

use crate::{
    LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid, OffsetDateTimeValid,
    TimestampRepresentsFixedInstant,
};

/// Proof for [`ParsedLocalDateTime`](crate::ParsedLocalDateTime) — the 2 proof sidecars `elicit_temporal`'s `parse_local_date_time` returns, folded into one composite proposition.
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
pub struct LocalDateTimeProof {
    /// The `local_date_time_valid` sub-claim.
    local_date_time_valid: LocalDateTimeValid,
    /// The `local_date_time_does_not_identify_fixed_instant` sub-claim.
    local_date_time_does_not_identify_fixed_instant: LocalDateTimeDoesNotIdentifyFixedInstant,
}
/// Proof for [`ParsedOffsetDateTime`](crate::ParsedOffsetDateTime) — the 2 proof sidecars `elicit_temporal`'s `parse_offset_date_time` returns, folded into one composite proposition.
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
pub struct OffsetDateTimeProof {
    /// The `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
}
