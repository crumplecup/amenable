//! Kani-only accommodation model for focused owned UTF-8 conversion laws.
//!
//! This module is where Amenable stops asking Kani to execute the direct owned
//! `String::from_utf8` / `FromUtf8Error` std path and instead proves against a
//! small package of explicit bounded UTF-8 and byte-recovery laws that the
//! real implementation is expected to refine.
//!
//! The direct std timeout path remains preserved in the proof gallery as a
//! false trail. Production proofs that use this model are therefore
//! conditional:
//!
//! - if the real owned UTF-8 conversion path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

use crate::KaniCompose;
use crate::compose::{kani_assume, symbolic_any};

const MAX_KANI_UTF8_BYTES: usize = 4;

/// Modeled owned valid UTF-8 bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniUtf8String(Vec<u8>);

/// Modeled owned UTF-8 conversion error that preserves the original bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFromUtf8Error(Vec<u8>);

/// Namespace for focused owned UTF-8 conversion laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniUtf8;

impl KaniUtf8 {
    /// Classify one owned byte vector as valid UTF-8 or an owned recovery error.
    pub fn classify_owned(bytes: Vec<u8>) -> Result<KaniUtf8String, KaniFromUtf8Error> {
        assert!(
            bytes.len() <= MAX_KANI_UTF8_BYTES,
            "KaniUtf8 models at most {MAX_KANI_UTF8_BYTES} bytes"
        );

        if is_valid_utf8(&bytes) {
            Ok(KaniUtf8String(bytes))
        } else {
            Err(KaniFromUtf8Error(bytes))
        }
    }

    /// Report whether the bounded byte slice is valid UTF-8.
    pub fn is_valid(bytes: &[u8]) -> bool {
        assert!(
            bytes.len() <= MAX_KANI_UTF8_BYTES,
            "KaniUtf8 models at most {MAX_KANI_UTF8_BYTES} bytes"
        );
        is_valid_utf8(bytes)
    }
}

impl KaniUtf8String {
    /// Borrow the modeled valid UTF-8 bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the modeled valid UTF-8 bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Report the byte length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Report whether the modeled string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the modeled valid UTF-8 content as `&str`.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("KaniUtf8String stores validated UTF-8")
    }
}

impl KaniFromUtf8Error {
    /// Borrow the original invalid owned bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the error and recover the original owned bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl KaniCompose for KaniUtf8String {
    fn kani_depth0() -> Self {
        Self(Vec::new())
    }

    fn kani_depth1() -> Self {
        Self(vec![symbolic_ascii_byte()])
    }

    fn kani_depth2() -> Self {
        Self(vec![symbolic_ascii_byte(), symbolic_ascii_byte()])
    }

    fn kani_any() -> Self {
        let len: usize = symbolic_any();
        kani_assume(len <= MAX_KANI_UTF8_BYTES);
        let mut bytes = Vec::new();
        for _ in 0..len {
            bytes.push(symbolic_ascii_byte());
        }
        Self(bytes)
    }
}

impl KaniCompose for KaniFromUtf8Error {
    fn kani_depth0() -> Self {
        Self(vec![0xFFu8])
    }

    fn kani_depth1() -> Self {
        Self(vec![b'x', 0xFFu8])
    }

    fn kani_depth2() -> Self {
        Self(vec![b'x', b'y', 0xFFu8])
    }

    fn kani_any() -> Self {
        let prefix_len: usize = symbolic_any();
        kani_assume(prefix_len < MAX_KANI_UTF8_BYTES);
        let mut bytes = Vec::new();
        for _ in 0..prefix_len {
            bytes.push(symbolic_ascii_byte());
        }
        bytes.push(0xFFu8);
        Self(bytes)
    }
}

fn symbolic_ascii_byte() -> u8 {
    let byte: u8 = symbolic_any();
    kani_assume(byte < 0x80);
    byte
}

fn is_valid_utf8(bytes: &[u8]) -> bool {
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
            if code_point > 0x10FFFF {
                return false;
            }

            i += 4;
            continue;
        }

        return false;
    }

    true
}
