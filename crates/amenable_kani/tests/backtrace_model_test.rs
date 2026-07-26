use amenable_kani::{KaniBacktrace, KaniBacktraceStatus, KaniCompose};

#[test]
fn force_capture_always_reports_captured() {
    let backtrace = KaniBacktrace::force_capture();

    assert_eq!(backtrace.status(), KaniBacktraceStatus::Captured);
}

#[test]
fn composed_statuses_cover_the_modeled_status_space() {
    assert_eq!(
        KaniBacktraceStatus::kani_depth0(),
        KaniBacktraceStatus::Disabled
    );
    assert_eq!(
        KaniBacktraceStatus::kani_depth1(),
        KaniBacktraceStatus::Captured
    );
    assert_eq!(
        KaniBacktraceStatus::kani_depth2(),
        KaniBacktraceStatus::Unsupported
    );
}
