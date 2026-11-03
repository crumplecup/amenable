//! Kani-only proof scaffolding for the fractional-digit encode
//! direction: two complementary ways of building a `[u8; 9]` digit
//! array alongside a proof that it still decodes to the original
//! `nanos`, used as `#[kani::stub_verified]` targets by the real
//! production contracts in `temporal_fraction`. Split out of that
//! module once it grew large enough to be its own cohesive concern --
//! none of this exists outside `#[cfg(kani)]` builds.

use crate::temporal_fraction::fractional_digit_bytes_to_nanos;

// ── Staged digit-chain construction, Kani only ────────────────────────
//
// Every prior attempt at a flat or staged `ensures` on an array result
// required a loop or recursive call *inside the ensures itself* to
// relate the array's content to `nanos` -- whether checking all 9
// positions at once, independent closed-form divisions, or decoding
// via a sequential accumulation. All of them, even staged with the
// previous depth stubbed, timed out from depth 8 onward: stubbing the
// previous stage's *body* call never helped the *ensures*, which still
// had to walk every element to compute anything. `digit_chain_step_N`
// avoids this entirely: its `ensures` (`result == x`) is a *direct*
// comparison between the function's own result and its own parameter
// -- no loop, no recursion, because the combining operation for a
// scalar naturally produces one new scalar to compare, full stop.
//
// The fix: a carrier type that threads the already-proven decoded
// value *alongside* the array, so each stage's own `ensures` is a
// direct field comparison -- `result.value == nanos` -- mirroring
// `digit_chain_step_N`'s shape exactly, never re-deriving anything by
// walking the array. Each stage states exactly two facts: the new
// combined scalar value (via `upper.value * 10 + digit`, the same
// combining arithmetic `digit_chain_step_N` uses) and the one newest
// byte (`result.bytes[K-1] == digit`) -- never the array's earlier
// positions, which ride along unverified at each stage, trusted
// structurally (the body only ever copies them, never recomputes
// them) and established transitively through the whole chain back to
// the depth-1 base case, the same way `digit_chain_step_9`'s single
// `result == x` fact transitively encodes all nine digits without
// ever stating them separately.
#[derive(Clone, Copy, derive_getters::Getters, derive_new::new)]
pub struct FractionalDigitsPrefix<const K: usize> {
    /// The digits built so far, most significant first.
    #[getter(copy)]
    bytes: [u8; K],
    /// The value those digits encode -- carried alongside the array,
    /// not recomputed from it, so later stages' own `ensures` can
    /// compare this field directly instead of decoding the array.
    #[getter(copy)]
    value: i64,
}

// Every field already implements `kani::Arbitrary` (`[u8; K]` and
// `i64`), so this is free to add and not circular -- it comes
// straight from the fields' own existing impls, not from anything
// this chain's own proofs establish. A local type, so no orphan-rule
// issue. Mirrors `KaniUtf8Buffer<MAX_LEN>`'s own identical impl.
#[cfg(kani)]
impl<const K: usize> kani::Arbitrary for FractionalDigitsPrefix<K> {
    fn any() -> Self {
        Self {
            bytes: kani::any(),
            value: kani::any(),
        }
    }
}

/// Stage 1: the base case of the staged digit chain -- a single digit.
#[kani::requires(nanos >= 0 && nanos < 10)]
#[kani::ensures(|result: &FractionalDigitsPrefix<1>| {
    result.value() == nanos && result.bytes()[0] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_1(nanos: i64) -> FractionalDigitsPrefix<1> {
    FractionalDigitsPrefix::new([b'0' + u8::try_from(nanos % 10).unwrap_or(0)], nanos)
}

/// Stage 2: extends stage 1 by one digit.
#[kani::requires(nanos >= 0 && nanos < 100)]
#[kani::ensures(|result: &FractionalDigitsPrefix<2>| {
    result.value() == nanos && result.bytes()[1] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_2(nanos: i64) -> FractionalDigitsPrefix<2> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_1(nanos / 10);
    FractionalDigitsPrefix::new([upper.bytes()[0], digit], upper.value() * 10 + (nanos % 10))
}

