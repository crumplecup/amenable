//! Kani-only accommodation model for process-argument semantics.
//!
//! This module is where Amenable stops asking Kani to reason directly about the
//! verifier's synthetic process state and instead proves against a small
//! package of explicit argv laws that a real process is expected to refine.
//!
//! The direct `std::env::args()` / `args_os()` path remains preserved in the
//! proof gallery as a model-mismatch false trail. Production proofs that use
//! this model are therefore conditional:
//!
//! - if the real process argv conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

#[cfg(kani)]
use crate::KaniCompose;
#[cfg(kani)]
use crate::compose::{kani_assume, symbolic_any};

/// Modeled process argv with one guaranteed program slot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniArgv {
    program_slot: String,
    extra_count: u8,
}

impl KaniArgv {
    /// Construct modeled argv from its required program slot and extra args.
    pub fn new(program_slot: String, extra_count: u8) -> Self {
        Self {
            program_slot,
            extra_count,
        }
    }

    /// Report the guaranteed first argv slot.
    pub fn program_slot(&self) -> &str {
        &self.program_slot
    }

    /// Report how many extra argument slots follow the program slot.
    pub fn extra_count(&self) -> u8 {
        self.extra_count
    }

    /// Report the total slot count in the UTF-8 argv view.
    pub fn args_count(&self) -> usize {
        1 + usize::from(self.extra_count)
    }

    /// Report the total slot count in the `OsString` argv view.
    pub fn args_os_count(&self) -> usize {
        self.args_count()
    }
}

#[cfg(kani)]
impl KaniCompose for KaniArgv {
    fn kani_depth0() -> Self {
        Self::new(String::new(), 0)
    }

    fn kani_depth1() -> Self {
        Self::new("program".to_owned(), 0)
    }

    fn kani_depth2() -> Self {
        Self::new("program".to_owned(), 1)
    }

    fn kani_any() -> Self {
        let extra_count: u8 = symbolic_any();
        kani_assume(extra_count <= 2);
        Self::new(String::new(), extra_count)
    }
}
