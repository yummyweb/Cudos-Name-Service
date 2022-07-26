use cosmwasm_std::{StdError, Addr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized ({sender} is not {owner})")]
    Unauthorized { sender: Addr, owner: Addr },

    #[error("Name too short (length {length} min_length {min_length})")]
    NameTooShort { length: u64, min_length: u64 },
}
