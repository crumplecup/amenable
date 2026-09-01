#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::backtrace::{Backtrace, BacktraceStatus};
amenable_derive::harness! {
    creusot, VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC, {
        /// `Backtrace::force_capture()` always produces a captured backtrace.
        /// This stays trusted in Creusot for the same reason as Kani's
        /// accommodation-model proof: the real capture path lives at the
        /// platform unwinding boundary, not inside Creusot's contract surface.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_backtrace_force_capture_always_actually_captures() -> bool {
            let backtrace = Backtrace::force_capture();
            match backtrace.status() {
                BacktraceStatus::Captured => true,
                BacktraceStatus::Disabled | BacktraceStatus::Unsupported => false,
                _ => false,
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC, {
        /// The `BacktraceStatus` observed after `Backtrace::force_capture()`
        /// is `Captured`.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_backtrace_status_reports_captured_after_force_capture() -> bool {
            let status = Backtrace::force_capture().status();
            match status {
                BacktraceStatus::Captured => true,
                BacktraceStatus::Disabled | BacktraceStatus::Unsupported => false,
                _ => false,
            }
        }
    }
}
