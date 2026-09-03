//! `KaniWitness` impls for `core::cell`, split into `cell_family` (the
//! `Cell<T>` scalar carriers), `ref_cell` (`RefCell`/`Ref`/`RefMut` and the
//! borrow-error carriers), and `once_lazy_unsafe` (`OnceCell`/`UnsafeCell`/
//! `LazyCell` and the shared `GetterRecoversTheStoredReference` type).
//!
//! `UnsafeCell`'s only raw accessor (`.get()`, returning `*mut T`) needs an
//! `unsafe` block to dereference, and this crate forbids unsafe code
//! (`#![forbid(unsafe_code)]` in `lib.rs`) -- so its harness sticks to the
//! safe accessors (`get_mut`, `into_inner`) instead.

mod cell_family;
mod once_lazy_unsafe;
mod ref_cell;

pub use once_lazy_unsafe::GetterRecoversTheStoredReference;
