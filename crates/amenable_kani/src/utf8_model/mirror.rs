use crate::KaniCompose;
use crate::compose::{kani_assume, symbolic_any};

use super::{KaniAssumedUtf8Validity, KaniFromUtf8Error, KaniUtf8String, MAX_KANI_UTF8_BYTES};

impl KaniAssumedUtf8Validity {
    /// Decide validity for the given bytes: assumed symbolically
    /// (`kani::any()`) under Kani, computed for real (`is_valid_utf8`)
    /// otherwise -- the same split `KaniUtf8Buffer::new` used inline
    /// before this assumption had a name. Two `#[cfg]`-gated definitions,
    /// not one with a single shared parameter name, since `bytes` is
    /// genuinely read under `not(kani)` but genuinely unused under
    /// `kani` -- no single name is honest for both.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn decide(_bytes: &[u8]) -> Self {
        let valid: bool = kani::any();
        Self { valid }
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn symbolic_ascii_byte() -> u8 {
    let byte: u8 = symbolic_any();
    kani_assume(byte < 0x80);
    byte
}

impl KaniCompose for KaniUtf8String {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        Self(Vec::new())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        Self(vec![symbolic_ascii_byte()])
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        Self(vec![symbolic_ascii_byte(), symbolic_ascii_byte()])
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        Self(vec![0xFF_u8])
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        Self(vec![b'x', 0xFF_u8])
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        Self(vec![b'x', b'y', 0xFF_u8])
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        let prefix_len: usize = symbolic_any();
        kani_assume(prefix_len < MAX_KANI_UTF8_BYTES);
        let mut bytes = Vec::new();
        for _ in 0..prefix_len {
            bytes.push(symbolic_ascii_byte());
        }
        bytes.push(0xFF_u8);
        Self(bytes)
    }
}
