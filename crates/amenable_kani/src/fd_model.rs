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

use amenable_derive::Standard;
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};

#[cfg(kani)]
use crate::KaniCompose;
#[cfg(kani)]
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

/// A raw Unix file descriptor known to be non-negative while live.
///
/// "A live raw fd is never negative" is independently hand-written at four
/// real sites in this crate — [`KaniFd::live`]'s own defensive assertion,
/// `rust_std::os_unix::verify_borrowed_fd_reports_the_same_raw_value_as_the_owner`,
/// and two gallery diagnostics in `gallery::replace_recommendations` — with
/// nothing tying them together. Unlike [`ValidUnicodeScalar`]
/// (`amenable_std`), this bound has no Creusot or Verus coverage yet, so
/// only `Ensures<KaniVerifier>` is implemented (in `rust_std::os_unix`,
/// alongside the harness it reuses) — it stays local to this crate rather
/// than moving to `amenable_std` until a second backend actually needs it.
///
/// [`ValidUnicodeScalar`]: amenable_std::ValidUnicodeScalar
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct NonNegativeFd {
    value: i32,
}

impl NonNegativeFd {
    /// Wrap an `i32` already known to be a non-negative, live raw fd value.
    pub const fn new(value: i32) -> Self {
        Self { value }
    }

    /// The wrapped raw fd value.
    pub const fn value(&self) -> i32 {
        self.value
    }
}

impl KaniFd {
    /// Construct a live modeled descriptor.
    ///
    /// The assertion below is the canonical home
    /// [`NonNegativeFd`] names — see that type for the same bound stated
    /// once, and its `Ensures<KaniVerifier>` impl (in
    /// `rust_std::os_unix`) for this exact fragment held as a reusable,
    /// backend-checkable claim.
    pub fn live(raw_fd: i32, resource_id: u64) -> Self {
        assert!(raw_fd >= 0, "live modeled fds must be non-negative");
        Self(raw_fd, true, resource_id)
    }

    /// Construct a dead modeled descriptor.
    pub fn dead(resource_id: u64) -> Self {
        Self(-1, false, resource_id)
    }

    /// Bounded symbolic live descriptor representative.
    #[cfg(kani)]
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
    #[cfg(kani)]
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

#[cfg(kani)]
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

#[cfg(kani)]
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
