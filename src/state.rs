use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Domain {
    pub owner: Addr,
    pub name: String,
}

pub const REGISTRY: Map<&str, Domain> = Map::new("registry");
