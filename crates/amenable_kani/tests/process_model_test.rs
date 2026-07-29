use amenable_kani::{
    KaniChildObservation, KaniChildStderrObservation, KaniChildStdinObservation,
    KaniChildStdoutObservation, KaniCommandArgsObservation, KaniCommandEnvObservation,
    KaniCommandEnvsObservation, KaniExitStatusObservation, KaniOutputObservation,
    KaniStdioObservation,
};

#[test]
fn child_observation_keeps_process_id_and_waited_exit_code() {
    let observation = KaniChildObservation::waitable(7, 3);

    assert_eq!(observation.process_id(), 7);
    assert_eq!(observation.waited_exit_code(), Some(3));
}

#[test]
fn child_stream_observations_keep_modeled_text() {
    let stderr = KaniChildStderrObservation::captured("", "error message\n");
    let stdin = KaniChildStdinObservation::echo("hello, child\n", "hello, child\n");
    let stdout = KaniChildStdoutObservation::captured("hello\n");

    assert_eq!(stderr.stdout_text(), "");
    assert_eq!(stderr.stderr_text(), "error message\n");
    assert_eq!(stdin.echoed_stdout(), stdin.input_text());
    assert_eq!(stdout.stdout_text(), "hello\n");
}

#[test]
fn command_observations_keep_args_and_env_overrides() {
    let args = KaniCommandArgsObservation::configured(["a", "b"]);
    let visible_env = KaniCommandEnvObservation::visible_override(
        "AMENABLE_TEST_VAR",
        "configured-value",
        "configured-value",
    );
    let stored_env = KaniCommandEnvsObservation::configured_override("SOME_KEY", "some_value");

    assert_eq!(args.args(), ["a", "b"]);
    assert_eq!(visible_env.key(), "AMENABLE_TEST_VAR");
    assert_eq!(visible_env.visible_stdout(), visible_env.value());
    assert_eq!(stored_env.key(), "SOME_KEY");
    assert_eq!(stored_env.value(), "some_value");
}

#[test]
fn exit_output_and_stdio_observations_keep_modeled_status_laws() {
    let exit_status = KaniExitStatusObservation::nonzero(3);
    let output = KaniOutputObservation::captured(0, "hello\n");
    let stdio = KaniStdioObservation::stdout_handle_policy(false, true);

    assert!(!exit_status.success());
    assert_eq!(exit_status.code(), Some(3));
    assert!(output.success());
    assert_eq!(output.status_code(), Some(0));
    assert_eq!(output.stdout_text(), "hello\n");
    assert!(!stdio.null_stdout_handle_present());
    assert!(stdio.piped_stdout_handle_present());
}
