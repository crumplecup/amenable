//! The remaining `std::sync` synchronization primitives, the remaining
//! `std::net` types, and `std::task`'s polling types.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::thread_env_mpsc::VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC;
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::sync::Once> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_runs_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_MODEL_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Once>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
        "verus",
        || { <RustStdStandard<std::sync::Once> as VerusWitness>::proof().to_string() },
    )
}

// The shared "exactly once" invocation-count postcondition `amenable_
// std::verus_witness` registers for `Once`/`Waker`.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::sync::Once>,
    "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
    "invoked_exactly_once"
);

const VERIFY_ONCE_STATE_MODEL_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::OnceState> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_state_model_reports_not_poisoned_on_a_clean_run".to_owned(),
            VERIFY_ONCE_STATE_MODEL_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::OnceState>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::OnceState>",
        "verus",
        || {
            <RustStdStandard<std::sync::OnceState> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ONCE_LOCK_MODEL_INITIALIZES_EXACTLY_ONCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_once_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::OnceLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_lock_model_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_LOCK_MODEL_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::OnceLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::OnceLock<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::OnceLock<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_barrier_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Barrier> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_barrier_model_of_one_is_its_own_leader".to_owned(),
            VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Barrier>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Barrier>",
        "verus",
        || {
            <RustStdStandard<std::sync::Barrier> as VerusWitness>::proof().to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::sync::BarrierWaitResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_barrier_model_of_one_is_its_own_leader".to_owned(),
            VERIFY_BARRIER_MODEL_OF_ONE_IS_ITS_OWN_LEADER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::BarrierWaitResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::BarrierWaitResult>",
        "verus",
        || {
            <RustStdStandard<std::sync::BarrierWaitResult> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_WAIT_TIMEOUT_RESULT_MODEL_REPORTS_TIMED_OUT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/sync_wait_timeout_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::WaitTimeoutResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_wait_timeout_result_model_reports_timed_out".to_owned(),
            VERIFY_WAIT_TIMEOUT_RESULT_MODEL_REPORTS_TIMED_OUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::WaitTimeoutResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::WaitTimeoutResult>",
        "verus",
        || {
            <RustStdStandard<std::sync::WaitTimeoutResult> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_INCOMING_MODEL_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Incoming<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_incoming_model_yields_an_already_queued_connection".to_owned(),
            VERIFY_INCOMING_MODEL_YIELDS_AN_ALREADY_QUEUED_CONNECTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Incoming<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Incoming<'static>>",
        "verus",
        || {
            <RustStdStandard<std::net::Incoming<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SHUTDOWN_MODEL_WRITE_PREVENTS_FURTHER_WRITES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::Shutdown> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_shutdown_model_write_prevents_further_writes".to_owned(),
            VERIFY_SHUTDOWN_MODEL_WRITE_PREVENTS_FURTHER_WRITES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::Shutdown>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::Shutdown>",
        "verus",
        || {
            <RustStdStandard<std::net::Shutdown> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TCP_LISTENER_MODEL_ACCEPTS_A_CONNECTING_STREAM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::TcpListener> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tcp_listener_model_accepts_a_connecting_stream".to_owned(),
            VERIFY_TCP_LISTENER_MODEL_ACCEPTS_A_CONNECTING_STREAM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::TcpListener>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::TcpListener>",
        "verus",
        || {
            <RustStdStandard<std::net::TcpListener> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TCP_STREAM_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::TcpStream> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tcp_stream_model_delivers_written_bytes_to_the_accepted_peer".to_owned(),
            VERIFY_TCP_STREAM_MODEL_DELIVERS_WRITTEN_BYTES_TO_THE_ACCEPTED_PEER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::TcpStream>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::TcpStream>",
        "verus",
        || {
            <RustStdStandard<std::net::TcpStream> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_UDP_SOCKET_MODEL_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/process_and_net/std_net_carrier.rs");

impl VerusWitness for RustStdStandard<std::net::UdpSocket> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_udp_socket_model_send_to_recv_from_round_trips_a_datagram".to_owned(),
            VERIFY_UDP_SOCKET_MODEL_SEND_TO_RECV_FROM_ROUND_TRIPS_A_DATAGRAM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::net::UdpSocket>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::net::UdpSocket>",
        "verus",
        || {
            <RustStdStandard<std::net::UdpSocket> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CONTEXT_MODEL_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Context<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_context_model_from_waker_exposes_the_same_waker".to_owned(),
            VERIFY_CONTEXT_MODEL_FROM_WAKER_EXPOSES_THE_SAME_WAKER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Context<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::task::Context<'static>>",
        "verus",
        || {
            <RustStdStandard<std::task::Context<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_POLL_MODEL_READY_AND_PENDING_ARE_DISJOINT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Poll<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_poll_model_ready_and_pending_are_disjoint".to_owned(),
            VERIFY_POLL_MODEL_READY_AND_PENDING_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Poll<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::task::Poll<i32>>",
        "verus",
        || {
            <RustStdStandard<std::task::Poll<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_WAKER_MODEL_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/task_and_thread/task_carrier.rs");

impl VerusWitness for RustStdStandard<std::task::Waker> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_waker_model_wake_by_ref_invokes_the_wake_impl".to_owned(),
            VERIFY_WAKER_MODEL_WAKE_BY_REF_INVOKES_THE_WAKE_IMPL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::task::Waker>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::task::Waker>",
        "verus",
        || {
            <RustStdStandard<std::task::Waker> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::task::Waker>,
    "amenable_std::rust_std::RustStdStandard<std::task::Waker>",
    "invoked_exactly_once"
);

pub(super) const VERIFY_ASSERT_UNWIND_SAFE_MODEL_DEREFS_TRANSPARENTLY_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/panic_carrier.rs");
