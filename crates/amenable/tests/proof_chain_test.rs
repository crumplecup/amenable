#[path = "support_chain.rs"]
mod support;

use amenable::ChainErrorKind;

fn assert_root_has_kani_and_creusot(report: &amenable::ProofChainReport, expected_suffix: &str) {
    let root = report.root();
    assert!(root.evidence().ends_with(expected_suffix));
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
}

fn proof_description<'a>(
    report: &'a amenable::ProofChainReport,
    verifier: &str,
) -> Option<&'a str> {
    report
        .root()
        .proofs()
        .iter()
        .find(|(registered, _)| *registered == verifier)
        .map(|(_, description)| description.as_str())
}

// These three verifier-completeness assertions only hold when both
// `amenable_creusot`'s registrations (the `creusot` feature; nightly-only,
// see justfile's `test-creusot` — `amenable_creusot` depends on
// `creusot-std`, which can't compile on stable, so it's not in
// `amenable`'s default features) and `amenable_verus`'s registrations
// (the `verus` feature) are linked in together — no justfile recipe
// currently runs that combination (`test-creusot`/`test-verus` each
// enable only one), so these three are ignored unless a caller passes
// both features explicitly, e.g. `cargo test -p amenable --features
// creusot,verus`.
#[test]
#[cfg_attr(not(all(feature = "creusot", feature = "verus")), ignore)]
fn bool_proof_chain_is_a_single_root_node_with_all_three_verifiers() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<bool>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<bool>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 3);
    assert_eq!(report.verifiers().len(), 3);

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    assert!(verifiers.contains(&"verus"));
    Ok(())
}

