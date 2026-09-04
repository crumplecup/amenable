use amenable_std::RustStdStandard;

use crate::rust_std::{impl_kani_witness_trusted, kani_ensures, kani_requires};

impl_kani_witness_trusted!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    ()
);

// `checked_add` not overflowing is a real precondition independently
// restated at 9 real sites across `rust_std::slice`'s `chunks_mut`/
// `chunks_exact_mut`/`rchunks_mut`/`rchunks_exact_mut` families (`i32`,
// a fixed literal addend) and `rust_std::time::verify_duration_new_normalizes_nanos_and_carries_into_secs`
// (`u64`, a symbolic addend). `checked_add` is inherent per fixed-width
// integer type, not a shared trait method available without pulling in
// an external crate (`num_traits`), so this is registered per concrete
// width rather than as one generic contract type.
kani_requires!(
    RustStdStandard<i32>,
    "amenable_std::rust_std::RustStdStandard<i32>",
    (i32, i32),
    |(a, b)| a.checked_add(b).is_some()
);

kani_requires!(
    RustStdStandard<u64>,
    "amenable_std::rust_std::RustStdStandard<u64>",
    (u64, u64),
    |(a, b)| a.checked_add(b).is_some()
);

kani_requires!(
    RustStdStandard<i64>,
    "amenable_std::rust_std::RustStdStandard<i64>",
    (i64, i64),
    |(a, b)| a.checked_add(b).is_some()
);

// `i32` equality is independently restated at several real sites
// spanning `rust_std::os_unix`'s raw-fd round-trips and
// `rust_std::num`'s `Wrapping`/`Saturating` inner-operation matches.
// Deliberately registered here rather than in `os_unix.rs` (its first
// real use): that file is `#![cfg(unix)]`-gated, and `rust_std::num`'s
// own sites (not platform-gated) would otherwise depend on a
// registration that silently disappears on a non-unix build.
kani_ensures!(
    RustStdStandard<i32>,
    "amenable_std::rust_std::RustStdStandard<i32>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

// A read/write count matching the buffer's own length is independently
// restated at 5 real sites across `rust_std::io`'s `Repeat`/`Sink`/
// `Chain`/`Cursor` harnesses (`assert_eq!(count, buffer.len(), ...)`).
// `RustStdStandard<usize>`'s `Ensures<KaniVerifier>` slot was free.
kani_ensures!(
    RustStdStandard<usize>,
    "amenable_std::rust_std::RustStdStandard<usize>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

// A round-tripped port number matching the port a socket address was
// constructed with is independently restated at 4 real sites across
// `rust_std::net`'s `SocketAddrV4`/`SocketAddrV6`/`SocketAddr`
// harnesses (`assert_eq!(addr.port(), port, ...)`).
// `RustStdStandard<u16>`'s `Ensures<KaniVerifier>` slot was free.
kani_ensures!(
    RustStdStandard<u16>,
    "amenable_std::rust_std::RustStdStandard<u16>",
    (u16, u16),
    |(actual, expected)| actual == expected
);

// Two hashers over identical input producing matching digests is
// independently restated at 3 real sites (`assert_eq!(h1.finish(),
// h2.finish(), ...)`) across `rust_std::hash`'s `BuildHasherDefault`/
// `SipHasher` harnesses and `rust_std::std_hash`'s `DefaultHasher`
// harness. `RustStdStandard<u64>`'s `Ensures<KaniVerifier>` slot was
// free (only `Requires` was previously registered there, for
// `checked_add`).
kani_ensures!(
    RustStdStandard<u64>,
    "amenable_std::rust_std::RustStdStandard<u64>",
    (u64, u64),
    |(actual, expected)| actual == expected
);
