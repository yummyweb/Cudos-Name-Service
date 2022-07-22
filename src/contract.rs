#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{Entry, ExecuteMsg, InstantiateMsg, QueryMsg, RecordResponse};
use crate::state::{Domain, REGISTRY};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cudos-name-service";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MIN_NAME_LENGTH: u8 = 3;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateRecord { name, id } => try_create_record(deps, info, name, id),
    }
}

pub fn try_create_record(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    id: String,
) -> Result<Response, ContractError> {
    validate_name(&name)?;
    let domain = Domain {
        name,
        owner: info.sender,
    };
    REGISTRY.save(deps.storage, &id, &domain)?;

    Ok(Response::new().add_attribute("method", "try_create_record"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRecords {} => to_binary(&query_records(deps)?),
    }
}

fn query_records(deps: Deps) -> StdResult<RecordResponse> {
    let all: StdResult<Vec<_>> = REGISTRY
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut resp: Vec<Entry> = Vec::new();
    for (id, data) in all? {
        resp.push(Entry {
            name: data.name,
            owner: data.owner,
        });
    }
    Ok(RecordResponse { entries: resp })
}

fn validate_name(name: &str) -> Result<(), ContractError> {
    let length = name.len() as u64;

    if length < (MIN_NAME_LENGTH as u64) {
        Err(ContractError::NameTooShort {
            length,
            min_length: MIN_NAME_LENGTH as u64,
        })
    } else {
        Ok(())
    }
}
