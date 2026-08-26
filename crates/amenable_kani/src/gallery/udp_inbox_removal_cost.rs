//! Isolates a real CBMC timeout hit while building `net_model::
//! KaniUdpSocket` for the `std::net` production proofs
//! (`rust_std::std_net`): a `Vec<(SocketAddr, Vec<u8>)>` inbox holding
//! exactly one queued datagram, drained via `Vec::remove(0)`, times out
//! -- even though every value involved is fully concrete (no
//! `kani::any()` anywhere), and the same byte-content comparison
//! (`assert_eq!` against a fixed byte-string literal) already verifies
//! fast elsewhere in this same crate
//! (`rust_std::std_net::verify_tcp_stream_delivers_written_bytes_to_the_accepted_peer`).
//!
//! **Root cause, isolated by direct substitution:** not the byte
//! comparison, not `SocketAddr`'s own `PartialEq` (a bare `assert_eq!`
//! on two `SocketAddr`s alone is unaffected -- see this module's own
//! passing case below) -- specifically `Vec::remove(0)` shifting
//! elements in a `Vec` whose element type is a tuple carrying a
//! `SocketAddr` (a moderately large, padded enum) alongside a
//! heap-backed `Vec<u8>`. CBMC's `memcmp` builtin unwinds past 1000+
//! iterations reasoning about the removal even with a single queued
//! element (nothing to shift), the same `<builtin-library-memcmp>`
//! signature this crate's own catalogued "symbolic-length memcmp"
//! timeout class already names (see `ledger_account_id_comparison`'s
//! doc comment) -- just reached through element removal instead of a
//! direct content comparison this time.
//!
//! **The fix**: `net_model::KaniUdpSocket`'s inbox never needed more
//! than one queued datagram for any of this crate's production claims
//! (matching `net_model::KaniTcpListener`'s own "one pending connection
//! at a time" backlog), so the `Vec<(SocketAddr, Vec<u8>)>` became
//! `Option<(SocketAddr, Vec<u8>)>`, drained via `.take()` instead of
//! `.remove(0)` -- same data, no removal/shift machinery at all. The
//! *only* change between the timeout and the pass below is that one
//! storage type.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::udp_inbox_removal_cost::vec_remove_from_single_element_socket_addr_tuple_times_out".to_owned(),
            "gallery::udp_inbox_removal_cost::vec_remove_from_single_element_socket_addr_tuple_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "Vec::remove(0) on a one-element Vec<(SocketAddr, Vec<u8>)> times out via unbounded memcmp unwinding, despite every value being concrete".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, VEC_REMOVE_FROM_SINGLE_ELEMENT_SOCKET_ADDR_TUPLE_TIMES_OUT_SRC, {
        /// The exact shape `net_model::KaniUdpSocket`'s inbox first
        /// used: a `Vec` holding one `(SocketAddr, Vec<u8>)` entry,
        /// drained with `.remove(0)`. Nothing here is symbolic -- the
        /// address and payload are both fixed literals -- yet this
        /// still times out.
        #[kani::proof]
        fn vec_remove_from_single_element_socket_addr_tuple_times_out() {
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1);
            let mut inbox: Vec<(SocketAddr, Vec<u8>)> = Vec::new();
            inbox.push((addr, b"ping".to_vec()));

            let (from, payload) = inbox.remove(0);
            assert_eq!(payload, b"ping");
            assert_eq!(from, addr);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::udp_inbox_removal_cost::option_take_from_the_identical_socket_addr_tuple_passes".to_owned(),
            "gallery::udp_inbox_removal_cost::option_take_from_the_identical_socket_addr_tuple_passes".to_owned(),
            "amenable_kani".to_owned(),
            "The identical (SocketAddr, Vec<u8>) payload and comparisons, stored in an Option and drained via .take() instead of Vec::remove(0), verifies fast".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, OPTION_TAKE_FROM_THE_IDENTICAL_SOCKET_ADDR_TUPLE_PASSES_SRC, {
        /// `vec_remove_from_single_element_socket_addr_tuple_times_out`,
        /// with the one change `net_model::KaniUdpSocket` actually
        /// shipped: `Option<(SocketAddr, Vec<u8>)>` plus `.take()`
        /// instead of `Vec<(SocketAddr, Vec<u8>)>` plus `.remove(0)`.
        /// Same address, same payload, same two `assert_eq!` checks.
        #[kani::proof]
        fn option_take_from_the_identical_socket_addr_tuple_passes() {
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1);
            let mut inbox: Option<(SocketAddr, Vec<u8>)> = Some((addr, b"ping".to_vec()));

            let (from, payload) = inbox.take().unwrap();
            assert_eq!(payload, b"ping");
            assert_eq!(from, addr);
        }
    }
}
