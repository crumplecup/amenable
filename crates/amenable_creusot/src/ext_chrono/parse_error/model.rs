//! `ZoneNameModelVariant`, gated file-wide (not per-item, matching
//! `gap_info::carrier`'s own precedent) so this file's `use` and
//! `enum` -- two different item kinds under one `cfg` -- don't
//! themselves trip cordial's `CFG-SCATTER` check in the parent file.

#![cfg(creusot)]

/// Two real zone names, standing in for chrono-tz's real ~597-name
/// space -- the same move `TzOffsetModelVariant` uses for offsets.
/// Both are chrono-tz's own real, shortest names (confirmed by reading
/// its real prebuilt table), chosen because they're the *tightest*
/// real case for the length argument in the parent module.
#[derive(Clone, Copy)]
pub enum ZoneNameModelVariant {
    /// `"GB"`.
    Gb,
    /// `"NZ"`.
    Nz,
}
