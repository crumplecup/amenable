//! `std::os::windows::*` `VerusWitness` witnesses, behind a single
//! `#[cfg(windows)]` gate. `amenable_verus::rust_std::os_windows_carrier`
//! (the `claim` these `include_str!` in) has only ever been checked by the
//! `verus-windows` GitHub Actions workflow, never on a Linux dev host.

// Re-exported for the `#[cfg(windows)] mod windows_witnesses` below, which
// pulls them in via `use super::{..}`; gated to match, so a non-Windows
// build (where that module compiles out) sees no unused imports.
#[cfg(windows)]
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
#[cfg(windows)]
use crate::RustStdStandard;
#[cfg(windows)]
use amenable_core::Evidence;

/// `std::os::windows::*` witnesses, one `#[cfg(windows)]` gate on this
/// `mod` instead of scattered per-item ones -- mirroring `rust_std::
/// os_windows`'s own gating. Unlike every other `VerusWitness` impl in
/// this file, `amenable_verus::rust_std::os_windows_carrier` (the
/// `claim` these `include_str!` in) has never been checked by `verus`
/// on this crate's primary development host (Linux) — only the
/// `verus-windows` GitHub Actions workflow (`workflow_dispatch`,
/// `windows-latest`) can. See that carrier's own module doc comment for
/// the full reasoning. Nothing here is `pub`, so the whole section
/// collapses into one private nested module with no re-export needed:
/// trait impls are visible crate-wide regardless of which module
/// defines them.
///
/// `EncodeWide`/`BorrowedHandle`/`BorrowedSocket`/`HandleOrInvalid`/
/// `OwnedHandle`/`OwnedSocket` need their own real `use` here -- a real,
/// pre-existing gap this module had before this consolidation (`cannot
/// find type` for all six, confirmed via a genuine `cross check --target
/// x86_64-pc-windows-gnu` run, not previously caught since the
/// `verus-windows` workflow is `workflow_dispatch`-only).
#[cfg(windows)]
mod windows_witnesses {
    use super::{Evidence, RustStdStandard, VerusCheckedProof, VerusWitness, bridge_verus_witness};
    use std::os::windows::ffi::EncodeWide;
    use std::os::windows::io::{
        BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket,
    };

    const VERIFY_ENCODE_WIDE_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<EncodeWide<'static>> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<EncodeWide<'_> as Iterator>::next".to_owned(),
                VERIFY_ENCODE_WIDE_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<EncodeWide<'static>>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
            "verus",
            || { <RustStdStandard<EncodeWide<'static>> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<EncodeWide<'static>>,
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "encode_wide_next_matches"
    );

    const VERIFY_BORROWED_HANDLE_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<BorrowedHandle<'static>> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<BorrowedHandle<'_> as AsRawHandle>::as_raw_handle".to_owned(),
                VERIFY_BORROWED_HANDLE_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<BorrowedHandle<'static>>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
            "verus",
            || { <RustStdStandard<BorrowedHandle<'static>> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<BorrowedHandle<'static>>,
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "as_raw_handle_addr_matches"
    );

    const VERIFY_BORROWED_SOCKET_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<BorrowedSocket<'static>> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<BorrowedSocket<'_> as AsRawSocket>::as_raw_socket".to_owned(),
                VERIFY_BORROWED_SOCKET_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<BorrowedSocket<'static>>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
            "verus",
            || { <RustStdStandard<BorrowedSocket<'static>> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<BorrowedSocket<'static>>,
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "as_raw_socket_matches"
    );

    const VERIFY_HANDLE_OR_INVALID_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<HandleOrInvalid> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<OwnedHandle as TryFrom<HandleOrInvalid>>::try_from".to_owned(),
                VERIFY_HANDLE_OR_INVALID_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<HandleOrInvalid>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
            "verus",
            || { <RustStdStandard<HandleOrInvalid> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<HandleOrInvalid>,
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "handle_or_invalid_try_from_matches"
    );

    const VERIFY_OWNED_HANDLE_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<OwnedHandle> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<OwnedHandle as AsRawHandle>::as_raw_handle".to_owned(),
                VERIFY_OWNED_HANDLE_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<OwnedHandle>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
            "verus",
            || { <RustStdStandard<OwnedHandle> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<OwnedHandle>,
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "owned_as_raw_handle_addr_matches"
    );

