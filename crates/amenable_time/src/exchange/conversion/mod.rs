//! `TemporalConversionFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.
//!
//! Each `*Request` type gets a real `derive_new::new` constructor —
//! the same real, pre-existing gap already fixed in `exchange/zone.rs`
//! (private fields, `Getters`-only read access, no public constructor
//! of any kind).
//!
//! Split by exchange method — the whole file is already one domain
//! (`conversion`), so the real concern boundary here is each
//! transition's own request/preconditions/established/input/output
//! lifecycle, not a further domain split.

mod adjust_precision_losslessly;
mod normalize_to_utc;
mod strip_named_zone;
mod truncate_subseconds;

pub use adjust_precision_losslessly::{
    AdjustPrecisionLosslesslyEstablished, AdjustPrecisionLosslesslyInput,
    AdjustPrecisionLosslesslyOutput, AdjustPrecisionLosslesslyPreconditions,
    AdjustPrecisionLosslesslyRequest,
};
pub use normalize_to_utc::{
    NormalizeToUtcEstablished, NormalizeToUtcInput, NormalizeToUtcOutput,
    NormalizeToUtcPreconditions, NormalizeToUtcRequest,
};
pub use strip_named_zone::{
    StripNamedZoneEstablished, StripNamedZoneInput, StripNamedZoneOutput,
    StripNamedZonePreconditions, StripNamedZoneRequest,
};
pub use truncate_subseconds::{
    TruncateSubsecondsEstablished, TruncateSubsecondsInput, TruncateSubsecondsOutput,
    TruncateSubsecondsPreconditions, TruncateSubsecondsRequest,
};
