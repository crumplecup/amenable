use super::{KaniUtf8, MAX_KANI_UTF8_BYTES};

/// Modeled UTF-8 validation error exposing the same position information as
/// `std::str::Utf8Error`: how many leading bytes were valid, and the byte
/// width of the invalid sequence at that position (when determinable
/// without more input).
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_getters::Getters)]
pub struct KaniUtf8PositionError {
    /// The number of leading bytes that were valid UTF-8.
    #[getter(copy)]
    valid_up_to: usize,
    /// The byte width of the invalid sequence at `valid_up_to`, or `None`
    /// if more input would be needed to determine it (an incomplete
    /// trailing multi-byte sequence).
    #[getter(copy)]
    error_len: Option<usize>,
}

impl KaniUtf8 {
    /// Validate a bounded byte slice, reporting the same `valid_up_to` /
    /// `error_len` position information as `std::str::from_utf8`'s error on
    /// the first invalid byte found.
    ///
    /// # Errors
    ///
    /// Returns `Err(KaniUtf8PositionError)` when the bytes are not valid
    /// UTF-8.
    pub fn error_position(bytes: &[u8]) -> Result<(), KaniUtf8PositionError> {
        assert!(
            bytes.len() <= MAX_KANI_UTF8_BYTES,
            "KaniUtf8 models at most {MAX_KANI_UTF8_BYTES} bytes"
        );

        match first_invalid_position(bytes) {
            None => Ok(()),
            Some((valid_up_to, error_len)) => Err(KaniUtf8PositionError {
                valid_up_to,
                error_len,
            }),
        }
    }
}

/// Same byte-classification shape as `is_valid_utf8`, but reports where
/// validation first fails instead of a bare bool.
fn first_invalid_position(bytes: &[u8]) -> Option<(usize, Option<usize>)> {
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let byte = bytes[i];

        if byte & 0b1000_0000 == 0 {
            i += 1;
            continue;
        }

        if byte & 0b1110_0000 == 0b1100_0000 {
            if byte & 0b0001_1110 == 0 {
                return Some((i, Some(1)));
            }
            if i + 1 >= len {
                return Some((i, None));
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(1)));
            }
            i += 2;
            continue;
        }

        if byte & 0b1111_0000 == 0b1110_0000 {
            if i + 1 >= len {
                return Some((i, None));
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000
                || (byte == 0b1110_0000 && bytes[i + 1] & 0b0010_0000 == 0)
            {
                return Some((i, Some(1)));
            }
            if i + 2 >= len {
                return Some((i, None));
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(2)));
            }

            let code_point = ((byte & 0x0F) as u32) << 12
                | ((bytes[i + 1] & 0x3F) as u32) << 6
                | (bytes[i + 2] & 0x3F) as u32;
            if (0xD800..=0xDFFF).contains(&code_point) {
                return Some((i, Some(1)));
            }

            i += 3;
            continue;
        }

        if byte & 0b1111_1000 == 0b1111_0000 {
            if i + 1 >= len {
                return Some((i, None));
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000
                || (byte == 0b1111_0000 && bytes[i + 1] & 0b0011_0000 == 0)
                || byte > 0b1111_0100
            {
                return Some((i, Some(1)));
            }
            if i + 2 >= len {
                return Some((i, None));
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(2)));
            }
            if i + 3 >= len {
                return Some((i, None));
            }
            if bytes[i + 3] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(3)));
            }

            let code_point = ((byte & 0x07) as u32) << 18
                | ((bytes[i + 1] & 0x3F) as u32) << 12
                | ((bytes[i + 2] & 0x3F) as u32) << 6
                | (bytes[i + 3] & 0x3F) as u32;
            if code_point > 0x10_FFFF {
                return Some((i, Some(1)));
            }

            i += 4;
            continue;
        }

        return Some((i, Some(1)));
    }

    None
}

#[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(bytes)))]
pub(super) fn is_valid_utf8(bytes: &[u8]) -> bool {
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let byte = bytes[i];

        if byte & 0b1000_0000 == 0 {
            i += 1;
            continue;
        }

        if byte & 0b1110_0000 == 0b1100_0000 {
            if i + 1 >= len {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte & 0b0001_1110 == 0 {
                return false;
            }
            i += 2;
            continue;
        }

        if byte & 0b1111_0000 == 0b1110_0000 {
            if i + 2 >= len {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte == 0b1110_0000 && bytes[i + 1] & 0b0010_0000 == 0 {
                return false;
            }

            let code_point = ((byte & 0x0F) as u32) << 12
                | ((bytes[i + 1] & 0x3F) as u32) << 6
                | (bytes[i + 2] & 0x3F) as u32;
            if (0xD800..=0xDFFF).contains(&code_point) {
                return false;
            }

            i += 3;
            continue;
        }

        if byte & 0b1111_1000 == 0b1111_0000 {
            if i + 3 >= len {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 3] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte == 0b1111_0000 && bytes[i + 1] & 0b0011_0000 == 0 {
                return false;
            }

            let code_point = ((byte & 0x07) as u32) << 18
                | ((bytes[i + 1] & 0x3F) as u32) << 12
                | ((bytes[i + 2] & 0x3F) as u32) << 6
                | (bytes[i + 3] & 0x3F) as u32;
            if code_point > 0x10_FFFF {
                return false;
            }

            i += 4;
            continue;
        }

        return false;
    }

    true
}
