//! `CreusotWitness` for `amenable_ext::ExtStandard<chrono_tz::TzOffset>`.
//!
//! Trusted, not proven -- but not because the claim has no real content
//! (it does: see `amenable_kani::chrono::tz_offset`'s own doc comment
//! for the full investigation, including a genuine CBMC wall and its
//! resolution). Kani achieves "proven" here by *really executing*
//! chrono-tz's real, concrete, black-box code via CBMC, confirmed
//! against 453 discovered real `(zone, date)` witnesses. Creusot has no
//! equivalent capability: it translates to Why3/SMT and can state an
//! `extern_spec!` axiom about a foreign function's behavior, but cannot
//! *execute* chrono-tz's real binary-search lookup to confirm that axiom
//! the way CBMC can -- stating `fix() == base + dst` as an `extern_spec!`
//! here would be circular (asserting the very fact the Kani proof had to
//! earn through real execution), not a genuine independent check. This
//! is an honest, asymmetric classification across backends: Kani's own
//! mechanism (real execution) answers a question Creusot's mechanism
//! (symbolic/SMT reasoning over translated, not executed, code)
//! structurally cannot, for a foreign type with no public constructor.

use crate::CreusotWitness;
use crate::ext_bridge::{bridge_creusot_witness, impl_creusot_witness_trusted_ext};
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

impl_creusot_witness_trusted_ext!(chrono_tz::TzOffset);
