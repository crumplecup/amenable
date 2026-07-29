use amenable_kani::{KaniCallerLocationObservation, KaniPanicHookObservation};

#[test]
fn caller_location_observation_keeps_the_shared_file_and_distinct_lines() {
    let observation = KaniCallerLocationObservation::same_file_distinct_lines("proof.rs", 10, 11);

    assert_eq!(observation.file(), "proof.rs");
    assert_eq!(observation.first_line(), 10);
    assert_eq!(observation.second_line(), 11);
    assert!(observation.lines_differ());
}

#[test]
fn panic_hook_observation_keeps_the_panic_message() {
    let observation = KaniPanicHookObservation::message("captured panic message");

    assert_eq!(observation.captured_message(), "captured panic message");
}