#[test]
fn char_proof_chain_carries_the_checked_harness_name_per_verifier() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<char>"))?;

    let root = report.root();
    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for char"))?;

    assert!(kani_description.contains("verify_char_unicode_scalar"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn array_i32_3_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<[i32; 3]>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<[i32; 3]>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for [i32; 3]"))?;
    assert!(kani_description.contains("verify_array_indexing_and_length"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for [i32; 3]"))?;
    assert!(creusot_description.contains("verify_array_indexing_and_length"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn atomic_bool_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<AtomicBool>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<AtomicBool>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for AtomicBool"))?;
    assert!(kani_description.contains("verify_atomic_bool"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for AtomicBool"))?;
    assert!(creusot_description.contains("verify_atomic_bool_load_store"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn atomic_ptr_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<AtomicPtr<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<AtomicPtr<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for AtomicPtr<i32>"))?;
    assert!(kani_description.contains("verify_atomic_ptr_load_store_swap_and_compare_exchange"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for AtomicPtr<i32>"))?;
    assert!(creusot_description.contains("verify_atomic_ptr_load_store"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn atomic_ordering_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Ordering>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Ordering>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for atomic Ordering"))?;
    assert!(kani_description.contains("verify_relaxed_ordering_still_makes_a_store_observable"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for atomic Ordering"))?;
    assert!(creusot_description.contains("verify_relaxed_ordering_still_makes_a_store_observable"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn system_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<System>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<System>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for System"))?;
    assert!(kani_description.contains("verify_system_allocates_and_deallocates_a_layout"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for System"))?;
    assert!(creusot_description.contains("verify_system_allocates_and_deallocates_a_layout"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn backtrace_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Backtrace>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Backtrace>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Backtrace"))?;
    assert!(kani_description.contains("verify_backtrace_force_capture_always_actually_captures"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Backtrace"))?;
    assert!(
        creusot_description.contains("verify_backtrace_force_capture_always_actually_captures")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn backtrace_status_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<BacktraceStatus>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<BacktraceStatus>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BacktraceStatus"))?;
    assert!(
        kani_description.contains("verify_backtrace_status_reports_captured_after_force_capture")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BacktraceStatus"))?;
    assert!(
        creusot_description
            .contains("verify_backtrace_status_reports_captured_after_force_capture")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn seek_from_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SeekFrom>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<SeekFrom>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SeekFrom"))?;
    assert!(kani_description.contains("verify_seek_from_round_trips_each_variants_offset"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for SeekFrom"))?;
    assert!(creusot_description.contains("verify_seek_from_round_trips_each_variants_offset"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn shutdown_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Shutdown>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Shutdown>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Shutdown"))?;
    assert!(kani_description.contains("verify_shutdown_write_prevents_further_writes"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Shutdown"))?;
    assert!(creusot_description.contains("verify_shutdown_write_prevents_further_writes"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn incoming_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Incoming<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Incoming<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_incoming_yields_an_already_queued_connection"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/net/struct.Incoming.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn tcp_listener_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<TcpListener>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<TcpListener>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_tcp_listener_accepts_a_connecting_stream"))
    );
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/net/struct.TcpListener.html")
        )
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn tcp_stream_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<TcpStream>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<TcpStream>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_tcp_stream_delivers_written_bytes_to_the_accepted_peer")
    }));
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/net/struct.TcpStream.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn udp_socket_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<UdpSocket>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<UdpSocket>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_udp_socket_send_to_recv_from_round_trips_a_datagram")
        )
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/net/struct.UdpSocket.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn child_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Child>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Child>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_child_has_a_process_id_and_can_be_waited_on"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/process/struct.Child.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn child_stderr_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChildStderr>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ChildStderr>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_child_stderr_captures_what_the_child_wrote_to_stderr")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/process/struct.ChildStderr.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn child_stdin_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChildStdin>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ChildStdin>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_child_stdin_is_readable_by_the_child_process"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/process/struct.ChildStdin.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn child_stdout_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChildStdout>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ChildStdout>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_child_stdout_captures_what_the_child_wrote_to_stdout")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/process/struct.ChildStdout.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn command_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Command>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Command>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_command_env_override_is_visible_to_the_spawned_process")
    }));
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/process/struct.Command.html")
        )
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn command_args_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<CommandArgs<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<CommandArgs<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_command_args_reports_the_configured_arguments"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/process/struct.CommandArgs.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn command_envs_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<CommandEnvs<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<CommandEnvs<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_command_envs_reports_the_configured_overrides"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/process/struct.CommandEnvs.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn exit_code_proof_chain_reports_the_trusted_kani_and_creusot_provenance() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ExitCode>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ExitCode>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/process/struct.ExitCode.html")
        )
    );
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/process/struct.ExitCode.html")
        )
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn exit_status_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ExitStatus>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ExitStatus>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_exit_status_reports_a_nonzero_exit_code"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/process/struct.ExitStatus.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn output_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Output>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Output>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_output_captures_stdout_and_the_exit_status"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/process/struct.Output.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stdio_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Stdio>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Stdio>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_stdio_null_discards_the_childs_output_handle"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/process/struct.Stdio.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ancestors_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Ancestors<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Ancestors<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_ancestors_yields_self_then_each_parent_up_to_root"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/struct.Ancestors.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn component_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Component<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Component<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_component_distinguishes_root_from_normal_segments"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/enum.Component.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn components_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Components<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Components<'static>>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_components_yields_root_then_named_segments_in_order")
        )
    );
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/path/struct.Components.html")
        )
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn path_display_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::path::Display<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::path::Display<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_display_renders_a_valid_utf8_path_verbatim"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/struct.Display.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn path_iter_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::path::Iter<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::path::Iter<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_iter_yields_the_named_segments"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/struct.Iter.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn path_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Path>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Path>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_path_derives_extension_file_name_and_parent"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/struct.Path.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn path_buf_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PathBuf>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<PathBuf>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_path_buf_push_pop_and_join_build_the_expected_path")
        )
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/struct.PathBuf.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn prefix_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Prefix<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Prefix<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_prefix_disk_identifies_the_drive_letter"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/path/enum.Prefix.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn prefix_component_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()>
{
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PrefixComponent<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<PrefixComponent<'static>>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_prefix_component_pairs_raw_text_with_parsed_prefix")
        )
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/path/struct.PrefixComponent.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn strip_prefix_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<StripPrefixError>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<StripPrefixError>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_strip_prefix_error_reports_a_non_matching_prefix"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/path/struct.StripPrefixError.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn instant_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Instant>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Instant>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_instant_is_monotonically_nondecreasing"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/time/struct.Instant.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn system_time_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SystemTime>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<SystemTime>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_system_time_duration_since_computes_the_elapsed_span")
    }));
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/time/struct.SystemTime.html")
        )
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn system_time_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SystemTimeError>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<SystemTimeError>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_system_time_error_recovers_how_far_backward_it_went")
        )
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/time/struct.SystemTimeError.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn panic_and_sync_lock_proof_chains_report_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep each subject literal on its own line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PanicHookInfo<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<PanicHookInfo<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_panic_hook_info_reports_the_panics_own_message"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/panic/struct.PanicHookInfo.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Barrier>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Barrier>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_barrier_of_one_is_its_own_leader"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/sync/struct.Barrier.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<BarrierWaitResult>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<BarrierWaitResult>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_barrier_wait_result_reports_the_sole_participant_as_leader")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/sync/struct.BarrierWaitResult.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<LazyLock<i32, fn() -> i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<LazyLock<i32, fn() -> i32>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_lazy_lock_caches_its_initializer_result"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/sync/struct.LazyLock.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::sync::Once>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::sync::Once>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_once_runs_its_closure_exactly_once"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/sync/struct.Once.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<OnceLock<i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<OnceLock<i32>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_once_lock_initializes_exactly_once"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/sync/struct.OnceLock.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<OnceState>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<OnceState>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_once_state_reports_not_poisoned_on_a_clean_run"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/sync/struct.OnceState.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<WaitTimeoutResult>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<WaitTimeoutResult>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_wait_timeout_result_reports_timed_out"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/sync/struct.WaitTimeoutResult.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn sync_mpsc_proof_chains_report_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep each subject literal on its own line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::sync::mpsc::Iter<'static, i32>>"))?;
    assert_root_has_kani_and_creusot(
        &report,
        "RustStdStandard<std::sync::mpsc::Iter<'static, i32>>",
    );
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_iter_yields_sent_values_then_stops"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/sync/mpsc/struct.Iter.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::sync::mpsc::Receiver<i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::sync::mpsc::Receiver<i32>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_receiver_fails_once_every_sender_is_dropped"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::sync::mpsc::Sender<i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::sync::mpsc::Sender<i32>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_sender_delivers_to_the_paired_receiver"))
    );
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html")
        )
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SyncSender<i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<SyncSender<i32>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_sync_sender_delivers_to_the_paired_receiver"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>"))?;
    assert_root_has_kani_and_creusot(
        &report,
        "RustStdStandard<std::sync::mpsc::TryIter<'static, i32>>",
    );
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_try_iter_does_not_block_on_an_empty_open_channel"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/sync/mpsc/struct.TryIter.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn thread_proof_chains_report_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep each subject literal on its own line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<AccessError>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<AccessError>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/thread/struct.AccessError.html")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/thread/struct.AccessError.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Builder>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Builder>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/thread/struct.Builder.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/thread/struct.Builder.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<JoinHandle<i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<JoinHandle<i32>>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/thread/struct.JoinHandle.html")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/thread/struct.JoinHandle.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<LocalKey<std::cell::Cell<i32>>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<LocalKey<std::cell::Cell<i32>>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_local_key_with_reads_the_initialized_value"))
    );
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/thread/struct.LocalKey.html")
        )
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Scope<'static, 'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Scope<'static, 'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/thread/struct.Scope.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/thread/struct.Scope.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ScopedJoinHandle<'static, i32>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ScopedJoinHandle<'static, i32>>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/thread/struct.ScopedJoinHandle.html")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/thread/struct.ScopedJoinHandle.html")
    }));

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Thread>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Thread>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_thread_current_is_stable_across_repeated_calls"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/thread/struct.Thread.html"))
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ThreadId>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ThreadId>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_thread_id_is_stable_across_repeated_calls"))
    );
    assert!(
        proof_description(&report, "creusot").is_some_and(
            |d| d.contains("https://doc.rust-lang.org/std/thread/struct.ThreadId.html")
        )
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn slice_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<[i32]>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<[i32]>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for [i32]"))?;
    assert!(kani_description.contains("verify_slice_indexing_and_length"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for [i32]"))?;
    assert!(creusot_description.contains("verify_slice_indexing_and_length"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn str_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<str>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<str>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for str"))?;
    assert!(kani_description.contains("verify_str_byte_length_and_content"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for str"))?;
    assert!(creusot_description.contains("verify_str_byte_length_and_content"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn core_str_plain_carrier_proof_chains_register_the_kani_and_creusot_proofs() -> miette::Result<()>
{
    // Keep each subject literal on its own line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::Bytes<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::Bytes<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<CharIndices<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<CharIndices<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Chars<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Chars<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<EncodeUtf16<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<EncodeUtf16<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::EscapeDebug<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::EscapeDebug<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::EscapeDefault<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::EscapeDefault<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::EscapeUnicode<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::EscapeUnicode<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::Lines<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::Lines<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitAsciiWhitespace<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<SplitAsciiWhitespace<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitWhitespace<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<SplitWhitespace<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Utf8Chunk<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Utf8Chunk<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Utf8Chunks<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Utf8Chunks<'static>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ParseBoolError>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ParseBoolError>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Utf8Error>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Utf8Error>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<LinesAny<'static>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<LinesAny<'static>>");
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn core_str_pattern_carrier_proof_chains_register_the_kani_and_creusot_proofs() -> miette::Result<()>
{
    // Keep each subject literal on its own line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::Split<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::Split<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::RSplit<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::RSplit<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::SplitN<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::SplitN<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::RSplitN<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::str::RSplitN<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::str::SplitInclusive<'static, char>>"))?;
    assert_root_has_kani_and_creusot(
        &report,
        "RustStdStandard<std::str::SplitInclusive<'static, char>>",
    );

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitTerminator<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<SplitTerminator<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RSplitTerminator<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<RSplitTerminator<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Matches<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Matches<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RMatches<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<RMatches<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<MatchIndices<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<MatchIndices<'static, char>>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RMatchIndices<'static, char>>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<RMatchIndices<'static, char>>");
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn slice_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::Iter<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::slice::Iter<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for std::slice::Iter<'static, i32>")
        })?;
    assert!(kani_description.contains("verify_iter_yields_shared_references_in_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for std::slice::Iter<'static, i32>")
        })?;
    assert!(creusot_description.contains("verify_slice_iter_yields_shared_references_in_order"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn slice_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::IterMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::slice::IterMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for std::slice::IterMut<'static, i32>")
        })?;
    assert!(
        kani_description.contains("verify_iter_mut_yields_mutable_references_that_write_through",)
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for std::slice::IterMut<'static, i32>")
        })?;
    assert!(
        creusot_description
            .contains("verify_slice_iter_mut_yields_mutable_references_that_write_through",)
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn chunks_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Chunks<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Chunks<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn chunks_exact_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChunksExact<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ChunksExact<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn chunks_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChunksMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ChunksMut<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn chunks_exact_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChunksExactMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ChunksExactMut<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn windows_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Windows<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Windows<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rchunks_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RChunks<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RChunks<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rchunks_exact_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RChunksExact<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RChunksExact<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rchunks_exact_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RChunksExactMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RChunksExactMut<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rchunks_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RChunksMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RChunksMut<'static, i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn chunk_by_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn chunk_by_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rsplit_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rsplit_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RSplitMut<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rsplit_n_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rsplit_n_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RSplitNMut<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_inclusive_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence().ends_with(
            "RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>>"
        )
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_inclusive_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<SplitInclusiveMut<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<SplitMut<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_n_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_n_mut_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<SplitNMut<'static, i32, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn escape_ascii_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<EscapeAscii<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<EscapeAscii<'static>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn get_disjoint_mut_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()>
{
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<GetDisjointMutError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<GetDisjointMutError>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn tuple_i32_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<(i32, i32)>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<(i32, i32)>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for (i32, i32)"))?;
    assert!(kani_description.contains("verify_tuple_field_access"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for (i32, i32)"))?;
    assert!(creusot_description.contains("verify_tuple_field_access"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fn_pointer_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<fn(i32) -> i32>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<fn(i32) -> i32>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for fn(i32) -> i32"))?;
    assert!(kani_description.contains("verify_fn_pointer_calls_the_underlying_function"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for fn(i32) -> i32"))?;
    assert!(creusot_description.contains("verify_fn_pointer_calls_the_underlying_function"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn const_pointer_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<*const i32>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<*const i32>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for *const i32"))?;
    assert!(kani_description.contains("verify_const_pointer_cast_is_reproducible"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for *const i32"))?;
    assert!(creusot_description.contains("verify_const_pointer_cast_preserves_the_address"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn mut_pointer_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<*mut i32>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<*mut i32>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for *mut i32"))?;
    assert!(kani_description.contains("verify_mut_pointer_cast_is_reproducible"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for *mut i32"))?;
    assert!(creusot_description.contains("verify_mut_pointer_cast_preserves_the_address"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn assert_unwind_safe_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<AssertUnwindSafe<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<AssertUnwindSafe<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for AssertUnwindSafe<i32>"))?;
    assert!(kani_description.contains("verify_assert_unwind_safe_derefs_transparently"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for AssertUnwindSafe<i32>"))?;
    assert!(creusot_description.contains("verify_assert_unwind_safe_derefs_transparently"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn pin_box_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Pin<Box<i32>>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Pin<Box<i32>>>"));
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn non_null_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonNull<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonNull<i32>>"));
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn shared_reference_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<&'static i32>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<&'static i32>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for &'static i32"))?;
    assert!(kani_description.contains("verify_shared_reference_dereferences_to_the_referent"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for &'static i32"))?;
    assert!(creusot_description.contains("verify_shared_reference_dereferences_to_the_referent"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn mutable_reference_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<&'static mut i32>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<&'static mut i32>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for &'static mut i32"))?;
    assert!(
        kani_description
            .contains("verify_mutable_reference_dereferences_to_and_updates_the_referent")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for &'static mut i32"))?;
    assert!(
        creusot_description
            .contains("verify_mutable_reference_dereferences_to_and_updates_the_referent")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn unit_proof_chain_reports_kani_and_creusot_provenance() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<()>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<()>"));
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn location_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Location<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Location<'static>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn panic_info_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PanicInfo<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<PanicInfo<'static>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn panic_message_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PanicMessage<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<PanicMessage<'static>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cow_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Cow<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Cow<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Cow<'static, i32>"))?;
    assert!(kani_description.contains("verify_cow_borrowed_and_owned_agree_on_their_value"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Cow<'static, i32>"))?;
    assert!(creusot_description.contains("verify_cow_destructure_recovers_the_wrapped_value"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Cell<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Cell<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Cell"))?;
    assert!(kani_description.contains("verify_cell_get_set_replace_take_round_trip"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ref_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<RefCell<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<RefCell<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RefCell"))?;
    assert!(kani_description.contains("verify_ref_cell_dynamic_borrow_rules"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ref_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Ref<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Ref<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Ref"))?;
    assert!(kani_description.contains("verify_ref_derefs_to_the_borrowed_value"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ref_mut_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RefMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RefMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RefMut"))?;
    assert!(kani_description.contains("verify_ref_mut_derefs_and_writes_through_to_the_cell"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn once_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<OnceCell<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<OnceCell<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for OnceCell"))?;
    assert!(kani_description.contains("verify_once_cell_initializes_exactly_once"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn unsafe_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<UnsafeCell<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<UnsafeCell<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for UnsafeCell"))?;
    assert!(kani_description.contains("verify_unsafe_cell_get_mut_and_into_inner_round_trip"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn lazy_cell_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<LazyCell<i32, fn() -> i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<LazyCell<i32, fn() -> i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LazyCell"))?;
    assert!(kani_description.contains("verify_lazy_cell_caches_its_initializer_result"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn borrow_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<BorrowError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<BorrowError>"));
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn borrow_mut_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<BorrowMutError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<BorrowMutError>"));
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_try_from_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<CharTryFromError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<CharTryFromError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for CharTryFromError"))?;
    assert!(
        kani_description
            .contains("verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn decode_utf16_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DecodeUtf16"))?;
    assert!(kani_description.contains("verify_decode_utf16_round_trips_a_bmp_code_unit"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn decode_utf16_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<DecodeUtf16Error>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DecodeUtf16Error>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DecodeUtf16Error"))?;
    assert!(kani_description.contains("verify_decode_utf16_error_reports_the_unpaired_surrogate"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_escape_debug_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::char::EscapeDebug>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::char::EscapeDebug>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::char::EscapeDebug"))?;
    assert!(kani_description.contains("verify_char_escape_debug_escapes_a_newline"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_escape_default_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::char::EscapeDefault>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::char::EscapeDefault>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::char::EscapeDefault"))?;
    assert!(kani_description.contains("verify_char_escape_default_escapes_a_newline"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn char_escape_unicode_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::char::EscapeUnicode>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::char::EscapeUnicode>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::char::EscapeUnicode"))?;
    assert!(kani_description.contains("verify_char_escape_unicode_renders_the_codepoint_escape"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn parse_char_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<ParseCharError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<ParseCharError>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ParseCharError"))?;
    assert!(
        kani_description
            .contains("verify_parse_char_error_occurs_for_empty_or_multi_character_strings")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn to_lowercase_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<ToLowercase>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<ToLowercase>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ToLowercase"))?;
    assert!(kani_description.contains("verify_to_lowercase_maps_an_uppercase_ascii_letter"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn to_uppercase_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<ToUppercase>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<ToUppercase>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ToUppercase"))?;
    assert!(kani_description.contains("verify_to_uppercase_maps_a_lowercase_ascii_letter"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_char_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<TryFromCharError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<TryFromCharError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for TryFromCharError"))?;
    assert!(
        kani_description.contains("verify_try_from_char_error_occurs_exactly_when_out_of_range")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn btree_map_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<BTreeMap<i32, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<BTreeMap<i32, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BTreeMap"))?;
    assert!(kani_description.contains("verify_btree_map_iterates_in_key_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BTreeMap"))?;
    assert!(creusot_description.contains("verify_btree_map_iterates_in_key_order"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn btree_set_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<BTreeSet<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<BTreeSet<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BTreeSet"))?;
    assert!(kani_description.contains("verify_btree_set_iterates_in_sorted_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BTreeSet"))?;
    assert!(creusot_description.contains("verify_btree_set_iterates_in_sorted_order"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<BinaryHeap<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<BinaryHeap<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BinaryHeap"))?;
    assert!(kani_description.contains("verify_binary_heap_pop_yields_the_maximum_first"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BinaryHeap"))?;
    assert!(creusot_description.contains("verify_binary_heap_pop_yields_the_maximum_first"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_drain_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BinaryHeap drain"))?;
    assert!(kani_description.contains("verify_binary_heap_drain_yields_every_pushed_element_once"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BinaryHeap drain"))?;
    assert!(
        creusot_description.contains("verify_binary_heap_drain_yields_every_pushed_element_once")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_into_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()>
{
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::binary_heap::IntoIter<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::binary_heap::IntoIter<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BinaryHeap into_iter"))?;
    assert!(
        kani_description.contains("verify_binary_heap_into_iter_yields_every_pushed_element_once")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BinaryHeap into_iter"))?;
    assert!(
        creusot_description
            .contains("verify_binary_heap_into_iter_yields_every_pushed_element_once")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BinaryHeap iter"))?;
    assert!(kani_description.contains("verify_binary_heap_iter_yields_every_pushed_element_once"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BinaryHeap iter"))?;
    assert!(
        creusot_description.contains("verify_binary_heap_iter_yields_every_pushed_element_once")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn binary_heap_peek_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BinaryHeap peek_mut"))?;
    assert!(kani_description.contains("verify_binary_heap_peek_mut_exposes_the_maximum"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for BinaryHeap peek_mut"))?;
    assert!(creusot_description.contains("verify_binary_heap_peek_mut_exposes_the_maximum"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<LinkedList<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<LinkedList<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LinkedList"))?;
    assert!(kani_description.contains("verify_linked_list_is_fifo_through_back_and_front"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for LinkedList"))?;
    assert!(creusot_description.contains("verify_linked_list_is_fifo_through_back_and_front"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::linked_list::Iter<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::linked_list::Iter<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LinkedList iter"))?;
    assert!(kani_description.contains("verify_linked_list_iter_yields_references_in_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for LinkedList iter"))?;
    assert!(creusot_description.contains("verify_linked_list_iter_yields_references_in_order"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LinkedList iter_mut"))?;
    assert!(kani_description.contains("verify_linked_list_iter_mut_writes_through"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for LinkedList iter_mut"))?;
    assert!(creusot_description.contains("verify_linked_list_iter_mut_writes_through"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_into_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()>
{
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::linked_list::IntoIter<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::linked_list::IntoIter<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LinkedList into_iter"))?;
    assert!(kani_description.contains("verify_linked_list_into_iter_yields_owned_values_in_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for LinkedList into_iter"))?;
    assert!(
        creusot_description.contains("verify_linked_list_into_iter_yields_owned_values_in_order")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn linked_list_extract_if_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()>
{
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LinkedList extract_if"))?;
    assert!(kani_description.contains("verify_linked_list_extract_if_partitions_by_the_predicate"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for LinkedList extract_if"))?;
    assert!(
        creusot_description.contains("verify_linked_list_extract_if_partitions_by_the_predicate")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_reserve_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<TryReserveError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<TryReserveError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for TryReserveError"))?;
    assert!(kani_description.contains("verify_try_reserve_rejects_an_impossible_capacity"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for TryReserveError"))?;
    assert!(creusot_description.contains("verify_try_reserve_rejects_an_impossible_capacity"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<VecDeque<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<VecDeque<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VecDeque"))?;
    assert!(kani_description.contains("verify_vec_deque_pushes_and_pops_from_both_ends"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VecDeque"))?;
    assert!(creusot_description.contains("verify_vec_deque_pushes_and_pops_from_both_ends"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_into_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::vec_deque::IntoIter<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::vec_deque::IntoIter<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VecDeque into_iter"))?;
    assert!(kani_description.contains("verify_vec_deque_into_iter_yields_owned_values_in_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VecDeque into_iter"))?;
    assert!(
        creusot_description.contains("verify_vec_deque_into_iter_yields_owned_values_in_order")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_drain_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VecDeque drain"))?;
    assert!(kani_description.contains("verify_vec_deque_drain_removes_and_yields_in_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VecDeque drain"))?;
    assert!(creusot_description.contains("verify_vec_deque_drain_removes_and_yields_in_order"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VecDeque iter"))?;
    assert!(kani_description.contains("verify_vec_deque_iter_yields_references_in_order"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VecDeque iter"))?;
    assert!(creusot_description.contains("verify_vec_deque_iter_yields_references_in_order"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_deque_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VecDeque iter_mut"))?;
    assert!(kani_description.contains("verify_vec_deque_iter_mut_writes_through"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VecDeque iter_mut"))?;
    assert!(creusot_description.contains("verify_vec_deque_iter_mut_writes_through"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Vec<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Vec<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Vec"))?;
    assert!(kani_description.contains("verify_vec_push_pop_round_trips"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_drain_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::vec::Drain<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::vec::Drain<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Vec drain"))?;
    assert!(kani_description.contains("verify_vec_drain_removes_and_yields_in_order"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_into_iter_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::vec::IntoIter<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::vec::IntoIter<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Vec into_iter"))?;
    assert!(kani_description.contains("verify_vec_into_iter_yields_owned_values_in_order"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_extract_if_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Vec extract_if"))?;
    assert!(kani_description.contains("verify_vec_extract_if_partitions_by_the_predicate"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vec_splice_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Vec splice"))?;
    assert!(kani_description.contains("verify_splice_replaces_a_range_and_yields_what_it_removed"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn type_id_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<TypeId>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<TypeId>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for TypeId"))?;
    assert!(
        kani_description.contains("verify_type_id_is_reflexive_and_distinguishes_distinct_types")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn layout_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Layout>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Layout>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Layout"))?;
    assert!(kani_description.contains("verify_layout_new_reports_the_types_size_and_alignment"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_slice_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<TryFromSliceError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<TryFromSliceError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for TryFromSliceError"))?;
    assert!(kani_description.contains("verify_try_from_slice_rejects_a_length_mismatch"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn array_into_iter_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<IntoIter<i32, 3>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<IntoIter<i32, 3>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for array IntoIter"))?;
    assert!(kani_description.contains("verify_array_into_iter_yields_elements_in_order"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ascii_escape_default_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain(
        "RustStdStandard<core::ascii::EscapeDefault>",
    ))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::ascii::EscapeDefault>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::ascii::EscapeDefault"))?;
    assert!(kani_description.contains("verify_escape_default_escapes_a_control_byte"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ordering_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::cmp::Ordering>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::cmp::Ordering>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Ordering"))?;
    assert!(kani_description.contains("verify_ordering_reverse_involution"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Ordering"))?;
    assert!(creusot_description.contains("verify_ordering_reverse_swaps_less_and_greater"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn reverse_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Reverse<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Reverse<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Reverse<i32>"))?;
    assert!(kani_description.contains("verify_reverse_inverts_comparison"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Reverse<i32>"))?;
    assert!(creusot_description.contains("verify_reverse_inverts_comparison"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rc_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Rc<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Rc<i32>>"));
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn rc_weak_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::rc::Weak<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::rc::Weak<i32>>")
    );
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn string_drain_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::string::Drain<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::string::Drain<'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for String drain"))?;
    assert!(kani_description.contains("verify_string_drain_removes_and_yields_the_content"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_utf16_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<FromUtf16Error>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<FromUtf16Error>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FromUtf16Error"))?;
    assert!(kani_description.contains("verify_from_utf16_rejects_a_lone_surrogate"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_utf8_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<FromUtf8Error>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<FromUtf8Error>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FromUtf8Error"))?;
    assert!(kani_description.contains("verify_from_utf8_error_recovers_the_original_bytes"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn arc_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Arc<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Arc<i32>>"));
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn arc_weak_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::sync::Weak<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::sync::Weak<i32>>")
    );
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn infallible_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Infallible>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Infallible>"));
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn layout_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<LayoutError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<LayoutError>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for LayoutError"))?;
    assert!(
        kani_description
            .contains("verify_layout_from_size_align_rejects_a_non_power_of_two_alignment")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn addr_parse_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<AddrParseError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<AddrParseError>"));
    assert!(root.is_root());
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "kani")
    );
    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ip_addr_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<IpAddr>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<IpAddr>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for IpAddr"))?;
    assert!(kani_description.contains("verify_ip_addr_variant_matches_its_kind"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ipv4_addr_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Ipv4Addr>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Ipv4Addr>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Ipv4Addr"))?;
    assert!(kani_description.contains("verify_ipv4_addr_octets_round_trip"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ipv6_addr_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Ipv6Addr>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Ipv6Addr>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Ipv6Addr"))?;
    assert!(kani_description.contains("verify_ipv6_addr_segments_round_trip"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn socket_addr_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SocketAddr>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<SocketAddr>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SocketAddr"))?;
    assert!(kani_description.contains("verify_socket_addr_variant_matches_its_kind"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn socket_addr_v4_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SocketAddrV4>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<SocketAddrV4>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SocketAddrV4"))?;
    assert!(kani_description.contains("verify_socket_addr_v4_round_trips_ip_and_port"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn socket_addr_v6_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SocketAddrV6>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<SocketAddrV6>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SocketAddrV6"))?;
    assert!(kani_description.contains("verify_socket_addr_v6_round_trips_all_fields"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
fn nonzero_i8_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<i8>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<i8>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<i8>"))?;
    assert!(kani_description.contains("verify_nonzero_i8"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn nonzero_i16_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<i16>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<i16>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<i16>"))?;
    assert!(kani_description.contains("verify_nonzero_i16"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for NonZero<i16>"))?;
    assert!(creusot_description.contains("verify_nonzero_i16_roundtrips"));
    Ok(())
}

#[test]
fn nonzero_i32_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<i32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<i32>"))?;
    assert!(kani_description.contains("verify_nonzero_i32"));
    Ok(())
}

#[test]
fn nonzero_i64_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<i64>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<i64>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<i64>"))?;
    assert!(kani_description.contains("verify_nonzero_i64"));
    Ok(())
}

#[test]
fn nonzero_i128_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<i128>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<i128>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<i128>"))?;
    assert!(kani_description.contains("verify_nonzero_i128"));
    Ok(())
}

#[test]
fn nonzero_isize_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<isize>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<isize>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<isize>"))?;
    assert!(kani_description.contains("verify_nonzero_isize"));
    Ok(())
}

#[test]
fn nonzero_u8_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<u8>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<u8>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<u8>"))?;
    assert!(kani_description.contains("verify_nonzero_u8"));
    Ok(())
}

#[test]
fn nonzero_u16_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<u16>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<u16>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<u16>"))?;
    assert!(kani_description.contains("verify_nonzero_u16"));
    Ok(())
}

#[test]
fn nonzero_u32_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<u32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<u32>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<u32>"))?;
    assert!(kani_description.contains("verify_nonzero_u32"));
    Ok(())
}

#[test]
fn nonzero_u64_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<u64>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<u64>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<u64>"))?;
    assert!(kani_description.contains("verify_nonzero_u64"));
    Ok(())
}

#[test]
fn nonzero_u128_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<u128>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<u128>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<u128>"))?;
    assert!(kani_description.contains("verify_nonzero_u128"));
    Ok(())
}

#[test]
fn nonzero_usize_proof_chain_registers_the_kani_proof() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<NonZero<usize>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NonZero<usize>>"));
    assert!(root.is_root());
    assert_eq!(root.proofs().len(), 1);
    assert_eq!(report.verifiers().len(), 1);

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NonZero<usize>"))?;
    assert!(kani_description.contains("verify_nonzero_usize"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn wrapping_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Wrapping<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Wrapping<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Wrapping<i32>"))?;
    assert!(kani_description.contains("verify_wrapping_add_matches_the_inner_wrapping_add"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Wrapping<i32>"))?;
    assert!(creusot_description.contains("verify_wrapping_i32_add_wraps"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn saturating_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Saturating<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Saturating<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Saturating<i32>"))?;
    assert!(kani_description.contains("verify_saturating_add_matches_the_inner_saturating_add"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Saturating<i32>"))?;
    assert!(creusot_description.contains("verify_saturating_i32_add_clamps"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn int_error_kind_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::num::IntErrorKind>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::num::IntErrorKind>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::num::IntErrorKind"))?;
    assert!(kani_description.contains("verify_int_error_kind_classifies_parse_failures"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for core::num::IntErrorKind"))?;
    assert!(creusot_description.contains("verify_int_error_kind_classifies_parse_failures"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_int_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::num::TryFromIntError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::num::TryFromIntError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::num::TryFromIntError"))?;
    assert!(
        kani_description.contains("verify_try_from_int_error_occurs_exactly_when_out_of_range")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for core::num::TryFromIntError")
        })?;
    assert!(
        creusot_description.contains("verify_try_from_int_error_occurs_exactly_when_out_of_range")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn parse_int_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::num::ParseIntError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::num::ParseIntError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::num::ParseIntError"))?;
    assert!(kani_description.contains("verify_parse_int_error_reports_the_kind_of_the_failure"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for core::num::ParseIntError"))?;
    assert!(creusot_description.contains("verify_parse_int_error_reports_the_kind_of_the_failure"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fp_category_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::num::FpCategory>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::num::FpCategory>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::num::FpCategory"))?;
    assert!(kani_description.contains("verify_fp_category_matches_the_value_it_classifies"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for core::num::FpCategory"))?;
    assert!(creusot_description.contains("verify_fp_category_matches_the_value_it_classifies"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn parse_float_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::num::ParseFloatError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::num::ParseFloatError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for core::num::ParseFloatError"))?;
    assert!(
        kani_description.contains("verify_parse_float_error_occurs_only_for_unparseable_input")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for core::num::ParseFloatError")
        })?;
    assert!(
        creusot_description.contains("verify_parse_float_error_occurs_only_for_unparseable_input")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cstring_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<CString>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<CString>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for CString"))?;
    assert!(
        kani_description
            .contains("verify_cstring_excludes_the_terminator_and_rejects_interior_nul")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for CString"))?;
    assert!(
        creusot_description
            .contains("verify_cstring_excludes_the_terminator_and_rejects_interior_nul")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn args_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Args>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Args>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Args"))?;
    assert!(kani_description.contains("verify_args_reports_at_least_the_program_path"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Args"))?;
    assert!(creusot_description.contains("verify_args_reports_at_least_the_program_path"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn args_os_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ArgsOs>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<ArgsOs>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ArgsOs"))?;
    assert!(kani_description.contains("verify_args_os_reports_at_least_the_program_path"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for ArgsOs"))?;
    assert!(creusot_description.contains("verify_args_os_reports_at_least_the_program_path"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn join_paths_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<JoinPathsError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<JoinPathsError>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for JoinPathsError"))?;
    assert!(kani_description.contains("verify_join_paths_error_reports_an_unjoinable_path"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for JoinPathsError"))?;
    assert!(creusot_description.contains("verify_join_paths_error_reports_an_unjoinable_path"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn split_paths_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SplitPaths<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<SplitPaths<'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SplitPaths<'static>"))?;
    assert!(kani_description.contains("verify_split_paths_recovers_paths_joined_by_join_paths"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for SplitPaths<'static>"))?;
    assert!(creusot_description.contains("verify_split_paths_recovers_paths_joined_by_join_paths"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn var_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<VarError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<VarError>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VarError"))?;
    assert!(kani_description.contains("type_name: std::env::VarError"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VarError"))?;
    assert!(
        creusot_description.contains("verify_var_error_distinguishes_not_present_from_not_unicode")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vars_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Vars>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Vars>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Vars"))?;
    assert!(kani_description.contains("type_name: std::env::Vars"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Vars"))?;
    assert!(creusot_description.contains("type_name: std::env::Vars"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn vars_os_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<VarsOs>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<VarsOs>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for VarsOs"))?;
    assert!(kani_description.contains("type_name: std::env::VarsOs"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for VarsOs"))?;
    assert!(creusot_description.contains("type_name: std::env::VarsOs"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn dir_builder_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DirBuilder>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<DirBuilder>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DirBuilder"))?;
    assert!(kani_description.contains("verify_dir_builder_creates_nested_directories_recursively"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for DirBuilder"))?;
    assert!(creusot_description.contains("type_name: std::fs::DirBuilder"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn dir_entry_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DirEntry>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<DirEntry>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DirEntry"))?;
    assert!(kani_description.contains("verify_dir_entry_reports_the_created_files_name_and_path"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for DirEntry"))?;
    assert!(creusot_description.contains("type_name: std::fs::DirEntry"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn file_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<File>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<File>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for File"))?;
    assert!(kani_description.contains("verify_file_write_then_read_round_trips_the_bytes"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for File"))?;
    assert!(creusot_description.contains("type_name: std::fs::File"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn file_times_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<FileTimes>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<FileTimes>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FileTimes"))?;
    assert!(kani_description.contains("verify_file_times_sets_the_recorded_modification_time"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for FileTimes"))?;
    assert!(creusot_description.contains("type_name: std::fs::FileTimes"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn file_type_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<FileType>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<FileType>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FileType"))?;
    assert!(kani_description.contains("verify_file_type_distinguishes_files_from_directories"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for FileType"))?;
    assert!(creusot_description.contains("type_name: std::fs::FileType"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn metadata_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Metadata>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Metadata>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Metadata"))?;
    assert!(kani_description.contains("verify_metadata_reports_the_written_length"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Metadata"))?;
    assert!(creusot_description.contains("type_name: std::fs::Metadata"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn open_options_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<OpenOptions>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<OpenOptions>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for OpenOptions"))?;
    assert!(kani_description.contains("verify_open_options_create_new_rejects_an_existing_file"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for OpenOptions"))?;
    assert!(creusot_description.contains("type_name: std::fs::OpenOptions"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn permissions_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Permissions>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Permissions>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Permissions"))?;
    assert!(
        kani_description
            .contains("verify_permissions_readonly_round_trips_through_set_permissions")
    );

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Permissions"))?;
    assert!(creusot_description.contains("type_name: std::fs::Permissions"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn read_dir_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ReadDir>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<ReadDir>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ReadDir"))?;
    assert!(kani_description.contains("verify_read_dir_iterates_every_entry_in_the_directory"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for ReadDir"))?;
    assert!(creusot_description.contains("type_name: std::fs::ReadDir"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fs_try_lock_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::fs::TryLockError>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::fs::TryLockError>");

    let (_, kani_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for std::fs::TryLockError"))?;
    assert!(kani_description.contains("verify_try_lock_error_reports_a_lock_already_held"));

    let (_, creusot_description) = report
        .root()
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for std::fs::TryLockError"))?;
    assert!(creusot_description.contains("type_name: std::fs::TryLockError"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn os_str_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<OsStr>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<OsStr>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for OsStr"))?;
    assert!(
        kani_description.contains("verify_os_str_valid_utf8_content_round_trips_through_to_str")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for OsStr"))?;
    assert!(
        creusot_description.contains("verify_os_str_valid_utf8_content_round_trips_through_to_str")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn os_string_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<OsString>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<OsString>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for OsString"))?;
    assert!(kani_description.contains("verify_os_string_push_appends_to_the_existing_content"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for OsString"))?;
    assert!(creusot_description.contains("verify_os_string_push_appends_to_the_existing_content"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn os_str_display_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::ffi::os_str::Display<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::ffi::os_str::Display<'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for std::ffi::os_str::Display<'static>")
        })?;
    assert!(
        kani_description.contains("verify_os_str_display_renders_valid_utf8_content_unchanged")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for std::ffi::os_str::Display<'static>")
        })?;
    assert!(
        creusot_description.contains("verify_os_str_display_renders_valid_utf8_content_unchanged")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_vec_with_nul_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()>
{
    let report = support::chain(amenable::proof_chain(
        "RustStdStandard<FromVecWithNulError>",
    ))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<FromVecWithNulError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FromVecWithNulError"))?;
    assert!(kani_description.contains("verify_from_vec_with_nul_requires_the_nul_only_at_the_end"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for FromVecWithNulError"))?;
    assert!(
        creusot_description.contains("verify_from_vec_with_nul_requires_the_nul_only_at_the_end")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn into_string_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<IntoStringError>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<IntoStringError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for IntoStringError"))?;
    assert!(kani_description.contains("verify_into_string_error_recovers_the_original_cstring"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for IntoStringError"))?;
    assert!(creusot_description.contains("verify_into_string_error_recovers_the_original_cstring"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn nul_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<NulError>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<NulError>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for NulError"))?;
    assert!(kani_description.contains("verify_nul_error_reports_the_interior_nuls_position"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for NulError"))?;
    assert!(creusot_description.contains("verify_nul_error_reports_the_interior_nuls_position"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cstr_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<CStr>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<CStr>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for CStr"))?;
    assert!(kani_description.contains("verify_cstr_excludes_the_terminating_nul_from_to_bytes"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for CStr"))?;
    assert!(creusot_description.contains("verify_cstr_excludes_the_terminating_nul_from_to_bytes"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_bytes_until_nul_error_proof_chain_reports_the_kani_and_creusot_harnesses()
-> miette::Result<()> {
    let report = support::chain(amenable::proof_chain(
        "RustStdStandard<FromBytesUntilNulError>",
    ))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<FromBytesUntilNulError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FromBytesUntilNulError"))?;
    assert!(kani_description.contains("verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for FromBytesUntilNulError"))?;
    assert!(
        creusot_description.contains("verify_from_bytes_until_nul_requires_a_nul_byte_somewhere")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn from_bytes_with_nul_error_proof_chain_reports_the_kani_and_creusot_harnesses()
-> miette::Result<()> {
    let report = support::chain(amenable::proof_chain(
        "RustStdStandard<FromBytesWithNulError>",
    ))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<FromBytesWithNulError>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FromBytesWithNulError"))?;
    assert!(
        kani_description.contains("verify_from_bytes_with_nul_requires_the_nul_only_at_the_end")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for FromBytesWithNulError"))?;
    assert!(
        creusot_description.contains("verify_from_bytes_with_nul_requires_the_nul_only_at_the_end")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn duration_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<Duration>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Duration>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Duration"))?;
    assert!(
        kani_description.contains("verify_duration_new_normalizes_nanos_and_carries_into_secs")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Duration"))?;
    assert!(
        creusot_description.contains("verify_duration_new_normalizes_nanos_and_carries_into_secs")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn try_from_float_secs_error_proof_chain_registers_the_kani_and_creusot_proofs()
-> miette::Result<()> {
    let report = support::chain(amenable::proof_chain(
        "RustStdStandard<TryFromFloatSecsError>",
    ))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<TryFromFloatSecsError>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn range_to_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RangeTo<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<RangeTo<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RangeTo<i32>"))?;
    assert!(kani_description.contains("verify_range_to_contains_matches_bound"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for RangeTo<i32>"))?;
    assert!(creusot_description.contains("verify_range_to_contains_matches_bound"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn range_full_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RangeFull>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<RangeFull>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RangeFull"))?;
    assert!(kani_description.contains("verify_range_full_contains_everything"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for RangeFull"))?;
    assert!(creusot_description.contains("verify_range_full_contains_everything"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn bound_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Bound<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Bound<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Bound<i32>"))?;
    assert!(kani_description.contains("verify_bound_round_trips_its_endpoint"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Bound<i32>"))?;
    assert!(creusot_description.contains("verify_bound_round_trips_its_endpoint"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn control_flow_i32_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ControlFlow<i32, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ControlFlow<i32, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ControlFlow<i32, i32>"))?;
    assert!(kani_description.contains("verify_control_flow_continue_and_break_are_disjoint"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for ControlFlow<i32, i32>"))?;
    assert!(creusot_description.contains("verify_control_flow_continue_and_break_are_disjoint"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn box_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Box<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Box<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Box<i32>"))?;
    assert!(kani_description.contains("verify_box_derefs_and_writes_through"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Box<i32>"))?;
    assert!(creusot_description.contains("verify_box_new_preserves_the_wrapped_value"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn c_void_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::ffi::c_void>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::ffi::c_void>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_alignment_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::fmt::Alignment>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::fmt::Alignment>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for std::fmt::Alignment"))?;
    assert!(
        kani_description.contains("verify_alignment_reaches_the_formatter_from_the_format_spec")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_arguments_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Arguments<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Arguments<'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Arguments"))?;
    assert!(kani_description.contains("verify_arguments_renders_the_same_as_the_value_itself"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_error_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::fmt::Error>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::fmt::Error>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_formatter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Formatter<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Formatter<'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Formatter"))?;
    assert!(kani_description.contains("verify_formatter_exposes_the_parsed_width_and_precision"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_list_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DebugList<'static, 'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DebugList<'static, 'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DebugList"))?;
    assert!(kani_description.contains("verify_debug_list_renders_entries_in_brackets"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_map_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DebugMap<'static, 'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DebugMap<'static, 'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DebugMap"))?;
    assert!(kani_description.contains("verify_debug_map_renders_key_value_pairs"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_set_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DebugSet<'static, 'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DebugSet<'static, 'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DebugSet"))?;
    assert!(kani_description.contains("verify_debug_set_renders_entries_in_braces"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_struct_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DebugStruct<'static, 'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DebugStruct<'static, 'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DebugStruct"))?;
    assert!(kani_description.contains("verify_debug_struct_renders_named_fields"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_debug_tuple_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DebugTuple<'static, 'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<DebugTuple<'static, 'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DebugTuple"))?;
    assert!(kani_description.contains("verify_debug_tuple_renders_positional_fields"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn fmt_from_fn_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FromFn"))?;
    assert!(kani_description.contains("verify_from_fn_forwards_display_to_the_supplied_closure"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn option_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Option<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Option<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Option<i32>"))?;
    assert!(kani_description.contains("verify_option_some_and_none_are_disjoint"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Option<i32>"))?;
    assert!(creusot_description.contains("verify_option_some_and_none_are_disjoint"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn result_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Result<i32, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Result<i32, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Result<i32, i32>"))?;
    assert!(kani_description.contains("verify_result_ok_and_err_are_disjoint"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Result<i32, i32>"))?;
    assert!(creusot_description.contains("verify_result_ok_and_err_are_disjoint"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn option_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::option::Iter<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::option::Iter<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for core::option::Iter<'static, i32>")
        })?;
    assert!(kani_description.contains("verify_option_iter_yields_zero_or_one_reference"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for core::option::Iter<'static, i32>")
        })?;
    assert!(creusot_description.contains("verify_option_iter_yields_zero_or_one_reference"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn option_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::option::IterMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::option::IterMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for core::option::IterMut<'static, i32>")
        })?;
    assert!(kani_description.contains("verify_option_iter_mut_writes_through_to_the_option"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for core::option::IterMut<'static, i32>")
        })?;
    assert!(creusot_description.contains("verify_option_iter_mut_writes_through_to_the_option"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn result_iter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::result::Iter<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::result::Iter<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for core::result::Iter<'static, i32>")
        })?;
    assert!(kani_description.contains("verify_result_iter_yields_a_reference_to_the_ok_value"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for core::result::Iter<'static, i32>")
        })?;
    assert!(creusot_description.contains("verify_result_iter_yields_a_reference_to_the_ok_value"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn result_iter_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<core::result::IterMut<'static, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<core::result::IterMut<'static, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for core::result::IterMut<'static, i32>")
        })?;
    assert!(kani_description.contains("verify_result_iter_mut_writes_through_to_the_result"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!("creusot proof registered for core::result::IterMut<'static, i32>")
        })?;
    assert!(creusot_description.contains("verify_result_iter_mut_writes_through_to_the_result"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn pending_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Pending<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Pending<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Pending<i32>"))?;
    assert!(kani_description.contains("verify_pending_never_resolves"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Pending<i32>"))?;
    assert!(creusot_description.contains("verify_pending_never_resolves"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn poll_fn_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<PollFn<fn(&mut Context<'_>) -> Poll<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| {
            miette::miette!("kani proof registered for PollFn<fn(&mut Context<'_>) -> Poll<i32>>")
        })?;
    assert!(kani_description.contains("verify_poll_fn_dispatches_through_to_its_closure"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| {
            miette::miette!(
                "creusot proof registered for PollFn<fn(&mut Context<'_>) -> Poll<i32>>"
            )
        })?;
    assert!(creusot_description.contains("verify_poll_fn_dispatches_through_to_its_closure"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn ready_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Ready<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Ready<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Ready<i32>"))?;
    assert!(kani_description.contains("verify_ready_resolves_immediately_with_its_value"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Ready<i32>"))?;
    assert!(creusot_description.contains("verify_ready_resolves_immediately_with_its_value"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn context_static_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Context<'static>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Context<'static>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Context<'static>"))?;
    assert!(kani_description.contains("verify_context_from_waker_exposes_the_same_waker"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Context<'static>"))?;
    assert!(creusot_description.contains("verify_context_from_waker_exposes_the_same_waker"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn poll_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Poll<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Poll<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Poll<i32>"))?;
    assert!(kani_description.contains("verify_poll_ready_and_pending_are_disjoint"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Poll<i32>"))?;
    assert!(creusot_description.contains("verify_poll_ready_and_pending_are_disjoint"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn raw_waker_and_vtable_proof_chains_register_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep each subject literal on its own line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RawWaker>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<RawWaker>");

    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RawWakerVTable>"))?;
    assert_root_has_kani_and_creusot(&report, "RustStdStandard<RawWakerVTable>");
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn waker_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Waker>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<Waker>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Waker"))?;
    assert!(kani_description.contains("verify_waker_wake_by_ref_invokes_the_wake_impl"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for Waker"))?;
    assert!(creusot_description.contains("verify_waker_wake_by_ref_invokes_the_wake_impl"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_chain_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Chain"))?;
    assert!(kani_description.contains("verify_chain_sequences_two_iterators_end_to_end"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_cloned_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Cloned<Iter<'static, i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Cloned<Iter<'static, i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Cloned"))?;
    assert!(kani_description.contains("verify_cloned_clones_each_referenced_item"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_copied_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Copied<Iter<'static, i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Copied<Iter<'static, i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Copied"))?;
    assert!(kani_description.contains("verify_copied_copies_each_referenced_item"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_cycle_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Cycle<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Cycle<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Cycle"))?;
    assert!(kani_description.contains("verify_cycle_repeats_its_sequence_forever"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_empty_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::iter::Empty<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::iter::Empty<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Empty"))?;
    assert!(kani_description.contains("verify_empty_yields_nothing"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_enumerate_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Enumerate<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Enumerate<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Enumerate"))?;
    assert!(kani_description.contains("verify_enumerate_pairs_each_item_with_its_index"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_filter_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Filter"))?;
    assert!(kani_description.contains("verify_filter_yields_only_items_matching_the_predicate"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_filter_map_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with(
        "RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>"
    ));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FilterMap"))?;
    assert!(kani_description.contains("verify_filter_map_applies_and_filters_in_one_step"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_flat_map_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with(
        "RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>"
    ));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for FlatMap"))?;
    assert!(kani_description.contains("verify_flat_map_flattens_each_generated_iterator"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_flatten_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Flatten<IntoIter<Range<i32>>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Flatten<IntoIter<Range<i32>>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Flatten"))?;
    assert!(kani_description.contains("verify_flatten_concatenates_the_inner_iterators"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_map_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Map"))?;
    assert!(kani_description.contains("verify_map_applies_its_closure_to_each_item"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_zip_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Zip<Range<i32>, Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Zip<Range<i32>, Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Zip"))?;
    assert!(kani_description.contains("verify_zip_pairs_items_from_two_iterators"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_rev_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Rev<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Rev<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Rev"))?;
    assert!(kani_description.contains("verify_rev_reverses_iteration_order"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_fuse_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Fuse<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Fuse<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Fuse"))?;
    assert!(kani_description.contains("verify_fuse_keeps_returning_none_once_exhausted"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_inspect_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Inspect<Range<i32>, fn(&i32)>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Inspect<Range<i32>, fn(&i32)>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Inspect"))?;
    assert!(
        kani_description.contains("verify_inspect_calls_once_per_item_without_changing_values")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_peekable_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Peekable<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Peekable<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Peekable"))?;
    assert!(kani_description.contains("verify_peekable_peek_does_not_consume"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_scan_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Scan"))?;
    assert!(kani_description.contains("verify_scan_threads_state_through_its_closure"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_skip_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Skip<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Skip<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Skip"))?;
    assert!(kani_description.contains("verify_skip_discards_the_first_n_items"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_skip_while_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SkipWhile"))?;
    assert!(
        kani_description.contains("verify_skip_while_discards_items_while_the_predicate_holds")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_step_by_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<StepBy<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<StepBy<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for StepBy"))?;
    assert!(kani_description.contains("verify_step_by_yields_every_nth_item"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_take_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::iter::Take<Range<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::iter::Take<Range<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Take"))?;
    assert!(kani_description.contains("verify_take_yields_at_most_n_items"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_take_while_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for TakeWhile"))?;
    assert!(kani_description.contains("verify_take_while_yields_items_while_the_predicate_holds"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_map_while_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for MapWhile"))?;
    assert!(
        kani_description.contains("verify_map_while_maps_items_while_the_closure_returns_some")
    );

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_once_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::iter::Once<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::iter::Once<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Once"))?;
    assert!(kani_description.contains("verify_once_yields_exactly_one_value"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_once_with_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<OnceWith<fn() -> i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<OnceWith<fn() -> i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for OnceWith"))?;
    assert!(kani_description.contains("verify_once_with_calls_its_closure_exactly_once"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_repeat_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::iter::Repeat<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<std::iter::Repeat<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Repeat"))?;
    assert!(kani_description.contains("verify_repeat_yields_the_same_value_forever"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_repeat_with_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RepeatWith<fn() -> i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<RepeatWith<fn() -> i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RepeatWith"))?;
    assert!(kani_description.contains("verify_repeat_with_calls_its_closure_once_per_item"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_repeat_n_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RepeatN<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<RepeatN<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RepeatN"))?;
    assert!(kani_description.contains("verify_repeat_n_yields_the_value_exactly_n_times"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn iter_successors_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Successors"))?;
    assert!(kani_description.contains("verify_successors_generates_from_the_previous_item"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_default_hasher_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<DefaultHasher>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<DefaultHasher>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for DefaultHasher"))?;
    assert!(
        kani_description.contains("verify_default_hasher_is_deterministic_across_fresh_instances")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for DefaultHasher"))?;
    assert!(
        creusot_description
            .contains("verify_default_hasher_is_deterministic_across_fresh_instances")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_random_state_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<RandomState>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<RandomState>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for RandomState"))?;
    assert!(
        kani_description.contains("verify_random_state_gives_the_same_hasher_seed_across_calls")
    );

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for RandomState"))?;
    assert!(
        creusot_description.contains("verify_random_state_gives_the_same_hasher_seed_across_calls")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_build_hasher_default_proof_chain_reports_the_kani_and_creusot_harnesses()
-> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<BuildHasherDefault<DefaultHasher>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<BuildHasherDefault<DefaultHasher>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for BuildHasherDefault"))?;
    assert!(kani_description.contains("verify_build_hasher_default_produces_consistent_hashers"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_map_i32_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<HashMap<i32, i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<HashMap<i32, i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for HashMap<i32, i32>"))?;
    assert!(kani_description.contains("verify_hash_map_insert_then_get_recovers_the_value"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for HashMap<i32, i32>"))?;
    assert!(creusot_description.contains("verify_hash_map_insert_then_get_recovers_the_value"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_set_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<HashSet<i32>>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<HashSet<i32>>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for HashSet<i32>"))?;
    assert!(kani_description.contains("verify_hash_set_insert_then_contains_reports_membership"));

    let (_, creusot_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "creusot")
        .ok_or_else(|| miette::miette!("creusot proof registered for HashSet<i32>"))?;
    assert!(
        creusot_description.contains("verify_hash_set_insert_then_contains_reports_membership")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn hash_sip_hasher_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<SipHasher>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<SipHasher>"));
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for SipHasher"))?;
    assert!(kani_description.contains("verify_sip_hasher_produces_consistent_hashes"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn marker_phantom_data_i32_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()>
{
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PhantomData<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<PhantomData<i32>>")
    );
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn marker_phantom_pinned_proof_chain_registers_the_kani_and_creusot_proofs() -> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PhantomPinned>"))?;

    let root = report.root();
    assert!(root.evidence().ends_with("RustStdStandard<PhantomPinned>"));
    assert!(root.is_root());

    let verifiers: Vec<&str> = root
        .proofs()
        .iter()
        .map(|(verifier, _)| verifier.as_str())
        .collect();
    assert!(verifiers.contains(&"kani"));
    assert!(verifiers.contains(&"creusot"));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn mem_manually_drop_i32_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()>
{
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<ManuallyDrop<i32>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<ManuallyDrop<i32>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for ManuallyDrop<i32>"))?;
    assert!(kani_description.contains("verify_manually_drop_derefs_and_into_inner_round_trip"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn mem_discriminant_option_i32_proof_chain_reports_the_kani_and_creusot_harnesses()
-> miette::Result<()> {
    // Keep the subject literal on this line: `elicit_doc` currently
    // scans proof-chain test subjects line-by-line.
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Discriminant<Option<i32>>>"))?;

    let root = report.root();
    assert!(
        root.evidence()
            .ends_with("RustStdStandard<Discriminant<Option<i32>>>")
    );
    assert!(root.is_root());

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for Discriminant<Option<i32>>"))?;
    assert!(kani_description.contains("verify_discriminant_identifies_variant_not_payload"));

    assert!(
        root.proofs()
            .iter()
            .any(|(verifier, _)| *verifier == "creusot")
    );
    Ok(())
}

#[test]
fn unregistered_subject_yields_a_not_found_error() {
    let result = amenable::proof_chain("NoSuchEvidenceType");
    assert!(
        matches!(
            &result,
            Err(error) if matches!(
                &**error.kind(),
                ChainErrorKind::NotFound(source) if source.subject() == "NoSuchEvidenceType"
            )
        ),
        "expected ChainError::NotFound(subject = \"NoSuchEvidenceType\"), got {result:?}"
    );
}

#[test]
fn proof_chain_report_renders_as_human_readable_text() -> miette::Result<()> {
    let report = support::chain(amenable::proof_chain("RustStdStandard<bool>"))?;
    let rendered = report.to_string();

    assert!(rendered.starts_with("Proof chain for"));
    assert!(rendered.contains("complete for: "));
    assert!(rendered.contains("(root)"));
    assert!(rendered.contains("proof [kani]:"));
    Ok(())
}

#[test]
fn calculation_over_two_arguments_fans_out_into_a_tree_that_bottoms_out_in_std()
-> miette::Result<()> {
    // AddEvidence/Debit/Credit only have kani proofs — the default (auto
    // discover every verifier seen anywhere in the tree) would find
    // creusot/verus too (present on the RustStdStandard<i64> leaf) and
    // correctly refuse to return a report, so scope this to kani only.
    let report = support::chain(amenable::proof_chain_for_verifiers(
        "AddEvidence",
        Some(&["kani"]),
    ))?;

    let root = report.root();
    assert!(root.evidence().ends_with("AddEvidence"));
    assert!(!root.is_root());
    assert_eq!(
        root.bases().len(),
        2,
        "Debit and Credit are separate branches"
    );

    // Argument order is preserved (a: Debit, b: Credit), not whatever
    // order `inventory` happened to iterate registrations in.
    assert!(root.bases()[0].evidence().ends_with("Debit"));
    assert!(root.bases()[1].evidence().ends_with("Credit"));

    // Neither Debit nor Credit is itself a root: both are thin domain
    // wrappers around i64, so both bottom out in the same std standard.
    for branch in root.bases() {
        assert!(!branch.is_root());
        assert_eq!(branch.bases().len(), 1);
        assert_eq!(branch.proofs().len(), 1, "access proof, kani-only");

        let std_node = &branch.bases()[0];
        assert!(std_node.evidence().ends_with("RustStdStandard<i64>"));
        assert!(std_node.is_root());
        assert_eq!(std_node.proofs().len(), 1, "kani-only trusted proof");
    }

    let (_, kani_description) = root
        .proofs()
        .iter()
        .find(|(verifier, _)| *verifier == "kani")
        .ok_or_else(|| miette::miette!("kani proof registered for AddEvidence"))?;
    assert!(kani_description.contains("add_impl_computes_exact_sum"));
    Ok(())
}

// Needs both features linked in together: scoped to "verus" alone, this
// relies on the `RustStdStandard<i64>` leaf actually having a verus
// proof (so it's excluded from the gap list) -- which is only true when
// the `verus` feature is also on. See the comment above
// `bool_proof_chain_is_a_single_root_node_with_all_three_verifiers` for
// why no justfile recipe currently exercises this combination.
#[test]
#[cfg_attr(not(all(feature = "creusot", feature = "verus")), ignore)]
fn calculation_chain_is_incomplete_for_creusot_and_verus() {
    for verifier in ["creusot", "verus"] {
        let result = amenable::proof_chain_for_verifiers("AddEvidence", Some(&[verifier]));
        assert!(
            matches!(
                &result,
                Err(error) if matches!(&**error.kind(), ChainErrorKind::Incomplete(_))
            ),
            "expected ChainError::Incomplete for {verifier}, got {result:?}"
        );
        // The assert! above already fails the test if this shape doesn't
        // hold, so these nested `if let`s only ever run once it's
        // established -- no `else` diverging arm needed.
        if let Err(error) = &result
            && let ChainErrorKind::Incomplete(source) = &**error.kind()
        {
            let (subject, required, gaps) = (source.subject(), source.required(), source.gaps());
            assert!(subject.ends_with("AddEvidence"));
            assert_eq!(required, &vec![verifier.to_string()]);

            // AddEvidence, Debit, and Credit all lack a proof for this
            // verifier; only the RustStdStandard<i64> leaf has one, so
            // exactly three gaps are expected.
            assert_eq!(gaps.len(), 3, "gaps for {verifier}: {gaps:?}");
            assert!(
                gaps.iter()
                    .any(|gap| gap.evidence().ends_with("AddEvidence"))
            );
            assert!(gaps.iter().any(|gap| gap.evidence().ends_with("Debit")));
            assert!(gaps.iter().any(|gap| gap.evidence().ends_with("Credit")));
            assert!(gaps.iter().all(|gap| gap.verifier() == verifier));
        }
    }
}

// Needs both features linked in together: the doc comment below assumes
// auto-discovery finds all three verifiers on the std leaf, which is
// only true when `verus` is also on. See the comment above
// `bool_proof_chain_is_a_single_root_node_with_all_three_verifiers` for
// why no justfile recipe currently exercises this combination.
#[test]
#[cfg_attr(not(all(feature = "creusot", feature = "verus")), ignore)]
fn calculation_chain_with_no_verifier_filter_is_also_incomplete() {
    // Auto-discovery finds kani, creusot, and verus (all present on the
    // std leaf); AddEvidence/Debit/Credit only ever proved kani, so the
    // unscoped lookup must refuse to return a report rather than quietly
    // showing a chain that looks uniformly proven.
    let result = amenable::proof_chain("AddEvidence");
    assert!(
        matches!(
            &result,
            Err(error) if matches!(&**error.kind(), ChainErrorKind::Incomplete(_))
        ),
        "expected ChainError::Incomplete, got {result:?}"
    );
    // The assert! above already fails the test if this shape doesn't
    // hold, so this nested `if let` only ever runs once it's
    // established -- no `else` diverging arm needed.
    if let Err(error) = &result
        && let ChainErrorKind::Incomplete(source) = &**error.kind()
    {
        let required = source.required();
        assert_eq!(required.len(), 3);
        assert!(required.iter().any(|v| v == "kani"));
        assert!(required.iter().any(|v| v == "creusot"));
        assert!(required.iter().any(|v| v == "verus"));
    }
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn buf_reader_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<BufReader<&'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<BufReader<&'static [u8]>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_buf_reader_reads_the_underlying_bytes"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.BufReader.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn buf_writer_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<BufWriter<Vec<u8>>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<BufWriter<Vec<u8>>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_buf_writer_flushes_to_the_underlying_writer"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.BufWriter.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_bytes_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Bytes<&'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Bytes<&'static [u8]>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_bytes_yields_one_byte_at_a_time"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Bytes.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_chain_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(
        &report,
        "RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
    );
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_chain_reads_the_first_source_then_the_second"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Chain.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn cursor_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Cursor<&'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Cursor<&'static [u8]>>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_cursor_read_advances_position_and_seek_repositions_it")
    }));
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Cursor.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_empty_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Empty>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Empty>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_empty_read_reports_end_of_file"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Empty.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Error>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Error>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_error_from_error_kind_preserves_the_kind"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Error.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn into_inner_error_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>"))?;

    assert_root_has_kani_and_creusot(
        &report,
        "RustStdStandard<IntoInnerError<BufWriter<Vec<u8>>>>",
    );
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_into_inner_error_recovers_the_writer_and_the_flush_error")
    }));
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/io/struct.IntoInnerError.html")
    }));
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_slice_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<IoSlice<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<IoSlice<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_io_slice_derefs_to_the_wrapped_bytes"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.IoSlice.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_slice_mut_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<IoSliceMut<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<IoSliceMut<'static>>");
    assert!(proof_description(&report, "kani").is_some_and(|d| {
        d.contains("verify_io_slice_mut_derefs_to_and_permits_mutating_the_wrapped_bytes")
    }));
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.IoSliceMut.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn line_writer_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<LineWriter<Vec<u8>>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<LineWriter<Vec<u8>>>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_line_writer_flushes_on_a_newline_but_not_before_one")
        )
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.LineWriter.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_lines_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Lines<&'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Lines<&'static [u8]>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_lines_splits_on_newlines_and_drops_the_terminator"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Lines.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn pipe_reader_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PipeReader>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<PipeReader>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_pipe_reader_reads_what_the_paired_writer_wrote"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.PipeReader.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn pipe_writer_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<PipeWriter>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<PipeWriter>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_pipe_writer_writes_arrive_at_the_paired_reader"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.PipeWriter.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_repeat_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Repeat>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Repeat>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_repeat_fills_the_buffer_with_the_given_byte"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Repeat.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn sink_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Sink>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Sink>");
    assert!(
        proof_description(&report, "kani").is_some_and(
            |d| d.contains("verify_sink_write_reports_full_length_and_discards_content")
        )
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Sink.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn io_split_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Split<&'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Split<&'static [u8]>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_split_segments_on_the_given_byte_and_drops_it"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Split.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stderr_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Stderr>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Stderr>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Stderr.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Stderr.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stderr_lock_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<StderrLock<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<StderrLock<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.StderrLock.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.StderrLock.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stdin_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Stdin>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Stdin>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Stdin.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Stdin.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stdin_lock_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<StdinLock<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<StdinLock<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.StdinLock.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.StdinLock.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stdout_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<Stdout>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<Stdout>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Stdout.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Stdout.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn stdout_lock_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<StdoutLock<'static>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<StdoutLock<'static>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.StdoutLock.html"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.StdoutLock.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn take_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<std::io::Take<&'static [u8]>>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<std::io::Take<&'static [u8]>>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_take_caps_reads_at_the_remaining_limit"))
    );
    assert!(
        proof_description(&report, "creusot")
            .is_some_and(|d| d.contains("https://doc.rust-lang.org/std/io/struct.Take.html"))
    );
    Ok(())
}

#[test]
#[cfg_attr(not(feature = "creusot"), ignore)]
fn writer_panicked_proof_chain_reports_the_kani_and_creusot_harnesses() -> miette::Result<()> {
    #[rustfmt::skip]
    let report = support::chain(amenable::proof_chain("RustStdStandard<WriterPanicked>"))?;

    assert_root_has_kani_and_creusot(&report, "RustStdStandard<WriterPanicked>");
    assert!(
        proof_description(&report, "kani")
            .is_some_and(|d| d.contains("verify_writer_panicked_recovers_the_buffered_data"))
    );
    assert!(proof_description(&report, "creusot").is_some_and(|d| {
        d.contains("https://doc.rust-lang.org/std/io/struct.WriterPanicked.html")
    }));
    Ok(())
}
