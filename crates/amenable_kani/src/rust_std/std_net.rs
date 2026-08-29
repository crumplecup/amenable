//! `KaniWitness` impls for `std::net`.
//!
//! Every harness here proves against `net_model`'s TCP/UDP accommodation
//! model rather than calling `TcpListener`/`TcpStream`/`UdpSocket`
//! directly: constructing any real socket at all reaches libc's
//! `socket()` syscall, which Kani reports unsupported, confirmed by
//! `gallery::replace_recommendations::
//! socket_construction_reaches_an_unsupported_socket_syscall_boundary`
//! identically across all five claims below. If the real std/libc path
//! conforms to `net_model`'s laws, the modeled proof carries the
//! intended Rust-facing claim.

#[cfg(kani)]
use std::net::SocketAddr;
use std::net::{Incoming, Shutdown, TcpListener, TcpStream, UdpSocket};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Incoming<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_incoming_yields_an_already_queued_connection".to_owned(),
            VERIFY_INCOMING_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Incoming<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Incoming<'static>>",
        "kani",
        || <RustStdStandard<Incoming<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_INCOMING_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC, {
        /// A connection that's already queued in the backlog is yielded
        /// by `.incoming()` without needing a second thread to drive
        /// it. Proven against `net_model::KaniTcpListener`: real socket
        /// construction reaches an unsupported `socket()` syscall (see
        /// this module's doc comment).
        #[kani::proof]
        fn verify_incoming_yields_an_already_queued_connection() {
            let mut listener = crate::KaniTcpListener::minimal();
            let client = listener.connect(1);

            let server = listener.incoming_next();
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    server.peer_addr(),
                    client.local_addr()
                )),
                "incoming should yield the already-queued client's connection"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Shutdown> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shutdown_write_prevents_further_writes".to_owned(),
            VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Shutdown>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Shutdown>",
        "kani",
        || <RustStdStandard<Shutdown> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC, {
        /// `.shutdown(Shutdown::Write)` closes the write half, so a
        /// later write on that stream fails. Proven against
        /// `net_model::KaniTcpListener`: real socket construction
        /// reaches an unsupported `socket()` syscall (see this module's
        /// doc comment).
        #[kani::proof]
        fn verify_shutdown_write_prevents_further_writes() {
            let mut listener = crate::KaniTcpListener::minimal();
            let client = listener.connect(1);
            let _server = listener.accept();

            listener.client_shutdown_write(client);
            assert!(
                FallibleOperationReportsFailure::ensures(
                    listener
                        .client_write(client, b"more data".to_vec())
                        .is_err()
                ),
                "a write after shutdown(Write) should fail"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<TcpListener> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tcp_listener_accepts_a_connecting_stream".to_owned(),
            VERIFY_TCP_LISTENER_ACCEPTS_A_CONNECTING_STREAM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TcpListener>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TcpListener>",
        "kani",
        || <RustStdStandard<TcpListener> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TCP_LISTENER_ACCEPTS_A_CONNECTING_STREAM_SRC, {
        /// `.accept()` succeeds for a stream that connected to the
        /// listener's bound address. Proven against
        /// `net_model::KaniTcpListener`: real socket construction
        /// reaches an unsupported `socket()` syscall (see this module's
        /// doc comment).
        #[kani::proof]
        fn verify_tcp_listener_accepts_a_connecting_stream() {
            let mut listener = crate::KaniTcpListener::minimal();
            let addr = listener.local_addr();
            let _client = listener.connect(1);

            let (_server_side, peer_addr) = listener.accept();
            assert!(CollectedSequenceMatchesExpected::ensures((
                peer_addr.ip(),
                addr.ip()
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<TcpStream> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tcp_stream_delivers_written_bytes_to_the_accepted_peer".to_owned(),
            VERIFY_TCP_STREAM_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TcpStream>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TcpStream>",
        "kani",
        || <RustStdStandard<TcpStream> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TCP_STREAM_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC, {
        /// Bytes written on a connected `TcpStream` arrive, unaltered,
        /// on the accepted peer's side. Proven against
        /// `net_model::KaniTcpListener`: real socket construction
        /// reaches an unsupported `socket()` syscall (see this module's
        /// doc comment).
        #[kani::proof]
        fn verify_tcp_stream_delivers_written_bytes_to_the_accepted_peer() {
            let mut listener = crate::KaniTcpListener::minimal();
            let client = listener.connect(1);
            let (server, _peer_addr) = listener.accept();

            listener
                .client_write(client, b"hello, server".to_vec())
                .unwrap();
            let delivered = listener.server_read(server);
            assert!(CollectedSequenceMatchesExpected::ensures((
                delivered,
                b"hello, server".to_vec()
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<UdpSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_udp_socket_send_to_recv_from_round_trips_a_datagram".to_owned(),
            VERIFY_UDP_SOCKET_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<UdpSocket>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<UdpSocket>",
        "kani",
        || <RustStdStandard<UdpSocket> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UDP_SOCKET_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC, {
        /// `.send_to()` delivers a datagram to the target address, and
        /// `.recv_from()` reports both its bytes and the real sender
        /// address. Proven against `net_model::KaniUdpSocket`: real
        /// socket construction reaches an unsupported `socket()`
        /// syscall (see this module's doc comment).
        #[kani::proof]
        fn verify_udp_socket_send_to_recv_from_round_trips_a_datagram() {
            let mut socket_a = crate::KaniUdpSocket::bind(0);
            let mut socket_b = crate::KaniUdpSocket::bind(1);
            let addr_b = socket_b.local_addr();

            socket_b.send_to(&mut socket_a, b"ping".to_vec());

            let (payload, from) = socket_a.recv_from();
            assert!(CollectedSequenceMatchesExpected::ensures((payload, b"ping".to_vec())));
            assert!(RustStdStandard::<SocketAddr>::ensures((from, addr_b)));
        }
    }
}
