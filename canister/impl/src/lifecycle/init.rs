use crate::{init_state, Data, State};
use canister_tracing_macros::trace;
use cycles_dispenser::init::Args;
use ic_cdk_macros::init;
use ic_ledger_types::Tokens;
use tracing::info;
use utils::env::canister::CanisterEnv;
use utils::env::Environment;

#[init]
#[trace]
fn init(args: Args) {
    ic_cdk::setup();
    canister_logger::setup(false);

    let env = Box::new(CanisterEnv::new());
    let data = Data::new(
        args.admins,
        args.canisters,
        args.max_top_up_amount,
        args.min_interval,
        args.min_cycles_balance,
        Tokens::from_e8s(args.icp_burn_amount_e8s),
        args.ledger_canister,
        args.cycles_minting_canister,
        env.now(),
    );
    let state = State::new(env, data);

    init_state(state);
    crate::jobs::start();

    info!("Initialization complete");
}
