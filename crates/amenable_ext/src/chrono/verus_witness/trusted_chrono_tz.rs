//! Trusted `chrono_tz::TzOffset`. Gated separately from `trusted`'s
//! `chrono::Utc`/`chrono::Local` since it needs the `chrono-tz` feature
//! specifically, not just `chrono`.
//!
//! Not because the claim is trivial -- `amenable_kani::chrono::
//! tz_offset`'s own doc comment records a real, genuinely proven claim
//! (`fix() == base_utc_offset() + dst_offset()`, confirmed by really
//! executing chrono-tz's real code against 453 discovered witnesses,
//! after a genuine CBMC wall and its resolution). Verus has no
//! equivalent capability here: it never resolves `Cargo.toml` at all,
//! so it has no way to even reach chrono-tz's real `TzOffset` type,
//! let alone execute its real lookup -- the same structural boundary
//! that makes every other chrono-tz accommodation model in this crate
//! (`verus_witness::checked_chrono_tz`'s own `Tz` model) a *minimal
//! structural* illustration rather than a literal re-derivation. For
//! `TzOffset` there is no analogous minimal structure to illustrate --
//! the claim is a concrete fact about a real, opaque foreign
//! computation, not a structural property -- so this stays trusted,
//! citing Kani's real proof as the actual justification, an honest,
//! asymmetric classification across backends rather than a shortcut.
//! The real "is this offset consistent" question with teeth -- whether
//! a *descriptor-supplied* offset matches one of the real offsets
//! chrono-tz's rules produce -- is checked for real by `amenable_ext::
//! chrono::zone`'s own `offset_is_consistent_with_named_zone`.

use crate::ExtProvenance;
use crate::ExtStandard;
use crate::ext_verus_witness::impl_verus_witness_trusted_ext;
use amenable_core::{ClassifiedWitness, Evidence, VerusVerifier, Witness, WitnessSupportSummary};

impl_verus_witness_trusted_ext!(chrono_tz::TzOffset);
