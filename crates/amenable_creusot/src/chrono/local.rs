//! `CreusotWitness` for `amenable_ext::ExtStandard<chrono::Local>`.
//!
//! Trusted, not proven, for the same real boundary `amenable_kani::chrono::
//! local`'s own doc comment states in full: `Local`'s offset rules come
//! from the host's timezone database at runtime, so there is no universal
//! claim to check statically. Pearlite has no more access to the host's
//! configured timezone than Kani does.

use crate::CreusotWitness;
use crate::ext_bridge::{bridge_creusot_witness, impl_creusot_witness_trusted_ext};
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

impl_creusot_witness_trusted_ext!(chrono::Local);
