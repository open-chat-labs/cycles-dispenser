use crate::model::canisters::{CanisterMetrics, Canisters};
use candid::{CandidType, Principal};
use canister_state_macros::canister_state;
use ic_ledger_types::{BlockIndex, Tokens};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use types::{CanisterId, Cycles, Milliseconds, TimestampMillis};
use utils::env::Environment;
use utils::memory;

mod guards;
mod jobs;
mod lifecycle;
mod model;
mod queries;
mod updates;

canister_state!(State);

struct State {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl State {
    pub fn new(env: Box<dyn Environment>, data: Data) -> State {
        State { env, data }
    }

    pub fn is_caller_admin(&self) -> bool {
        self.data.admins.contains(&self.env.caller())
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            memory_used: memory::used(),
            now: self.env.now(),
            cycles_balance: self.env.cycles_balance(),
            admins: self.data.admins.iter().copied().collect(),
            canisters: self.data.canisters.metrics(),
            max_top_up_amount: self.data.max_top_up_amount,
            min_interval: self.data.min_interval,
            min_cycles_balance: self.data.min_cycles_balance,
            icp_burn_amount: self.data.icp_burn_amount,
            ledger_canister: self.data.ledger_canister,
            cycles_minting_canister: self.data.cycles_minting_canister,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub admins: HashSet<Principal>,
    pub canisters: Canisters,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
    pub icp_burn_amount: Tokens,
    pub ledger_canister: CanisterId,
    pub cycles_minting_canister: CanisterId,
    pub cycles_top_up_pending_notification: Option<BlockIndex>,
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        admins: Vec<Principal>,
        canisters: Vec<CanisterId>,
        max_top_up_amount: Cycles,
        min_interval: Milliseconds,
        min_cycles_balance: Cycles,
        icp_burn_amount: Tokens,
        ledger_canister: CanisterId,
        cycles_minting_canister: CanisterId,
        now: TimestampMillis,
    ) -> Data {
        Data {
            admins: admins.into_iter().collect(),
            canisters: Canisters::new(canisters, now),
            max_top_up_amount,
            min_interval,
            min_cycles_balance,
            icp_burn_amount,
            ledger_canister,
            cycles_minting_canister,
            cycles_top_up_pending_notification: None,
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub admins: Vec<Principal>,
    pub canisters: Vec<CanisterMetrics>,
    pub max_top_up_amount: Cycles,
    pub min_interval: Milliseconds,
    pub min_cycles_balance: Cycles,
    pub icp_burn_amount: Tokens,
    pub ledger_canister: CanisterId,
    pub cycles_minting_canister: CanisterId,
}
