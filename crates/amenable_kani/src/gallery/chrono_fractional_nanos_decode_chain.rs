//! Two false trails and the resolution for the fractional-second
//! decode chain's own verification cost -- `amenable_ext::
//! fractional_digit_bytes_to_nanos`, the `[u8; 9]` -> `i64` core the
//! civil reflect direction's fractional-second path needs. The
//! winning pattern now lives in production
//! (`amenable_kani::chrono::offset_date_time::
//! check_fractional_digit_bytes_to_nanos`); this gallery case exists
//! to preserve *why* the two more obvious designs don't work, so they
//! are not rediscovered.
//!
//! **False trail 1 (not reproduced here -- crashed the host machine):**
//! a `fractional_nanos_decode_2..9` chain of nested wrapper functions,
//! each one stubbing the *previous* stage in its body. The intent was
//! to mirror the encode direction's own staged `FractionalDigitsPrefix<K>`
//! carrier-type fix. But each stage's own `#[kani::ensures]` closure
//! *also* called the previous stage's function a second time, to state
//! what the body had already computed, rather than referencing the
//! body's own stubbed result directly. That doubled the stub-
//! substitution count at every level: stage K's proof needed roughly
//! 2^K substitutions, not K. Verifying stage 6 that way produced
//! unbounded CBMC memory growth and crashed the host. Nothing about
//! this shape is safe to re-run even in reduced form, so this gallery
//! case does not reconstruct it -- the lesson (ensures must reference
//! the stub's own result, never re-call the stubbed function a second
//! time) is captured in prose only.
//!
//! **False trail 2 (reproduced below, safe -- times out, stays memory-
//! bounded):** fixing the duplication above is not sufficient on its
//! own. A *flat* loop -- no nested wrappers, one proven-cheap one-
//! digit lemma, called once per iteration via `#[kani::stub_verified]`
//! -- still times out proving the real 9-digit function's own
//! `#[kani::proof_for_contract]`. `#[kani::stub_verified]`'s own
//! per-call-site precondition re-verification has to relate each
//! iteration's existentially-quantified stub result to the next
//! iteration's own precondition, chained across all 9 calls within
//! one proof. That chaining cost is independent of the duplicate-call
//! bug above: it shows up even with no duplication anywhere.
//!
//! **Resolution (corrected 2026-11-01):** the real bottleneck in both
//! false trails is `#[kani::stub_verified]`'s own per-call-site
//! precondition re-verification chaining across 9 calls to a helper
//! function -- not `#[kani::proof_for_contract]` itself, and not
//! "Kani's contract-checking machinery" in general, as an earlier
//! version of this doc claimed. `fractional_digit_bytes_to_nanos`'s
//! real loop body calls no helper function at all (just inline `i64`
//! arithmetic), so a direct `#[kani::proof_for_contract]` on the real,
//! untouched function -- no stubbing, no chaining, nothing to
//! substitute -- was never actually exercised by either false trail
//! above, and it passes cheaply (confirmed: 83s, 0 of 145 failed).
//! That direct form is what `amenable_kani::chrono::offset_date_time::
//! check_fractional_digit_bytes_to_nanos` proves today. The lesson:
//! "every form timed out" is only ever true of the forms actually
//! tried -- rewriting a loop body to call a stubbable helper is what
//! introduces the chaining cost; proving a function's own contract
//! directly, with nothing to stub, does not inherit it.

mod kani_only {
    #![cfg(kani)]

    /// Gallery-local copy of the one-digit combine lemma, so this case
    /// stays self-contained and does not depend on production code
    /// that may change independently of this investigation's own
    /// record.
    #[kani::requires(new_digit.is_ascii_digit() && upper >= 0 && upper <= 99_999_999)]
    #[kani::ensures(|result: &i64| *result == upper * 10 + (i64::from(new_digit) - i64::from(b'0')))]
    pub(super) fn combine_one_digit(upper: i64, new_digit: u8) -> i64 {
        upper * 10 + (i64::from(new_digit) - i64::from(b'0'))
    }

    /// False trail 2's own reproduction: a flat loop (no nested
    /// wrapper stages) calling the proven-cheap lemma above once per
    /// iteration.
    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| *result >= 0 && *result <= 999_999_999)]
    pub(super) fn decode_flat_loop(digits: &[u8; 9]) -> i64 {
        let mut nanos: i64 = 0;
        for &byte in digits {
            nanos = combine_one_digit(nanos, byte);
        }
        nanos
    }
}

#[cfg(kani)]
use kani_only::{combine_one_digit, decode_flat_loop};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_chain::combine_one_digit_passes_cheaply".to_owned(),
            "gallery::chrono_fractional_nanos_decode_chain::check_combine_one_digit".to_owned(),
            "amenable_kani".to_owned(),
            "the one-digit combine lemma's own contract, in complete isolation -- no loop, no array -- confirmed cheap; required so decode_flat_loop's own #[kani::stub_verified] target has a corresponding #[kani::proof_for_contract] harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_ONE_DIGIT_SRC, {
        /// `combine_one_digit`'s own contract, in complete isolation.
        #[kani::proof_for_contract(combine_one_digit)]
        fn check_combine_one_digit() {
            let upper: i64 = kani::any();
            kani::assume(upper >= 0 && upper <= 99_999_999);
            let new_digit: u8 = kani::any();
            kani::assume(new_digit.is_ascii_digit());
            let _ = combine_one_digit(upper, new_digit);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_chain::flat_loop_stub_verified_still_times_out".to_owned(),
            "gallery::chrono_fractional_nanos_decode_chain::check_decode_flat_loop".to_owned(),
            "amenable_kani".to_owned(),
            "a flat (non-nested) loop calling one already-proven-cheap one-digit lemma once per iteration via #[kani::stub_verified] still times out proving the whole function's own #[kani::proof_for_contract] -- the per-call-site precondition re-verification chains across all 9 calls regardless of the earlier duplicate-call bug being fixed. Confirmed safe to re-run: stays memory-bounded, times out within the harness budget rather than growing unbounded.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_FLAT_LOOP_SRC, {
        /// `decode_flat_loop`'s own contract, stubbing the one-digit
        /// lemma -- confirmed to time out at the 3-minute harness
        /// budget, safely (no unbounded memory growth), unlike the
        /// nested-wrapper version this reduces from.
        #[kani::proof_for_contract(decode_flat_loop)]
        #[kani::stub_verified(combine_one_digit)]
        fn check_decode_flat_loop() {
            let digits: [u8; 9] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_flat_loop(&digits);
        }
    }
}
