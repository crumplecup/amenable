//! `KaniWitness` impls for `std::process`.
//!
//! The direct `Command` / `Child` paths hit unsupported libc probes,
//! `CString` conversion, and pipe-construction machinery under Kani today.
//! The reduced direct failures remain preserved in the proof gallery, while
//! production proofs use Amenable-owned bounded process observations to carry
//! the Rust-facing laws each carrier is supposed to expose.
//!
//! Split by the real API family each file covers: [`child_and_streams`]
//! (`Child`, `ChildStderr`, `ChildStdin`, `ChildStdout`), [`command`]
//! (`Command`, `CommandArgs`, `CommandEnvs`), and
//! [`exit_status_output_stdio`] (`ExitStatus`, `Output`, `Stdio`, plus the
//! trusted `ExitCode`).

mod child_and_streams;
mod command;
mod exit_status_output_stdio;
