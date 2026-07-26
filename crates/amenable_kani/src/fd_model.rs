//! Kani-only accommodation model for Unix file-descriptor semantics.
//!
//! This module is where Amenable stops asking Kani to execute libc-backed
//! descriptor machinery directly and instead proves against a small package of
//! explicit laws that the real implementation is expected to refine.
//!
//! The direct `BorrowedFd::try_clone_to_owned()` path remains preserved in the
//! proof gallery as an unsupported `fcntl` boundary. Production proofs that use
//! this model are therefore conditional:
//!
//! - if the real std/libc path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

use crate::KaniCompose;
use crate::compose::{kani_assume, symbolic_any};

/// Modeled owned Unix file descriptor.
///
/// Field meaning, in order:
///
/// - raw fd value
/// - liveness flag
/// - modeled resource identity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFd(i32, bool, u64);

/// Modeled borrowed Unix file descriptor.
///
/// This carries the same observable facts as [`KaniFd`] but without ownership.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniBorrowedFd(i32, bool, u64);

/// Modeled file owner whose descriptor can be transferred into an owned fd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFile(KaniFd);

impl KaniFd {
    /// Construct a live modeled descriptor.
    pub fn live(raw_fd: i32, resource_id: u64) -> Self {
        assert!(raw_fd >= 0, "live modeled fds must be non-negative");
        Self(raw_fd, true, resource_id)
    }

    /// Construct a dead modeled descriptor.
    pub fn dead(resource_id: u64) -> Self {
        Self(-1, false, resource_id)
    }

    /// Bounded symbolic live descriptor representative.
    pub fn kani_live_any() -> Self {
        let raw_fd: u16 = symbolic_any();
        let resource_id: u64 = symbolic_any();
        Self::live(i32::from(raw_fd), resource_id)
    }

    /// Report the raw fd value.
    pub fn raw_fd(&self) -> i32 {
        self.0
    }

    /// Report whether the descriptor is modeled as live.
    pub fn is_live(&self) -> bool {
        self.1
    }

    /// Report the modeled resource identity.
    pub fn resource_id(&self) -> u64 {
        self.2
    }

    /// Borrow the descriptor without changing its observable identity.
    pub fn borrow(&self) -> KaniBorrowedFd {
        assert!(self.is_live(), "only live descriptors can be borrowed");
        KaniBorrowedFd(self.raw_fd(), self.is_live(), self.resource_id())
    }
}

impl KaniBorrowedFd {
    /// Report the raw fd value.
    pub fn raw_fd(&self) -> i32 {
        self.0
    }

    /// Report whether the descriptor is modeled as live.
    pub fn is_live(&self) -> bool {
        self.1
    }

    /// Report the modeled resource identity.
    pub fn resource_id(&self) -> u64 {
        self.2
    }

    /// Duplicate the borrowed descriptor into a fresh owned descriptor.
    ///
    /// The duplicate is assumed to stay live and refer to the same underlying
    /// resource, but it may carry a different raw fd value.
    pub fn duplicate_as_owned_with_raw(&self, duplicated_raw_fd: i32) -> KaniFd {
        assert!(
            self.is_live(),
            "only live borrowed descriptors can be duplicated"
        );
        kani_assume(duplicated_raw_fd >= 0);
        KaniFd::live(duplicated_raw_fd, self.resource_id())
    }
}

impl KaniFile {
    /// Construct a file owner from an owned descriptor.
    pub fn from_owned_fd(fd: KaniFd) -> Self {
        assert!(fd.is_live(), "modeled files require a live descriptor");
        Self(fd)
    }

    /// Report the raw fd value the file owner exposes.
    pub fn raw_fd(&self) -> i32 {
        self.0.raw_fd()
    }

    /// Report the modeled resource identity.
    pub fn resource_id(&self) -> u64 {
        self.0.resource_id()
    }

    /// Report whether the file is modeled as live.
    pub fn is_live(&self) -> bool {
        self.0.is_live()
    }

    /// Transfer ownership into an owned fd without reopening anything.
    pub fn into_owned_fd(self) -> KaniFd {
        self.0
    }
}

impl KaniCompose for KaniFd {
    fn kani_depth0() -> Self {
        Self::live(0, 0)
    }

    fn kani_depth1() -> Self {
        Self::live(1, 1)
    }

    fn kani_depth2() -> Self {
        Self::live(2, 2)
    }

    fn kani_any() -> Self {
        let live: bool = symbolic_any();
        let resource_id: u64 = symbolic_any();
        if live {
            let raw_fd: u16 = symbolic_any();
            Self::live(i32::from(raw_fd), resource_id)
        } else {
            Self::dead(resource_id)
        }
    }
}

impl KaniCompose for KaniFile {
    fn kani_depth0() -> Self {
        Self::from_owned_fd(KaniFd::kani_depth0())
    }

    fn kani_depth1() -> Self {
        Self::from_owned_fd(KaniFd::kani_depth1())
    }

    fn kani_depth2() -> Self {
        Self::from_owned_fd(KaniFd::kani_depth2())
    }

    fn kani_any() -> Self {
        Self::from_owned_fd(KaniFd::kani_live_any())
    }
}
