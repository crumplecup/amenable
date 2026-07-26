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

use crate::KaniCompose;
use crate::compose::{kani_assume, symbolic_any};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniBacktrace {
    status: KaniBacktraceStatus,
}

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

impl KaniBacktrace {
    /// Construct a modeled backtrace with the provided status.
    pub fn new(status: KaniBacktraceStatus) -> Self {
        Self { status }
    }

    /// Model `Backtrace::force_capture()` as a guaranteed captured backtrace.
    pub fn force_capture() -> Self {
        Self::new(KaniBacktraceStatus::Captured)
    }

    /// Report the modeled backtrace status.
    pub fn status(&self) -> KaniBacktraceStatus {
        self.status
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
