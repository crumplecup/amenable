//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>` — the real proof
//! content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::fmt_std_io_write` sibling instead (see this
//! crate's own root doc comment for why); this file only wires the
//! `VERIFY_FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_SRC` constant it
//! emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.
//!
//! The sibling's proof checks `Vec<u8>`'s own `std::io::Write::
//! write_all`, not `jiff::fmt::Write::write_str` directly — the
//! identical real generics-matching wall `ext::jiff::
//! fmt_std_fmt_write`'s own doc comment documents for `StdFmtWrite<
//! String>` (see the sibling's own doc comment for why `Vec<u8>`'s
//! own impl needed the generic-over-`Allocator` extern_spec form
//! rather than the non-generic workaround `String` allowed).

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_std_io_write_write_str_never_fails".to_owned(),
            VERIFY_FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>",
        "creusot",
        || <ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>> as CreusotWitness>::proof().to_string(),
    )
}
