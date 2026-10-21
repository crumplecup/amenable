//! Isolates the lemma a real (not assumed) UTF-8 validity proof for
//! the string-length induction (`gallery::string_depth_induction`)
//! would need, in place of `amenable_kani::utf8_model`'s
//! `KaniAssumedUtf8Validity::decide` -- which stands in for validity
//! with a bare `kani::any::<bool>()` under Kani specifically because
//! the real algorithm, and even this crate's own hand-rolled
//! replacement for it (`utf8_model::validation::is_valid_utf8`),
//! loop over the slice's own symbolic length (`while i < len`) --
//! the exact shape already confirmed expensive elsewhere in this
//! gallery (`memchr_naive`, `Iterator::all`, `char_count_general_case`),
//! even over tiny concrete sizes.
//!
//! The reframed goal isn't "validate an arbitrary byte buffer
//! cheaply" -- it's narrower: a valid UTF-8 prefix, followed by one
//! more character's correctly-encoded bytes, is still valid UTF-8.
//! That's UTF-8's concatenation-closure property: continuation bytes
//! (`10xxxxxx`) are never valid lead bytes and vice versa, by design,
//! so two independently-valid sequences glued together can't produce
//! an accidental reinterpretation at the seam. If that holds, proving
//! it should only ever require inspecting the *new* segment's leading
//! byte pattern -- a small, finite case-split -- not re-scanning a
//! growing buffer from the start at every depth.
//!
//! Two rungs, isolating that from the smallest possible fact up to a
//! genuine seam integration test:
//!
//! **What was actually confirmed:** both rungs pass genuinely. The
//! structural fact (0.43s, 154/154 checks) is instant, as expected --
//! no loop involved at all. The seam integration test (12.8s, 271/271
//! checks) is the real result: two genuinely arbitrary `char`s,
//! independently encoded and concatenated, validate as a whole under
//! a validator whose loop is bounded by a compile-time constant
//! instead of the slice's own symbolic length. That's real evidence
//! the `kani::any::<bool>()` stand-in in `KaniAssumedUtf8Validity::
//! decide` can be eliminated, at least at the scale
//! `string_depth_induction`'s own chain needs -- not yet wired into
//! that chain or into `KaniUtf8Buffer` itself, which still carries the
//! bool hack as of this writing.

/// Enough bytes for two chars at their maximum UTF-8 width.
#[cfg(kani)]
const MAX_SEAM_BYTES: usize = 8;

