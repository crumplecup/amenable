//! Real Creusot proof content (`extern_spec!`/`harness!` Pearlite
//! functions) for `amenable_ext`'s jiff carriers — the proof-content
//! sibling to `ext::jiff`'s `CreusotWitness` bridge, exactly the same
//! split `rust_std`/`rust_std_witness` use and for the same reason (see
//! this crate's own root doc comment): `creusot-rustc`'s whole-crate
//! translation pass can't handle the ordinary Rust machinery
//! (`inventory::submit!`, trait dispatch) the witness bridge needs when
//! it's *local* to the translated crate, so that bridge stays
//! `#[cfg(not(creusot))]`-gated inside `ext`, and this module — pure
//! Pearlite proof content, the thing `cargo creusot` actually
//! translates — stays unconditional.

pub(crate) mod civil_date;
pub(crate) mod civil_era;
pub(crate) mod date_series;
pub(crate) mod date_time_series;
pub(crate) mod error;
pub(crate) mod offset;
pub(crate) mod signed_duration;
pub(crate) mod span;
pub(crate) mod span_fieldwise;
pub(crate) mod timestamp_series;
pub(crate) mod zoned_series;
