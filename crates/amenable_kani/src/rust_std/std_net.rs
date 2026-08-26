//! `KaniWitness` impls for `std::net`.
//!
//! Every harness runs on the loopback interface, single-threaded: a
//! connecting `TcpStream::connect()` (or a UDP `send_to`) queues in the
//! OS itself before the harness ever calls `accept()`/`recv_from()`, so
//! one thread can drive both sides sequentially without spawning a
//! second one.

use std::net::{Incoming, Shutdown, TcpListener, TcpStream, UdpSocket};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
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
        /// A connection that's already queued in the OS backlog is
        /// yielded by `.incoming()` without needing a second thread to
        /// drive it.
        #[kani::proof]
        fn verify_incoming_yields_an_already_queued_connection() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let _client = TcpStream::connect(addr).unwrap();

            let mut incoming = listener.incoming();
            assert!(incoming.next().unwrap().is_ok());
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
        /// later write on that stream fails.
        #[kani::proof]
        fn verify_shutdown_write_prevents_further_writes() {
            use std::io::Write;

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let mut client = TcpStream::connect(addr).unwrap();
            let _server_side = listener.accept().unwrap();

            client.shutdown(Shutdown::Write).unwrap();
            assert!(
                client.write(b"more data").is_err(),
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
        /// listener's bound address.
        #[kani::proof]
        fn verify_tcp_listener_accepts_a_connecting_stream() {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let _client = TcpStream::connect(addr).unwrap();

            let (_server_side, peer_addr) = listener.accept().unwrap();
            assert_eq!(peer_addr.ip(), addr.ip());
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
        /// on the accepted peer's side.
        #[kani::proof]
        fn verify_tcp_stream_delivers_written_bytes_to_the_accepted_peer() {
            use std::io::{Read, Write};

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let addr = listener.local_addr().unwrap();
            let mut client = TcpStream::connect(addr).unwrap();
            let (mut server_side, _peer_addr) = listener.accept().unwrap();

            client.write_all(b"hello, server").unwrap();
            let mut buf = [0u8; 32];
            let n = server_side.read(&mut buf).unwrap();
            assert_eq!(&buf[..n], b"hello, server");
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
        /// address.
        #[kani::proof]
        fn verify_udp_socket_send_to_recv_from_round_trips_a_datagram() {
            let socket_a = UdpSocket::bind("127.0.0.1:0").unwrap();
            let socket_b = UdpSocket::bind("127.0.0.1:0").unwrap();
            let addr_a = socket_a.local_addr().unwrap();

            socket_b.send_to(b"ping", addr_a).unwrap();

            let mut buf = [0u8; 32];
            let (n, from) = socket_a.recv_from(&mut buf).unwrap();
            assert_eq!(&buf[..n], b"ping");
            assert_eq!(from, socket_b.local_addr().unwrap());
        }
    }
}
