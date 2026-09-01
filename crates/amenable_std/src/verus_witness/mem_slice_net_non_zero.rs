//! `mem::Discriminant`, the basic slice iterators, every
//! `std::num::NonZero<T>` instantiation (via `impl_non_zero_verus_witness!`),
//! and the `std::net` address types.

use super::iter_adapters_c_and_fmt::VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::mem::Discriminant<Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_discriminant_model_identifies_variant_not_payload".to_owned(),
            VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::mem::Discriminant<Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::mem::Discriminant<Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::mem::Discriminant<Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

/// `RustStdStandard<NonZero<T>>`'s Verus proof states its claim as two
/// separate real `ensures` clauses (`non_zero_new_accepts_nonzero`,
/// `non_zero_new_rejects_zero`) — an iff split into its two
/// implications, not one expression, since Verus has no single iff
/// operator. `Ensures<VerusVerifier>::Bound = &'static [&'static str]`
/// holds both, uniformly, as first-class elements of the trait's own
/// value -- the original motivating case for that shape (see Design E
/// in `docs/VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`'s companion
/// contract-type work): no more picking one direction as "canonical"
/// and smuggling the other in through a bespoke supplementary
/// `ContractRecord`.
macro_rules! impl_non_zero_verus_witness {
    ($($ty:ty => $harness:literal),* $(,)?) => {
        $(
            impl VerusWitness for RustStdStandard<std::num::NonZero<$ty>> {
                type SupportingEvidence = Self;
                type ProofArtifact = VerusCheckedProof;

                fn proof() -> Self::ProofArtifact {
                    VerusCheckedProof::new(
                        $harness.to_owned(),
                        include_str!("../../../amenable_verus/src/rust_std/num/non_zero_carrier.rs").to_owned(),
                        <Self::SupportingEvidence as Evidence>::basis().audit(),
                    )
                }
            }

            bridge_verus_witness!(RustStdStandard<std::num::NonZero<$ty>>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                    "verus",
                    || <RustStdStandard<std::num::NonZero<$ty>> as VerusWitness>::proof().to_string(),
                )
            }

            amenable_derive::verus_ensures_witness!(
                RustStdStandard<std::num::NonZero<$ty>>,
                concat!("amenable_std::rust_std::RustStdStandard<std::num::NonZero<", stringify!($ty), ">>"),
                $harness
            );
        )*
    };
}

const VERIFY_ITER_MODEL_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/slice_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_iter_model_yields_shared_references_in_order".to_owned(),
            VERIFY_ITER_MODEL_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::slice::Iter<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ITER_MUT_MODEL_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/slice_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_iter_mut_model_yields_mutable_references_that_write_through".to_owned(),
            VERIFY_ITER_MUT_MODEL_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::slice::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_IPV4_ADDR_MODEL_OCTETS_ROUND_TRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Ipv4Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ipv4_addr_model_octets_round_trip".to_owned(),
            VERIFY_IPV4_ADDR_MODEL_OCTETS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Ipv4Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Ipv4Addr>",
        "verus",
        || {
            <RustStdStandard<std::net::Ipv4Addr> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IPV6_ADDR_MODEL_SEGMENTS_ROUND_TRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Ipv6Addr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ipv6_addr_model_segments_round_trip".to_owned(),
            VERIFY_IPV6_ADDR_MODEL_SEGMENTS_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Ipv6Addr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Ipv6Addr>",
        "verus",
        || {
            <RustStdStandard<std::net::Ipv6Addr> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_IP_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::IpAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ip_addr_model_variant_matches_its_kind".to_owned(),
            VERIFY_IP_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::IpAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::IpAddr>",
        "verus",
        || {
            <RustStdStandard<std::net::IpAddr> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: `IpAddr`'s `V4` variant always round-trips its
// wrapped octets exactly.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::net::IpAddr>,
    "amenable_std::rust_std::RustStdStandard<std::net::IpAddr>",
    "ip_addr_model_v4_octets_match_input"
);

const VERIFY_SOCKET_ADDR_V4_MODEL_ROUND_TRIPS_IP_AND_PORT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddrV4> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_socket_addr_v4_model_round_trips_ip_and_port".to_owned(),
            VERIFY_SOCKET_ADDR_V4_MODEL_ROUND_TRIPS_IP_AND_PORT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddrV4>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::SocketAddrV4>",
        "verus",
        || {
            <RustStdStandard<std::net::SocketAddrV4> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SOCKET_ADDR_V6_MODEL_ROUND_TRIPS_ALL_FIELDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddrV6> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_socket_addr_v6_model_round_trips_all_fields".to_owned(),
            VERIFY_SOCKET_ADDR_V6_MODEL_ROUND_TRIPS_ALL_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddrV6>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::SocketAddrV6>",
        "verus",
        || {
            <RustStdStandard<std::net::SocketAddrV6> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SOCKET_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::SocketAddr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_socket_addr_model_variant_matches_its_kind".to_owned(),
            VERIFY_SOCKET_ADDR_MODEL_VARIANT_MATCHES_ITS_KIND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::SocketAddr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::SocketAddr>",
        "verus",
        || {
            <RustStdStandard<std::net::SocketAddr> as VerusWitness>::proof().to_string()
        },
    )
}

impl_non_zero_verus_witness!(
    i8 => "verify_non_zero_i8_model_round_trips_iff_nonzero",
    i16 => "verify_non_zero_i16_model_round_trips_iff_nonzero",
    i32 => "verify_non_zero_i32_model_round_trips_iff_nonzero",
    i64 => "verify_non_zero_i64_model_round_trips_iff_nonzero",
    i128 => "verify_non_zero_i128_model_round_trips_iff_nonzero",
    isize => "verify_non_zero_isize_model_round_trips_iff_nonzero",
    u8 => "verify_non_zero_u8_model_round_trips_iff_nonzero",
    u16 => "verify_non_zero_u16_model_round_trips_iff_nonzero",
    u32 => "verify_non_zero_u32_model_round_trips_iff_nonzero",
    u64 => "verify_non_zero_u64_model_round_trips_iff_nonzero",
    u128 => "verify_non_zero_u128_model_round_trips_iff_nonzero",
    usize => "verify_non_zero_usize_model_round_trips_iff_nonzero",
);

pub(super) const VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/ordered_pair_iter_mut_carrier.rs");
