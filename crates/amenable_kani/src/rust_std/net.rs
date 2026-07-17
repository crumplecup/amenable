//! `KaniWitness` impls for `core::net`.

use std::net::{
    AddrParseError, IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl KaniWitness for RustStdStandard<Ipv4Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ipv4_addr_octets_round_trip",
            claim: VERIFY_IPV4_ADDR_OCTETS_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Ipv4Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ipv4Addr>",
        verifier: "kani",
        describe: || <RustStdStandard<Ipv4Addr> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_IPV4_ADDR_OCTETS_ROUND_TRIP_SRC, {
        /// `Ipv4Addr::octets` round-trips the four bytes passed to `new`.
        #[kani::proof]
        fn verify_ipv4_addr_octets_round_trip() {
            let a: u8 = kani::any();
            let b: u8 = kani::any();
            let c: u8 = kani::any();
            let d: u8 = kani::any();
            let addr = Ipv4Addr::new(a, b, c, d);
            assert_eq!(
                addr.octets(),
                [a, b, c, d],
                "Ipv4Addr::octets round-trips its constructor arguments"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Ipv6Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ipv6_addr_segments_round_trip",
            claim: VERIFY_IPV6_ADDR_SEGMENTS_ROUND_TRIP_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Ipv6Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ipv6Addr>",
        verifier: "kani",
        describe: || <RustStdStandard<Ipv6Addr> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_IPV6_ADDR_SEGMENTS_ROUND_TRIP_SRC, {
        /// `Ipv6Addr::segments` round-trips the eight 16-bit groups
        /// passed to `new`.
        #[kani::proof]
        fn verify_ipv6_addr_segments_round_trip() {
            let a: u16 = kani::any();
            let b: u16 = kani::any();
            let c: u16 = kani::any();
            let d: u16 = kani::any();
            let e: u16 = kani::any();
            let f: u16 = kani::any();
            let g: u16 = kani::any();
            let h: u16 = kani::any();
            let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);
            assert_eq!(
                addr.segments(),
                [a, b, c, d, e, f, g, h],
                "Ipv6Addr::segments round-trips its constructor arguments"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IpAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ip_addr_variant_matches_its_kind",
            claim: VERIFY_IP_ADDR_VARIANT_MATCHES_ITS_KIND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IpAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IpAddr>",
        verifier: "kani",
        describe: || <RustStdStandard<IpAddr> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_IP_ADDR_VARIANT_MATCHES_ITS_KIND_SRC, {
        /// `IpAddr::is_ipv4`/`is_ipv6` agree with the constructing variant,
        /// and each variant round-trips its wrapped address. Kani has no
        /// `Arbitrary` impl for `IpAddr`, so both variants are constructed
        /// explicitly rather than sampled symbolically, matching the
        /// `Ordering`/`Bound` harnesses' approach; the `V4` case still
        /// varies its address symbolically since `Ipv4Addr` is plain
        /// bytes, while `V6` uses a fixed representative address (its own
        /// byte-level behavior is covered by
        /// `verify_ipv6_addr_segments_round_trip`).
        #[kani::proof]
        fn verify_ip_addr_variant_matches_its_kind() {
            let a: u8 = kani::any();
            let b: u8 = kani::any();
            let c: u8 = kani::any();
            let d: u8 = kani::any();
            let v4 = Ipv4Addr::new(a, b, c, d);
            let ip = IpAddr::V4(v4);
            assert!(ip.is_ipv4(), "V4 reports is_ipv4");
            assert!(!ip.is_ipv6(), "V4 reports !is_ipv6");
            match ip {
                IpAddr::V4(inner) => assert_eq!(inner, v4, "V4 round-trips its address"),
                IpAddr::V6(_) => unreachable!("constructed as V4"),
            }

            let v6 = Ipv6Addr::LOCALHOST;
            let ip = IpAddr::V6(v6);
            assert!(ip.is_ipv6(), "V6 reports is_ipv6");
            assert!(!ip.is_ipv4(), "V6 reports !is_ipv4");
            match ip {
                IpAddr::V6(inner) => assert_eq!(inner, v6, "V6 round-trips its address"),
                IpAddr::V4(_) => unreachable!("constructed as V6"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<SocketAddrV4> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_socket_addr_v4_round_trips_ip_and_port",
            claim: VERIFY_SOCKET_ADDR_V4_ROUND_TRIPS_IP_AND_PORT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SocketAddrV4>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SocketAddrV4>",
        verifier: "kani",
        describe: || <RustStdStandard<SocketAddrV4> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SOCKET_ADDR_V4_ROUND_TRIPS_IP_AND_PORT_SRC, {
        /// `SocketAddrV4::new` round-trips both its address and port,
        /// with the address varied symbolically (unlike a fixed-loopback
        /// check, this also exercises `.ip()` itself, not just `.port()`).
        #[kani::proof]
        fn verify_socket_addr_v4_round_trips_ip_and_port() {
            let a: u8 = kani::any();
            let b: u8 = kani::any();
            let c: u8 = kani::any();
            let d: u8 = kani::any();
            let port: u16 = kani::any();
            let ip = Ipv4Addr::new(a, b, c, d);
            let addr = SocketAddrV4::new(ip, port);
            assert_eq!(*addr.ip(), ip, "SocketAddrV4::ip round-trips its address");
            assert_eq!(addr.port(), port, "SocketAddrV4::port round-trips its port");
        }
    }
}

impl KaniWitness for RustStdStandard<SocketAddrV6> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_socket_addr_v6_round_trips_all_fields",
            claim: VERIFY_SOCKET_ADDR_V6_ROUND_TRIPS_ALL_FIELDS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SocketAddrV6>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SocketAddrV6>",
        verifier: "kani",
        describe: || <RustStdStandard<SocketAddrV6> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SOCKET_ADDR_V6_ROUND_TRIPS_ALL_FIELDS_SRC, {
        /// `SocketAddrV6::new` round-trips port, flow info, and scope id.
        /// The address is a fixed representative value: its own
        /// byte-level behavior is covered by
        /// `verify_ipv6_addr_segments_round_trip`, so this harness
        /// focuses on the three fields `SocketAddrV6` adds.
        #[kani::proof]
        fn verify_socket_addr_v6_round_trips_all_fields() {
            let port: u16 = kani::any();
            let flowinfo: u32 = kani::any();
            let scope_id: u32 = kani::any();
            let ip = Ipv6Addr::LOCALHOST;
            let addr = SocketAddrV6::new(ip, port, flowinfo, scope_id);
            assert_eq!(*addr.ip(), ip, "SocketAddrV6::ip round-trips its address");
            assert_eq!(addr.port(), port, "SocketAddrV6::port round-trips its port");
            assert_eq!(
                addr.flowinfo(),
                flowinfo,
                "SocketAddrV6::flowinfo round-trips"
            );
            assert_eq!(
                addr.scope_id(),
                scope_id,
                "SocketAddrV6::scope_id round-trips"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SocketAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_socket_addr_variant_matches_its_kind",
            claim: VERIFY_SOCKET_ADDR_VARIANT_MATCHES_ITS_KIND_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SocketAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SocketAddr>",
        verifier: "kani",
        describe: || <RustStdStandard<SocketAddr> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SOCKET_ADDR_VARIANT_MATCHES_ITS_KIND_SRC, {
        /// `SocketAddr::is_ipv4`/`is_ipv6` agree with the constructing
        /// variant, and `.port()` dispatches through either variant to
        /// the same symbolic port.
        #[kani::proof]
        fn verify_socket_addr_variant_matches_its_kind() {
            let port: u16 = kani::any();

            let v4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
            let addr = SocketAddr::V4(v4);
            assert!(addr.is_ipv4(), "V4 reports is_ipv4");
            assert!(!addr.is_ipv6(), "V4 reports !is_ipv6");
            assert_eq!(addr.port(), port, "SocketAddr::port dispatches through V4");

            let v6 = SocketAddrV6::new(Ipv6Addr::LOCALHOST, port, 0, 0);
            let addr = SocketAddr::V6(v6);
            assert!(addr.is_ipv6(), "V6 reports is_ipv6");
            assert!(!addr.is_ipv4(), "V6 reports !is_ipv4");
            assert_eq!(addr.port(), port, "SocketAddr::port dispatches through V6");
        }
    }
}

impl_kani_witness_trusted!(AddrParseError);
