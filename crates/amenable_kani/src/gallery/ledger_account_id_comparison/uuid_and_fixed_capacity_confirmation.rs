::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_uuid_shaped_comparison_passes".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_with_uuid_shaped_comparison_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Account identity modeled as a UUID's real internal representation ([u8; 16], what uuid::Uuid actually wraps -- fixed-size, stack-allocated, not heap-allocated like String) -- no `uuid` crate dependency needed to test the shape".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_WITH_UUID_SHAPED_COMPARISON_PASSES_SRC, {
        /// Does a UUID work as account identity? The `uuid` crate isn't
        /// a workspace dependency, so this models `uuid::Uuid`'s real
        /// internal representation directly: `Bytes = [u8; 16]`, a
        /// fixed-size, `Copy`, stack-allocated value -- structurally
        /// nothing like `String` (heap-allocated, variable-length).
        /// The project's own Kani failure-pattern catalog flags
        /// *symbolic-length* memcmp as a real, distinct timeout cause;
        /// a UUID's length is fixed at compile time, so this is really
        /// testing "is a bounded 16-byte comparison as cheap as a
        /// bounded 8-byte one (`u64`)," not re-litigating the `String`
        /// finding.
        #[kani::proof]
        fn ensures_closure_with_uuid_shaped_comparison_passes() {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[cfg_attr(kani, derive(kani::Arbitrary))]
            struct UuidShapedId([u8; 16]);

            struct UuidPayload {
                from: UuidShapedId,
                to: UuidShapedId,
                amount: i64,
            }

            struct UuidIdentityLedger {
                balance: i64,
            }

            impl UuidIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<UuidPayload, amenable_gaap::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: UuidPayload,
                ) -> Result<UuidPayload, amenable_gaap::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let from: UuidShapedId = kani::any();
            let to: UuidShapedId = kani::any();
            let ledger = UuidIdentityLedger { balance };
            let payload = UuidPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_fixed_capacity_string_comparison_times_out".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_with_fixed_capacity_string_comparison_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "Account identity is a fixed-CAPACITY string (stack-allocated [u8; 24] buffer + a used-length field, PartialEq compares only the first `len` bytes -- a bounded variable-length comparison, not a bare fixed-array one, the shape arrayvec::ArrayString/heapless::String actually have) -- still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_WITH_FIXED_CAPACITY_STRING_COMPARISON_TIMES_OUT_SRC, {
        /// `ensures_closure_with_uuid_shaped_comparison_passes` tested a
        /// bare `[u8; 16]` -- a fixed-size array with no notion of
        /// "used length" (the whole 16 bytes are always meaningful,
        /// same as a plain integer). A *string* is different: even
        /// with a fixed capacity, its comparison is naturally "compare
        /// only the first `len` bytes," which is a *bounded* variable-
        /// length operation, not a pure fixed-size one -- closer in
        /// spirit to `String`'s own comparison (which was expensive)
        /// than to `UuidShapedId`'s. This tests that shape directly:
        /// `{ bytes: [u8; 24], len: u8 }`, `PartialEq` comparing only
        /// `bytes[..len]` -- the real representation `arrayvec::
        /// ArrayString`/`heapless::String` use, modeled directly rather
        /// than adding either crate as a dependency. **Times out**,
        /// same as `String`: "bounded capacity, no heap" is *not*
        /// enough on its own. The real dividing line, corrected by
        /// this result: it's not "heap-allocated vs. not," it's
        /// whether the *comparison itself* has a symbolic/variable
        /// length. `self.len`/`other.len` here are themselves symbolic
        /// (`kani::any()`), so CBMC still has to reason about a
        /// variable-length compare -- the identical shape the
        /// project's own catalogued "symbolic-length memcmp" timeout
        /// class describes, just over a stack buffer instead of a heap
        /// one.
        #[kani::proof]
        fn ensures_closure_with_fixed_capacity_string_comparison_times_out() {
            const CAPACITY: usize = 24;

            #[derive(Debug, Clone, Copy)]
            #[cfg_attr(kani, derive(kani::Arbitrary))]
            struct FixedCapacityString {
                bytes: [u8; CAPACITY],
                len: u8,
            }

            impl PartialEq for FixedCapacityString {
                fn eq(&self, other: &Self) -> bool {
                    self.len == other.len
                        && self.bytes[..self.len as usize] == other.bytes[..other.len as usize]
                }
            }
            impl Eq for FixedCapacityString {}

            struct FixedStringPayload {
                from: FixedCapacityString,
                to: FixedCapacityString,
                amount: i64,
            }

            struct FixedStringIdentityLedger {
                balance: i64,
            }

            impl FixedStringIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<FixedStringPayload, amenable_gaap::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: FixedStringPayload,
                ) -> Result<FixedStringPayload, amenable_gaap::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let mut from: FixedCapacityString = kani::any();
            let mut to: FixedCapacityString = kani::any();
            kani::assume(from.len as usize <= CAPACITY);
            kani::assume(to.len as usize <= CAPACITY);
            from.len = from.len.min(CAPACITY as u8);
            to.len = to.len.min(CAPACITY as u8);
            let ledger = FixedStringIdentityLedger { balance };
            let payload = FixedStringPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}
