//! Reference-counting pointers (Rc/Arc), the remaining C-string FFI errors,
//! the deprecated `SipHasher`, and `Cow`.
//!
//! `SipHasher`'s witness block below references a deprecated item, and
//! `#[expect(deprecated)]` attached to the individual impl/macro-
//! invocation/`inventory::submit!` sites didn't line up with where the
//! lint actually fires through macro expansion (confirmed: those
//! per-site attributes reported "unused attribute" while the warning
//! still fired elsewhere) — expecting it at the whole-module level
//! instead, the same fix `amenable_verus::rust_std::sip_hasher_carrier`
//! already uses for the identical reason.
#![expect(
    deprecated,
    reason = "SipHasher itself is stable (only deprecated as a recommendation to use DefaultHasher instead); covering it is a coverage-completeness question, not a call to use it"
)]

use super::char_ffi_errors::VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::rc::Rc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_rc_derefs_to_the_wrapped_value".to_owned(),
            VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::rc::Rc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::rc::Rc<i32>>",
        "verus",
        || <RustStdStandard<std::rc::Rc<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_ARC_DEREFS_TO_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/arc_carrier.rs");

impl VerusWitness for RustStdStandard<std::sync::Arc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_arc_derefs_to_the_wrapped_value".to_owned(),
            VERIFY_ARC_DEREFS_TO_THE_WRAPPED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Arc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Arc<i32>>",
        "verus",
        || <RustStdStandard<std::sync::Arc<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/into_string_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::ffi::IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_into_string_error_recovers_the_original_cstring".to_owned(),
            VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::IntoStringError>",
        "verus",
        || {
            <RustStdStandard<std::ffi::IntoStringError> as VerusWitness>::proof().to_string()
        },
    )
}

// `CString::into_string`'s/`::as_bytes`'s own real postconditions --
// named once each, this file's own `into_string_error_carrier.rs`.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::ffi::IntoStringError>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::IntoStringError>",
    [
        "into_string_rejects_a_leading_0xff_byte",
        "into_string_error_recovers_the_original_bytes",
        "as_bytes_matches_cstring_bytes_spec",
    ]
);

// The two-byte probe input `verify_into_string_error_recovers_the_
// original_cstring`'s own harness constructs a `CString` from.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::ffi::IntoStringError>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::IntoStringError>",
    "probe_starts_with_0xff_and_second_byte_nonzero"
);

const VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/cstr_carrier.rs");

const CSTR_FROM_BYTES_UNTIL_NUL_RESULT_MATCHES_NUL_PRESENCE_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_from_bytes_until_nul_result_matches_nul_presence<'a>(
    bytes: &'a [u8],
    result: Result<&'a CStr, std::ffi::FromBytesUntilNulError>,
) -> bool {
    (cstr_bytes_contain_a_nul(bytes) ==> result is Ok)
        && (cstr_bytes_contain_no_nul(bytes) ==> result is Err)
}"#;

impl VerusWitness for RustStdStandard<core::ffi::FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere".to_owned(),
            VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::FromBytesUntilNulError>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<core::ffi::FromBytesUntilNulError>,
    "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
    "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        "verus",
        || {
            <RustStdStandard<core::ffi::FromBytesUntilNulError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesUntilNulError>",
        "verus",
        "ensures",
        || CSTR_FROM_BYTES_UNTIL_NUL_RESULT_MATCHES_NUL_PRESENCE_VERUS_FRAGMENT,
    )
}

const VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/cstr_carrier.rs");

const CSTR_FROM_BYTES_WITH_NUL_RESULT_MATCHES_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_from_bytes_with_nul_result_matches_bytes<'a>(
    bytes: &'a [u8],
    result: Result<&'a CStr, std::ffi::FromBytesWithNulError>,
) -> bool {
    (cstr_bytes_have_only_a_trailing_nul(bytes) ==> {
        &&& result is Ok
        &&& cstr_bytes_spec(result->Ok_0) == bytes@.subrange(0, bytes@.len() - 1)
    }) && (cstr_bytes_contain_no_nul(bytes) ==> result is Err)
        && (cstr_bytes_have_an_interior_nul(bytes) ==> result is Err)
}"#;

impl VerusWitness for RustStdStandard<core::ffi::FromBytesWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesWithNulError>",
        "verus",
        || {
            <RustStdStandard<core::ffi::FromBytesWithNulError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::FromBytesWithNulError>",
        "verus",
        "ensures",
        || CSTR_FROM_BYTES_WITH_NUL_RESULT_MATCHES_BYTES_VERUS_FRAGMENT,
    )
}

const VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/hash_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_build_hasher_default_produces_consistent_hashers".to_owned(),
            VERIFY_BUILD_HASHER_DEFAULT_PRODUCES_CONSISTENT_HASHERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>>",
        "verus",
        || {
            <RustStdStandard<
                std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
            > as VerusWitness>::proof()
            .to_string()
        },
    )
}

const VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/sip_hasher_carrier.rs");

impl VerusWitness for RustStdStandard<std::hash::SipHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_sip_hasher_produces_consistent_hashes".to_owned(),
            VERIFY_SIP_HASHER_PRODUCES_CONSISTENT_HASHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::hash::SipHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::hash::SipHasher>",
        "verus",
        || {
            <RustStdStandard<std::hash::SipHasher> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::hash::SipHasher>,
    "amenable_std::rust_std::RustStdStandard<std::hash::SipHasher>",
    [
        "sip_hasher_new_view_is_empty",
        "sip_hasher_write_appends_to_view",
        "sip_hasher_finish_matches_spec",
    ]
);

const VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/cow_carrier.rs");

const I32_TO_OWNED_SPEC_IS_IDENTITY_VERUS_FRAGMENT: &str = r#"pub open spec fn i32_to_owned_spec_is_identity(value: i32) -> bool {
    to_owned_spec(&value) == value
}"#;

const COW_INTO_OWNED_PRESERVES_VARIANT_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cow_into_owned_preserves_variant_value<'a, B: ToOwned + ?Sized>(
    cow: Cow<'a, B>,
    result: <B as ToOwned>::Owned,
) -> bool {
    match cow {
        Cow::Borrowed(b) => result == to_owned_spec(b),
        Cow::Owned(o) => result == o,
    }
}"#;

impl VerusWitness for RustStdStandard<std::borrow::Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cow_borrowed_and_owned_agree_on_their_value".to_owned(),
            VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::borrow::Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::borrow::Cow<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        "verus",
        "ensures",
        || I32_TO_OWNED_SPEC_IS_IDENTITY_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::borrow::Cow<'static, i32>>",
        "verus",
        "ensures",
        || COW_INTO_OWNED_PRESERVES_VARIANT_VALUE_VERUS_FRAGMENT,
    )
}

pub(super) const VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/btree_carrier.rs");