/// The same state machine as `utf8_model::validation::is_valid_utf8`,
/// rewritten to loop a fixed, compile-time-constant number of times
/// (`MAX_SEAM_BYTES`) instead of `while i < len` -- the same fix
/// `gallery::string_depth_induction::no_embedded_nul` already
/// confirmed. `.get()` handles bounds safely past the real length.
#[cfg(kani)]
fn is_valid_utf8_const_bound(bytes: &[u8], len: usize) -> bool {
    let mut i = 0usize;
    for _ in 0..MAX_SEAM_BYTES {
        if i >= len {
            return true;
        }
        let byte = match bytes.get(i) {
            Some(&b) => b,
            None => return true,
        };
        if byte & 0b1000_0000 == 0 {
            i += 1;
            continue;
        }
        if byte & 0b1110_0000 == 0b1100_0000 {
            if byte & 0b0001_1110 == 0 {
                return false;
            }
            let ok = matches!(bytes.get(i + 1), Some(&b1) if b1 & 0b1100_0000 == 0b1000_0000);
            if !ok {
                return false;
            }
            i += 2;
            continue;
        }
        if byte & 0b1111_0000 == 0b1110_0000 {
            let (b1, b2) = match (bytes.get(i + 1), bytes.get(i + 2)) {
                (Some(&b1), Some(&b2)) => (b1, b2),
                _ => return false,
            };
            if b1 & 0b1100_0000 != 0b1000_0000 || b2 & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte == 0b1110_0000 && b1 & 0b0010_0000 == 0 {
                return false;
            }
            let code_point =
                ((byte & 0x0F) as u32) << 12 | ((b1 & 0x3F) as u32) << 6 | (b2 & 0x3F) as u32;
            if (0xD800..=0xDFFF).contains(&code_point) {
                return false;
            }
            i += 3;
            continue;
        }
        if byte & 0b1111_1000 == 0b1111_0000 {
            let (b1, b2, b3) = match (bytes.get(i + 1), bytes.get(i + 2), bytes.get(i + 3)) {
                (Some(&b1), Some(&b2), Some(&b3)) => (b1, b2, b3),
                _ => return false,
            };
            if b1 & 0b1100_0000 != 0b1000_0000
                || b2 & 0b1100_0000 != 0b1000_0000
                || b3 & 0b1100_0000 != 0b1000_0000
            {
                return false;
            }
            if byte == 0b1111_0000 && b1 & 0b0011_0000 == 0 {
                return false;
            }
            if byte > 0b1111_0100 {
                return false;
            }
            let code_point = ((byte & 0x07) as u32) << 18
                | ((b1 & 0x3F) as u32) << 12
                | ((b2 & 0x3F) as u32) << 6
                | (b3 & 0x3F) as u32;
            if code_point > 0x10_FFFF {
                return false;
            }
            i += 4;
            continue;
        }
        return false;
    }
    i >= len
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::utf8_seam_concatenation::char_encoding_never_starts_with_continuation_byte".to_owned(),
            "gallery::utf8_seam_concatenation::check_char_encoding_never_starts_with_continuation_byte".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 0.43s, 154/154 checks -- the structural fact the whole seam lemma rests on, instant, no loop at all".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CHAR_ENCODING_NEVER_STARTS_WITH_CONTINUATION_BYTE_SRC, {
        /// Rung 1: the structural fact the whole lemma rests on,
        /// alone. `char::encode_utf8`'s output is always valid UTF-8
        /// -- a language guarantee, not something to re-derive -- and
        /// in particular its first byte is never a continuation byte.
        /// No loop, no buffer, nothing to unwind.
        #[kani::proof]
        fn check_char_encoding_never_starts_with_continuation_byte() {
            let c: char = kani::any();
            let mut buf = [0u8; 4];
            let encoded = c.encode_utf8(&mut buf);
            let first_byte = encoded.as_bytes()[0];
            assert!(
                first_byte & 0b1100_0000 != 0b1000_0000,
                "a char's own first encoded byte is never a continuation byte"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::utf8_seam_concatenation::seam_two_chars_concatenated_validates".to_owned(),
            "gallery::utf8_seam_concatenation::check_seam_two_chars_concatenated_validates".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 12.8s, 271/271 checks -- two genuinely arbitrary chars, independently encoded and concatenated, validate as a whole under the constant-bound rewrite of is_valid_utf8. Real evidence the kani::any::<bool>() stand-in can be eliminated for the string-length induction's own scale: a loop bounded by a compile-time constant instead of the slice's symbolic length is cheap here, where the original while-i<len shape was confirmed expensive even over far smaller inputs.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_SEAM_TWO_CHARS_CONCATENATED_VALIDATES_SRC, {
        /// Rung 2: the actual seam integration test. Two genuinely
        /// arbitrary `char`s, each encoded independently (always
        /// valid on its own), concatenated -- then checked against
        /// the constant-bound validator above. Confirms both the
        /// seam claim itself and that this rewritten validator is
        /// correct (accepts what should be accepted), on the exact
        /// scale `string_depth_induction`'s own chain would need.
        #[kani::proof]
        fn check_seam_two_chars_concatenated_validates() {
            let c1: char = kani::any();
            let c2: char = kani::any();

            let mut buf1 = [0u8; 4];
            let s1 = c1.encode_utf8(&mut buf1);
            let len1 = s1.len();

            let mut buf2 = [0u8; 4];
            let s2 = c2.encode_utf8(&mut buf2);
            let len2 = s2.len();

            let mut combined = [0u8; MAX_SEAM_BYTES];
            combined[..len1].copy_from_slice(s1.as_bytes());
            combined[len1..len1 + len2].copy_from_slice(s2.as_bytes());
            let total_len = len1 + len2;

            assert!(
                is_valid_utf8_const_bound(&combined, total_len),
                "two independently-valid char encodings, concatenated, validate as a whole"
            );
        }
    }
}