    const VERIFY_OWNED_SOCKET_AXIOM_SRC: &str =
        include_str!("../../../amenable_verus/src/rust_std/misc/os_windows_carrier.rs");

    impl VerusWitness for RustStdStandard<OwnedSocket> {
        type SupportingEvidence = Self;
        type ProofArtifact = VerusCheckedProof;

        #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
        fn proof() -> Self::ProofArtifact {
            VerusCheckedProof::new(
                "<OwnedSocket as AsRawSocket>::as_raw_socket".to_owned(),
                VERIFY_OWNED_SOCKET_AXIOM_SRC.to_owned(),
                <Self::SupportingEvidence as Evidence>::basis().audit(),
            )
        }
    }

    bridge_verus_witness!(RustStdStandard<OwnedSocket>);

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
            "verus",
            || { <RustStdStandard<OwnedSocket> as VerusWitness>::proof().to_string() },
        )
    }

    amenable_derive::verus_ensures_predicate!(
        RustStdStandard<OwnedSocket>,
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "owned_as_raw_socket_matches"
    );
}

/// `windows_witnesses`'s own 6 `verus_ensures_predicate!` calls generate
/// real `ContractRecord` registrations naming each real `open spec fn` --
/// but the whole module they live in is `#[cfg(windows)]`, so on this
/// project's own Linux dev/CI host (where `amenable dump-registry` always
/// runs) those registrations never fire, and cordial's own
/// `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001` check -- which reads that
/// registry dump, not the source directly -- sees 6 real, named `ensures`
/// clauses in `amenable_verus::rust_std::os_windows_carrier` as if they
/// were unnamed raw equations. `ContractRecord::new` takes only string/fn
/// -pointer data (`evidence: &'static str, verifier: &'static str, kind:
/// &'static str, fragment: fn() -> &'static str`) -- no dependency on the
/// real Windows types at all -- so the fix is the same one
/// `amenable_kani::os_windows_model`'s own doc comment already
/// established for `ProofRecord` on the Kani side: hand-write the
/// registration, with `fragment` a verbatim copy of the real spec fn's
/// own signature and body (`os_windows_carrier.rs`, not paraphrased), so
/// the registry is honest about what's already true and documented in
/// the real source, on the one platform this tooling actually runs on.
/// `#[cfg(not(windows))]`: the real `#[cfg(windows)]` registrations above
/// already cover a genuine Windows build; this is purely the Linux-side
/// fallback, never both at once.
#[cfg(not(windows))]
mod windows_contract_bounds_linux_fallback {
    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
            "verus",
            "ensures",
            || "pub open spec fn encode_wide_next_matches(before: Seq<u16>, after: Seq<u16>, result: Option<u16>) -> bool { (before.len() == 0 ==> result is None && after == before) && (before.len() > 0 ==> result == Some(before[0]) && after == before.subrange(1, before.len() as int)) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
            "verus",
            "ensures",
            || "pub open spec fn as_raw_handle_addr_matches(result: RawHandle, h: BorrowedHandle) -> bool { result.addr() == borrowed_handle_addr_spec(h) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
            "verus",
            "ensures",
            || "pub open spec fn as_raw_socket_matches(result: RawSocket, s: BorrowedSocket) -> bool { result == borrowed_socket_value_spec(s) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
            "verus",
            "ensures",
            || "pub open spec fn handle_or_invalid_try_from_matches(handle_or_invalid: HandleOrInvalid, result: Result<OwnedHandle, <OwnedHandle as core::convert::TryFrom<HandleOrInvalid>>::Error>) -> bool { (handle_or_invalid_addr_spec(handle_or_invalid) == usize::MAX <==> result is Err) && (result is Ok ==> owned_handle_addr_spec(result->Ok_0) == handle_or_invalid_addr_spec(handle_or_invalid)) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
            "verus",
            "ensures",
            || "pub open spec fn owned_as_raw_handle_addr_matches(result: RawHandle, h: OwnedHandle) -> bool { result.addr() == owned_handle_addr_spec(h) }",
        )
    }

    ::inventory::submit! {
        ::amenable_core::ContractRecord::new(
            "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
            "verus",
            "ensures",
            || "pub open spec fn owned_as_raw_socket_matches(result: RawSocket, s: OwnedSocket) -> bool { result == owned_socket_value_spec(s) }",
        )
    }
}
