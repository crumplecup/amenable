//! Verifies the real `amenable_ext::chrono::utc_offset`/`utc_offset_
//! reflect` contracts: `amenable_time::UtcOffsetDescriptor` to and
//! from a real `chrono::FixedOffset`, in both directions, plus the
//! staged, stub-friendly decomposition each direction's own contract
//! composes from. Split out of `offset_date_time.rs` once that file
//! grew large enough to be unnavigable as one unit.


amenable_derive::harness! {
    kani, CHECK_FIXED_OFFSET_SIGN_AND_MAGNITUDE_SRC, {
        /// The real `amenable_ext::fixed_offset_sign_and_magnitude`'s
        /// own contract -- the base case, nothing to stub yet.
        #[kani::proof_for_contract(amenable_ext::fixed_offset_sign_and_magnitude)]
        fn check_fixed_offset_sign_and_magnitude() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = amenable_ext::fixed_offset_sign_and_magnitude(offset);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_NARROW_TO_U8_IN_RANGE_SRC, {
        /// `narrow_to_u8_in_range`'s own contract -- the base case,
        /// nothing to stub yet, and the only one that can ever be
        /// stubbed anywhere else (plain `u8` return, no `Arbitrary`
        /// issue).
        #[kani::proof_for_contract(amenable_ext::narrow_to_u8_in_range)]
        fn check_narrow_to_u8_in_range() {
            let value: u32 = kani::any();
            kani::assume(value <= 255);
            let _ = amenable_ext::narrow_to_u8_in_range(value);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_NARROW_TO_U8_SRC, {
        /// `narrow_to_u8`'s own contract: the base case, nothing to
        /// stub yet. Proven directly over its real, restricted domain
        /// (`value <= 255`, matching every real call site through
        /// `fixed_offset_to_utc_offset_descriptor`), so the `Err` arm
        /// -- unreachable here, and unsupported by Kani's
        /// `#[track_caller]` handling regardless -- is provably never
        /// taken.
        #[kani::proof_for_contract(amenable_ext::narrow_to_u8)]
        fn check_narrow_to_u8() {
            let value: u32 = kani::any();
            kani::assume(value <= 255);
            let _ = amenable_ext::narrow_to_u8(value, amenable_time::TemporalComponent::Hour);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_OFFSET_DESCRIPTOR_PARTS_2_SRC, {
        /// Stage 2's own contract: stubs `fixed_offset_sign_and_magnitude`.
        #[kani::proof_for_contract(amenable_ext::offset_descriptor_parts_2)]
        #[kani::stub_verified(amenable_ext::fixed_offset_sign_and_magnitude)]
        fn check_offset_descriptor_parts_2() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = amenable_ext::offset_descriptor_parts_2(offset);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_OFFSET_DESCRIPTOR_PARTS_3_SRC, {
        /// Stage 3's own contract: stubs stage 2.
        #[kani::proof_for_contract(amenable_ext::offset_descriptor_parts_3)]
        #[kani::stub_verified(amenable_ext::offset_descriptor_parts_2)]
        fn check_offset_descriptor_parts_3() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = amenable_ext::offset_descriptor_parts_3(offset);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_OFFSET_DESCRIPTOR_PARTS_4_SRC, {
        /// Stage 4's own contract: stubs stage 3 -- the full four-fact
        /// relation, staged, resolving the fact-count wall that
        /// defeats the flat/unstubbed version.
        #[kani::proof_for_contract(amenable_ext::offset_descriptor_parts_4)]
        #[kani::stub_verified(amenable_ext::offset_descriptor_parts_3)]
        fn check_offset_descriptor_parts_4() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = amenable_ext::offset_descriptor_parts_4(offset);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_BUILD_UTC_OFFSET_DESCRIPTOR_FROM_PARTS_SRC, {
        /// The builder's own four-fact transcription conjunction,
        /// given already-decomposed parts -- no `FixedOffset`
        /// arithmetic left to reason about at all.
        #[kani::proof_for_contract(amenable_ext::build_utc_offset_descriptor_from_parts)]
        fn check_build_utc_offset_descriptor_from_parts() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            let minutes: u8 = kani::any();
            let seconds: u8 = kani::any();
            kani::assume(hours <= 23);
            let _ = amenable_ext::build_utc_offset_descriptor_from_parts(sign, hours, minutes, seconds);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_SIGNED_SECONDS_PARTS_2_SRC, {
        /// Stage 2's own contract, realize direction: no stubbing
        /// needed -- the base case.
        #[kani::proof_for_contract(amenable_ext::signed_seconds_parts_2)]
        fn check_signed_seconds_parts_2() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let _ = amenable_ext::signed_seconds_parts_2(sign, hours);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_SIGNED_SECONDS_PARTS_3_SRC, {
        /// Stage 3's own contract: stubs stage 2.
        #[kani::proof_for_contract(amenable_ext::signed_seconds_parts_3)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_2)]
        fn check_signed_seconds_parts_3() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let _ = amenable_ext::signed_seconds_parts_3(sign, hours, minutes);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_SIGNED_SECONDS_PARTS_4_SRC, {
        /// Stage 4's own contract: stubs stage 3 -- the full signed-
        /// seconds total, staged the same way the reflect direction's
        /// four-fact relation was.
        #[kani::proof_for_contract(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_3)]
        fn check_signed_seconds_parts_4() {
            let sign: amenable_time::UtcOffsetSign = kani::any();
            let hours: u8 = kani::any();
            kani::assume(hours <= 23);
            let minutes: u8 = kani::any();
            kani::assume(minutes <= 59);
            let seconds: u8 = kani::any();
            kani::assume(seconds <= 59);
            let _ = amenable_ext::signed_seconds_parts_4(sign, hours, minutes, seconds);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_BUILD_FIXED_OFFSET_FROM_SIGNED_SECONDS_SRC, {
        /// The final, cheap step's own contract, in isolation from the
        /// signed-seconds arithmetic -- just the one `FixedOffset::
        /// east_opt` call.
        #[kani::proof_for_contract(amenable_ext::build_fixed_offset_from_signed_seconds)]
        fn check_build_fixed_offset_from_signed_seconds() {
            let seconds: i32 = kani::any();
            kani::assume(seconds.unsigned_abs() <= 86_399);
            let _ = amenable_ext::build_fixed_offset_from_signed_seconds(seconds);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_BUILD_REFINED_FIXED_OFFSET_IN_RANGE_SRC, {
        /// `build_refined_fixed_offset_in_range`'s own contract --
        /// the `RefinedFixedOffset` newtype's infallible core, proven
        /// standalone before anything stubs it. Confirmed first in
        /// the gallery (`chrono_offset_date_time_round_trip_realize_
        /// staged_fix::build_refined_fixed_offset_in_range_isolation`)
        /// before this ever touched production code.
        #[kani::proof_for_contract(amenable_ext::build_refined_fixed_offset_in_range)]
        fn check_build_refined_fixed_offset_in_range() {
            let seconds: i32 = kani::any();
            kani::assume(seconds.unsigned_abs() <= 86_399);
            let _ = amenable_ext::build_refined_fixed_offset_in_range(seconds);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_UTC_OFFSET_DESCRIPTOR_TO_FIXED_OFFSET_SRC, {
        /// `utc_offset_descriptor_to_fixed_offset`'s own contract, in
        /// isolation from the reflect direction -- stubs both the
        /// staged arithmetic (`signed_seconds_parts_4`) and, via the
        /// `RefinedFixedOffset` refinement type, the final build step
        /// (`build_refined_fixed_offset_in_range`). Inlining that
        /// final step instead (mirroring the reflect direction's own
        /// successful shape) was tried first and confirmed to still
        /// time out -- `Runtime Symex: 133.816s`, `4816 VCC(s)` --
        /// because unlike the reflect direction's builder
        /// transcription, this step's real body (`FixedOffset::
        /// east_opt`) compounds under already-stubbed symbolic state.
        /// Leaning on stubs here instead, confirmed first in the
        /// gallery (`chrono_offset_date_time_round_trip_realize_
        /// staged_fix::realize_staged_stubbed_resolves_the_wall`)
        /// before landing here.
        ///
        /// Takes the descriptor via `UtcOffsetDescriptor`'s own
        /// `kani::Arbitrary` impl directly rather than constructing it
        /// through `UtcOffsetDescriptorBuilder` -- a second, separate
        /// stub-leverage opportunity, isolating the builder's own
        /// construction machinery (real code, unstubbed, that was
        /// running inside every version of this proof so far) out of
        /// the harness entirely. That `Arbitrary` impl covers the
        /// type's whole real domain (the builder validates nothing,
        /// so `hours > 23` is a genuinely constructible real value,
        /// not excluded there) -- the narrowing to the bounded case
        /// this proof actually cares about comes from the real
        /// function's own `#[kani::requires]`, the same division of
        /// labor used everywhere else in this investigation.
        #[kani::proof_for_contract(amenable_ext::utc_offset_descriptor_to_fixed_offset)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(amenable_ext::build_refined_fixed_offset_in_range)]
        fn check_utc_offset_descriptor_to_fixed_offset() {
            let descriptor: amenable_time::UtcOffsetDescriptor = kani::any();
            let _ = amenable_ext::utc_offset_descriptor_to_fixed_offset(descriptor);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_UTC_OFFSET_DESCRIPTOR_TO_FIXED_OFFSET_INFALLIBLE_SRC, {
        /// `utc_offset_descriptor_to_fixed_offset_infallible`'s own
        /// contract -- the whole-function infallible sibling, built
        /// so `offset_round_trips` can `#[kani::stub_verified]` the
        /// realize direction as a unit instead of inlining the real
        /// (un-stubbable, `Result<_, TemporalError>`-returning)
        /// function in full.
        #[kani::proof_for_contract(amenable_ext::utc_offset_descriptor_to_fixed_offset_infallible)]
        #[kani::stub_verified(amenable_ext::signed_seconds_parts_4)]
        #[kani::stub_verified(amenable_ext::build_refined_fixed_offset_in_range)]
        fn check_utc_offset_descriptor_to_fixed_offset_infallible() {
            let descriptor: amenable_time::UtcOffsetDescriptor = kani::any();
            let _ = amenable_ext::utc_offset_descriptor_to_fixed_offset_infallible(descriptor);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FIXED_OFFSET_TO_UTC_OFFSET_DESCRIPTOR_SRC, {
        /// `fixed_offset_to_utc_offset_descriptor`'s own contract,
        /// now composed from the staged arithmetic decomposition
        /// (`offset_descriptor_parts_4`, stubbed) -- the real source
        /// of the fact-count wall -- while leaving
        /// `build_utc_offset_descriptor_from_parts` inlined: its own
        /// `Result<UtcOffsetDescriptor, TemporalError>` return can't
        /// be `stub_verified` (no `kani::Arbitrary` on `TemporalError`,
        /// deliberately), but its own contract already proved cheap
        /// and standalone, so inlining its simple builder-transcription
        /// body here, decoupled from the arithmetic relation that was
        /// the actual cost, is the cheaper option anyway.
        #[kani::proof_for_contract(amenable_ext::fixed_offset_to_utc_offset_descriptor)]
        #[kani::stub_verified(amenable_ext::offset_descriptor_parts_4)]
        fn check_fixed_offset_to_utc_offset_descriptor() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = amenable_ext::fixed_offset_to_utc_offset_descriptor(offset);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FIXED_OFFSET_TO_UTC_OFFSET_DESCRIPTOR_INFALLIBLE_SRC, {
        /// `fixed_offset_to_utc_offset_descriptor_infallible`'s own
        /// contract -- the whole-function infallible sibling, built
        /// so `offset_round_trips` can `#[kani::stub_verified]` the
        /// reflect direction as a unit instead of inlining the real
        /// (un-stubbable) function in full.
        #[kani::proof_for_contract(amenable_ext::fixed_offset_to_utc_offset_descriptor_infallible)]
        #[kani::stub_verified(amenable_ext::offset_descriptor_parts_4)]
        fn check_fixed_offset_to_utc_offset_descriptor_infallible() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = amenable_ext::fixed_offset_to_utc_offset_descriptor_infallible(offset);
        }
    }
}

