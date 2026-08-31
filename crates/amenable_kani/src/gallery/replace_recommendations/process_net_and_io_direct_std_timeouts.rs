::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::command_construction_reaches_an_unsupported_cstring_boundary".to_owned(),
            "gallery::replace_recommendations::process_net_and_io_direct_std_timeouts::command_construction_reaches_an_unsupported_cstring_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Command::new(...).arg(...) reaches an unsupported CString strlen boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMAND_CONSTRUCTION_REACHES_AN_UNSUPPORTED_CSTRING_BOUNDARY_SRC, {
        /// This is the reduced form behind the `Command`-args review: pure
        /// builder introspection with no spawning at all still reaches
        /// `strlen` (via `CString::from_raw`), since `Command`'s Unix
        /// representation converts the program path and arguments to
        /// `CString` unconditionally at construction time. An OS-backed
        /// boundary reached before any spawn-specific claim is even in
        /// play, not a proof-side deficiency.
        #[kani::proof]
        fn command_construction_reaches_an_unsupported_cstring_boundary() {
            let mut command = std::process::Command::new("prog");
            command.arg("a");
            let _: Vec<&std::ffi::OsStr> = command.get_args().collect();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::command_spawn_reaches_an_unsupported_glibc_version_boundary".to_owned(),
            "gallery::replace_recommendations::process_net_and_io_direct_std_timeouts::command_spawn_reaches_an_unsupported_glibc_version_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Command::spawn() reaches an unsupported gnu_get_libc_version boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMAND_SPAWN_REACHES_AN_UNSUPPORTED_GLIBC_VERSION_BOUNDARY_SRC, {
        /// This is the reduced form behind both the `Child`-process-id and
        /// `ExitStatus` reviews: spawning any process at all, regardless of
        /// what it does, reaches `gnu_get_libc_version` (glibc version
        /// detection used to pick a `posix_spawn` vs. `fork`/`exec`
        /// strategy) before any spawn-specific claim can be checked. An
        /// OS/libc-backed boundary, not a proof-side deficiency.
        #[kani::proof]
        fn command_spawn_reaches_an_unsupported_glibc_version_boundary() {
            let _ = std::process::Command::new("true").spawn();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::stdio_conversion_reaches_an_unsupported_c_string_literal_boundary".to_owned(),
            "gallery::replace_recommendations::process_net_and_io_direct_std_timeouts::stdio_conversion_reaches_an_unsupported_c_string_literal_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Stdio::to_child_stdio reaches an unsupported C string literal construct".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, STDIO_CONVERSION_REACHES_AN_UNSUPPORTED_C_STRING_LITERAL_BOUNDARY_SRC, {
        /// This is the reduced form behind the `Output`/`Stdio` reviews:
        /// configuring a piped/null standard stream and spawning reaches a
        /// C string literal construct in `Stdio::to_child_stdio` Kani
        /// reports unsupported, before any output-capture or
        /// handle-discarding claim can be checked.
        #[kani::proof]
        fn stdio_conversion_reaches_an_unsupported_c_string_literal_boundary() {
            let _ = std::process::Command::new("true")
                .stdout(std::process::Stdio::null())
                .spawn();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::socket_construction_reaches_an_unsupported_socket_syscall_boundary".to_owned(),
            "gallery::replace_recommendations::process_net_and_io_direct_std_timeouts::socket_construction_reaches_an_unsupported_socket_syscall_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "TcpListener::bind reaches an unsupported socket() syscall boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SOCKET_CONSTRUCTION_REACHES_AN_UNSUPPORTED_SOCKET_SYSCALL_BOUNDARY_SRC, {
        /// This is the reduced form behind every `std::net` review
        /// (`TcpListener`, `TcpStream`, `UdpSocket`, `Incoming`,
        /// shutdown): whatever the specific claim, constructing any socket
        /// at all reaches the `socket()` syscall Kani reports unsupported,
        /// before any connect/accept/send/recv-specific claim can be
        /// checked. An OS-backed networking boundary, not a proof-side
        /// deficiency -- confirmed identical across all five production
        /// proofs in this review pass.
        #[kani::proof]
        fn socket_construction_reaches_an_unsupported_socket_syscall_boundary() {
            let _ = std::net::TcpListener::bind("127.0.0.1:0");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::buf_reader_read_to_string_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::process_net_and_io_direct_std_timeouts::buf_reader_read_to_string_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct BufReader::read_to_string still times out in the pure std path".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BUF_READER_READ_TO_STRING_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the reduced direct `BufReader` path retained after the
        /// production proof moved to a bounded buffered-read observation:
        /// in-memory input only, exact byte-for-byte string recovery, and no
        /// OS boundary at all. If this still times out, the issue is std's
        /// buffered-reader implementation expansion rather than proof-side
        /// scaffolding.
        #[kani::proof]
        fn buf_reader_read_to_string_times_out_in_the_direct_std_path() {
            use std::io::Read;

            let mut reader = std::io::BufReader::new(&b"hello"[..]);
            let mut collected = String::new();
            reader.read_to_string(&mut collected).unwrap();
            assert_eq!(collected, "hello");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::into_inner_error_recovery_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::process_net_and_io_direct_std_timeouts::into_inner_error_recovery_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct BufWriter::into_inner error recovery still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, INTO_INNER_ERROR_RECOVERY_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the direct `IntoInnerError` path retained after the
        /// production proof moved to a bounded recovery observation: the
        /// writer always fails, and the harness observes both the surfaced
        /// error and writer recovery. If this still times out, the issue is
        /// std's buffered-writer recovery path rather than proof-side setup.
        #[kani::proof]
        fn into_inner_error_recovery_times_out_in_the_direct_std_path() {
            use std::io::Write;

            struct FailingWriter;
            impl Write for FailingWriter {
                fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
                    Err(std::io::Error::other("always fails"))
                }
                fn flush(&mut self) -> std::io::Result<()> {
                    Err(std::io::Error::other("always fails"))
                }
            }

            let mut failing = std::io::BufWriter::new(FailingWriter);
            failing.write_all(b"buffered, not yet flushed").unwrap();
            match failing.into_inner() {
                Err(err) => {
                    assert_eq!(err.error().to_string(), "always fails");
                    let _recovered_writer: std::io::BufWriter<FailingWriter> = err.into_inner();
                }
                Ok(_) => panic!("expected into_inner to fail when flushing fails"),
            }
        }
    }
}
