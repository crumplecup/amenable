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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn live(raw_fd: i32, resource_id: u64) -> Self {
        assert!(raw_fd >= 0, "live modeled fds must be non-negative");
        Self(raw_fd, true, resource_id)
    }

    /// Construct a dead modeled descriptor.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn dead(resource_id: u64) -> Self {
        Self(-1, false, resource_id)
    }

    /// Report the raw fd value.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn raw_fd(&self) -> i32 {
        self.0
    }

    /// Report whether the descriptor is modeled as live.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_live(&self) -> bool {
        self.1
    }

    /// Report the modeled resource identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn resource_id(&self) -> u64 {
        self.2
    }

    /// Borrow the descriptor without changing its observable identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn borrow(&self) -> KaniBorrowedFd {
        assert!(self.is_live(), "only live descriptors can be borrowed");
        KaniBorrowedFd(self.raw_fd(), self.is_live(), self.resource_id())
    }
}

impl KaniBorrowedFd {
    /// Report the raw fd value.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn raw_fd(&self) -> i32 {
        self.0
    }

    /// Report whether the descriptor is modeled as live.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_live(&self) -> bool {
        self.1
    }

    /// Report the modeled resource identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn resource_id(&self) -> u64 {
        self.2
    }
}

impl KaniFile {
    /// Construct a file owner from an owned descriptor.
    pub fn from_owned_fd(fd: KaniFd) -> Self {
        assert!(fd.is_live(), "modeled files require a live descriptor");
        Self(fd)
    }

    /// Report the raw fd value the file owner exposes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn raw_fd(&self) -> i32 {
        self.0.raw_fd()
    }

    /// Report the modeled resource identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn resource_id(&self) -> u64 {
        self.0.resource_id()
    }

    /// Report whether the file is modeled as live.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_live(&self) -> bool {
        self.0.is_live()
    }

    /// Transfer ownership into an owned fd without reopening anything.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_owned_fd(self) -> KaniFd {
        self.0
    }
}

/// The `#[cfg(kani)]` imports, extra inherent methods, and `KaniCompose`
/// impls this file needs, consolidated into one gate on this `mod`
/// instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. No bridging re-export needed at all: inherent
/// methods (`KaniFd::kani_live_any`, `KaniBorrowedFd::
/// duplicate_as_owned_with_raw`) attach to their type regardless of which
/// module the `impl` block lives in, so `rust_std::os_unix`'s own
/// `crate::KaniFd::kani_live_any()` call keeps resolving unchanged; trait
/// impls (`KaniCompose for KaniFd`/`for KaniFile`) are likewise globally
/// visible the moment they're compiled.
#[cfg(kani)]
mod mirror {
    use crate::KaniCompose;
    use crate::compose::{kani_assume, symbolic_any};

    use super::{KaniBorrowedFd, KaniFd, KaniFile};

    impl KaniFd {
        /// Bounded symbolic live descriptor representative.
        pub fn kani_live_any() -> Self {
            let raw_fd: u16 = symbolic_any();
            let resource_id: u64 = symbolic_any();
            Self::live(i32::from(raw_fd), resource_id)
        }
    }

    impl KaniBorrowedFd {
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
}
