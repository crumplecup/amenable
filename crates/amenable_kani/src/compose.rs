//! Bounded symbolic construction for Kani-facing proof models.
//!
//! `kani::any::<T>()` is fine for leaf scalars, but it becomes a poor default
//! for recursive or heap-backed shapes such as `String`, `Vec<T>`, and nested
//! user-defined carriers: Kani then has to reason about unconstrained
//! destructor paths and collection growth, which is exactly where unbounded
//! unwinding and state blow-up start to dominate.
//!
//! `KaniCompose` gives Amenable a verifier-specific modeling surface instead:
//! small depth-indexed constructors plus a bounded `kani_any()`. It belongs in
//! `amenable_kani`, not `amenable_core`, because it is not a constitutional
//! proof role. It is a Kani-only input-construction discipline.

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

#[cfg(kani)]
pub(crate) trait KaniArbitrary: kani::Arbitrary {}

#[cfg(kani)]
impl<T> KaniArbitrary for T where T: kani::Arbitrary {}

#[cfg(not(kani))]
pub(crate) trait KaniArbitrary {}

#[cfg(not(kani))]
impl<T> KaniArbitrary for T {}

pub(crate) fn symbolic_any<T>() -> T
where
    T: KaniArbitrary,
{
    #[cfg(kani)]
    {
        kani::any()
    }

    #[cfg(not(kani))]
    {
        panic!("KaniCompose symbolic construction is only available under cfg(kani)")
    }
}

pub(crate) fn kani_assume(condition: bool) {
    #[cfg(kani)]
    {
        kani::assume(condition);
    }

    #[cfg(not(kani))]
    {
        assert!(
            condition,
            "KaniCompose assumption violated outside cfg(kani)"
        );
    }
}

fn symbolic_ascii_char() -> char {
    let byte: u8 = symbolic_any();
    kani_assume(byte < 128);
    byte as char
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

impl KaniCompose for () {
    fn kani_depth0() -> Self {}

    fn kani_depth1() -> Self {}

    fn kani_depth2() -> Self {}

    fn kani_any() -> Self {}
}

impl_kani_compose_symbolic!(
    bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

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

    fn kani_any() -> Self {
        T::kani_vec_closure(1, 3)
    }
}

impl<T, const N: usize> KaniCompose for [T; N]
where
    T: KaniCompose,
{
    fn kani_depth0() -> Self {
        std::array::from_fn(|_| T::kani_depth0())
    }

    fn kani_depth1() -> Self {
        std::array::from_fn(|_| T::kani_depth1())
    }

    fn kani_depth2() -> Self {
        std::array::from_fn(|_| T::kani_depth2())
    }

    fn kani_any() -> Self {
        std::array::from_fn(|_| T::kani_any())
    }
}

impl<T> KaniCompose for Option<T>
where
    T: KaniCompose,
{
    fn kani_depth0() -> Self {
        None
    }

    fn kani_depth1() -> Self {
        Some(T::kani_depth0())
    }

    fn kani_depth2() -> Self {
        Some(T::kani_depth1())
    }

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
    fn kani_depth0() -> Self {
        std::collections::BTreeMap::new()
    }

    fn kani_depth1() -> Self {
        std::collections::BTreeMap::new()
    }

    fn kani_depth2() -> Self {
        std::collections::BTreeMap::new()
    }

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
    fn kani_depth0() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }

    fn kani_depth1() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }

    fn kani_depth2() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }

    fn kani_any() -> Self {
        std::collections::HashMap::with_hasher(S::default())
    }
}

impl<T> KaniCompose for Box<T>
where
    T: KaniCompose,
{
    fn kani_depth0() -> Self {
        Box::new(T::kani_depth0())
    }

    fn kani_depth1() -> Self {
        Box::new(T::kani_depth1())
    }

    fn kani_depth2() -> Self {
        Box::new(T::kani_depth2())
    }

    fn kani_any() -> Self {
        Box::new(T::kani_any())
    }
}

