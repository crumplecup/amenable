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
