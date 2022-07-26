use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Domain {
    pub owner: Addr,
    pub name: String,
    pub ttl: u8,
    pub text_record: TextRecord,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TextRecord {
    pub url: String,
    pub avatar: String,
    pub email: String,
}

pub const REGISTRY: Map<&str, Domain> = Map::new("registry");
