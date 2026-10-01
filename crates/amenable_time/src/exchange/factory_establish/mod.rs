//! Output tokens and their [`Establish`](amenable_core::Establish)
//! edges for the zone / conversion / interval factories.
//!
//! Each transition mints two edges: a `*PreconditionsToken`
//! established from the received input (the caller's individual
//! precondition sidecars are folded into the `*Preconditions`
//! composite, whose `Witness<V>` carries the real content), and a
//! `*EstablishedToken` established from that preconditions token.
//!
//! Split by real domain: `zone`, `conversion`, `interval` — matching
//! the domain split already used by `exchange::zone`/`::conversion`
//! and `exchange::native`'s own interval/order-offset-endpoints bucket.

mod conversion;
mod interval;
mod zone;

pub use conversion::{
    AdjustPrecisionLosslesslyEstablishedToken, AdjustPrecisionLosslesslyPreconditionsToken,
    NormalizeToUtcEstablishedToken, NormalizeToUtcPreconditionsToken,
    StripNamedZoneEstablishedToken, StripNamedZonePreconditionsToken,
    TruncateSubsecondsEstablishedToken, TruncateSubsecondsPreconditionsToken,
};
pub use interval::{OrderOffsetEndpointsEstablishedToken, OrderOffsetEndpointsPreconditionsToken};
pub use zone::{
    AttachNamedZoneEstablishedToken, AttachNamedZonePreconditionsToken,
    ConfirmNamedZoneRevisionEstablishedToken, ConfirmNamedZoneRevisionPreconditionsToken,
    ConfirmZoneAuthorityEstablishedToken, ConfirmZoneAuthorityPreconditionsToken,
    NamedTimeZoneIdentityValidToken, ResolveLocalDateTimeEstablishedToken,
    ResolveLocalDateTimePreconditionsToken,
};
