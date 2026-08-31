::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::line_writer_newline_flush_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::io_and_str_pattern_direct_std_timeouts::line_writer_newline_flush_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct LineWriter newline flushing still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, LINE_WRITER_NEWLINE_FLUSH_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the direct `LineWriter` path retained after the production
        /// proof moved to a bounded line-buffer observation: the harness
        /// distinguishes automatic newline flush from a trailing partial line
        /// that remains buffered. If this still times out, the issue is std's
        /// line-buffering internals rather than proof-side materialization.
        #[kani::proof]
        fn line_writer_newline_flush_times_out_in_the_direct_std_path() {
            use std::io::Write;

            let mut writer = std::io::LineWriter::new(Vec::new());
            writer.write_all(b"abc\n").unwrap();
            assert_eq!(writer.get_ref().as_slice(), b"abc\n");

            writer.write_all(b"def").unwrap();
            assert_eq!(
                writer.get_ref().as_slice(),
                b"abc\n",
                "the partial line stays buffered until a newline or flush"
            );

            writer.flush().unwrap();
            assert_eq!(writer.get_ref().as_slice(), b"abc\ndef");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::bufread_lines_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::io_and_str_pattern_direct_std_timeouts::bufread_lines_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct BufRead::lines still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BUFREAD_LINES_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the direct `BufRead::lines` path retained after the
        /// production proof moved to a bounded line-splitting observation:
        /// fixed in-memory input only, with exact expected line bodies. If
        /// this still times out, the issue is std's line iteration / string
        /// machinery rather than any richer proof-side setup.
        #[kani::proof]
        fn bufread_lines_times_out_in_the_direct_std_path() {
            use std::io::BufRead;

            let lines: Vec<String> = (b"a\nb\nc"[..]).lines().map(|l| l.unwrap()).collect();
            assert_eq!(lines, vec!["a", "b", "c"]);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::bufread_split_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::io_and_str_pattern_direct_std_timeouts::bufread_split_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct BufRead::split still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BUFREAD_SPLIT_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the direct `BufRead::split` path retained after the
        /// production proof moved to a bounded delimiter-splitting
        /// observation: incremental `next()` checks only, with no eager
        /// collection. If this still times out, the issue is std's own split
        /// state machine rather than proof-side materialization.
        #[kani::proof]
        fn bufread_split_times_out_in_the_direct_std_path() {
            use std::io::BufRead;

            let mut pieces = BufRead::split(&b"a,b,c"[..], b',');
            assert_eq!(pieces.next().unwrap().unwrap(), b"a".to_vec());
            assert_eq!(pieces.next().unwrap().unwrap(), b"b".to_vec());
            assert_eq!(pieces.next().unwrap().unwrap(), b"c".to_vec());
            assert!(
                pieces.next().is_none(),
                "the separator is dropped and no extra segment is produced",
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call".to_owned(),
            "gallery::replace_recommendations::io_and_str_pattern_direct_std_timeouts::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call".to_owned(),
            "amenable_kani".to_owned(),
            "reverse str Pattern search (rsplit and friends) times out even for one next() call".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, STR_RSPLIT_REVERSE_PATTERN_SEARCH_TIMES_OUT_EVEN_FOR_A_SINGLE_NEXT_CALL_SRC, {
        /// This is the reduced representative for reverse `char`-pattern
        /// search under Kani: a five-byte fixed str, one `char` pattern,
        /// and a single `.next()` call. Forward search over the identical
        /// str (`"a,b,c".split(',')`) passes in well under a second — see
        /// `amenable_kani::rust_std::str::verify_split_yields_substrings_between_pattern_matches`
        /// — so this is a distinct root cause from the forward
        /// `SplitTerminator`/`Matches`/`MatchIndices` timeouts documented
        /// in `str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`
        /// below. This one was isolated via a standalone probe:
        /// `CharSearcher`'s backward search (`next_match_back`) bottoms
        /// out in `memchr::memrchr`, whose internal chunked/SIMD-shaped
        /// scan loop CBMC still can't bound even for a five-byte
        /// haystack — observed unwinding past 580 iterations of
        /// `<slice::Iter<u8> as Iterator>::rposition` before timing out.
        /// `RSplit`/`RSplitN`/`RSplitTerminator`/`RMatches`/
        /// `RMatchIndices` (`core::str`) all route through the same
        /// `next_match_back` call and hit this identically; none of them
        /// have a passing direct Kani proof for that reason, confirmed
        /// individually for each, not just this reduced case.
        #[kani::proof]
        fn str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call() {
            let mut it = "a,b,c".rsplit(',');
            assert_eq!(it.next(), Some("c"));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate".to_owned(),
            "gallery::replace_recommendations::io_and_str_pattern_direct_std_timeouts::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate".to_owned(),
            "amenable_kani".to_owned(),
            "forward str Pattern iteration (split_terminator/matches/match_indices) times out for real, despite passing in an isolated probe crate".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, STR_SPLIT_TERMINATOR_MATCHES_FORWARD_PATTERN_ITERATION_TIMES_OUT_IN_THE_REAL_CRATE_SRC, {
        /// This is the reduced representative for `str::split_terminator`'s
        /// forward-direction timeout, and stands in for the identical
        /// situation on `Matches`/`MatchIndices`. Unlike the `rsplit` case
        /// above, this one does *not* have a clean isolated root cause: a
        /// minimal standalone probe crate (one file, `#[kani::proof] fn
        /// probe() { let mut it = "A.B.".split_terminator('.');
        /// assert_eq!(it.next(), Some("A")); assert_eq!(it.next(),
        /// Some("B")); }`) verifies in well under a second. The identical
        /// harness, run for real as
        /// `amenable_kani::rust_std::str::verify_split_terminator_suppresses_a_trailing_empty_substring`
        /// inside this crate, times out. Whole-crate reachability/
        /// compilation scale appears to matter to CBMC independently of
        /// the harness's own logical complexity — a probe crate passing
        /// is not sufficient evidence that the same code will pass for
        /// real. Recorded here as a methodological warning as much as a
        /// root-cause note; see also
        /// `amenable_kani::rust_std::str`'s module doc.
        #[kani::proof]
        fn str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate() {
            let mut it = "A.B.".split_terminator('.');
            assert_eq!(it.next(), Some("A"));
            assert_eq!(it.next(), Some("B"));
            assert_eq!(it.next(), None);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::buf_writer_panic_recovery_reaches_the_unsupported_catch_unwind_boundary".to_owned(),
            "gallery::replace_recommendations::io_and_str_pattern_direct_std_timeouts::buf_writer_panic_recovery_reaches_the_unsupported_catch_unwind_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "direct BufWriter panic recovery reaches the unsupported catch_unwind boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BUF_WRITER_PANIC_RECOVERY_REACHES_THE_UNSUPPORTED_CATCH_UNWIND_BOUNDARY_SRC, {
        /// This is the direct `WriterPanicked` path retained after the
        /// production proof moved to a bounded panic-recovery observation:
        /// the claim is a straightforward buffered-data recovery law, but the
        /// direct proof reaches `catch_unwind` before that law can be checked
        /// under Kani.
        #[kani::proof]
        fn buf_writer_panic_recovery_reaches_the_unsupported_catch_unwind_boundary() {
            use std::io::Write;

            struct PanickingWriter;
            impl Write for PanickingWriter {
                fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                    panic!("writer panicked");
                }
                fn flush(&mut self) -> std::io::Result<()> {
                    Ok(())
                }
            }

            let mut writer = std::io::BufWriter::new(PanickingWriter);
            writer.write_all(b"data").unwrap();
            let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                writer.flush().unwrap();
            }));
            assert!(caught.is_err(), "the inner writer's panic propagates out");
            match writer.into_parts().1 {
                Err(writer_panicked) => assert_eq!(writer_panicked.into_inner(), b"data"),
                Ok(_) => panic!("expected WriterPanicked after a caught panic"),
            }
        }
    }
}
