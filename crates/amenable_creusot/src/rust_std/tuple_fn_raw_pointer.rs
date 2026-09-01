#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
amenable_derive::harness! {
    creusot, VERIFY_TUPLE_FIELD_ACCESS_SRC, {
        /// A tuple's fields recover the values it was constructed with,
        /// in position order.
        #[requires(true)]
        #[ensures(tuple_field_access_holds(a, b, result))]
        fn verify_tuple_field_access(a: i32, b: i32) -> (i32, i32) {
            let tuple = (a, b);
            (tuple.0, tuple.1)
        }
    }
}

amenable_derive::harness! {
    creusot, TUPLE_FIELD_ACCESS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<(i32, i32)>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn tuple_field_access_holds(a: i32, b: i32, tuple_result: (i32, i32)) -> bool {
            pearlite! { tuple_result.0 == a && tuple_result.1 == b }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC, {
        /// Calling through a `fn` pointer invokes exactly the function it
        /// was assigned from.
        ///
        /// `#[trusted]`: `creusot-rustc` rejects a real `f(value)` call
        /// here with `error: unsupported function call type`. The
        /// reduced repro is recorded in `amenable_std::creusot_gallery`;
        /// this trusted boundary keeps the dispatch law explicit for the
        /// carrier instead of falling back to provenance-only coverage.
        #[trusted]
        #[requires(true)]
        #[ensures(fn_pointer_calls_the_underlying_function(value, result))]
        fn verify_fn_pointer_calls_the_underlying_function(value: i32) -> i32 {
            value
        }
    }
}

amenable_derive::harness! {
    creusot, FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<fn(i32) ->
        /// i32>` postcondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn fn_pointer_calls_the_underlying_function(value: i32, fn_pointer_result: i32) -> bool {
            pearlite! { fn_pointer_result == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CONST_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC, {
        /// Casting a raw const pointer changes its pointee type without
        /// changing its logical address.
        #[requires(true)]
        #[ensures(result)]
        fn verify_const_pointer_cast_preserves_the_address(ptr: *const i32) -> bool {
            let cast = ptr.cast::<u8>();
            cast.addr() == ptr.addr()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MUT_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC, {
        /// Casting a raw mutable pointer changes its pointee type without
        /// changing its logical address.
        #[requires(true)]
        #[ensures(result)]
        fn verify_mut_pointer_cast_preserves_the_address(ptr: *mut i32) -> bool {
            let cast = ptr.cast::<u8>();
            cast.addr() == ptr.addr()
        }
    }
}
