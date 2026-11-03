//! Caps a spawned `cargo kani` child's own virtual memory, so a
//! verification run that explodes (a genuine CBMC state-space blowup,
//! not a hypothetical) fails cleanly -- the child sees its own
//! allocation fail and CBMC reports "out of memory", the same shape
//! `cargo kani ... 2>&1` already prints when the cap is hit -- instead
//! of exhausting the host's real RAM and taking the rest of the
//! machine down with it. This is the same protection every `cargo
//! kani` call in this workspace's own verification history has needed
//! applied manually (`ulimit -v <bytes> && cargo kani ...`); this
//! module makes `amenable verify kani` apply it automatically, with a
//! user-adjustable threshold instead of a hand-picked constant.
//!
//! Implemented without `Command::pre_exec`/raw `libc::setrlimit`,
//! deliberately: those require an `unsafe` block at the call site
//! (`pre_exec` itself is an `unsafe fn`, regardless of what the
//! closure does), and this crate's own `#![forbid(unsafe_code)]`
//! cannot be locally overridden for one module. Instead this lowers
//! *this* process's own `RLIMIT_AS` immediately before `spawn` --
//! `fork` copies the parent's rlimits into the child at that instant,
//! so the spawned `cargo kani` inherits the lowered cap -- then
//! restores the original limit right after `spawn` returns. The one
//! window where this process itself runs under the lowered cap is the
//! `spawn` call alone, which allocates nothing of its own; the
//! restore never affects the child, since its rlimits were already
//! copied at fork time, not kept live against the parent's.

use std::io;
use std::process::{Child, Command};

use tracing::instrument;

/// Default fraction of currently available memory a spawned `cargo
/// kani` child may use, as a percentage. Chosen to leave real
/// headroom for the rest of the machine -- a CBMC blowup can grow
/// right up to whatever ceiling it's given, so the cap has to sit
/// meaningfully below total available memory, not just below it.
pub const DEFAULT_MEMORY_LIMIT_PERCENT: u8 = 75;

/// Reads `MemAvailable` from `/proc/meminfo`, in bytes -- the kernel's
/// own estimate of memory available for new allocations without
/// swapping, not raw total memory (which ignores what's already in
/// use) and not raw free memory (which ignores reclaimable cache).
/// `None` off Linux, or if the file is missing/unparsable -- callers
/// treat that as "cannot compute a limit", not a hard error, since a
/// missing memory cap is a safety regression, never a reason to
/// refuse to run a proof at all.
#[instrument(level = "debug")]
fn available_memory_bytes() -> Option<u64> {
    let contents = std::fs::read_to_string("/proc/meminfo").ok()?;
    contents.lines().find_map(|line| {
        let rest = line.strip_prefix("MemAvailable:")?;
        let kib: u64 = rest.trim().strip_suffix("kB")?.trim().parse().ok()?;
        Some(kib.saturating_mul(1024))
    })
}

/// The byte threshold for `percent` of currently available memory, or
/// `None` if available memory could not be determined.
#[instrument(level = "debug")]
fn memory_limit_bytes(percent: u8) -> Option<u64> {
    let available = available_memory_bytes()?;
    Some(available.saturating_mul(u64::from(percent)) / 100)
}

/// Spawns `command`, capping its own virtual memory (`RLIMIT_AS`) at
/// `percent` of memory available right now -- see this module's own
/// doc comment for the lower/spawn/restore mechanism. `percent == 0`
/// spawns with no cap, an explicit opt-out rather than a silent one.
/// Spawns uncapped (not an error) if available memory can't be
/// determined, or if reading/restoring the rlimit itself fails -- a
/// missed cap is a safety regression, never a reason to refuse to run
/// a proof outright.
#[cfg(unix)]
#[instrument(level = "debug", skip(command))]
pub fn spawn_with_memory_limit(mut command: Command, percent: u8) -> io::Result<Child> {
    if percent == 0 {
        return command.spawn();
    }
    let Some(limit_bytes) = memory_limit_bytes(percent) else {
        return command.spawn();
    };

    let original = rlimit::Resource::AS.get().ok();
    let _ = rlimit::Resource::AS.set(limit_bytes, limit_bytes);
    let result = command.spawn();
    if let Some((soft, hard)) = original {
        let _ = rlimit::Resource::AS.set(soft, hard);
    }
    result
}

/// No-op passthrough off Unix -- see this module's own doc comment.
#[cfg(not(unix))]
#[instrument(level = "debug", skip(command))]
pub fn spawn_with_memory_limit(mut command: Command, _percent: u8) -> io::Result<Child> {
    command.spawn()
}
