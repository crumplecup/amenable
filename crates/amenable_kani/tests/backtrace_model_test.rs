use amenable_kani::{KaniBacktrace, KaniBacktraceStatus};

#[test]
fn force_capture_always_reports_captured() {
    let backtrace = KaniBacktrace::force_capture();

    assert_eq!(backtrace.status(), KaniBacktraceStatus::Captured);
}

// composed_statuses_cover_the_modeled_status_space moved to a real
// #[kani::proof] harness in backtrace_model.rs's own `mod proofs`: it
// was testing KaniCompose's own contract (a Kani-only trait -- see
// docs/KANI_COMPOSE_PLAN.md's "Scope Correction"), not KaniBacktraceStatus
// itself, so it belongs with the other KaniCompose self-tests, not here.
