//! Gallery cases for the main `recommendation = "replace"` failure patterns.
//!
//! These cases are intentionally smaller than the production proofs they stand
//! in for. The goal is not to re-prove each library contract here, but to keep
//! an executable record of why certain proof families are poor fits for direct
//! Kani verification in this repository.
//!
//! The current issue classes are:
//!
//! - unsupported foreign boundaries in reachable std implementations
//! - Unix file-descriptor duplication paths that bottom out in `fcntl`
//! - anonymous pipe creation paths that bottom out in `pipe2`
//! - unsupported `#[track_caller]` / `Location::caller()` boundaries
//! - unsupported panic-capture boundaries
//! - Kani environment-model mismatches against real-process invariants
//! - PATH-style helper expansion that still times out in direct std execution
//! - first-pass concrete `String` / `Vec` PATH models that still leak too much
//!   owned-string machinery into Kani
//! - OS-backed filesystem boundaries with real external state
//! - pure in-memory std implementation blow-up that still times out under the
//!   native multi-minute harness timeout (`hash`, `fmt`, `BTree*`,
//!   `HashMap`/`HashSet`, `LinkedList::extract_if`, `String::from_utf8`,
//!   and similar cases)
//! - OS entropy-source boundaries reached by process-randomized seeding
//!   (`RandomState::new()`)
//! - thread-local-storage boundaries reached by `std::thread::current()`
//!   (`pthread_key_create`)
//! - real futex/clock syscall boundaries reached by `Barrier`/`Condvar`
//!   (`futex_wait`, `clock_gettime`)
//! - Kani's no-concurrency-support environment not enforcing real mutual
//!   exclusion for `Mutex::try_lock`
//! - `std::process::Command`/`Child` construction and spawning reaching
//!   several distinct unsupported foreign constructs (`strlen` via
//!   `CString`, `gnu_get_libc_version`, C string literals in `Stdio`)
//! - any `std::net` socket construction (`TcpListener`/`TcpStream`/
//!   `UdpSocket`) reaching an unsupported `socket` syscall
//! - reverse `str::pattern::Pattern` search (`rsplit`/`rsplitn`/
//!   `rsplit_terminator`/`rmatches`/`rmatch_indices`) times out even for a
//!   single `.next()` call on a five-byte fixed str, unlike every forward
//!   counterpart
//! - forward `str::pattern::Pattern` iteration (`split_terminator`/
//!   `matches`/`match_indices`) times out for real despite passing in an
//!   isolated probe crate — a methodological warning about probe-crate
//!   timing not predicting real-crate Kani/CBMC behavior
//!
//! Split by issue class, cases in their original order:
//! [`collections_and_foreign_boundary_timeouts`],
//! [`caller_panic_and_process_env_boundaries`],
//! [`path_model_and_inmemory_blowup_timeouts`],
//! [`entropy_tls_futex_and_concurrency_model_gaps`],
//! [`process_net_and_io_direct_std_timeouts`], and
//! [`io_and_str_pattern_direct_std_timeouts`].

mod caller_panic_and_process_env_boundaries;
mod collections_and_foreign_boundary_timeouts;
mod entropy_tls_futex_and_concurrency_model_gaps;
mod io_and_str_pattern_direct_std_timeouts;
mod path_model_and_inmemory_blowup_timeouts;
mod process_net_and_io_direct_std_timeouts;
