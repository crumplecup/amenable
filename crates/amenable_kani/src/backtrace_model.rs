//! Kani-only accommodation model for forced backtrace-capture semantics.
//!
//! This module is where Amenable stops asking Kani to execute platform
//! unwinding directly and instead proves against a small package of explicit
//! status laws that the real implementation is expected to refine.
//!
//! The direct `Backtrace::force_capture()` path remains preserved in the proof
//! gallery as an unsupported `_Unwind_Backtrace` boundary. Production proofs
//! that use this model are therefore conditional:
//!
//! - if the real backtrace capture path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

/// Modeled backtrace status observed from Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KaniBacktraceStatus {
    /// The modeled capture is disabled.
    Disabled,
    /// The modeled capture succeeded.
    Captured,
    /// The modeled capture is unsupported.
    Unsupported,
}

/// Modeled backtrace whose observable property is its status.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_getters::Getters,
    derive_new::new,
)]
pub struct KaniBacktrace {
    /// The modeled backtrace status.
    #[getter(copy)]
    status: KaniBacktraceStatus,
}

impl KaniBacktrace {
    /// Model `Backtrace::force_capture()` as a guaranteed captured backtrace.
    pub fn force_capture() -> Self {
        Self::new(KaniBacktraceStatus::Captured)
    }
}

/// The `#[cfg(kani)]` imports, extra inherent method, and `KaniCompose`
/// impls this file needs, consolidated into one gate on this `mod`
/// instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. No bridging re-export needed: `from_index` is
/// private and only ever called from `kani_any`, right beside it here;
/// the `KaniCompose` impls are globally visible the moment they're
/// compiled.
#[cfg(kani)]
mod mirror {
    pub(super) use crate::KaniCompose;
    use crate::compose::{kani_assume, symbolic_any};

    use super::{KaniBacktrace, KaniBacktraceStatus};

    impl KaniBacktraceStatus {
        fn from_index(index: u8) -> Self {
            match index {
                0 => Self::Disabled,
                1 => Self::Captured,
                2 => Self::Unsupported,
                _ => unreachable!("bounded backtrace status index"),
            }
        }
    }

    impl KaniCompose for KaniBacktraceStatus {
        fn kani_depth0() -> Self {
            Self::Disabled
        }

        fn kani_depth1() -> Self {
            Self::Captured
        }

        fn kani_depth2() -> Self {
            Self::Unsupported
        }

        fn kani_any() -> Self {
            let index: u8 = symbolic_any();
            kani_assume(index <= 2);
            Self::from_index(index)
        }
    }

    impl KaniCompose for KaniBacktrace {
        fn kani_depth0() -> Self {
            Self::new(KaniBacktraceStatus::kani_depth0())
        }

        fn kani_depth1() -> Self {
            Self::force_capture()
        }

        fn kani_depth2() -> Self {
            Self::new(KaniBacktraceStatus::kani_depth2())
        }

        fn kani_any() -> Self {
            Self::new(KaniBacktraceStatus::kani_any())
        }
    }
}
#[cfg(kani)]
use mirror::KaniCompose;

// Self-test of KaniCompose's own contract for KaniBacktraceStatus, not a
// production proof -- same reasoning as compose.rs's own `mod proofs`:
// KaniCompose is Kani-only modeling infrastructure (see
// docs/KANI_COMPOSE_PLAN.md), so a claim about what its depth
// constructors return belongs here, not in an ordinary #[test].
#[cfg(kani)]
mod proofs {
    use amenable_core::Ensures;

    use super::{KaniBacktraceStatus, KaniCompose};
    use crate::CollectedSequenceMatchesExpected;

    amenable_derive::harness! {
        kani, VERIFY_KANI_COMPOSE_BACKTRACE_STATUS_DEPTHS_SRC, {
            #[kani::proof]
            fn verify_kani_compose_backtrace_status_depths() {
                assert!(CollectedSequenceMatchesExpected::ensures((
                    KaniBacktraceStatus::kani_depth0(),
                    KaniBacktraceStatus::Disabled,
                )));
                assert!(CollectedSequenceMatchesExpected::ensures((
                    KaniBacktraceStatus::kani_depth1(),
                    KaniBacktraceStatus::Captured,
                )));
                assert!(CollectedSequenceMatchesExpected::ensures((
                    KaniBacktraceStatus::kani_depth2(),
                    KaniBacktraceStatus::Unsupported,
                )));
            }
        }
    }
}
