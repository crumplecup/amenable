//! Kani-only accommodation model for anonymous pipe semantics.
//!
//! This module is where Amenable stops asking Kani to execute libc-backed
//! anonymous-pipe creation directly and instead proves against a small package
//! of explicit byte-channel laws that the real implementation is expected to
//! refine.
//!
//! The direct `std::io::pipe()` path remains preserved in the proof gallery as
//! an unsupported `pipe2` boundary. Production proofs that use this model are
//! therefore conditional:
//!
//! - if the real std/libc path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

#[cfg(kani)]
use crate::KaniCompose;
#[cfg(kani)]
use crate::compose::{kani_assume, symbolic_any};
use crate::fd_model::KaniFd;

/// Modeled anonymous-pipe reader endpoint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniPipeReader(KaniFd);

/// Modeled anonymous-pipe writer endpoint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniPipeWriter(KaniFd);

/// Modeled anonymous pipe with a bounded byte buffer and explicit close state.
///
/// `reader`/`writer` getters stay hand-written: they return an owned
/// clone of a non-`Copy` type (`KaniPipeReader`/`KaniPipeWriter`), and
/// `derive_getters` has no action for that -- `#[getter(copy)]` is its
/// only owned-return option and requires genuine `Copy` (confirmed
/// against its own source: only skip/rename/copy exist).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaniPipe {
    reader: KaniPipeReader,
    writer: KaniPipeWriter,
    buffered: Vec<u8>,
    reader_open: bool,
    writer_open: bool,
}

impl KaniPipeReader {
    /// Report the reader's raw fd value.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn raw_fd(&self) -> i32 {
        self.0.raw_fd()
    }

    /// Report the shared modeled resource identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn resource_id(&self) -> u64 {
        self.0.resource_id()
    }

    /// Report whether the reader endpoint is live.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_live(&self) -> bool {
        self.0.is_live()
    }
}

impl KaniPipeWriter {
    /// Report the writer's raw fd value.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn raw_fd(&self) -> i32 {
        self.0.raw_fd()
    }

    /// Report the shared modeled resource identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn resource_id(&self) -> u64 {
        self.0.resource_id()
    }

    /// Report whether the writer endpoint is live.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_live(&self) -> bool {
        self.0.is_live()
    }
}

