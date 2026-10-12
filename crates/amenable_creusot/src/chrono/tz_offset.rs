//! `CreusotWitness` for `amenable_ext::ExtStandard<chrono_tz::TzOffset>`.
//!
//! Trusted, not proven, for the same real boundary
//! `amenable_kani::chrono::tz_offset`'s own doc comment states in full:
//! `TzOffset` has no public constructor to vary, and its own accessors'
//! "fix() == base + dst" fact is a restatement of the type's own field
//! layout, true by construction with no wider claim available. Pearlite
//! has no more access to an adversarial `TzOffset` than Kani does.

use crate::CreusotWitness;
use crate::ext_bridge::{bridge_creusot_witness, impl_creusot_witness_trusted_ext};
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

impl_creusot_witness_trusted_ext!(chrono_tz::TzOffset);
