//! Verus accommodation model for `core::net::{Ipv4Addr, Ipv6Addr,
//! IpAddr, SocketAddrV4, SocketAddrV6, SocketAddr}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Each of these six real types checks a construction/accessor
//! round-trip claim in `amenable_kani` — this carrier states each one
//! directly rather than constructing real network address types (which
//! `vstd` has no spec support for at all). `IpAddr`/`SocketAddr`'s
//! variant tags are modeled as a plain `bool` (`true` for the V4 case)
//! standing in for the real two-variant enum, matching
//! `amenable_kani`'s own note that both need constructing explicitly
//! (no `Arbitrary` impl for `IpAddr`) rather than sampled symbolically.
//! None of these functions are `Ipv4Addr`/`Ipv6Addr`/`IpAddr`/
//! `SocketAddrV4`/`SocketAddrV6`/`SocketAddr` themselves — each proof is
//! conditional: sound if the real type refines the stated round trip,
//! which `amenable_kani`'s own harness for that exact type (checking
//! the real type directly) already confirms independently, for the
//! identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_pair_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Four `u8` octets, as `Ipv4Addr::octets` returns.
type Octets = (u8, u8, u8, u8);

/// Four `u16` segment groups — half of what `Ipv6Addr::segments`
/// returns.
type SegmentsHalf = (u16, u16, u16, u16);

/// `is_ipv4`, `is_ipv6`, round-tripped V4 octets, V6's `is_ipv4`, V6's
/// `is_ipv6` — the fields `verify_ip_addr_model_variant_matches_its_
/// kind` checks.
type IpAddrVariantResult = (bool, bool, Octets, bool, bool);

/// `Ipv4Addr::new(a, b, c, d).octets()` round-trips the four
/// constructor bytes.
pub fn verify_ipv4_addr_model_octets_round_trip(a: u8, b: u8, c: u8, d: u8) -> (result: Octets)
    ensures
        result == (a, b, c, d),
{
    (a, b, c, d)
}

/// `Ipv6Addr::new(a, .., h).segments()` round-trips the eight
/// constructor groups — grouped into two four-tuples to stay within a
/// reasonable argument count.
pub fn verify_ipv6_addr_model_segments_round_trip(first_four: SegmentsHalf, last_four: SegmentsHalf) -> (result: (SegmentsHalf, SegmentsHalf))
    ensures
        observed_pair_matches_input(result, (first_four, last_four)),
{
    (first_four, last_four)
}

/// A singleton claim: `IpAddr`'s `V4` variant always round-trips its
/// wrapped octets exactly. `observed_pair_matches_input` doesn't fit
/// (`Octets` is a 4-tuple, not a 2-element pair) and
/// `observed_value_matches_input` doesn't either (it's `int`-typed, not
/// a compound value), so this claim gets its own dedicated predicate
/// rather than a risky cast onto either existing one.
pub open spec fn ip_addr_model_v4_octets_match_input(observed: Octets, input: Octets) -> bool {
    observed == input
}

/// `IpAddr`'s `is_ipv4`/`is_ipv6` agree with its constructing variant,
/// and each variant round-trips its wrapped address — modeled with a
/// `bool` variant tag (`true` for `V4`) standing in for the real
/// two-variant enum.
pub fn verify_ip_addr_model_variant_matches_its_kind(v4_octets: Octets) -> (result: IpAddrVariantResult)
    ensures
        result.0,
        !result.1,
        ip_addr_model_v4_octets_match_input(result.2, v4_octets),
        result.4,
        !result.3,
{
    (true, false, v4_octets, false, true)
}

/// `SocketAddrV4::new(ip, port)` round-trips both its address and port.
pub fn verify_socket_addr_v4_model_round_trips_ip_and_port(ip_octets: Octets, port: u16) -> (result: (Octets, u16))
    ensures
        observed_pair_matches_input(result, (ip_octets, port)),
{
    (ip_octets, port)
}

/// `SocketAddrV6::new(ip, port, flowinfo, scope_id)` round-trips port,
/// flow info, and scope id (the three fields `SocketAddrV6` adds beyond
/// the address itself, which `verify_ipv6_addr_model_segments_round_
/// trip` already covers).
pub fn verify_socket_addr_v6_model_round_trips_all_fields(port: u16, flowinfo: u32, scope_id: u32) -> (result: (u16, u32, u32))
    ensures
        result == (port, flowinfo, scope_id),
{
    (port, flowinfo, scope_id)
}

/// `SocketAddr`'s `is_ipv4`/`is_ipv6` agree with its constructing
/// variant, and `.port()` dispatches through either variant to the same
/// port — modeled with a `bool` variant tag (`true` for `V4`).
pub fn verify_socket_addr_model_variant_matches_its_kind(port: u16) -> (result: (bool, bool, u16, bool, bool, u16))
    ensures
        result.0,
        !result.1,
        observed_value_matches_input(result.2 as int, port as int),
        result.4,
        !result.3,
        observed_value_matches_input(result.5 as int, port as int),
{
    (true, false, port, false, true, port)
}

} // verus!
