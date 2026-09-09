//! [`TemporalZoneFactory`] — the zone half of the temporal seam,
//! ported from `elicit_temporal::traits::TemporalZoneFactory`. Same shape
//! as [`TemporalParser`](crate::TemporalParser): the supertrait bundle
//! *is* the zone exchanges. The `where <Prop>: Witness<V>` bounds —
//! one per distinct precondition *and* re-issued proposition — are the
//! honest precondition, backend-provided.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    AttachNamedZoneEstablished, AttachNamedZoneInput, AttachNamedZoneOutput,
    AttachNamedZonePreconditions, ConfirmNamedZoneRevisionEstablished,
    ConfirmNamedZoneRevisionInput, ConfirmNamedZoneRevisionOutput,
    ConfirmNamedZoneRevisionPreconditions, ConfirmZoneAuthorityEstablished,
    ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput, ConfirmZoneAuthorityPreconditions,
    NamedTimeZoneIdentityValid, RawInput, ResolveLocalDateTimeEstablished,
    ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput, ResolveLocalDateTimePreconditions,
    ResolvedNamedTimeZone, TemporalError, TemporalInputReceived,
};

/// A backend that performs every zone transition as an
/// [`Exchange`], for verifier `V`.
pub trait TemporalZoneFactory<V: Verifier>:
    Send
    + Sync
    + Exchange<RawInput, ResolvedNamedTimeZone, V, Error = TemporalError>
    + Exchange<ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput, V, Error = TemporalError>
    + Exchange<ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput, V, Error = TemporalError>
    + Exchange<AttachNamedZoneInput, AttachNamedZoneOutput, V, Error = TemporalError>
    + Exchange<
        ConfirmNamedZoneRevisionInput,
        ConfirmNamedZoneRevisionOutput,
        V,
        Error = TemporalError,
    >
where
    TemporalInputReceived: Witness<V>,
    NamedTimeZoneIdentityValid: Witness<V>,
    ConfirmZoneAuthorityPreconditions: Witness<V>,
    ConfirmZoneAuthorityEstablished: Witness<V>,
    ResolveLocalDateTimePreconditions: Witness<V>,
    ResolveLocalDateTimeEstablished: Witness<V>,
    AttachNamedZonePreconditions: Witness<V>,
    AttachNamedZoneEstablished: Witness<V>,
    ConfirmNamedZoneRevisionPreconditions: Witness<V>,
    ConfirmNamedZoneRevisionEstablished: Witness<V>,
{
}

impl<T, V> TemporalZoneFactory<V> for T
where
    V: Verifier,
    TemporalInputReceived: Witness<V>,
    NamedTimeZoneIdentityValid: Witness<V>,
    ConfirmZoneAuthorityPreconditions: Witness<V>,
    ConfirmZoneAuthorityEstablished: Witness<V>,
    ResolveLocalDateTimePreconditions: Witness<V>,
    ResolveLocalDateTimeEstablished: Witness<V>,
    AttachNamedZonePreconditions: Witness<V>,
    AttachNamedZoneEstablished: Witness<V>,
    ConfirmNamedZoneRevisionPreconditions: Witness<V>,
    ConfirmNamedZoneRevisionEstablished: Witness<V>,
    T: Send
        + Sync
        + Exchange<RawInput, ResolvedNamedTimeZone, V, Error = TemporalError>
        + Exchange<ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput, V, Error = TemporalError>
        + Exchange<ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput, V, Error = TemporalError>
        + Exchange<AttachNamedZoneInput, AttachNamedZoneOutput, V, Error = TemporalError>
        + Exchange<
            ConfirmNamedZoneRevisionInput,
            ConfirmNamedZoneRevisionOutput,
            V,
            Error = TemporalError,
        >,
{
}
