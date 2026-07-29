use amenable::{
    KaniGalleryCase, KaniGalleryDisposition, KaniGalleryExpectation, KaniGalleryRegistration,
};

#[test]
fn kani_gallery_cases_self_register_with_stable_ids_and_expectations() {
    let cases: Vec<KaniGalleryCase> = inventory::iter::<KaniGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();

    assert!(
        cases.len() >= 21,
        "the proof gallery should include scaffold, iterator, filesystem, slice-split, string-drain, and replace-issue cases"
    );
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::vacuity::assume_false_is_vacuous_pass"
            && case.harness == "gallery::vacuity::assume_false_is_vacuous_pass"
            && case.package == "amenable_kani"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Passed
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::vacuity::explicit_contradiction_fails"
            && case.disposition == KaniGalleryDisposition::Hypothesis
            && case.expected == KaniGalleryExpectation::Failed
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::vacuity::bounded_assumption_passes_nonvacuously"
            && case.disposition == KaniGalleryDisposition::BestPractice
            && case.expected == KaniGalleryExpectation::Passed
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::iter_materialization::flatten_collect_times_out"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::iter_materialization::flatten_incremental_next_passes"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::iter_materialization::flatten_incremental_fixed_lengths_passes"
            && case.disposition == KaniGalleryDisposition::Hypothesis
            && case.expected == KaniGalleryExpectation::Passed
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::filesystem_observation_granularity::generic_filesystem_state_machine_times_out_for_create_new"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::filesystem_observation_granularity::single_path_create_new_observation_passes"
            && case.disposition == KaniGalleryDisposition::BestPractice
            && case.expected == KaniGalleryExpectation::Passed
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::slice_split_position::bounded_split_observation_passes"
            && case.disposition == KaniGalleryDisposition::BestPractice
            && case.expected == KaniGalleryExpectation::Passed
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::string_drain::single_char_collect_times_out"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id == "amenable_kani::gallery::string_drain::single_char_incremental_next_passes"
            && case.disposition == KaniGalleryDisposition::Hypothesis
            && case.expected == KaniGalleryExpectation::Passed
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::btree_map_symbolic_iteration_times_out_in_the_direct_std_path"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::linked_list_extract_if_times_out_even_with_incremental_observation"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::backtrace_force_capture_reaches_unsupported_foreign_boundary"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Failed
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::catch_unwind_reaches_an_unsupported_panic_boundary"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Failed
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::env_args_process_invariant_fails_under_the_synthetic_kani_model"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Failed
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::join_paths_unjoinable_path_times_out_in_the_direct_std_path"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_direct_std_path"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_first_concrete_string_model"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::default_hasher_determinism_times_out_in_the_direct_std_path"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::mutex_poisoning_reaches_the_unsupported_catch_unwind_boundary"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Failed
    }));
    assert!(cases.iter().any(|case| {
        case.id
            == "amenable_kani::gallery::replace_recommendations::format_arguments_rendering_times_out_in_the_direct_std_path"
            && case.disposition == KaniGalleryDisposition::FalseTrail
            && case.expected == KaniGalleryExpectation::Timeout
    }));

    let mut ids: Vec<_> = cases.iter().map(|case| case.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), cases.len(), "gallery identifiers must be unique");
}
