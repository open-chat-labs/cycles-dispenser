use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{CanisterId, Cycles, TimestampMillis};

#[derive(Serialize, Deserialize)]
pub struct Canisters {
    canisters: HashMap<CanisterId, Canister>,
}

impl Canisters {
    pub fn new(canister_ids: Vec<CanisterId>, now: TimestampMillis) -> Canisters {
        Canisters {
            canisters: canister_ids
                .into_iter()
                .map(|c| {
                    (
                        c,
                        Canister {
                            added: now,
                            top_ups: Vec::new(),
                            top_up_in_progress: false,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn get_mut(&mut self, canister_id: &CanisterId) -> Option<&mut Canister> {
        self.canisters.get_mut(canister_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Canister {
    added: TimestampMillis,
    top_ups: Vec<CyclesTopUp>,
    top_up_in_progress: bool,
}

impl Canister {
    pub fn top_up_in_progress(&self) -> bool {
        self.top_up_in_progress
    }

    pub fn set_top_up_in_progress(&mut self, in_progress: bool) {
        self.top_up_in_progress = in_progress;
    }

    pub fn latest_top_up(&self) -> Option<TimestampMillis> {
        self.top_ups.last().map(|t| t.date)
    }

    pub fn record_top_up(&mut self, amount: Cycles, now: TimestampMillis) {
        self.top_ups.push(CyclesTopUp { date: now, amount });
    }
}

#[derive(Serialize, Deserialize)]
struct CyclesTopUp {
    pub date: TimestampMillis,
    pub amount: Cycles,
}
