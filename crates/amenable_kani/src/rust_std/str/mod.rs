//! `KaniWitness` impls for `core::str`.
//!
//! `Bytes`/`CharIndices`/`Chars`/`EncodeUtf16` check a symbolic ASCII byte
//! (`byte as char` is a valid, safe conversion for `byte < 128`, giving a
//! genuinely symbolic single-character `str` without needing to construct
//! arbitrary `String` content). The escaping/splitting/UTF-8-validation
//! adapters instead check fixed representative examples: their behavior
//! turns on specific character classes (whitespace, control characters,
//! invalid byte sequences) that don't reduce to a single symbolic byte the
//! way plain iteration does.
//!
//! The `Pattern`-generic split/match family is monomorphized on `char`,
//! mirroring `amenable_std::rust_std::str`'s registration choice. Of the
//! 11 such types, `Split`/`SplitN`/`SplitInclusive` verify directly
//! against fixed representative strings via `.collect()`. The other 8
//! (`RSplit`, `RSplitN`, `SplitTerminator`, `RSplitTerminator`, `Matches`,
//! `RMatches`, `MatchIndices`, `RMatchIndices`) don't have a passing
//! direct proof for two distinct reasons, both still preserved as gallery
//! false trails:
//!
//! - **Reverse traversal** (`RSplit`/`RSplitN`/`RSplitTerminator`/
//!   `RMatches`/`RMatchIndices`): root-caused via an isolated probe --
//!   `CharSearcher::next_match_back` bottoms out in `memchr::memrchr`,
//!   whose internal scan loop CBMC can't bound even for a single `.next()`
//!   call on a five-byte fixed str (confirmed unwinding past 580
//!   iterations). Independent of proof style. See
//!   `gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`.
//! - **Forward `SplitTerminator`/`Matches`/`MatchIndices`**: root cause
//!   not fully isolated. Switching from `.collect()` to explicit
//!   sequential `.next()` calls passed instantly in a minimal standalone
//!   probe crate, but the identical harness still timed out when actually
//!   run inside this crate — an isolated probe crate's timing does not
//!   reliably predict real-crate behavior for Kani/CBMC (whole-crate
//!   reachability/compilation scale appears to matter on its own). See
//!   `gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`.
//!
//! All 8 instead go through `crate::str_pattern_model`'s bounded,
//! symbolic-`char` accommodation model (`KaniStrRSplitObservation`,
//! `KaniStrRSplitNObservation`, `KaniStrSplitTerminatorObservation`,
//! `KaniStrMatchObservation`): each proof is conditional on the real
//! `core::str` path refining the model's fixed window shape, exactly as
//! `slice_split_model`'s proofs already are for `std::slice::Split` and
//! friends (see `amenable_kani::rust_std::slice`).
//!
//! Split by the real API family each file covers: [`bytes_chars`],
//! [`lines_and_markers`] (`Lines`, plus the `CollectedSequenceMatchesExpected`
//! marker), [`whitespace_utf8`], [`utf8_error_and_markers`]
//! (`AccessorRecoversTheExpectedValue`, `Utf8Chunk`, `Utf8Error`,
//! `LinesAnyStatic`), [`split_family`], [`split_terminator_and_matches`],
//! and [`match_indices`].

mod bytes_chars;
mod lines_and_markers;
mod match_indices;
mod split_family;
mod split_terminator_and_matches;
mod utf8_error_and_markers;
mod whitespace_utf8;

pub use lines_and_markers::CollectedSequenceMatchesExpected;
pub use utf8_error_and_markers::VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC;
pub use whitespace_utf8::AccessorRecoversTheExpectedValue;