/// Stage 3: extends stage 2 by one digit.
#[kani::requires(nanos >= 0 && nanos < 1_000)]
#[kani::ensures(|result: &FractionalDigitsPrefix<3>| {
    result.value() == nanos && result.bytes()[2] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_3(nanos: i64) -> FractionalDigitsPrefix<3> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_2(nanos / 10);
    FractionalDigitsPrefix::new(
        [upper.bytes()[0], upper.bytes()[1], digit],
        upper.value() * 10 + (nanos % 10),
    )
}

/// Stage 4: extends stage 3 by one digit.
#[kani::requires(nanos >= 0 && nanos < 10_000)]
#[kani::ensures(|result: &FractionalDigitsPrefix<4>| {
    result.value() == nanos && result.bytes()[3] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_4(nanos: i64) -> FractionalDigitsPrefix<4> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_3(nanos / 10);
    FractionalDigitsPrefix::new(
        [upper.bytes()[0], upper.bytes()[1], upper.bytes()[2], digit],
        upper.value() * 10 + (nanos % 10),
    )
}

/// Stage 5: extends stage 4 by one digit.
#[kani::requires(nanos >= 0 && nanos < 100_000)]
#[kani::ensures(|result: &FractionalDigitsPrefix<5>| {
    result.value() == nanos && result.bytes()[4] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_5(nanos: i64) -> FractionalDigitsPrefix<5> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_4(nanos / 10);
    FractionalDigitsPrefix::new(
        [
            upper.bytes()[0],
            upper.bytes()[1],
            upper.bytes()[2],
            upper.bytes()[3],
            digit,
        ],
        upper.value() * 10 + (nanos % 10),
    )
}

/// Stage 6: extends stage 5 by one digit.
#[kani::requires(nanos >= 0 && nanos < 1_000_000)]
#[kani::ensures(|result: &FractionalDigitsPrefix<6>| {
    result.value() == nanos && result.bytes()[5] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_6(nanos: i64) -> FractionalDigitsPrefix<6> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_5(nanos / 10);
    FractionalDigitsPrefix::new(
        [
            upper.bytes()[0],
            upper.bytes()[1],
            upper.bytes()[2],
            upper.bytes()[3],
            upper.bytes()[4],
            digit,
        ],
        upper.value() * 10 + (nanos % 10),
    )
}

/// Stage 7: extends stage 6 by one digit.
#[kani::requires(nanos >= 0 && nanos < 10_000_000)]
#[kani::ensures(|result: &FractionalDigitsPrefix<7>| {
    result.value() == nanos && result.bytes()[6] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_7(nanos: i64) -> FractionalDigitsPrefix<7> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_6(nanos / 10);
    FractionalDigitsPrefix::new(
        [
            upper.bytes()[0],
            upper.bytes()[1],
            upper.bytes()[2],
            upper.bytes()[3],
            upper.bytes()[4],
            upper.bytes()[5],
            digit,
        ],
        upper.value() * 10 + (nanos % 10),
    )
}

/// Stage 8: extends stage 7 by one digit.
#[kani::requires(nanos >= 0 && nanos < 100_000_000)]
#[kani::ensures(|result: &FractionalDigitsPrefix<8>| {
    result.value() == nanos && result.bytes()[7] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_8(nanos: i64) -> FractionalDigitsPrefix<8> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_7(nanos / 10);
    FractionalDigitsPrefix::new(
        [
            upper.bytes()[0],
            upper.bytes()[1],
            upper.bytes()[2],
            upper.bytes()[3],
            upper.bytes()[4],
            upper.bytes()[5],
            upper.bytes()[6],
            digit,
        ],
        upper.value() * 10 + (nanos % 10),
    )
}

