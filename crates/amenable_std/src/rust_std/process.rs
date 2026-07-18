//! `RustStdType` registrations for `std::process`.
//!
//! `ExitStatusError` is deliberately not covered here — unstable
//! (`exit_status_error`, rust-lang/rust#84908), despite `ExitStatus`
//! itself being stable.

use std::process::{
    Child, ChildStderr, ChildStdin, ChildStdout, Command, CommandArgs, CommandEnvs, ExitCode,
    ExitStatus, Output, Stdio,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
};

impl_rust_std_type!(
    Child,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.Child.html",
    "The Child carrier is a handle to a running or exited child process."
);

impl_rust_std_type!(
    ChildStderr,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.ChildStderr.html",
    "The ChildStderr carrier is a handle to a child process's standard error stream."
);

impl_rust_std_type!(
    ChildStdin,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.ChildStdin.html",
    "The ChildStdin carrier is a handle to a child process's standard input stream."
);

impl_rust_std_type!(
    ChildStdout,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.ChildStdout.html",
    "The ChildStdout carrier is a handle to a child process's standard output stream."
);

impl_rust_std_type!(
    Command,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.Command.html",
    "The Command carrier configures and spawns a child process."
);

impl_rust_std_type_lifetime0!(
    CommandArgs,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.CommandArgs.html",
    "The CommandArgs carrier lazily yields the arguments configured on a Command."
);

impl_rust_std_type_lifetime0!(
    CommandEnvs,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.CommandEnvs.html",
    "The CommandEnvs carrier lazily yields the environment variable overrides configured on a Command."
);

impl_rust_std_type!(
    ExitCode,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.ExitCode.html",
    "The ExitCode carrier represents a process exit status in a portable, non-i32 form suitable for returning from main."
);

impl_rust_std_type!(
    ExitStatus,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.ExitStatus.html",
    "The ExitStatus carrier describes how a child process terminated."
);

impl_rust_std_type!(
    Output,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.Output.html",
    "The Output carrier bundles a finished child process's exit status with its collected stdout and stderr."
);

impl_rust_std_type!(
    Stdio,
    "std",
    "std::process",
    "https://doc.rust-lang.org/std/process/struct.Stdio.html",
    "The Stdio carrier configures how a child process's stdin/stdout/stderr should be handled."
);

// `'static` is the representative lifetime this module's proof batch
// covers.
register_rust_std_standard_evidence!(
    Child,
    ChildStderr,
    ChildStdin,
    ChildStdout,
    Command,
    CommandArgs<'static>,
    CommandEnvs<'static>,
    ExitCode,
    ExitStatus,
    Output,
    Stdio,
);
