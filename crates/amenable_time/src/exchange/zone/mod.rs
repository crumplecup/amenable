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
//! included since some fields — the plain-value case — round-trip
//! fine, but a real caller needs to set genuinely different, specific
//! field values (an actual IANA identifier, a real resolution
//! authority), not the all-defaulted case.
//!
//! Split by exchange method — the whole file is already one domain
//! (`zone`), so the real concern boundary here is each transition's own
//! request/preconditions/established/input/output lifecycle, not a
//! further domain split.

mod attach_named_zone;
mod confirm_named_zone_revision;
mod identity_and_authority;
mod resolve_local_date_time;

pub use attach_named_zone::{
    AttachNamedZoneEstablished, AttachNamedZoneInput, AttachNamedZoneOutput,
    AttachNamedZonePreconditions, AttachNamedZoneRequest,
};
pub use confirm_named_zone_revision::{
    ConfirmNamedZoneRevisionEstablished, ConfirmNamedZoneRevisionInput,
    ConfirmNamedZoneRevisionOutput, ConfirmNamedZoneRevisionPreconditions,
    ConfirmNamedZoneRevisionRequest,
};
pub use identity_and_authority::{
    ConfirmZoneAuthorityEstablished, ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput,
    ConfirmZoneAuthorityPreconditions, ConfirmZoneAuthorityRequest, ResolvedNamedTimeZone,
};
pub use resolve_local_date_time::{
    ResolveLocalDateTimeEstablished, ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput,
    ResolveLocalDateTimePreconditions, ResolveLocalDateTimeRequest,
};
