//! `RustStdType` registrations for `core::fmt`.
//!
//! `FormattingOptions` (unstable `formatting_options`) and `NumBuffer<T>`
//! (unstable `int_format_into`, and generic over a sealed
//! `NumBufferTrait` besides) are deliberately not covered here.

use core::fmt::{
    Alignment, Arguments, DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple, Error, Formatter,
    FromFn,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_lifetime0,
    register_rust_std_standard_evidence,
};

impl_rust_std_type!(
    Alignment,
    "core",
    "core::fmt",
    "https://doc.rust-lang.org/core/fmt/enum.Alignment.html",
    "The Alignment carrier names a formatting fill alignment as Left, Right, or Center."
);

impl_rust_std_type_lifetime0!(
    Arguments,
    "core",
    "core::fmt",
    "https://doc.rust-lang.org/core/fmt/struct.Arguments.html",
    "The Arguments carrier holds the precompiled pieces produced by format_args!, ready to be written out."
);

impl_rust_std_type!(
    Error,
    "core",
    "core::fmt",
    "https://doc.rust-lang.org/core/fmt/struct.Error.html",
    "The Error carrier signals that a formatting trait implementation failed to write to its Formatter."
);

impl_rust_std_type_lifetime0!(
    Formatter,
    "core",
    "core::fmt",
    "https://doc.rust-lang.org/core/fmt/struct.Formatter.html",
    "The Formatter carrier configures how a Display/Debug implementation writes output, and receives the written output itself."
);

// The Debug*-builder family (DebugStruct, DebugTuple, DebugList, DebugMap,
// DebugSet) shares the same '<'a, 'b: 'a>' shape (a borrowed Formatter plus
// its own borrow lifetime), which none of the shared macros model — each
// gets its own impl instead of a one-off macro for just five call sites.
macro_rules! impl_debug_builder {
    ($ty:ident, $summary:expr) => {
        impl<'a, 'b: 'a> crate::RustStdType for $ty<'a, 'b> {
            fn rust_language_provenance() -> crate::RustLanguageProvenance {
                crate::RustLanguageProvenance::for_source("core", "core::fmt")
            }

            fn rust_doc_url() -> &'static str {
                concat!(
                    "https://doc.rust-lang.org/core/fmt/struct.",
                    stringify!($ty),
                    ".html"
                )
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

impl_debug_builder!(
    DebugList,
    "The DebugList carrier incrementally builds a Debug-formatted list representation."
);
impl_debug_builder!(
    DebugMap,
    "The DebugMap carrier incrementally builds a Debug-formatted map representation."
);
impl_debug_builder!(
    DebugSet,
    "The DebugSet carrier incrementally builds a Debug-formatted set representation."
);
impl_debug_builder!(
    DebugStruct,
    "The DebugStruct carrier incrementally builds a Debug-formatted struct representation."
);
impl_debug_builder!(
    DebugTuple,
    "The DebugTuple carrier incrementally builds a Debug-formatted tuple representation."
);

impl_rust_std_type_generic1!(
    FromFn,
    "core",
    "core::fmt",
    "https://doc.rust-lang.org/core/fmt/struct.FromFn.html",
    "The FromFn carrier implements Debug and Display by forwarding to a supplied closure."
);

// `'static` is the representative lifetime this module's proof batch
// covers (the Debug*-builder family needs two, `'a, 'b`, per its own
// `impl<'a, 'b: 'a>` shape above).
//
// Alignment/Error are written fully-qualified: `core::mem::Alignment`
// (unstable, but already present in rustdoc's own inventory) and
// `std::io::Error` share these bare names — only the qualified path
// disambiguates which one a given registration means for tooling reading
// the registry (e.g. `elicit_doc`'s coverage report).
register_rust_std_standard_evidence!(
    std::fmt::Alignment,
    Arguments<'static>,
    std::fmt::Error,
    Formatter<'static>,
    DebugList<'static, 'static>,
    DebugMap<'static, 'static>,
    DebugSet<'static, 'static>,
    DebugStruct<'static, 'static>,
    DebugTuple<'static, 'static>,
    FromFn<fn(&mut Formatter<'_>) -> std::fmt::Result>,
);