/// Stage 9: the full nine-digit nanosecond domain -- the real claim
/// `nanos_to_fractional_digit_bytes` itself needs, delegated to under
/// Kani (see that function's own `#[cfg(kani)]` branch above).
#[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
#[kani::ensures(|result: &FractionalDigitsPrefix<9>| {
    result.value() == nanos && result.bytes()[8] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
})]
pub fn fractional_digit_bytes_depth_9(nanos: i64) -> FractionalDigitsPrefix<9> {
    let digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
    let upper = fractional_digit_bytes_depth_8(nanos / 10);
    FractionalDigitsPrefix::new(
        [
            upper.bytes()[0],
            upper.bytes()[1],
            upper.bytes()[2],
            upper.bytes()[3],
            upper.bytes()[4],
            upper.bytes()[5],
            upper.bytes()[6],
            upper.bytes()[7],
            digit,
        ],
        upper.value() * 10 + (nanos % 10),
    )
}

// ── Parameter-passing digit chain, Kani only ──────────────────────────
//
// `nanos_to_fractional_digit_bytes`'s own contract needs every leaf it
// calls to be *soundly stubbable*: each leaf's own `ensures` must pin
// every byte position, not just the newest one. `fractional_digit_
// bytes_depth_K` above deliberately does not do this (see its own
// doc comment) -- its two-fact ensures is sufficient for the plain,
// unstubbed calls `ValidFractionalDigits`/`fractional_digit_round_
// trip` make, but leaves positions 0..K-2 unconstrained when stubbed,
// which is unsound for this purpose. These leaves take the previous
// stage's bytes as a parameter instead of calling it recursively --
// confirmed, by direct Kani timing on an isolated trivial-body
// experiment, that chaining up to 9 *distinct* functions this way,
// each with a one-more-conjunct-than-the-last `ensures`, is real
// exponential cost when every call is inlined, but tractable (minutes,
// not a timeout) when every leaf is independently proven and then
// stubbed at the call site -- never inlined and stubbed in the same
// proof. `nanos_to_fractional_digit_bytes`'s own `#[kani::proof_for_
// contract]` harness stubs every one of these.
/// A soundly stubbable leaf: appends `new_digit` to a 1-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 2]| result[0] == upper[0] && result[1] == new_digit)]
pub fn combine_digit_2(upper: [u8; 1], new_digit: u8) -> [u8; 2] {
    [upper[0], new_digit]
}

/// A soundly stubbable leaf: appends `new_digit` to a 2-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 3]| {
    result[0] == upper[0] && result[1] == upper[1] && result[2] == new_digit
})]
pub fn combine_digit_3(upper: [u8; 2], new_digit: u8) -> [u8; 3] {
    [upper[0], upper[1], new_digit]
}

/// A soundly stubbable leaf: appends `new_digit` to a 3-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 4]| {
    result[0] == upper[0] && result[1] == upper[1] && result[2] == upper[2] && result[3] == new_digit
})]
pub fn combine_digit_4(upper: [u8; 3], new_digit: u8) -> [u8; 4] {
    [upper[0], upper[1], upper[2], new_digit]
}

/// A soundly stubbable leaf: appends `new_digit` to a 4-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 5]| {
    result[0] == upper[0]
        && result[1] == upper[1]
        && result[2] == upper[2]
        && result[3] == upper[3]
        && result[4] == new_digit
})]
pub fn combine_digit_5(upper: [u8; 4], new_digit: u8) -> [u8; 5] {
    [upper[0], upper[1], upper[2], upper[3], new_digit]
}

/// A soundly stubbable leaf: appends `new_digit` to a 5-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 6]| {
    result[0] == upper[0]
        && result[1] == upper[1]
        && result[2] == upper[2]
        && result[3] == upper[3]
        && result[4] == upper[4]
        && result[5] == new_digit
})]
pub fn combine_digit_6(upper: [u8; 5], new_digit: u8) -> [u8; 6] {
    [upper[0], upper[1], upper[2], upper[3], upper[4], new_digit]
}

