use crate::msg::{BalanceResponse, DenomResponse, QueryMsg};
use crate::state::{Config, CONFIG, VAULT_ADDRESSES};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, Order, QueryRequest, StdResult, WasmQuery};
use cw20::BalanceResponse as Cw20BalanceResponse;
use cw20::Cw20QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { owner, denom } => to_binary(&query_balance(
            deps,
            env,
            deps.api.addr_validate(owner.as_str())?,
            denom,
        )?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::VaultAddress { denom } => to_binary(&query_vault_address(deps, denom)?),
        QueryMsg::DenomList {} => to_binary(&query_denom_list(deps)?),
    }
}

/// Returns the native bonded balance of the given owner for the given denom
fn query_balance(deps: Deps, _env: Env, owner: Addr, denom: String) -> StdResult<BalanceResponse> {
    let vault_address = VAULT_ADDRESSES.load(deps.storage, denom)?;

    // Query balance
    let balance_response: Cw20BalanceResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: vault_address.to_string(),
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: owner.to_string(),
            })?,
        }))?;
    let balance = balance_response.balance;

    Ok(BalanceResponse { balance })
}

/// Returns the corresponding cw20 vault address for the given denom
fn query_vault_address(deps: Deps, denom: String) -> StdResult<Addr> {
    VAULT_ADDRESSES.load(deps.storage, denom)
}

/// Returns all supported denoms
fn query_denom_list(deps: Deps) -> StdResult<DenomResponse> {
    // Get all keys from VAULT_ADDRESSES
    let denoms: Vec<String> = VAULT_ADDRESSES
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|item| item.unwrap_or("Error while getting keys".to_string()))
        .collect();
    return Ok(DenomResponse { denoms });
}

/// Returns the config of the contract
fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
