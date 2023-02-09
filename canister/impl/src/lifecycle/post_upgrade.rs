use crate::lifecycle::UPGRADE_BUFFER_SIZE;
use crate::{init_state, Data, State};
use canister_logger::LogEntry;
use canister_tracing_macros::trace;
use ic_cdk_macros::post_upgrade;
use stable_memory::deserialize_from_stable_memory;
use tracing::info;
use utils::env::canister::CanisterEnv;

#[post_upgrade]
#[trace]
fn post_upgrade() {
    ic_cdk::setup();

    let env = Box::new(CanisterEnv::new());
    let (data, logs, traces): (Data, Vec<LogEntry>, Vec<LogEntry>) =
        deserialize_from_stable_memory(UPGRADE_BUFFER_SIZE).unwrap();

    canister_logger::init_with_logs(false, logs, traces);

    init_state(State::new(env, data));

    crate::jobs::start();

    info!("Post-upgrade complete");
}
