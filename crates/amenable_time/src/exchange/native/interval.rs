//! Offset-endpoint-ordering native-carrier exchange types.
//!
//! Phase 5 Step 4b -- the `*_native` factory analogs' exchange types.
//! Carrier<->carrier: the native-carrier versions of the
//! descriptor-side factory transitions. Multi-input methods fold their
//! runtime values into a `<M>NativeRequest<B>` primary (basis = the
//! [`NativeCarrierRequest`] marker, so the compound needs no `Default`);
//! methods with an extra output proof fold it into a `<M>NativeEstablished`
//! composite behind a `<M>NativeToken`.

use derive_new::new;

use crate::{
    IntervalEndpointOrderingBundle, IntervalEndpointOrderingBundleToken, TemporalInputToken,
    TemporalInstantProps,
};

/// Runtime values the `order_offset_endpoints_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters, new)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct OrderOffsetEndpointsNativeRequest<B: TemporalInstantProps> {
    /// The `start` input.
    start: B::OffsetDateTime,
    /// The `end` input.
    end: B::OffsetDateTime,
}
/// Input sidecar for the `order_offset_endpoints_native` exchange.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct OrderOffsetEndpointsNativeInput<B: TemporalInstantProps> {
    #[sidecar(primary)]
    request: OrderOffsetEndpointsNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}
/// Output sidecar for the `order_offset_endpoints_native` exchange — pure proof,
/// the [`IntervalEndpointOrderingBundle`](crate::IntervalEndpointOrderingBundle) it re-issues + its token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::IntervalEndpointOrderingBundle",
    constructor = "pub"
)]
pub struct OrderOffsetEndpointsNativeOutput {
    #[sidecar(primary)]
    bundle: IntervalEndpointOrderingBundle,
    #[sidecar(token)]
    token: IntervalEndpointOrderingBundleToken,
}
