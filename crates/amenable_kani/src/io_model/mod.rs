//! Kani-only accommodation models for the buffered `std::io` proof family.
//!
//! The direct `std::io` paths for `BufReader`, `IntoInnerError`, `LineWriter`,
//! `Lines`, and `Split` are all pure in-memory, but still expand into enough
//! buffered-reader / buffered-writer machinery to time out under Kani. The
//! `WriterPanicked` path adds an unsupported `catch_unwind` boundary on top of
//! that. This module captures the bounded observable laws the production proofs
//! actually claim, so the verifier checks the semantic contract rather than the
//! entire std implementation.
//!
//! Split by the `(Window, Observation)` pair each file covers:
//! [`buffered_read_and_flush_error`], [`line_writer_and_lines`], and
//! [`buf_read_split_and_writer_panicked`].

mod buf_read_split_and_writer_panicked;
mod buffered_read_and_flush_error;
mod line_writer_and_lines;

pub use buf_read_split_and_writer_panicked::{
    KaniBufReadSplitObservation, KaniBufReadSplitObservationBuilder, KaniWriterPanickedObservation,
};
pub use buffered_read_and_flush_error::{KaniBufferedReadObservation, KaniFlushErrorObservation};
pub use line_writer_and_lines::{KaniLineWriterObservation, KaniLinesObservation};
