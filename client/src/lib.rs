use canister_state_macros::canister_state;
use ic_cdk::api::call::CallResult;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use types::{CanisterId, Cycles, Milliseconds, TimestampMillis};

canister_state!(State);

#[derive(Serialize, Deserialize)]
struct State {
    min_cycles_balance: Cycles,
    min_interval: Milliseconds,
    next_due: TimestampMillis,
    recent_invocations: VecDeque<InvocationResult>,
    cycles_dispenser_canister_id: CanisterId,
}

#[derive(Serialize, Deserialize)]
struct InvocationResult {
    timestamp: TimestampMillis,
    cycles_balance: Cycles,
    top_up_result: Option<Result<Cycles, String>>,
}

pub fn init(cycles_dispenser_canister_id: CanisterId, min_cycles_balance: Cycles) {
    init_state(State {
        min_cycles_balance,
        min_interval: 60 * 1000, // 1 minute
        next_due: 0,
        recent_invocations: VecDeque::new(),
        cycles_dispenser_canister_id,
    })
}

pub fn run() {
    let now = utils::time::now_millis();

    if let Some(top_up_request) =
        mutate_state(|state| is_due(now, state).then_some(run_internal(now, state))).flatten()
    {
        ic_cdk::spawn(request_top_up(top_up_request));
    };
}

pub fn serialize_to_bytes() -> Vec<u8> {
    read_state(|state| rmp_serde::to_vec_named(&state).unwrap())
}

pub fn init_from_bytes(bytes: &[u8]) {
    let state = rmp_serde::from_slice(bytes).unwrap();

    init_state(state);
}

fn is_due(now: TimestampMillis, state: &State) -> bool {
    state.next_due > now
}

fn run_internal(now: TimestampMillis, state: &mut State) -> Option<TopUpRequest> {
    state.next_due = now + state.min_interval;

    let cycles_balance = ic_cdk::api::canister_balance128();

    if cycles_balance < state.min_cycles_balance {
        Some(TopUpRequest {
            timestamp: now,
            cycles_balance,
            cycles_dispenser_canister_id: state.cycles_dispenser_canister_id,
        })
    } else {
        state.recent_invocations.push_back(InvocationResult {
            timestamp: now,
            cycles_balance,
            top_up_result: None,
        });
        None
    }
}

async fn request_top_up(request: TopUpRequest) {
    let args = cycles_dispenser::c2c_request_cycles::Args { amount: None };

    let response: CallResult<(cycles_dispenser::c2c_request_cycles::Response,)> = ic_cdk::call(
        request.cycles_dispenser_canister_id,
        "c2c_request_cycles",
        (&args,),
    )
    .await;

    let top_up_result = match response {
        Ok((result,)) => match result {
            cycles_dispenser::c2c_request_cycles::Response::Success(cycles) => Ok(cycles),
            error => Err(format!("{:?}", error)),
        },
        Err(error) => Err(format!("{:?}", error)),
    };

    mutate_state(|state| {
        state.recent_invocations.push_back(InvocationResult {
            timestamp: request.timestamp,
            cycles_balance: request.cycles_balance,
            top_up_result: Some(top_up_result),
        })
    })
}

struct TopUpRequest {
    timestamp: TimestampMillis,
    cycles_balance: Cycles,
    cycles_dispenser_canister_id: CanisterId,
}
