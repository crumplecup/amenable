//! `CreusotWitness` bridges for chrono's registered types. The proof content is in
//! `crate::ext_chrono`. These bridges stay `cfg(not(creusot))`, for the same reason
//! the jiff bridges do.

mod civil_naive_date;
mod fixed_offset;
mod naive_date_time;
mod naive_time;
mod utc;
