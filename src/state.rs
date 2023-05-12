use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub job_id: String,
    pub owner: Addr,
}

pub const RETRY_DELAY: Item<u64> = Item::new("retry_delay");
pub const STATE: Item<State> = Item::new("state");
pub const WITHDRAW_TIMESTAMP: Map<(&[u8], &[u8]), Timestamp> = Map::new("withdraw_timestamp");
