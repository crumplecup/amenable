mod net_carrier;
#[cfg(verus_keep_ghost)]
pub use net_carrier::ip_addr_model_v4_octets_match_input;
pub use net_carrier::{
    verify_ip_addr_model_variant_matches_its_kind, verify_ipv4_addr_model_octets_round_trip,
    verify_ipv6_addr_model_segments_round_trip, verify_socket_addr_model_variant_matches_its_kind,
    verify_socket_addr_v4_model_round_trips_ip_and_port,
    verify_socket_addr_v6_model_round_trips_all_fields,
};
mod process_child_carrier;
#[cfg(verus_keep_ghost)]
pub use process_child_carrier::process_id_is_nonzero;
pub use process_child_carrier::{
    verify_child_model_has_a_process_id_and_can_be_waited_on,
    verify_child_stderr_model_captures_what_the_child_wrote_to_stderr,
    verify_child_stdin_model_is_readable_by_the_child_process,
    verify_child_stdout_model_captures_what_the_child_wrote_to_stdout,
};
mod process_command_carrier;
pub use process_command_carrier::{
    verify_command_args_model_reports_the_configured_arguments,
    verify_command_envs_model_reports_the_configured_overrides,
    verify_command_model_env_override_is_visible_to_the_spawned_process,
};
mod process_exit_carrier;
#[cfg(verus_keep_ghost)]
pub use process_exit_carrier::{exit_code_is_nonzero, output_exit_code_is_success};
pub use process_exit_carrier::{
    verify_exit_status_model_reports_a_nonzero_exit_code,
    verify_output_model_captures_stdout_and_the_exit_status,
};
mod process_stdio_carrier;
pub use process_stdio_carrier::verify_stdio_model_null_discards_the_childs_output_handle;
mod std_net_carrier;
pub use std_net_carrier::{
    verify_incoming_model_yields_an_already_queued_connection,
    verify_shutdown_model_write_prevents_further_writes,
    verify_tcp_listener_model_accepts_a_connecting_stream,
    verify_tcp_stream_model_delivers_written_bytes_to_the_accepted_peer,
    verify_udp_socket_model_send_to_recv_from_round_trips_a_datagram,
};
