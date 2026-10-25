use amenable_core::Ensures;
use amenable_std::{
    ComposeAnyLengthHasMinimum, ComposeAnyLengthIsBounded, ComposeArrayLengthIsFixed,
    ComposeDepthZeroIsEmpty, ComposeFieldPresenceTracksDepth,
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
            // `.chars().count()` would state the exact-char-count claim
            // directly, but it's expensive for Kani in its own right --
            // `core::str::count::char_count_general_case` has a loop
            // CBMC cannot cheaply bound even over 1-2 chars, confirmed
            // directly (unwound 1480+ iterations, harness timeout).
            // `.len()` (byte count) is a cheap O(1) stored-field read,
            // but since `kani_depth1/2` now draw from the full Unicode
            // domain (`amenable_kani::compose::model::symbolic_char`,
            // fixed from an ASCII-only restriction -- see
            // `amenable_kani::gallery::string_compose_ascii_restriction`),
            // a multi-byte `char` makes exact byte-length no longer
            // match char count. `ComposeAnyLengthIsBounded`'s `<=`
            // check states the true, cheaply-checkable bound instead
            // (at most 4 UTF-8 bytes per `char`): weaker than "exactly
            // N", but correct for any char, not just ASCII.
            assert!(ComposeDepthZeroIsEmpty::ensures(String::kani_depth0().len()));
            let depth1 = String::kani_depth1();
            assert!(ComposeAnyLengthHasMinimum::ensures((depth1.len(), 1)));
            assert!(ComposeAnyLengthIsBounded::ensures((depth1.len(), 4)));
            let depth2 = String::kani_depth2();
            assert!(ComposeAnyLengthHasMinimum::ensures((depth2.len(), 2)));
            assert!(ComposeAnyLengthIsBounded::ensures((depth2.len(), 8)));
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
            // Bounded, not exact byte length -- see
            // `verify_kani_compose_string_depths`'s own comment.
            assert!(ComposeAnyLengthHasMinimum::ensures((depth1.name.len(), 1)));
            assert!(ComposeAnyLengthIsBounded::ensures((depth1.name.len(), 4)));
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
                    // `kani_any`'s bound (4) is on char count; byte
                    // length can be up to 4x that for multi-byte
                    // chars -- see `verify_kani_compose_string_depths`'s
                    // own comment on why `.len()`, not
                    // `.chars().count()`, is the cheap check here.
                    assert!(ComposeAnyLengthIsBounded::ensures((name.len(), 16)));
                    assert!(ComposeAnyLengthIsBounded::ensures((values.len(), 3)));
                }
                DerivedChoice::Boxed { inner } => {
                    let DerivedLeaf(value, maybe_text) = *inner;
                    let _ = value;
                    if let Some(text) = maybe_text {
                        assert!(ComposeAnyLengthIsBounded::ensures((text.len(), 16)));
                    }
                }
            }
        }
    }
}