impl KaniPipe {
    /// Construct a fixed, deterministic open reader/writer pair backed by
    /// resource id 0 -- unlike [`Self::fresh`], no non-deterministic
    /// construction, so (unlike `fresh`) this has nothing Kani-specific
    /// about it and stays available outside `cfg(kani)`.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn minimal() -> Self {
        Self {
            reader: KaniPipeReader(KaniFd::live(0, 0)),
            writer: KaniPipeWriter(KaniFd::live(1, 0)),
            buffered: Vec::new(),
            reader_open: true,
            writer_open: true,
        }
    }

    /// Construct a fresh open reader/writer pair backed by one modeled resource.
    #[cfg(kani)]
    pub fn fresh() -> Self {
        let resource_id: u64 = symbolic_any();
        let reader_raw: u16 = symbolic_any();
        let writer_raw: u16 = symbolic_any();
        kani_assume(reader_raw != writer_raw);

        Self {
            reader: KaniPipeReader(KaniFd::live(i32::from(reader_raw), resource_id)),
            writer: KaniPipeWriter(KaniFd::live(i32::from(writer_raw), resource_id)),
            buffered: Vec::new(),
            reader_open: true,
            writer_open: true,
        }
    }

    /// Snapshot the reader endpoint.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn reader(&self) -> KaniPipeReader {
        self.reader.clone()
    }

    /// Snapshot the writer endpoint.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn writer(&self) -> KaniPipeWriter {
        self.writer.clone()
    }

    /// Report the shared modeled resource identity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn resource_id(&self) -> u64 {
        self.reader.resource_id()
    }

    /// Report whether the reader endpoint is modeled as open.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn reader_is_open(&self) -> bool {
        self.reader_open
    }

    /// Report whether the writer endpoint is modeled as open.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn writer_is_open(&self) -> bool {
        self.writer_open
    }

    /// Report the current buffered byte count.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn buffered_len(&self) -> usize {
        self.buffered.len()
    }

    /// Append bytes written by the paired writer.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self, writer)))]
    pub fn write_all(&mut self, writer: KaniPipeWriter, payload: Vec<u8>) {
        assert!(
            self.reader_open,
            "modeled writes require an open reader end"
        );
        assert!(
            self.writer_open,
            "modeled writes require an open writer end"
        );
        assert!(writer.is_live(), "modeled writes require a live writer");
        assert_eq!(
            writer.resource_id(),
            self.resource_id(),
            "writer must target this modeled pipe resource"
        );
        assert_eq!(
            writer.raw_fd(),
            self.writer.raw_fd(),
            "writer raw fd must match the paired writer endpoint"
        );

        self.buffered.extend(payload);
    }

    /// Close the writer endpoint while preserving buffered bytes for the reader.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, writer)))]
    pub fn close_writer(&mut self, writer: KaniPipeWriter) {
        assert!(self.writer_open, "writer cannot be closed twice");
        assert_eq!(
            writer.resource_id(),
            self.resource_id(),
            "writer must target this modeled pipe resource"
        );
        assert_eq!(
            writer.raw_fd(),
            self.writer.raw_fd(),
            "writer raw fd must match the paired writer endpoint"
        );

        self.writer_open = false;
    }

    /// Drain all buffered bytes from the reader once the writer has closed.
    #[cfg_attr(not(kani), tracing::instrument(level = "info", skip(self, reader)))]
    pub fn read_to_end(&mut self, reader: KaniPipeReader) -> Vec<u8> {
        assert!(self.reader_open, "modeled reads require an open reader end");
        assert!(reader.is_live(), "modeled reads require a live reader");
        assert_eq!(
            reader.resource_id(),
            self.resource_id(),
            "reader must target this modeled pipe resource"
        );
        assert_eq!(
            reader.raw_fd(),
            self.reader.raw_fd(),
            "reader raw fd must match the paired reader endpoint"
        );
        assert!(
            !self.writer_open,
            "modeled read_to_end assumes the writer has closed so EOF is reachable"
        );

        let delivered = self.buffered.clone();
        self.buffered.clear();
        delivered
    }
}

#[cfg(kani)]
impl KaniCompose for KaniPipeReader {
    fn kani_depth0() -> Self {
        KaniPipe::kani_depth0().reader()
    }

    fn kani_depth1() -> Self {
        KaniPipe::kani_depth1().reader()
    }

    fn kani_depth2() -> Self {
        KaniPipe::kani_depth2().reader()
    }

    fn kani_any() -> Self {
        KaniPipe::kani_any().reader()
    }
}

#[cfg(kani)]
impl KaniCompose for KaniPipeWriter {
    fn kani_depth0() -> Self {
        KaniPipe::kani_depth0().writer()
    }

    fn kani_depth1() -> Self {
        KaniPipe::kani_depth1().writer()
    }

    fn kani_depth2() -> Self {
        KaniPipe::kani_depth2().writer()
    }

    fn kani_any() -> Self {
        KaniPipe::kani_any().writer()
    }
}

#[cfg(kani)]
impl KaniCompose for KaniPipe {
    fn kani_depth0() -> Self {
        Self::minimal()
    }

    fn kani_depth1() -> Self {
        let mut pipe = Self::kani_depth0();
        pipe.buffered.push(0);
        pipe
    }

    fn kani_depth2() -> Self {
        let mut pipe = Self::kani_depth0();
        pipe.buffered.push(0);
        pipe.buffered.push(0);
        pipe
    }

    fn kani_any() -> Self {
        let mut pipe = Self::fresh();
        pipe.buffered = <Vec<u8> as KaniCompose>::kani_any();
        pipe
    }
}
