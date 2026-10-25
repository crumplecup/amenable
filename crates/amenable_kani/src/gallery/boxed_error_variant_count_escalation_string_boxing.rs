//! Escalation ladder, rung 6 plus its two bounded-`String` follow-ups
//! (rungs 12-13), for the CBMC out-of-memory investigation -- see
//! `boxed_error_variant_count.rs`'s own module doc comment for the full
//! narrative this file is one chapter of. Rung 6: `Box<String>` inside
//! an enum variant, wrapped in an extra struct layer -- **confirmed
//! OOM**, the first rung since the first-pass file to reproduce the
//! failure. `BoxedStringOnlyError` (this file's own type, no enum or
//! wrapper struct required) then confirms `Box<String>` alone is
//! sufficient, no surrounding shape needed. Rungs 12-13 ask whether
//! *bounding* the `String`'s size changes anything: both still OOM,
//! leaving that specific question open (see the parent file's own
//! "Open thread, not yet resolved" section) -- the recommended fix
//! (remove all heap allocation under `cfg(kani)`) doesn't depend on the
//! answer either way.

mod kani_only {
    #![cfg(kani)]

    /// Escalation ladder, rung 7: `WrappedBoxedStringError` (rung 6)
    /// -- the same outer-struct-wrapped enum shape as the passing
    /// `WrappedMixedError`, but `Heavy` now holds `Box<String>`
    /// (double heap indirection) instead of a bare `String` -- is
    /// **confirmed out-of-memory** (`RawVecInner::deallocate`-driven,
    /// 17,971 steps / 253 VCCs). Found it: `Box<String>` is the first
    /// configuration on this ladder that reproduces the blowup, where
    /// bare `String` in the identical position passed. One more pull
    /// to separate the last conflated variable: does `Box<String>`
    /// need the enum/sum-type context to trigger it, or is it
    /// sufficient entirely on its own -- no enum, no outer struct
    /// beyond the error type itself?
    pub(super) struct BoxedStringOnlyError {
        value: Box<String>,
    }

    impl BoxedStringOnlyError {
        fn new(value: &str) -> Self {
            Self {
                value: Box::new(value.to_owned()),
            }
        }
    }

    pub(super) fn boxed_string_only_fallible(value: u32) -> Result<u8, BoxedStringOnlyError> {
        u8::try_from(value).map_err(|_err| BoxedStringOnlyError::new("placeholder"))
    }

    /// The same, calling `boxed_string_only_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_string_only_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_string_only_fallible(u32::from(v1));
        let _ = boxed_string_only_fallible(u32::from(v2));
        let _ = boxed_string_only_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 12: does bounding the `String`'s size
    /// tame the `Box<String>` blowup? Every `Box<String>` rung so far
    /// used a fixed, *concrete* 11-byte literal (`"placeholder"`) --
    /// never symbolic at all. This builds a genuinely symbolic
    /// `String`, bounded to exactly 8 bytes, each byte constrained to
    /// printable ASCII so the result is always valid UTF-8 (so
    /// `String::from_utf8`'s own validation never has a failure path
    /// to reason about -- that would be a separate confound). Reuses
    /// `BoxedStringOnlyError` itself (rung 7) unchanged; only the
    /// content fed into it changes. Isolates whether the cost comes
    /// from CBMC's freedom to reason about an unconstrained symbolic
    /// buffer, or is inherent to `Box<String>`'s type/drop-glue shape
    /// regardless of content.
    pub(super) fn boxed_bounded_string_fallible(value: u32) -> Result<u8, BoxedStringOnlyError> {
        u8::try_from(value).map_err(|_err| {
            let bytes: [u8; 8] = kani::any();
            for byte in &bytes {
                kani::assume(*byte >= 0x20 && *byte < 0x7f);
            }
            let text = String::from_utf8(bytes.to_vec()).unwrap();
            BoxedStringOnlyError::new(&text)
        })
    }

