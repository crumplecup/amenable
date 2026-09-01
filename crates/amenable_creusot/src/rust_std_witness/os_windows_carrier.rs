use super::CheckedProof;

use crate::{
    ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_HOLDS_SRC,
    ENCODE_WIDE_HEADROOM_HOLDS_SRC,
    VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_SRC,
    VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC,
    VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC,
    VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC,
    WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC,
    WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_HOLDS_SRC,
    WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC,
};

use amenable_std::{RustLanguageProvenance, RustStdProvenance};

// `std::os::windows::{ffi::EncodeWide, io::{BorrowedHandle, BorrowedSocket,
// HandleOrInvalid, OwnedHandle, OwnedSocket}}`: unlike every other type in
// this file, these can never get a real `impl CreusotWitness for
// RustStdStandard<T>` here, on any platform -- not because the type can't
// be named (on Windows, `amenable_std` names it just fine, `#[cfg(windows)]`,
// same as the `verus_witness` bridge does), but because `creusot-rustc`
// itself has no Windows target and cannot run natively on Windows the way
// `verus` does (see `amenable_verus::rust_std::os_windows_carrier`'s
// module doc comment for that contrast). There is no host, ever, on which
// `cargo creusot` could check a claim about the real type. So this bypasses
// `CreusotWitness`/`bridge_creusot_witness!` entirely, exactly like
// `amenable_kani::os_windows_model` bypasses `KaniWitness` for the
// identical reason (Kani/CBMC also never run on Windows): each harness in
// `amenable_creusot::rust_std`'s windows cluster proves a property of a
// synthetic, real-type-agnostic model (plain `isize`/`u64`/`u32` values,
// checked for real by `cargo creusot prove`), and the `evidence` string
// below connects that proof to the real type's evidence entry by name
// only -- `amenable_core::ProofRecord`'s `evidence` field is just a
// string, never required to come from naming a type that actually
// compiled here. Unconditional (no `#[cfg(windows)]`), for the same
// reason `os_windows_model.rs`'s own bypass is unconditional: nothing
// here ever names the real type in Rust syntax, only in a string literal.

fn windows_provenance(
    source_module: &str,
    url: &str,
    type_name: &str,
    summary: &str,
) -> RustStdProvenance {
    RustStdProvenance::new(
        RustLanguageProvenance::for_source("std", source_module),
        url,
        type_name,
        summary,
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "creusot",
        || CheckedProof::new(
            "verify_windows_handle_as_raw_handle_recovers_the_wrapped_value".to_string(),
            VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC.to_string(),
            windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedHandle.html",
                "std::os::windows::io::BorrowedHandle<'static>",
                "The BorrowedHandle carrier borrows a raw Windows HANDLE without taking ownership of it.",
            ),
        )
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "creusot",
        || CheckedProof::new(
            "verify_windows_handle_as_raw_handle_recovers_the_wrapped_value".to_string(),
            VERIFY_WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_SRC.to_string(),
            windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedHandle.html",
                "std::os::windows::io::OwnedHandle",
                "The OwnedHandle carrier owns a raw Windows HANDLE, closing it on drop.",
            ),
        )
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "creusot",
        || CheckedProof::new(
            "verify_windows_handle_or_invalid_rejects_only_the_sentinel".to_string(),
            VERIFY_WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_SRC.to_string(),
            windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.HandleOrInvalid.html",
                "std::os::windows::io::HandleOrInvalid",
                "The HandleOrInvalid carrier owns a Windows HANDLE that may be the sentinel INVALID_HANDLE_VALUE, deferring that check to conversion time.",
            ),
        )
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "creusot",
        || CheckedProof::new(
            "verify_windows_socket_as_raw_socket_recovers_the_wrapped_value".to_string(),
            VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC.to_string(),
            windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.BorrowedSocket.html",
                "std::os::windows::io::BorrowedSocket<'static>",
                "The BorrowedSocket carrier borrows a raw Windows SOCKET without taking ownership of it.",
            ),
        )
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "creusot",
        || CheckedProof::new(
            "verify_windows_socket_as_raw_socket_recovers_the_wrapped_value".to_string(),
            VERIFY_WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_SRC.to_string(),
            windows_provenance(
                "std::os::windows::io",
                "https://doc.rust-lang.org/std/os/windows/io/struct.OwnedSocket.html",
                "std::os::windows::io::OwnedSocket",
                "The OwnedSocket carrier owns a raw Windows SOCKET, closing it on drop.",
            ),
        )
        .to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "creusot",
        || CheckedProof::new(
            "verify_encode_wide_model_encodes_a_bmp_code_point_as_one_code_unit".to_string(),
            VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_SRC.to_string(),
            windows_provenance(
                "std::os::windows::ffi",
                "https://doc.rust-lang.org/std/os/windows/ffi/struct.EncodeWide.html",
                "std::os::windows::ffi::EncodeWide<'static>",
                "The EncodeWide carrier lazily encodes an OsStr as UTF-16 code units, as Windows APIs expect.",
            ),
        )
        .to_string(),
    )
}

// None of these five types can carry a real `Ensures<CreusotVerifier>`
// impl the way every other harness in this file does: they don't exist
// as real Rust types on this platform at all (see this module's own
// bypass above), so there's no `Self` to hang a trait impl on. The
// captured `_SRC` consts below are still the real, `harness!`-captured
// single source of each bound's text, though -- these `ContractRecord`s
// just point straight at them, the same bypass this module's
// `ProofRecord` entries above already use for the same reason.
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedHandle<'static>>",
        "creusot",
        "ensures",
        || WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedHandle>",
        "creusot",
        "ensures",
        || WINDOWS_HANDLE_AS_RAW_HANDLE_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<HandleOrInvalid>",
        "creusot",
        "ensures",
        || WINDOWS_HANDLE_OR_INVALID_REJECTS_ONLY_THE_SENTINEL_HOLDS_SRC,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BorrowedSocket<'static>>",
        "creusot",
        "ensures",
        || WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<OwnedSocket>",
        "creusot",
        "ensures",
        || WINDOWS_SOCKET_AS_RAW_SOCKET_RECOVERS_THE_WRAPPED_VALUE_HOLDS_SRC,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "creusot",
        "requires",
        || ENCODE_WIDE_HEADROOM_HOLDS_SRC,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeWide<'static>>",
        "creusot",
        "ensures",
        || ENCODE_WIDE_ENCODES_A_BMP_CODE_POINT_AS_ONE_CODE_UNIT_HOLDS_SRC,
    )
}
