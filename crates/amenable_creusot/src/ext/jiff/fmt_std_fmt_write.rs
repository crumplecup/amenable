//! `CreusotWitness` bridge for `amenable_ext::
//! ExtStandard<jiff::fmt::StdFmtWrite<String>>` — the real proof
//! content (`extern_spec!`/`harness!`) lives in the unconditional
//! `crate::ext_jiff::fmt_std_fmt_write` sibling instead (see this
//! crate's own root doc comment for why); this file only wires the
//! `VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_NEVER_FAILS_SRC` constant it
//! emits into a `CreusotWitness` impl, mirroring every
//! `rust_std_witness` file's relationship to its own `rust_std`
//! sibling exactly.
//!
//! The sibling's proof checks `String`'s own `core::fmt::Write::
//! write_str`, not `jiff::fmt::Write::write_str` directly — a real
//! `cargo creusot` "extern spec generics don't match" error confirmed
//! jiff's generic `impl<W: core::fmt::Write> Write for StdFmtWrite<W>`
//! can't be extern-spec'd at a concrete `W = String` instantiation
//! (see the sibling's own doc comment).

use super::{ExtCheckedProof, bridge_creusot_witness};
use crate::{CreusotWitness, VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_NEVER_FAILS_SRC};
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

impl CreusotWitness for ExtStandard<jiff::fmt::StdFmtWrite<String>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_std_fmt_write_write_str_never_fails".to_owned(),
            VERIFY_FMT_STD_FMT_WRITE_WRITE_STR_NEVER_FAILS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ExtStandard<jiff::fmt::StdFmtWrite<String>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::StdFmtWrite<String>>",
        "creusot",
        || <ExtStandard<jiff::fmt::StdFmtWrite<String>> as CreusotWitness>::proof().to_string(),
    )
}
