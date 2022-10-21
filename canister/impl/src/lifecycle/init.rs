use crate::{init_state, Data, State};
use canister_tracing_macros::trace;
use cycles_dispenser::init::Args;
use ic_cdk_macros::init;
use tracing::info;
use utils::env::canister::CanisterEnv;
use utils::env::Environment;

#[init]
#[trace]
fn init(args: Args) {
    ic_cdk::setup();

    let env = Box::new(CanisterEnv::new());
    let data = Data::new(
        args.admins,
        args.canisters,
        args.top_up_amount,
        args.min_interval,
        args.min_cycles_balance,
        env.now(),
    );
    let state = State::new(env, data);

    init_state(state);

    info!("Initialization complete");
}
