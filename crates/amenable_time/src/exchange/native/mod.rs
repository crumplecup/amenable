//! Phase 5 Step 4b — the `*_native` factory analogs' exchange types.
//! Carrier<->carrier: the native-carrier versions of the
//! descriptor-side factory transitions. Multi-input methods fold their
//! runtime values into a `<M>NativeRequest<B>` primary (basis = the
//! [`NativeCarrierRequest`] marker, so the compound needs no `Default`);
//! methods with an extra output proof fold it into a `<M>NativeEstablished`
//! composite behind a `<M>NativeToken`.
//!
//! Split by real domain: `conversion` (normalize/strip-zone/adjust-
//! precision/truncate-subseconds, further split by sub-concern since it
//! holds 14 types), `zone` (named-zone/local-date-time resolution),
//! `interval` (offset-endpoint ordering), `extension` (date-time-formula
//! evaluation) — matching the domain split already used throughout this
//! crate.

mod conversion;
mod extension;
mod interval;
mod zone;

pub use conversion::{
    AdjustPrecisionLosslesslyNativeEstablished, AdjustPrecisionLosslesslyNativeInput,
    AdjustPrecisionLosslesslyNativeOutput, AdjustPrecisionLosslesslyNativeRequest,
    AdjustPrecisionLosslesslyNativeToken, NativeCarrierRequest, NormalizeToUtcNativeEstablished,
    NormalizeToUtcNativeOutput, NormalizeToUtcNativeToken, TruncateSubsecondsNativeEstablished,
    TruncateSubsecondsNativeInput, TruncateSubsecondsNativeOutput, TruncateSubsecondsNativeRequest,
    TruncateSubsecondsNativeToken,
};
pub use extension::{
    EvaluateDateTimeFormulaNativeEstablished, EvaluateDateTimeFormulaNativeOutput,
    EvaluateDateTimeFormulaNativeToken,
};
pub use interval::{
    OrderOffsetEndpointsNativeInput, OrderOffsetEndpointsNativeOutput,
    OrderOffsetEndpointsNativeRequest,
};
pub use zone::{
    AttachNamedZoneNativeInput, AttachNamedZoneNativeRequest, ConfirmNamedZoneRevisionNativeOutput,
    ResolveLocalDateTimeNativeEstablished, ResolveLocalDateTimeNativeInput,
    ResolveLocalDateTimeNativeOutput, ResolveLocalDateTimeNativeRequest,
    ResolveLocalDateTimeNativeToken,
};
