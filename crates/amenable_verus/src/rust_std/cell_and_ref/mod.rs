mod cell_carrier;
pub use cell_carrier::{VerusCellModel, verify_cell_model_get_set_replace_round_trip};
#[cfg(verus_keep_ghost)]
pub use cell_carrier::{
    cell_model_get_reads_current_value, cell_model_get_set_replace_round_trip_holds,
    cell_model_new_stores_initial_value, cell_model_replace_returns_previous_value,
    write_stores_new_value,
};
mod lazy_cell_carrier;
pub use lazy_cell_carrier::{
    VerusLazyCellModel, verify_lazy_cell_model_caches_its_initializer_result,
};
#[cfg(verus_keep_ghost)]
pub use lazy_cell_carrier::{
    force_caches_on_first_call, force_returns_cached_value_on_later_calls,
    lazy_cell_uninitialized_has_no_cached_value,
};
mod once_cell_carrier;
pub use once_cell_carrier::{VerusOnceCellModel, verify_once_cell_model_initializes_exactly_once};
#[cfg(verus_keep_ghost)]
pub use once_cell_carrier::{
    once_cell_empty_has_no_value, once_cell_set_rejected_when_occupied,
    once_cell_set_succeeds_when_empty,
};
mod ref_carrier;
pub use ref_carrier::verify_ref_model_derefs_to_the_borrowed_value;
mod ref_cell_carrier;
pub use ref_cell_carrier::{VerusRefCellModel, verify_ref_cell_model_dynamic_borrow_rules};
#[cfg(verus_keep_ghost)]
pub use ref_cell_carrier::{
    release_shared_decrements_borrow_state, release_shared_requires_a_live_shared_borrow,
    try_borrow_headroom_holds, try_borrow_mut_result_matches, try_borrow_result_matches,
};
mod ref_mut_carrier;
pub use ref_mut_carrier::verify_ref_mut_model_derefs_and_writes_through_to_the_cell;
mod unsafe_cell_carrier;
pub use unsafe_cell_carrier::{
    VerusUnsafeCellModel, verify_unsafe_cell_model_get_mut_and_into_inner_round_trip,
};
