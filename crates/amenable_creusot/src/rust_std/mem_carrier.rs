/// The `#[cfg(creusot)]` imports and trusted logic-wrapper function this
/// file needs, consolidated into one gate on this `mod` instead of one per
/// item -- see `stoplight::mirror`'s own doc comment for the general
/// rationale. Every name is re-exported: the `extern_spec! { .. }`/
/// `harness! { .. }` blocks below (both macro invocations, invisible to
/// the cfg-scatter scanner) reference all of it, unqualified, from this
/// file's own top level.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
    pub(super) use std::mem::ManuallyDrop;

    // `ManuallyDrop<T>` is uncontracted everywhere — no `creusot-std`
    // coverage (checked), no `elicitation` prior art (checked). Its private
    // field (`value: MaybeDangling<T>`, not a public `.0`) and `into_inner`'s
    // real `unsafe` raw-pointer-cast body rule out the field-projection idiom
    // `Wrapping`/`Saturating`/`Reverse` use, but that doesn't block a real
    // extern_spec here: every extern_spec in this file is already a trusted
    // axiom on the real method, never independently checked against the
    // method's own body (unsafe or not) — the same relationship `Ordering::
    // reverse`'s extern_spec has to its real `match` body. `new`/`deref`/
    // `into_inner` are each defined once, generically, over unconstrained `T`
    // (`impl<T: ?Sized> const Deref for ManuallyDrop<T>`, `impl<T> ManuallyDrop<T>`)
    // — no sealed/unstable bound the way `NonZero::new`'s `ZeroablePrimitive`
    // is, so the generic extern_spec form is directly writable.
    //
    // `*self` can't appear in `deref`'s OWN postcondition, though — `self:
    // &ManuallyDrop<T>`, so `*self` yields `ManuallyDrop<T>` (one layer of
    // reference removed), not the wrapped `T`; using `deref`'s own Target
    // inside a contract ABOUT `deref` is circular, confirmed by a real type
    // error, not a guess (`expected type parameter T, found struct
    // ManuallyDrop<T>`). Fixed the same way `String::len`/`NonZero::get`
    // route around a private/unreachable value: a `#[trusted]
    // #[logic(opaque)]` accessor axiomatizing "the value a `ManuallyDrop<T>`
    // wraps," generic over `T` this time (every earlier trusted wrapper in
    // this file was monomorphic).
    #[trusted]
    #[logic(opaque)]
    pub(super) fn manually_drop_value<T>(_m: &ManuallyDrop<T>) -> T {
        dead
    }
}
#[cfg(creusot)]
use mirror::{ManuallyDrop, check, ensures, extern_spec, logic, manually_drop_value, requires};

#[cfg(creusot)]
extern_spec! {
    impl<T> ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(manually_drop_value(&result) == value)]
        fn new(value: T) -> ManuallyDrop<T>;

        #[check(ghost)]
        #[ensures(result == manually_drop_value(&slot))]
        fn into_inner(slot: ManuallyDrop<T>) -> T;
    }

    impl<T> std::ops::Deref for ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(*result == manually_drop_value(self))]
        fn deref(&self) -> &T;
    }
}

amenable_derive::harness! {
    creusot, MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// ManuallyDrop<i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn manually_drop_derefs_and_into_inner_round_trip_holds(
            value: i32,
            manually_drop_result: (i32, i32),
        ) -> bool {
            pearlite! { manually_drop_result.0 == value && manually_drop_result.1 == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC, {
        /// `ManuallyDrop` is transparent to its wrapped value through
        /// both `Deref` and `into_inner` — the same claim
        /// `amenable_kani::rust_std::mem::verify_manually_drop_derefs_and_into_inner_round_trip`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, the same relationship every non-`char`/`String` harness
        /// in this file has to a trusted axiom on the real method it
        /// exercises.
        #[requires(true)]
        #[ensures(manually_drop_derefs_and_into_inner_round_trip_holds(value, result))]
        fn verify_manually_drop_derefs_and_into_inner_round_trip(value: i32) -> (i32, i32) {
            let wrapped = ManuallyDrop::new(value);
            let deref_value = *wrapped;
            (deref_value, ManuallyDrop::into_inner(wrapped))
        }
    }
}
