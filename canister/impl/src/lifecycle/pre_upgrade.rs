use crate::lifecycle::UPGRADE_BUFFER_SIZE;
use crate::take_state;
use canister_tracing_macros::trace;
use ic_cdk_macros::pre_upgrade;
use stable_memory::serialize_to_stable_memory;
use tracing::info;

#[pre_upgrade]
#[trace]
fn pre_upgrade() {
    info!("Pre-upgrade starting");

    let state = take_state();

    let log_messages = canister_logger::export_logs();
    let trace_messages = canister_logger::export_traces();

    let stable_state = (state.data, log_messages, trace_messages);

    serialize_to_stable_memory(stable_state, UPGRADE_BUFFER_SIZE).unwrap();
}
