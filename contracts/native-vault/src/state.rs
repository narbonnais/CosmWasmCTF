use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    /// The address of the admin that can create new vaults
    pub admin: Addr,
}

/// The config of the contract
pub const CONFIG: Item<Config> = Item::new("config");

/// Links the native denom to the corresponding cw20 vault
pub const VAULT_ADDRESSES: Map<String, Addr> = Map::new("vault_addresses");
