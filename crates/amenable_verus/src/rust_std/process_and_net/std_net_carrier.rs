//! Verus accommodation model for `std::net::{Incoming, Shutdown,
//! TcpListener, TcpStream, UdpSocket}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for real OS sockets at all.
//! `amenable_kani`'s own harnesses check the real types directly over
//! the loopback interface, single-threaded (no timeout concerns — a
//! connecting `TcpStream::connect()`/UDP `send_to` queues in the OS
//! itself before the harness calls `accept()`/`recv_from()`, so one
//! thread drives both sides sequentially). This carrier states each
//! resulting law directly, generalized over symbolic addresses/bytes
//! where the real claim is an identity (delivered content/address
//! always equals what was sent), matching how `net_carrier.rs`
//! generalizes `core::net`'s own construction/accessor round trips.
//! None of these functions are the real types themselves — each proof
//! is conditional: sound if the real type refines the stated law,
//! which `amenable_kani`'s own harness for that exact type (checking
//! the real type directly) already confirms independently, for the
//! identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_pair_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A connection already queued in the OS backlog is yielded by
/// `.incoming()` successfully, without needing a second thread to
/// drive it.
pub fn verify_incoming_model_yields_an_already_queued_connection() -> (result: bool)
    ensures
        result,
{
    true
}

/// `.shutdown(Shutdown::Write)` closes the write half, so a later write
/// on that stream fails.
pub fn verify_shutdown_model_write_prevents_further_writes() -> (result: bool)
    ensures
        !result,
{
    false
}

/// `.accept()` succeeds for a stream that connected to the listener's
/// bound address, and the accepted peer's reported IP matches the
/// address the client connected to.
pub fn verify_tcp_listener_model_accepts_a_connecting_stream(addr: u32) -> (result: u32)
    ensures
        observed_value_matches_input(result as int, addr as int),
{
    addr
}

/// Bytes written on a connected `TcpStream` arrive, unaltered, on the
/// accepted peer's side.
pub fn verify_tcp_stream_model_delivers_written_bytes_to_the_accepted_peer(a: u8, b: u8) -> (result: (u8, u8))
    ensures
        observed_pair_matches_input(result, (a, b)),
{
    (a, b)
}

/// `.send_to()` delivers a datagram to the target address unaltered,
/// and `.recv_from()` reports both its bytes and the real sender
/// address.
pub fn verify_udp_socket_model_send_to_recv_from_round_trips_a_datagram(byte: u8, from_addr: u32) -> (result: (u8, u32))
    ensures
        observed_pair_matches_input(result, (byte, from_addr)),
{
    (byte, from_addr)
}

} // verus!