    /// The same, calling `boxed_bounded_string_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_bounded_string_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_bounded_string_fallible(u32::from(v1));
        let _ = boxed_bounded_string_fallible(u32::from(v2));
        let _ = boxed_bounded_string_fallible(u32::from(v3));
    }

    /// Escalation ladder, rung 13: the corrected version of rung 12.
    /// `String::from_utf8` on fully symbolic bytes turned out to be
    /// its own expensive confound (an unwound `run_utf8_validation`
    /// loop, not a heap-OOM question at all). This builds the same
    /// genuinely symbolic, exactly-8-byte `String` without calling
    /// `from_utf8`: each byte is pushed as a `char` directly
    /// (`char::from(u8)` is total -- every `u8` is a valid Unicode
    /// scalar value, no validation, no panic path), restricted to
    /// `< 0x80` so every `char` encodes as exactly one UTF-8 byte,
    /// keeping the buffer's length an exact, known 8 bytes. The loop
    /// itself has a fixed, compile-time-constant bound (`0..8`), so
    /// there's nothing for CBMC to unwind unboundedly.
    ///
    /// Confirmed OOM at 418,600 steps / 7,154 VCCs -- but this result
    /// is **conflated, not conclusive**: building the `String` via 8
    /// sequential `push` calls measures the cost of incremental
    /// `RawVec` growth/reallocation, repeated three times by the
    /// `thrice_contracted` wrapper -- not the cost of a known size
    /// bound as a fact the solver has to represent. A `String` of
    /// provably-bounded size built in one shot, without incremental
    /// growth, was never actually tested here. Left open rather than
    /// chased further without checking in first.
    pub(super) fn boxed_bounded_string_v2_fallible(value: u32) -> Result<u8, BoxedStringOnlyError> {
        u8::try_from(value).map_err(|_err| {
            let mut text = String::new();
            for _ in 0..8 {
                let byte: u8 = kani::any();
                kani::assume(byte < 0x80);
                text.push(char::from(byte));
            }
            BoxedStringOnlyError::new(&text)
        })
    }

    /// The same, calling `boxed_bounded_string_v2_fallible` instead.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_boxed_bounded_string_v2_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = boxed_bounded_string_v2_fallible(u32::from(v1));
        let _ = boxed_bounded_string_v2_fallible(u32::from(v2));
        let _ = boxed_bounded_string_v2_fallible(u32::from(v3));
    }

    /// Reads every fixture above outside any contract boundary (no
    /// `#[kani::proof_for_contract]`, no DFCC) -- confirmed cheap, the
    /// shape this file's own history already established. Must not
    /// touch any of the specific step/VCC counts cited in the parent
    /// file's doc comment, since those are historical measurements of
    /// the *contracted* calls, not of this.
    #[kani::proof]
    fn verify_fixtures_hold_their_own_fields() {
        let boxed_string_only = BoxedStringOnlyError::new("placeholder");
        assert_eq!(*boxed_string_only.value, "placeholder");
    }
}

#[cfg(kani)]
use kani_only::{
    call_boxed_bounded_string_thrice_contracted, call_boxed_bounded_string_v2_thrice_contracted,
    call_boxed_string_only_thrice_contracted,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_string_only_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_string_only_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 7: Box<String> entirely on its own -- no enum, no sum-type context, no extra outer struct layer -- confirmed OUT OF MEMORY (RawVecInner::deallocate, 16,685 steps / 246 VCCs): the enum/sum-type context from rung 6 was never required -- Box wrapping an already heap-allocating type is sufficient by itself".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_STRING_ONLY_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_string_only_thrice_contracted)]
        fn check_call_boxed_string_only_thrice_contracted() {
            call_boxed_string_only_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_bounded_string_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_bounded_string_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 12: Box<String>, but the content is a genuinely symbolic, UTF-8-safe, 8-byte string instead of the fixed 'placeholder' literal every prior Box<String> rung used -- INVALIDATED by a confound: times out (CBMC's own harness-timeout, not OOM) inside core::str::validations::run_utf8_validation's internal loop, unwound 2600+ iterations over 8 symbolic bytes. Calling String::from_utf8 on fully unconstrained bytes forces CBMC to validate UTF-8 generically -- an unrelated cost, not the heap-allocation question this rung meant to isolate. Superseded by rung 13, which builds the symbolic string without from_utf8.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_BOUNDED_STRING_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_bounded_string_thrice_contracted)]
        fn check_call_boxed_bounded_string_thrice_contracted() {
            call_boxed_bounded_string_thrice_contracted();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::boxed_error_variant_count::boxed_bounded_string_v2_isolation".to_owned(),
            "gallery::boxed_error_variant_count::check_call_boxed_bounded_string_v2_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "escalation ladder rung 13: the corrected rung 12 -- a genuinely symbolic, exactly-8-byte String built by pushing char::from(byte) in a fixed 0..8 loop, no String::from_utf8, no unbounded validation loop -- confirmed OUT OF MEMORY, same RawVecInner::deallocate signature, 418,600 steps / 7,154 VCCs (vs. rung 7's 16,685 steps / 246 VCCs for a fixed literal). CONFLATED, not conclusive: this measures the cost of constructing a String via 8 sequential incremental pushes (each carrying its own RawVec capacity-growth/reallocation branching, tripled by the thrice-contracted wrapper), not the cost of a size bound as a fact the solver has to represent. Whether a String genuinely known to be <=8 bytes, built in one shot without incremental growth, behaves differently from an unbounded one remains untested -- this rung does not answer the question it set out to ask.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_BOXED_BOUNDED_STRING_V2_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_boxed_bounded_string_v2_thrice_contracted)]
        fn check_call_boxed_bounded_string_v2_thrice_contracted() {
            call_boxed_bounded_string_v2_thrice_contracted();
        }
    }
}