impl<A, B> KaniCompose for (A, B)
where
    A: KaniCompose,
    B: KaniCompose,
{
    fn kani_depth0() -> Self {
        (A::kani_depth0(), B::kani_depth0())
    }

    fn kani_depth1() -> Self {
        (A::kani_depth1(), B::kani_depth1())
    }

    fn kani_depth2() -> Self {
        (A::kani_depth2(), B::kani_depth2())
    }

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
    fn kani_depth0() -> Self {
        (A::kani_depth0(), B::kani_depth0(), C::kani_depth0())
    }

    fn kani_depth1() -> Self {
        (A::kani_depth1(), B::kani_depth1(), C::kani_depth1())
    }

    fn kani_depth2() -> Self {
        (A::kani_depth2(), B::kani_depth2(), C::kani_depth2())
    }

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
    fn kani_depth0() -> Self {
        (
            A::kani_depth0(),
            B::kani_depth0(),
            C::kani_depth0(),
            D::kani_depth0(),
        )
    }

    fn kani_depth1() -> Self {
        (
            A::kani_depth1(),
            B::kani_depth1(),
            C::kani_depth1(),
            D::kani_depth1(),
        )
    }

    fn kani_depth2() -> Self {
        (
            A::kani_depth2(),
            B::kani_depth2(),
            C::kani_depth2(),
            D::kani_depth2(),
        )
    }

    fn kani_any() -> Self {
        (A::kani_any(), B::kani_any(), C::kani_any(), D::kani_any())
    }
}

#[cfg(kani)]
mod proofs {
    use super::KaniCompose;

    #[derive(amenable_derive::KaniCompose, Debug, Clone, PartialEq, Eq)]
    struct DerivedNode {
        name: String,
        maybe_child: Option<Box<DerivedLeaf>>,
        flags: Vec<bool>,
    }

    #[derive(amenable_derive::KaniCompose, Debug, Clone, PartialEq, Eq)]
    struct DerivedLeaf(i32, Option<String>);

    #[derive(amenable_derive::KaniCompose, Debug, Clone, PartialEq, Eq)]
    enum DerivedChoice {
        Empty,
        Pair(String, Vec<i32>),
        Boxed { inner: Box<DerivedLeaf> },
    }

    amenable_derive::harness! {
        kani, VERIFY_KANI_COMPOSE_STRING_DEPTHS_SRC, {
            #[kani::proof]
            fn verify_kani_compose_string_depths() {
                assert!(String::kani_depth0().is_empty());
                assert_eq!(String::kani_depth1().len(), 1);
                assert_eq!(String::kani_depth2().len(), 2);
            }
        }
    }

    amenable_derive::harness! {
        kani, VERIFY_KANI_COMPOSE_VEC_DEPTHS_SRC, {
            #[kani::proof]
            fn verify_kani_compose_vec_depths() {
                assert!(Vec::<u8>::kani_depth0().is_empty());
                assert_eq!(Vec::<u8>::kani_depth1().len(), 1);
                assert_eq!(Vec::<u8>::kani_depth2().len(), 2);
            }
        }
    }

    amenable_derive::harness! {
        kani, VERIFY_KANI_COMPOSE_ARRAY_DEPTHS_SRC, {
            #[kani::proof]
            fn verify_kani_compose_array_depths() {
                let depth0 = <[u8; 3]>::kani_depth0();
                let depth1 = <[u8; 3]>::kani_depth1();
                let depth2 = <[u8; 3]>::kani_depth2();

                assert_eq!(depth0.len(), 3);
                assert_eq!(depth1.len(), 3);
                assert_eq!(depth2.len(), 3);
            }
        }
    }

    amenable_derive::harness! {
        kani, VERIFY_DERIVE_KANI_COMPOSE_STRUCT_SHAPES_SRC, {
            #[kani::proof]
            fn verify_derive_kani_compose_struct_shapes() {
                let depth0 = DerivedNode::kani_depth0();
                assert!(depth0.name.is_empty());
                assert!(depth0.maybe_child.is_none());
                assert!(depth0.flags.is_empty());

                let depth1 = DerivedNode::kani_depth1();
                assert_eq!(depth1.name.len(), 1);
                assert!(depth1.maybe_child.is_some());
                assert_eq!(depth1.flags.len(), 1);
            }
        }
    }

    amenable_derive::harness! {
        kani, VERIFY_DERIVE_KANI_COMPOSE_ENUM_IS_BOUNDED_SRC, {
            #[kani::proof]
            fn verify_derive_kani_compose_enum_is_bounded() {
                let choice = DerivedChoice::kani_any();
                match choice {
                    DerivedChoice::Empty => {}
                    DerivedChoice::Pair(name, values) => {
                        assert!(name.len() <= 4);
                        assert!(values.len() <= 3);
                    }
                    DerivedChoice::Boxed { inner } => {
                        let DerivedLeaf(value, maybe_text) = *inner;
                        let _ = value;
                        if let Some(text) = maybe_text {
                            assert!(text.len() <= 4);
                        }
                    }
                }
            }
        }
    }
}
