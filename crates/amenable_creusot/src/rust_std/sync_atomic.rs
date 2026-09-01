/// The `#[cfg(creusot)]` imports this file needs, consolidated into one
/// gate on this `mod` instead of one per item -- see `stoplight::mirror`'s
/// own doc comment for the general rationale. Every name is re-exported:
/// `atomic_sc_load_store_harness!`'s own expansions and the standalone
/// `AtomicPtr` harness below need all of them, unqualified, at this file's
/// own top level.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
    pub(super) use creusot_std::prelude::ghost;
    pub(super) use creusot_std::std::sync::atomic::Ordering::{
        None as AtomicNone, SeqCst as AtomicSeqCst,
    };
    pub(super) use creusot_std::std::sync::atomic_sc::{
        AtomicBool as CreusotAtomicBool, AtomicI8 as CreusotAtomicI8,
        AtomicI16 as CreusotAtomicI16, AtomicI32 as CreusotAtomicI32,
        AtomicI64 as CreusotAtomicI64, AtomicIsize as CreusotAtomicIsize,
        AtomicPtr as CreusotAtomicPtr, AtomicU8 as CreusotAtomicU8, AtomicU16 as CreusotAtomicU16,
        AtomicU32 as CreusotAtomicU32, AtomicU64 as CreusotAtomicU64,
        AtomicUsize as CreusotAtomicUsize,
    };
    pub(super) use creusot_std::std::sync::committer::Committer;
}
#[cfg(creusot)]
use mirror::{
    AtomicNone, AtomicSeqCst, Committer, CreusotAtomicBool, CreusotAtomicI8, CreusotAtomicI16,
    CreusotAtomicI32, CreusotAtomicI64, CreusotAtomicIsize, CreusotAtomicPtr, CreusotAtomicU8,
    CreusotAtomicU16, CreusotAtomicU32, CreusotAtomicU64, CreusotAtomicUsize, check, ensures,
    extern_spec, ghost, logic, requires, trusted,
};

macro_rules! atomic_sc_load_store_harness {
    ($const_name:ident, $fn_name:ident, $atomic_ty:ident, $value_ty:ty, $doc_atomic:literal) => {
        amenable_derive::harness! {
            creusot, $const_name, {
                #[doc = concat!(
                    "`",
                    $doc_atomic,
                    "::new` sets the value observable via `load`, and `store` overwrites it, under sequentially consistent ordering."
                )]
                #[requires(true)]
                #[ensures(result.0 == initial)]
                #[ensures(result.1 == next)]
                fn $fn_name(initial: $value_ty, next: $value_ty) -> ($value_ty, $value_ty) {
                    let (atomic, mut own) = $atomic_ty::new(initial);
                    let observed_initial = atomic.load(ghost!(
                        |c: &Committer<$atomic_ty, $value_ty, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
                    ));
                    atomic.store(
                        next,
                        ghost!(
                            |c: &mut Committer<$atomic_ty, $value_ty, AtomicNone, AtomicSeqCst>| c.shoot_store(&mut **own)
                        ),
                    );
                    let observed_next = atomic.load(ghost!(
                        |c: &Committer<$atomic_ty, $value_ty, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
                    ));
                    (observed_initial, observed_next)
                }
            }
        }
    };
}

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_BOOL_LOAD_STORE_SRC,
    verify_atomic_bool_load_store,
    CreusotAtomicBool,
    bool,
    "AtomicBool"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I8_LOAD_STORE_SRC,
    verify_atomic_i8_load_store,
    CreusotAtomicI8,
    i8,
    "AtomicI8"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I16_LOAD_STORE_SRC,
    verify_atomic_i16_load_store,
    CreusotAtomicI16,
    i16,
    "AtomicI16"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I32_LOAD_STORE_SRC,
    verify_atomic_i32_load_store,
    CreusotAtomicI32,
    i32,
    "AtomicI32"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_I64_LOAD_STORE_SRC,
    verify_atomic_i64_load_store,
    CreusotAtomicI64,
    i64,
    "AtomicI64"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_ISIZE_LOAD_STORE_SRC,
    verify_atomic_isize_load_store,
    CreusotAtomicIsize,
    isize,
    "AtomicIsize"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U8_LOAD_STORE_SRC,
    verify_atomic_u8_load_store,
    CreusotAtomicU8,
    u8,
    "AtomicU8"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U16_LOAD_STORE_SRC,
    verify_atomic_u16_load_store,
    CreusotAtomicU16,
    u16,
    "AtomicU16"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U32_LOAD_STORE_SRC,
    verify_atomic_u32_load_store,
    CreusotAtomicU32,
    u32,
    "AtomicU32"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_U64_LOAD_STORE_SRC,
    verify_atomic_u64_load_store,
    CreusotAtomicU64,
    u64,
    "AtomicU64"
);

atomic_sc_load_store_harness!(
    VERIFY_ATOMIC_USIZE_LOAD_STORE_SRC,
    verify_atomic_usize_load_store,
    CreusotAtomicUsize,
    usize,
    "AtomicUsize"
);

amenable_derive::harness! {
    creusot, VERIFY_ATOMIC_PTR_LOAD_STORE_SRC, {
        /// `AtomicPtr::new` sets the pointer observable via `load`, and
        /// `store` overwrites it, under sequentially consistent ordering.
        #[requires(true)]
        #[ensures(result.0)]
        #[ensures(result.1)]
        fn verify_atomic_ptr_load_store(initial: *mut i32, next: *mut i32) -> (bool, bool) {
            let (atomic, mut own) = CreusotAtomicPtr::new(initial);
            let observed_initial = atomic.load(ghost!(
                |c: &Committer<CreusotAtomicPtr<i32>, *mut i32, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
            ));
            atomic.store(
                next,
                ghost!(
                    |c: &mut Committer<CreusotAtomicPtr<i32>, *mut i32, AtomicNone, AtomicSeqCst>| c.shoot_store(&mut **own)
                ),
            );
            let observed_next = atomic.load(ghost!(
                |c: &Committer<CreusotAtomicPtr<i32>, *mut i32, AtomicSeqCst, AtomicNone>| c.shoot_load(&**own)
            ));
            (observed_initial.addr() == initial.addr(), observed_next.addr() == next.addr())
        }
    }
}
