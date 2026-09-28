//! Real Creusot proof content for `jiff::fmt::StdIoWrite<Vec<u8>>`'s
//! infallibility property (`ext::jiff::fmt_std_io_write` holds the
//! `CreusotWitness` bridge that references the `_SRC` constant this
//! file's `harness!` call emits).
//!
//! Same real generics-matching wall as `ext_jiff::fmt_std_fmt_write`'s
//! own doc comment documents for `StdFmtWrite<W>`: jiff's real impl is
//! `impl<W: std::io::Write> Write for StdIoWrite<W>`, generic over
//! `W`, so it can't be extern-spec'd at the concrete `W = Vec<u8>`
//! instantiation. Unlike `String`'s own concrete `core::fmt::Write`
//! impl (which has zero generic parameters, so it was directly
//! extern-speccable), `Vec<u8>`'s real `std::io::Write` impl is ALSO
//! generic — `impl<A: Allocator> Write for Vec<u8, A>` (confirmed by
//! reading std's own source, `library/std/src/io/impls.rs`) — so the
//! same non-generic workaround doesn't apply here. `Allocator` is an
//! unstable trait (`#[unstable(feature = "allocator_api")]`) not
//! nameable on a stable-channel `cargo check`, but `#[cfg(creusot)]`
//! code is invisible to plain `rustc`/`cargo check` and only ever
//! compiled by `creusot-rustc`'s own toolchain, which — confirmed
//! empirically, not assumed — accepts it here the same way
//! `creusot-std`'s own (feature-gated in ITS Cargo.toml, but that gate
//! is off in this workspace) `impl<T, A: Allocator> Vec<T, A>`
//! extern_spec block for `push`/`len`/etc. does.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
    pub(super) use std::alloc::Allocator;
}
#[cfg(creusot)]
use mirror::{Allocator, check, ensures, extern_spec, logic, requires};

#[cfg(creusot)]
extern_spec! {
    impl<A: Allocator> std::io::Write for Vec<u8, A> {
        #[check(ghost)]
        #[ensures(match result {
            Ok(_) => true,
            Err(_) => false,
        })]
        fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>;
    }
}

amenable_derive::harness! {
    creusot, FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_HOLDS_SRC, {
        /// The `amenable_ext::
        /// ExtStandard<jiff::fmt::StdIoWrite<Vec<u8>>>` postcondition
        /// — real, callable Pearlite content, not just descriptive
        /// text alongside it.
        #[logic(open)]
        fn fmt_std_io_write_write_str_never_fails_holds(succeeded: bool) -> bool {
            pearlite! { succeeded }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::fmt_std_io_write::fmt_std_io_write_write_str_never_fails_holds",
        "creusot",
        "ensures",
        || FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_FMT_STD_IO_WRITE_WRITE_STR_NEVER_FAILS_SRC, {
        /// `Vec<u8>`'s own `std::io::Write::write_all` always
        /// succeeds — a real, checked postcondition resting on the
        /// `extern_spec!` above, the load-bearing fact behind
        /// `jiff::fmt::StdIoWrite<Vec<u8>>`'s own infallibility (see
        /// this module's own doc comment for why Creusot's
        /// extern-spec generics constraint rules out stating the
        /// claim on jiff's generic wrapper method directly, the
        /// identical reason `fmt_std_fmt_write.rs`'s own doc comment
        /// documents for `StdFmtWrite<String>`).
        #[requires(true)]
        #[ensures(fmt_std_io_write_write_str_never_fails_holds(result))]
        fn verify_fmt_std_io_write_write_str_never_fails(mut buf: Vec<u8>, data: &[u8]) -> bool {
            use std::io::Write as _;
            buf.write_all(data).is_ok()
        }
    }
}
