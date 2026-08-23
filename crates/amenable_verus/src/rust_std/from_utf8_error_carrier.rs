//! Verus accommodation model for `alloc::string::FromUtf8Error`.
//!
//! `vstd` has no `String::from_utf8` spec support at all — the same
//! zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own real proof for this type
//! (`verify_from_utf8_error_recovers_the_original_bytes`) already
//! routes through its own Kani-side accommodation model
//! (`KaniUtf8`/`KaniFromUtf8Error`) rather than the real validation
//! path — real UTF-8 validation hits Kani's own symbolic-length-memcmp
//! timeout wall (see the workspace's Kani gallery), so this isn't a
//! case of a real proof this model merely mirrors for a second
//! verifier; both sides model the same real claim independently.
//! `FromUtf8Error`'s whole public contract is `as_bytes`/`into_bytes`
//! recovering the exact invalid byte sequence that failed validation,
//! which `VerusFromUtf8ErrorModel` states directly. `VerusFromUtf8ErrorModel`
//! isn't `FromUtf8Error` and doesn't claim to be — the proof is
//! conditional: sound if the real `FromUtf8Error` refines this
//! round-trip law, which `amenable_kani`'s own harness (checking the
//! real accessor round trip, on its own accommodation model of the
//! byte sequence) already confirms independently, for the identical
//! claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Models `FromUtf8Error`'s accessor round trip — the invalid bytes
/// that failed validation are recoverable exactly, via either accessor
/// — not `FromUtf8Error` itself.
pub struct VerusFromUtf8ErrorModel {
    pub bytes: Vec<u8>,
}

pub open spec fn from_utf8_error_model_new_preserves_bytes(
    bytes: Vec<u8>,
    result: VerusFromUtf8ErrorModel,
) -> bool {
    result.bytes@ == bytes@
}

pub open spec fn from_utf8_error_model_as_bytes_preserves_bytes(
    model: &VerusFromUtf8ErrorModel,
    result: &Vec<u8>,
) -> bool {
    result@ == model.bytes@
}

pub open spec fn from_utf8_error_model_into_bytes_preserves_bytes(
    model: VerusFromUtf8ErrorModel,
    result: Vec<u8>,
) -> bool {
    result@ == model.bytes@
}

impl VerusFromUtf8ErrorModel {
    pub fn new(bytes: Vec<u8>) -> (result: Self)
        ensures
            from_utf8_error_model_new_preserves_bytes(bytes, result),
    {
        Self { bytes }
    }

    /// Borrows the original invalid byte sequence.
    pub fn as_bytes(&self) -> (result: &Vec<u8>)
        ensures
            from_utf8_error_model_as_bytes_preserves_bytes(self, result),
    {
        &self.bytes
    }

    /// Consumes the model and returns the original invalid byte
    /// sequence.
    pub fn into_bytes(self) -> (result: Vec<u8>)
        ensures
            from_utf8_error_model_into_bytes_preserves_bytes(self, result),
    {
        self.bytes
    }
}

/// Both `as_bytes` and `into_bytes` recover the exact byte sequence
/// that failed UTF-8 validation — the round-trip law the real
/// `FromUtf8Error` is expected to refine.
pub fn verify_from_utf8_error_model_recovers_the_original_bytes(a: u8, b: u8) -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    let original: Vec<u8> = vec![a, b];

    let model = VerusFromUtf8ErrorModel::new(original.clone());
    let borrowed_matches = *model.as_bytes() == original;

    let recovered = model.into_bytes();
    let recovered_matches = recovered == original;

    (borrowed_matches, recovered_matches)
}

} // verus!
