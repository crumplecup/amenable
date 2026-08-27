use std::process::Command;

use miette::{IntoDiagnostic, WrapErr};

#[test]
fn gallery_list_reports_registered_cases_with_disposition_and_expectation() -> miette::Result<()> {
    amenable::init_tracing();
    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args(["gallery", "list"])
        .output()
        .into_diagnostic()
        .wrap_err("gallery list CLI should start")?;

    assert!(
        output.status.success(),
        "gallery list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout)
        .into_diagnostic()
        .wrap_err("gallery list should be UTF-8")?;
    assert!(stdout.contains(
        "amenable_kani::gallery::vacuity::assume_false_is_vacuous_pass [false_trail / passed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::vacuity::explicit_contradiction_fails [hypothesis / failed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::vacuity::bounded_assumption_passes_nonvacuously [best_practice / passed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::iter_materialization::flatten_collect_times_out [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::iter_materialization::flatten_incremental_next_passes [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::iter_materialization::flatten_incremental_fixed_lengths_passes [hypothesis / passed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::string_drain::single_char_collect_times_out [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::string_drain::single_char_incremental_next_passes [hypothesis / passed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::btree_map_symbolic_iteration_times_out_in_the_direct_std_path [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::linked_list_extract_if_times_out_even_with_incremental_observation [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::backtrace_force_capture_reaches_unsupported_foreign_boundary [false_trail / failed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::catch_unwind_reaches_an_unsupported_panic_boundary [false_trail / failed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::env_args_process_invariant_fails_under_the_synthetic_kani_model [false_trail / failed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::join_paths_unjoinable_path_times_out_in_the_direct_std_path [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_direct_std_path [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_first_concrete_string_model [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::default_hasher_determinism_times_out_in_the_direct_std_path [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario [false_trail / timeout]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::mutex_poisoning_reaches_the_unsupported_catch_unwind_boundary [false_trail / failed]"
    ));
    assert!(stdout.contains(
        "amenable_kani::gallery::replace_recommendations::format_arguments_rendering_times_out_in_the_direct_std_path [false_trail / timeout]"
    ));
    Ok(())
}

#[test]
fn gallery_run_rejects_unknown_case_before_spawning_kani() -> miette::Result<()> {
    amenable::init_tracing();
    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "gallery",
            "run",
            "--case",
            "amenable_kani::gallery::missing",
        ])
        .output()
        .into_diagnostic()
        .wrap_err("gallery run CLI should start")?;

    assert!(!output.status.success(), "unknown gallery case must fail");
    let stderr = String::from_utf8(output.stderr)
        .into_diagnostic()
        .wrap_err("gallery stderr should be UTF-8")?;
    // Asserted as separate substrings rather than one long string: miette
    // wraps its rendered diagnostic across lines, so a single combined
    // assertion would be brittle against wrap width.
    assert!(
        stderr.contains("amenable::Invariant"),
        "stderr should carry the Invariant diagnostic code: {stderr}"
    );
    assert!(
        stderr.contains("unknown Kani proof-gallery case ID"),
        "stderr: {stderr}"
    );
    assert!(
        stderr.contains("amenable_kani::gallery::missing"),
        "stderr: {stderr}"
    );
    Ok(())
}
