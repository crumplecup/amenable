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

/// The `#[cfg(kani)]` `KaniCompose` trait, its supporting helpers, and
/// every hand-written `impl KaniCompose` this file needs, consolidated
/// into one gate on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. `impl_kani_compose_symbolic!`'s own generated
/// impls (its invocation below, unconditional -- macro invocations are
/// invisible to the cfg-scatter scanner) reference `KaniCompose` from
/// outside this module, so `KaniCompose` is re-exported `pub`, matching
/// `lib.rs`'s own `pub use compose::KaniCompose;` at the crate's public
/// boundary; `symbolic_any`/`kani_assume` are re-exported `pub(crate)`,
/// matching every other model file's own `use crate::compose::{
/// kani_assume, symbolic_any};`. `symbolic_ascii_char` stays private: only
/// `String`'s own impl, inside this module, ever calls it.
#[cfg(kani)]
mod mirror {
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

    pub(crate) fn symbolic_any<T>() -> T
    where
        T: kani::Arbitrary,
    {
        kani::any()
    }

    pub(crate) fn kani_assume(condition: bool) {
        kani::assume(condition);
    }

    fn symbolic_ascii_char() -> char {
        let byte: u8 = symbolic_any();
        kani_assume(byte < 128);
        byte as char
    }

    impl KaniCompose for () {
        fn kani_depth0() -> Self {}

        fn kani_depth1() -> Self {}

        fn kani_depth2() -> Self {}

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
        fn kani_depth0() -> Self {
            Self::from_u128(symbolic_any())
        }

        fn kani_depth1() -> Self {
            Self::from_u128(symbolic_any())
        }

        fn kani_depth2() -> Self {
            Self::from_u128(symbolic_any())
        }

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
}
#[cfg(kani)]
pub use mirror::KaniCompose;
#[cfg(kani)]
pub(crate) use mirror::{kani_assume, symbolic_any};

macro_rules! impl_kani_compose_symbolic {
    ($($ty:ty),* $(,)?) => {
        $(
            #[cfg(kani)]
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

// Registered unconditionally (not inside `#[cfg(kani)] mod proofs` below)
// so `amenable dump-registry`'s ordinary, non-Kani build still sees these
// `ContractRecord`s -- the self-test proof bodies that call them are
// Kani-only (the whole module is `#[cfg(kani)]`-gated, since the derived
// `KaniCompose` impls it exercises call `kani::any()`/`kani::assume`
// directly, and this crate has no unconditional `kani` dependency), but
// the named claim they call into is not. Each carries the "trusted"
// disposition (chain-derived provenance only, no dedicated harness of its
// own) rather than pointing at one of the three self-test harnesses that
// use it, since those harnesses aren't nameable from unconditional code.
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

macro_rules! impl_compose_claim_witness {
    ($ty:ty) => {
        impl crate::KaniWitness for $ty {
            type SupportingEvidence = Self;
            type ProofArtifact = amenable_std::RustStdProvenance;

            fn proof() -> Self::ProofArtifact {
                amenable_core::Evidence::audit(
                    &<Self::SupportingEvidence as amenable_core::Evidence>::basis(),
                )
            }
        }

        bridge_kani_witness!($ty);
    };
}

impl_compose_claim_witness!(amenable_std::ComposeDepthZeroIsEmpty);
impl_compose_claim_witness!(amenable_std::ComposeFieldPresenceTracksDepth);
impl_compose_claim_witness!(amenable_std::ComposeAnyLengthIsBounded);
impl_compose_claim_witness!(amenable_std::ComposeArrayLengthIsFixed);

kani_ensures!(
    amenable_std::ComposeDepthZeroIsEmpty,
    "amenable_std::ComposeDepthZeroIsEmpty",
    usize,
    |len| len == 0
);

kani_ensures!(
    amenable_std::ComposeFieldPresenceTracksDepth,
    "amenable_std::ComposeFieldPresenceTracksDepth",
    (bool, usize),
    |(is_some, depth)| is_some == (depth > 0)
);

kani_ensures!(
    amenable_std::ComposeAnyLengthIsBounded,
    "amenable_std::ComposeAnyLengthIsBounded",
    (usize, usize),
    |(actual, bound)| actual <= bound
);

kani_ensures!(
    amenable_std::ComposeArrayLengthIsFixed,
    "amenable_std::ComposeArrayLengthIsFixed",
    (usize, usize),
    |(actual, expected)| actual == expected
);

#[cfg(kani)]
mod proofs {
    use amenable_core::Ensures;
    use amenable_std::{
        ComposeAnyLengthIsBounded, ComposeArrayLengthIsFixed, ComposeDepthZeroIsEmpty,
        ComposeFieldPresenceTracksDepth,
    };

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
                assert!(ComposeDepthZeroIsEmpty::ensures(String::kani_depth0().len()));
                assert!(ComposeArrayLengthIsFixed::ensures((String::kani_depth1().len(), 1)));
                assert!(ComposeArrayLengthIsFixed::ensures((String::kani_depth2().len(), 2)));
            }
        }
    }

    amenable_derive::harness! {
        kani, VERIFY_KANI_COMPOSE_VEC_DEPTHS_SRC, {
            #[kani::proof]
            fn verify_kani_compose_vec_depths() {
                assert!(ComposeDepthZeroIsEmpty::ensures(Vec::<u8>::kani_depth0().len()));
                assert!(ComposeArrayLengthIsFixed::ensures((Vec::<u8>::kani_depth1().len(), 1)));
                assert!(ComposeArrayLengthIsFixed::ensures((Vec::<u8>::kani_depth2().len(), 2)));
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

                assert!(ComposeArrayLengthIsFixed::ensures((depth0.len(), 3)));
                assert!(ComposeArrayLengthIsFixed::ensures((depth1.len(), 3)));
                assert!(ComposeArrayLengthIsFixed::ensures((depth2.len(), 3)));
            }
        }
    }

    amenable_derive::harness! {
        kani, VERIFY_DERIVE_KANI_COMPOSE_STRUCT_SHAPES_SRC, {
            #[kani::proof]
            fn verify_derive_kani_compose_struct_shapes() {
                let depth0 = DerivedNode::kani_depth0();
                assert!(ComposeDepthZeroIsEmpty::ensures(depth0.name.len()));
                assert!(ComposeFieldPresenceTracksDepth::ensures((
                    depth0.maybe_child.is_some(),
                    0
                )));
                assert!(ComposeDepthZeroIsEmpty::ensures(depth0.flags.len()));

                let depth1 = DerivedNode::kani_depth1();
                assert!(ComposeArrayLengthIsFixed::ensures((depth1.name.len(), 1)));
                assert!(ComposeFieldPresenceTracksDepth::ensures((
                    depth1.maybe_child.is_some(),
                    1
                )));
                assert!(ComposeArrayLengthIsFixed::ensures((depth1.flags.len(), 1)));
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
                        assert!(ComposeAnyLengthIsBounded::ensures((name.len(), 4)));
                        assert!(ComposeAnyLengthIsBounded::ensures((values.len(), 3)));
                    }
                    DerivedChoice::Boxed { inner } => {
                        let DerivedLeaf(value, maybe_text) = *inner;
                        let _ = value;
                        if let Some(text) = maybe_text {
                            assert!(ComposeAnyLengthIsBounded::ensures((text.len(), 4)));
                        }
                    }
                }
            }
        }
    }
}