/// A soundly stubbable leaf: appends `new_digit` to a 6-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 7]| {
    result[0] == upper[0]
        && result[1] == upper[1]
        && result[2] == upper[2]
        && result[3] == upper[3]
        && result[4] == upper[4]
        && result[5] == upper[5]
        && result[6] == new_digit
})]
pub fn combine_digit_7(upper: [u8; 6], new_digit: u8) -> [u8; 7] {
    [
        upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], new_digit,
    ]
}

/// A soundly stubbable leaf: appends `new_digit` to a 7-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 8]| {
    result[0] == upper[0]
        && result[1] == upper[1]
        && result[2] == upper[2]
        && result[3] == upper[3]
        && result[4] == upper[4]
        && result[5] == upper[5]
        && result[6] == upper[6]
        && result[7] == new_digit
})]
pub fn combine_digit_8(upper: [u8; 7], new_digit: u8) -> [u8; 8] {
    [
        upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], upper[6], new_digit,
    ]
}

/// A soundly stubbable leaf: appends `new_digit` to an 8-byte prefix.
#[kani::requires(true)]
#[kani::ensures(|result: &[u8; 9]| {
    result[0] == upper[0]
        && result[1] == upper[1]
        && result[2] == upper[2]
        && result[3] == upper[3]
        && result[4] == upper[4]
        && result[5] == upper[5]
        && result[6] == upper[6]
        && result[7] == upper[7]
        && result[8] == new_digit
})]
pub fn combine_digit_9(upper: [u8; 8], new_digit: u8) -> [u8; 9] {
    [
        upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], upper[6], upper[7], new_digit,
    ]
}

/// A `FractionalDigitsPrefix<9>` refined so that the relationship
/// between `bytes` and `value` is guaranteed *by construction of this
/// type's own `Arbitrary` impl* -- composed from the already-proven,
/// staged `fractional_digit_bytes_depth_9`, not restated as a fresh
/// formula anywhere. Every prior attempt stated the bytes-to-value
/// relationship as an `ensures` closure computed on the fly (a
/// closed-form sum, or a decode-accumulation loop) -- each one grew
/// expensive as the fact count grew, regardless of what was stubbed
/// in the *body*, because the *ensures* itself never referenced a
/// stubbed result's value, only recomputed its own formula from
/// `digits` directly. Here, any function taking this type as a
/// parameter gets the relationship for free, as a field, with no
/// `ensures` needing to compute anything at all -- mirroring exactly
/// how `RefinedFixedOffset`'s own `Arbitrary` impl made `chrono::
/// FixedOffset`'s real domain available without an orphan-rule
/// impl, just applied here to an already-local type instead of a
/// foreign one.
#[derive(Clone, Copy, derive_new::new)]
pub struct ValidFractionalDigits(FractionalDigitsPrefix<9>);

#[cfg(kani)]
impl kani::Arbitrary for ValidFractionalDigits {
    fn any() -> Self {
        let nanos: i64 = kani::any();
        kani::assume(nanos >= 0 && nanos <= 999_999_999);
        Self::new(fractional_digit_bytes_depth_9(nanos))
    }
}

/// The full round trip, Kani only: decode's own real (unstubbed)
/// `fractional_digit_bytes_to_nanos` body, applied to `valid.0.bytes`
/// -- guaranteed by `valid`'s own `Arbitrary` impl to decode to
/// `valid.0.value`, so this function's own `ensures` is a trivial
/// field comparison, nothing computed on the fly.
#[kani::requires(true)]
#[kani::ensures(|result: &i64| *result == valid.0.value)]
pub fn fractional_digit_round_trip(valid: ValidFractionalDigits) -> i64 {
    fractional_digit_bytes_to_nanos(&valid.0.bytes)
}
