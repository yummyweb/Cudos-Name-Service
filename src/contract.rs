#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{DomainResponse, Entry, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Domain, TextRecord, REGISTRY};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cudos-name-service";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const MIN_NAME_LENGTH: u8 = 3;
const EXTENSION: &str = ".cudo";

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDomain { name, ttl, id } => {
            try_create_domain(deps, env, info, name, ttl, id)
        }
        ExecuteMsg::TransferRecord { id, new } => try_transfer_record(deps, env, info, id, new),
        ExecuteMsg::CreateRecord {
            url,
            avatar,
            email,
            id,
        } => try_create_record(deps, env, info, url, avatar, email, id),
    }
}

pub fn try_create_domain(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    ttl: u8,
    id: String,
) -> Result<Response, ContractError> {
    validate_name(&name)?;

    let text_record = TextRecord {
        url: String::from(""),
        avatar: String::from(""),
        email: String::from(""),
    };
    let domain = Domain {
        name: name + EXTENSION,
        owner: info.sender,
        ttl,
        text_record,
    };
    REGISTRY.save(deps.storage, &id, &domain)?;

    Ok(Response::new().add_attribute("method", "try_create_domain"))
}

pub fn try_transfer_record(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: String,
    new: String,
) -> Result<Response, ContractError> {
    let domain = REGISTRY.may_load(deps.storage, &id)?.unwrap();
    if domain.owner != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
            owner: domain.owner,
        });
    }
    REGISTRY.update(
        deps.storage,
        &id,
        |domain: Option<Domain>| -> StdResult<_> {
            Ok(Domain {
                name: (*domain.as_ref().unwrap().name).to_string(),
                owner: Addr::unchecked(new),
                ttl: domain.as_ref().unwrap().ttl,
                text_record: domain.unwrap().text_record,
            })
        },
    )?;
    Ok(Response::new().add_attribute("method", "try_create_record"))
}

pub fn try_create_record(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    url: String,
    avatar: String,
    email: String,
    id: String,
) -> Result<Response, ContractError> {
    let domain = REGISTRY.may_load(deps.storage, &id)?.unwrap();
    if domain.owner != info.sender {
        return Err(ContractError::Unauthorized {
            sender: info.sender,
            owner: domain.owner,
        });
    }
    REGISTRY.update(
        deps.storage,
        &id,
        |domain: Option<Domain>| -> StdResult<_> {
            Ok(Domain {
                name: (*domain.as_ref().unwrap().name).to_string(),
                ttl: domain.as_ref().unwrap().ttl,
                owner: domain.unwrap().owner,
                text_record: TextRecord {
                    url,
                    avatar,
                    email,
                },
            })
        },
    )?;
    Ok(Response::new().add_attribute("method", "try_create_record"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRecords {} => to_binary(&query_records(deps)?),
    }
}

fn query_records(deps: Deps) -> StdResult<DomainResponse> {
    let all: StdResult<Vec<_>> = REGISTRY
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut resp: Vec<Entry> = Vec::new();
    for (id, data) in all? {
        resp.push(Entry {
            name: data.name,
            owner: data.owner,
            ttl: data.ttl,
            text_record: data.text_record,
        });
    }
    Ok(DomainResponse { entries: resp })
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
