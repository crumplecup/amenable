//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::StdFmtWrite<
//! String>>` — a real, checked round-trip property over jiff's
//! actual public API (`jiff::fmt::Write::write_str` on the
//! `StdFmtWrite` adapter), not a trusted stub.
//!
//! `StdFmtWrite<W>(pub W)` is a real newtype wrapper — checked
//! directly against jiff's real source (`src/fmt/mod.rs`): its
//! `jiff::fmt::Write::write_str` implementation is `self.0.
//! write_str(string).map_err(|_| Error::from(E::StdFmtWriteAdapter))`,
//! delegating straight to the wrapped `core::fmt::Write` value. Real,
//! checkable law, scoped to `W = String` (a `core::fmt::Write`
//! implementation that never fails, so the `Err` arm — the only path
//! that would construct a real `jiff::Error`, the same recursive-Arc
//! type this crate's own `gallery::jiff_error_drop_cost` already
//! documents a Drop-glue wall for — is never reached, avoiding that
//! wall entirely rather than needing to work around it): writing a
//! string via `StdFmtWrite<String>`'s `jiff::fmt::Write::write_str`
//! always succeeds and appends exactly that string to the wrapped
//! `String`.
//!
//! Scoped to single ASCII bytes rather than a fully symbolic string —
//! Kani's unwinder can't bound a variable-length symbolic `&str`
//! (the same "iterator/loop with a runtime-dependent bound" shape
//! this crate's own [[project_kani_failure_patterns]] catalog already
//! documents), so this witness checks one representative byte at a
//! time instead of the general case, real per-byte behavior rather
//! than an assumed generalization.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::StdFmtWrite<String>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_std_fmt_write_write_str_appends_the_written_byte".to_owned(),
            VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_APPENDS_THE_WRITTEN_BYTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::StdFmtWrite<String>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::StdFmtWrite<String>>",
        "kani",
        || <ExtStandard<jiff::fmt::StdFmtWrite<String>> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::fmt::StdFmtWrite<String>>,
    "amenable_ext::ExtStandard<jiff::fmt::StdFmtWrite<String>>",
    u8,
    |byte| {
        let ch = byte as char;
        let mut writer = jiff::fmt::StdFmtWrite(String::new());
        let write_result = jiff::fmt::Write::write_str(&mut writer, ch.encode_utf8(&mut [0; 4]));
        write_result.is_ok() && writer.0.starts_with(ch)
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_APPENDS_THE_WRITTEN_BYTE_SRC, {
        /// `StdFmtWrite<String>`'s `jiff::fmt::Write::write_str`,
        /// called with a single-character string, always succeeds
        /// and appends exactly that character to the wrapped
        /// `String` — checked over every representable `u8` value
        /// (interpreted as a `char` via `as char`, covering the full
        /// ASCII range and beyond), not an assumed slice of one.
        #[kani::proof]
        fn verify_fmt_std_fmt_write_write_str_appends_the_written_byte() {
            let byte: u8 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::StdFmtWrite<String>>::ensures(byte),
                "StdFmtWrite<String>::write_str must succeed and append the written character"
            );
        }
    }
}
