use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{CONFIG, VAULT_ADDRESSES};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};
use cw_utils::one_coin;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bond { denom, amount } => execute_bond(deps, env, info, denom, amount),
        ExecuteMsg::Unbond { denom, amount } => execute_unbond(deps, env, info, denom, amount),
        ExecuteMsg::AddVault { denom, address } => {
            execute_add_vault(deps, env, info, denom, address)
        }
        ExecuteMsg::UpdateConfig { admin } => execute_update_config(deps, env, info, admin),
    }
}

/// Update config
pub fn execute_update_config(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    admin: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if let Some(admin) = admin {
        config.admin = deps.api.addr_validate(&admin)?;
    }

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

/// Bond native tokens to the contract
pub fn execute_bond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let _coin = one_coin(&info)
        .map_err(|e| ContractError::PaymentError(format!("Expected one coin, got: {}", e)))?;

    // Get the address of the vault for this denom
    let vault_address = VAULT_ADDRESSES.load(deps.storage, denom).map_err(
        // Vault doesn't exist error
        |e| ContractError::VaultDoesNotExist(e.to_string()),
    )?;

    // Mint the tokens
    let mint_msg = cw20_base::msg::ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount: amount,
    };
    let mint_cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: vault_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "bond")
        .add_message(mint_cosmos_msg))
}

/// Unbond native tokens from the contract
pub fn execute_unbond(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Get the address of the vault for this denom
    let vault_address = VAULT_ADDRESSES.load(deps.storage, denom.clone()).map_err(
        // Vault doesn't exist error
        |e| ContractError::VaultDoesNotExist(e.to_string()),
    )?;

    // Burn the tokens
    let burn_from_msg = cw20_base::msg::ExecuteMsg::BurnFrom {
        owner: info.sender.to_string(),
        amount,
    };
    let burn_cosmos_msg = WasmMsg::Execute {
        contract_addr: vault_address.to_string(),
        msg: to_binary(&burn_from_msg)?,
        funds: vec![],
    };

    // Send the funds to the user
    let send_cosmos_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: denom.clone(),
            amount: amount.clone(),
        }],
    };

    Ok(Response::new()
        .add_attribute("action", "unbond")
        .add_message(burn_cosmos_msg)
        .add_message(send_cosmos_msg))
}

/// Add a new vault to the contract
pub fn execute_add_vault(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    denom: String,
    address: String,
) -> Result<Response, ContractError> {
    // Add the vault to the list of vaults
    VAULT_ADDRESSES.save(
        deps.storage,
        denom.clone(),
        &deps.api.addr_validate(&address)?,
    )?;

    // Make sure that we are the minter by minting one token and burning it
    let mint_msg = cw20_base::msg::ExecuteMsg::Mint {
        recipient: env.contract.address.to_string(),
        amount: Uint128::from(1u128),
    };
    let mint_cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    let burn_msg = cw20_base::msg::ExecuteMsg::Burn {
        amount: Uint128::from(1u128),
    };
    let burn_cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: address.to_string(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "add_vault")
        .add_message(mint_cosmos_msg)
        .add_message(burn_cosmos_msg))
}
