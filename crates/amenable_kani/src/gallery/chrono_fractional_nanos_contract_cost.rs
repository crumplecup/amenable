//! The contract-cost investigation's shared anchor: `plain_combine_
//! digit_K`, the contract-free twins of the real `combine_digit_K`
//! leaves, composed into `plain_full_encode` -- the baseline every
//! other file in this investigation (`chrono_fractional_nanos_
//! contract_bound_shape`, `..._contract_array_bound_shape`, `..._
//! growing_ensures`, `..._chain_distinct`) compares against. Kept
//! here, rather than moved with its own former neighbors, because
//! `plain_full_encode` is the one piece those sibling files still
//! depend on.

mod kani_only {
    #![cfg(kani)]



    // ── Hypothesis 12: isolate whether #[kani::requires]/
    // #[kani::ensures] attributes themselves add cost to a function
    // when it is called plainly (not stubbed). `full_encode_param_
    // passing` (leaf-based, no recursion at all, calling
    // combine_digit_K) was confirmed expensive to run for real alone
    // -- but `encode_flat_loop` (identical arithmetic, no function
    // calls, no contract attributes on the loop body) was cheap.
    // Recursion was never the real variable in that comparison; the
    // presence of contract attributes on the called functions was
    // never isolated. These are contract-free twins of combine_
    // digit_2/3: identical bodies, no #[kani::requires]/#[kani::
    // ensures] at all.

    pub(super) fn plain_combine_digit_2(upper: [u8; 1], new_digit: u8) -> [u8; 2] {
        [upper[0], new_digit]
    }


    pub(super) fn plain_combine_digit_3(upper: [u8; 2], new_digit: u8) -> [u8; 3] {
        [upper[0], upper[1], new_digit]
    }


    pub(super) fn plain_combine_digit_4(upper: [u8; 3], new_digit: u8) -> [u8; 4] {
        [upper[0], upper[1], upper[2], new_digit]
    }


    pub(super) fn plain_combine_digit_5(upper: [u8; 4], new_digit: u8) -> [u8; 5] {
        [upper[0], upper[1], upper[2], upper[3], new_digit]
    }


    pub(super) fn plain_combine_digit_6(upper: [u8; 5], new_digit: u8) -> [u8; 6] {
        [upper[0], upper[1], upper[2], upper[3], upper[4], new_digit]
    }


    pub(super) fn plain_combine_digit_7(upper: [u8; 6], new_digit: u8) -> [u8; 7] {
        [
            upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], new_digit,
        ]
    }


    pub(super) fn plain_combine_digit_8(upper: [u8; 7], new_digit: u8) -> [u8; 8] {
        [
            upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], upper[6], new_digit,
        ]
    }


    pub(super) fn plain_combine_digit_9(upper: [u8; 8], new_digit: u8) -> [u8; 9] {
        [
            upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], upper[6], upper[7],
            new_digit,
        ]
    }

    pub(crate) fn plain_full_encode(nanos: i64) -> [u8; 9] {
        let d1 = [b'0' + u8::try_from((nanos / 100_000_000) % 10).unwrap_or(0)];
        let d2 = plain_combine_digit_2(d1, b'0' + u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0));
        let d3 = plain_combine_digit_3(d2, b'0' + u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0));
        let d4 = plain_combine_digit_4(d3, b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0));
        let d5 = plain_combine_digit_5(d4, b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0));
        let d6 = plain_combine_digit_6(d5, b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0));
        let d7 = plain_combine_digit_7(d6, b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0));
        let d8 = plain_combine_digit_8(d7, b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0));
        plain_combine_digit_9(d8, b'0' + u8::try_from(nanos % 10).unwrap_or(0))
    }
}

#[cfg(kani)]
pub(crate) use kani_only::plain_full_encode;

