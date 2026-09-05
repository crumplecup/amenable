//! Bounded symbolic construction logic for Kani-facing proof models.
//!
//! The `KaniCompose` trait, its supporting helpers, and every hand-written
//! `impl KaniCompose` this module needs, consolidated into one `#[cfg(kani)]`
//! file instead of scattering gates per item. `impl_kani_compose_symbolic!`'s
//! generated impls reference `KaniCompose` from outside this file, so
//! `compose::mod` re-exports `KaniCompose` publicly and the internal helpers
//! with crate visibility. `symbolic_ascii_char` stays private: only `String`'s
//! own impl calls it.

/// Build verifier-friendly bounded values for Kani harnesses.
pub trait KaniCompose: Sized {
    /// Smallest meaningful inhabitant of the type.
    fn kani_depth0() -> Self;

    /// One-step expansion from the base case.
    fn kani_depth1() -> Self;

    /// Two-step expansion from the base case.
    fn kani_depth2() -> Self;

    /// Build a chunk of `n` depth-0 values.
    fn kani_chunk(n: usize) -> Vec<Self> {
        (0..n).map(|_| Self::kani_depth0()).collect()
    }

    /// Empty vector chunk.
    fn kani_vec_chunk_d0() -> Vec<Self> {
        Self::kani_chunk(0)
    }

    /// Single-element vector chunk.
    fn kani_vec_chunk_d1() -> Vec<Self> {
        Self::kani_chunk(1)
    }

    /// Two-element vector chunk.
    fn kani_vec_chunk_d2() -> Vec<Self> {
        Self::kani_chunk(2)
    }

    /// Compose a bounded vector from repeated fixed-size chunks.
    fn kani_vec_closure(chunk_len: usize, max_chunks: usize) -> Vec<Self> {
        let chunk_count: usize = symbolic_any();
        kani_assume(chunk_count <= max_chunks);
        let mut values = Vec::new();
        for _ in 0..chunk_count {
            values.extend(Self::kani_chunk(chunk_len));
        }
        values
    }

    /// Bounded symbolic representative of the type.
    fn kani_any() -> Self;
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub(crate) fn symbolic_any<T>() -> T
where
    T: kani::Arbitrary,
{
    kani::any()
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub(crate) fn kani_assume(condition: bool) {
    kani::assume(condition);
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn symbolic_ascii_char() -> char {
    let byte: u8 = symbolic_any();
    kani_assume(byte < 128);
    byte as char
}

impl KaniCompose for () {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {}

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {}

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {}

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {}
}

/// A foreign type (the `uuid` crate), so this impl can't live anywhere
/// but here -- neither `KaniCompose` nor `Uuid` is local to any other
/// crate. `Uuid` has no recursive structure the way `String`/`Vec<T>`
/// do (it's a flat 128-bit value, not built up from smaller expandable
/// pieces), so unlike those two, every depth is equally, fully
/// symbolic -- matching the primitive scalars above (`impl_kani_compose_
/// symbolic!`), just written out by hand since `Uuid::from_u128` needs
/// calling.
impl KaniCompose for uuid::Uuid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        Self::from_u128(symbolic_any())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        Self::from_u128(symbolic_any())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        Self::from_u128(symbolic_any())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        Self::from_u128(symbolic_any())
    }
}

impl KaniCompose for String {
    fn kani_depth0() -> Self {
        Self::new()
    }

    fn kani_depth1() -> Self {
        symbolic_ascii_char().to_string()
    }

    fn kani_depth2() -> Self {
        let mut s = String::new();
        s.push(symbolic_ascii_char());
        s.push(symbolic_ascii_char());
        s
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        let len: usize = symbolic_any();
        kani_assume(len <= 4);
        let mut s = String::new();
        for _ in 0..len {
            s.push(symbolic_ascii_char());
        }
        s
    }
}

impl<T> KaniCompose for Vec<T>
where
    T: KaniCompose,
{
    fn kani_depth0() -> Self {
        Vec::new()
    }

    fn kani_depth1() -> Self {
        vec![T::kani_depth0()]
    }

    fn kani_depth2() -> Self {
        vec![T::kani_depth0(), T::kani_depth0()]
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        T::kani_vec_closure(1, 3)
    }
}

impl<T, const N: usize> KaniCompose for [T; N]
where
    T: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        std::array::from_fn(|_| T::kani_depth0())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        std::array::from_fn(|_| T::kani_depth1())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        std::array::from_fn(|_| T::kani_depth2())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        std::array::from_fn(|_| T::kani_any())
    }
}

impl<T> KaniCompose for Option<T>
where
    T: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        None
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        Some(T::kani_depth0())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        Some(T::kani_depth1())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        let present: bool = symbolic_any();
        if present { Some(T::kani_any()) } else { None }
    }
}

impl<K, V> KaniCompose for std::collections::BTreeMap<K, V>
where
    K: KaniCompose + Ord,
    V: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        std::collections::BTreeMap::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        std::collections::BTreeMap::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        std::collections::BTreeMap::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        std::collections::BTreeMap::new()
    }
}

impl<K, V, S> KaniCompose for std::collections::HashMap<K, V, S>
where
    K: KaniCompose + Eq + std::hash::Hash,
    V: KaniCompose,
    S: Default + std::hash::BuildHasher,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }
}

impl<T> KaniCompose for Box<T>
where
    T: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        Box::new(T::kani_depth0())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        Box::new(T::kani_depth1())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        Box::new(T::kani_depth2())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        Box::new(T::kani_any())
    }
}

impl<A, B> KaniCompose for (A, B)
where
    A: KaniCompose,
    B: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        (A::kani_depth0(), B::kani_depth0())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        (A::kani_depth1(), B::kani_depth1())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        (A::kani_depth2(), B::kani_depth2())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        (A::kani_any(), B::kani_any())
    }
}

impl<A, B, C> KaniCompose for (A, B, C)
where
    A: KaniCompose,
    B: KaniCompose,
    C: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        (A::kani_depth0(), B::kani_depth0(), C::kani_depth0())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        (A::kani_depth1(), B::kani_depth1(), C::kani_depth1())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        (A::kani_depth2(), B::kani_depth2(), C::kani_depth2())
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        (A::kani_any(), B::kani_any(), C::kani_any())
    }
}

impl<A, B, C, D> KaniCompose for (A, B, C, D)
where
    A: KaniCompose,
    B: KaniCompose,
    C: KaniCompose,
    D: KaniCompose,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth0() -> Self {
        (
            A::kani_depth0(),
            B::kani_depth0(),
            C::kani_depth0(),
            D::kani_depth0(),
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth1() -> Self {
        (
            A::kani_depth1(),
            B::kani_depth1(),
            C::kani_depth1(),
            D::kani_depth1(),
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_depth2() -> Self {
        (
            A::kani_depth2(),
            B::kani_depth2(),
            C::kani_depth2(),
            D::kani_depth2(),
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn kani_any() -> Self {
        (A::kani_any(), B::kani_any(), C::kani_any(), D::kani_any())
    }
}

macro_rules! impl_kani_compose_symbolic {
    ($($ty:ty),* $(,)?) => {
        $(
            impl KaniCompose for $ty {
                fn kani_depth0() -> Self {
                    symbolic_any()
                }

                fn kani_depth1() -> Self {
                    symbolic_any()
                }

                fn kani_depth2() -> Self {
                    symbolic_any()
                }

                fn kani_any() -> Self {
                    symbolic_any()
                }
            }
        )*
    };
}

impl_kani_compose_symbolic!(
    bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);
