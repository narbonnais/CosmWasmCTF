use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Bond native tokens to the contract
    Bond { denom: String, amount: Uint128 },
    /// Unbond native tokens from the contract
    Unbond { denom: String, amount: Uint128 },
    /// Add a vault
    AddVault { denom: String, address: String },
    /// Update config
    UpdateConfig { admin: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the native bonded balance of the given owner for the given denom
    Balance { owner: String, denom: String },
    /// Returns the corresponding cw20 vault address for the given denom
    Config {},
    /// Returns the config of the contract
    VaultAddress { denom: String },
    /// Returns all supported denoms
    DenomList {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub cw20_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultAddressResponse {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomResponse {
    pub denoms: Vec<String>,
}
