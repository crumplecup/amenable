//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::StdIoWrite<
//! Vec<u8>>>` — a real, checked round-trip property over jiff's
//! actual public API (`jiff::fmt::Write::write_str` on the
//! `StdIoWrite` adapter), not a trusted stub.
//!
//! `StdIoWrite<W>(pub W)` is a real newtype wrapper — checked directly
//! against jiff's real source (`src/fmt/mod.rs`): its `jiff::fmt::
//! Write::write_str` implementation is `self.0.write_all(string.
//! as_bytes()).map_err(Error::io)`, delegating straight to the
//! wrapped `std::io::Write` value. Real, checkable law, scoped to
//! `W = Vec<u8>` (a `std::io::Write` implementation documented to
//! never fail — `Vec<u8>`'s own `write_all` just extends the vector,
//! so the `Err` arm — the only path that would construct a real
//! `jiff::Error`, the same recursive-Arc type this crate's own
//! `gallery::jiff_error_drop_cost` already documents a Drop-glue wall
//! for — is never reached, avoiding that wall entirely rather than
//! needing to work around it): writing a string via
//! `StdIoWrite<Vec<u8>>`'s `jiff::fmt::Write::write_str` always
//! succeeds and appends exactly that string's bytes to the wrapped
//! `Vec<u8>`.
//!
//! Scoped to single ASCII bytes rather than a fully symbolic string —
//! the same reason `ext::jiff::fmt_std_fmt_write`'s own doc comment
//! documents for `StdFmtWrite<String>`: Kani's unwinder can't bound a
//! variable-length symbolic `&str`.
//!
//! **A real, confirmed CBMC wall, distinct from any Drop-glue
//! finding**: a first draft compared `writer.0 == expected` (`Vec<u8>`
//! `PartialEq`) directly and timed out at 3 minutes. Isolated via the
//! standard technique (build up the call chain piece by piece,
//! `mem::forget`-ing intermediate results) — bare `Vec<u8>::write_all`
//! and the full `write_str` call both verify in under 3 seconds on
//! their own; only ADDING the `Vec<u8>` equality comparison
//! reintroduces the timeout, confirmed via a probe hitting CBMC's
//! `builtin-library-memcmp` unwinding thousands of iterations. `Vec<u8>`'s
//! `PartialEq` goes through a specialized memcmp intrinsic whose
//! CBMC unwind cost doesn't scale down to this domain's genuinely tiny
//! (1-2 byte) real lengths. Fixed by comparing length and the (at
//! most two) individual bytes directly via `.first()`/`.get(1)`
//! instead of the whole-`Vec` equality operator — no manual unwind
//! bound needed, since the real fix is avoiding the memcmp intrinsic
//! entirely, not raising a bound on it.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_std_io_write_write_str_appends_the_written_byte".to_owned(),
            VERIFY_FMT_STD_IO_WRITE_WRITE_STR_APPENDS_THE_WRITTEN_BYTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>",
        "kani",
        || <ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>> as crate::KaniWitness>::proof().to_string(),
    )
}

kani_ensures_ext!(
    ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>,
    "amenable_ext::ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>",
    u8,
    |byte| {
        let ch = byte as char;
        let mut buf = [0u8; 4];
        let expected = ch.encode_utf8(&mut buf).as_bytes().to_vec();
        let mut writer = jiff::fmt::StdIoWrite(Vec::new());
        let write_result = jiff::fmt::Write::write_str(&mut writer, ch.encode_utf8(&mut buf));
        write_result.is_ok()
            && writer.0.len() == expected.len()
            && writer.0.first() == expected.first()
            && writer.0.get(1) == expected.get(1)
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_STD_IO_WRITE_WRITE_STR_APPENDS_THE_WRITTEN_BYTE_SRC, {
        /// `StdIoWrite<Vec<u8>>`'s `jiff::fmt::Write::write_str`,
        /// called with a single-character string, always succeeds
        /// and appends exactly that character's UTF-8 bytes to the
        /// wrapped `Vec<u8>` — checked over every representable `u8`
        /// value (interpreted as a `char` via `as char`, covering the
        /// full ASCII range and beyond), not an assumed slice of one.
        #[kani::proof]
        fn verify_fmt_std_io_write_write_str_appends_the_written_byte() {
            let byte: u8 = kani::any();
            assert!(
                ExtStandard::<jiff::fmt::StdIoWrite<Vec<u8>>>::ensures(byte),
                "StdIoWrite<Vec<u8>>::write_str must succeed and append the written character's bytes"
            );
        }
    }
}
