//! One file per std carrier, each holding exactly the `verus! { ... }`
//! spec function(s) that carrier's real invariant needs — matching how
//! `elicitation_verus` groups its own proof files (`chars.rs`,
//! `durations.rs`, one file per type cluster), so `amenable_std::
//! verus_witness` can `include_str!` a single file as one type's whole
//! `claim`, the same one-claim-per-carrier granularity `amenable_kani`/
//! `amenable_creusot` get from `amenable_derive::harness!` capturing one
//! function at a time.

pub mod char_carrier;
pub mod option_carrier;
pub mod ordering_carrier;
pub mod result_carrier;
pub mod saturating_carrier;
pub mod string_carrier;
pub mod wrapping_carrier;
