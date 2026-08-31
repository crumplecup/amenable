//! `KaniWitness` impls for `std::io`.
//!
//! `Bytes` and the simple `BufWriter` flush law still verify directly over
//! in-memory `&'static [u8]` / `Vec<u8>` readers and writers, as do the
//! zero-state adapters `Empty`, `Repeat`, `SeekFrom`, and `Sink`: none of
//! them carry internal buffering state, so there's nothing for Kani to
//! time out on. The rest of the
//! buffered `std::io` family (`BufReader`, `IntoInnerError`, `LineWriter`,
//! `Lines`, `Split`, `WriterPanicked`) uses Amenable-owned bounded
//! observations instead: these direct std paths are pure in-memory, but still
//! time out or hit unsupported panic capture under Kani. `Stdin`/`Stdout`/
//! `Stderr` and their lock guards are process-attached global handles with no
//! checkable invariant beyond what the type system already guarantees
//! (exercising them for real would mean writing to, or blocking on, the
//! actual process's standard streams during the proof), so those six stay
//! "trusted." The direct `PipeReader`/`PipeWriter` setup path reaches
//! unsupported `pipe2` under Kani, so the production proofs use an
//! Amenable-owned anonymous-pipe model instead; the direct std path remains
//! preserved in the gallery as a false trail.
//!
//! Split by the real API family each file covers: [`buffered_reader_writer`]
//! (`BufReader`, `BufWriter`, `Bytes`, `IntoInnerError`),
//! [`line_writer_and_lines`] (`LineWriter`, `Lines`, plus the
//! `ByteIsDistinctFromTheMarker`/`ByteIsAsciiAndNotALineTerminator`
//! markers), [`pipe_and_split`] (`PipeReader`, `PipeWriter`, `Split`),
//! [`writer_panicked_and_empty_repeat`] (`WriterPanicked`, `Empty`,
//! `Repeat`), [`seek_sink_chain`] (`SeekFrom`, `Sink`, `Chain`),
//! [`cursor_error`] (`Cursor`, `Error`), and [`error_kind_and_io_slice`]
//! (the `ErrorKindMatchesExpected` marker, `IoSlice`, `IoSliceMut`, `Take`).

mod buffered_reader_writer;
mod cursor_error;
mod error_kind_and_io_slice;
mod line_writer_and_lines;
mod pipe_and_split;
mod seek_sink_chain;
mod writer_panicked_and_empty_repeat;
