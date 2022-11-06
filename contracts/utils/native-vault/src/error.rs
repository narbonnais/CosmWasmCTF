use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Payment error: {0}")]
    PaymentError(String),

    #[error("Vault does not exist: {0}")]
    VaultDoesNotExist(String),

    #[error("Vault already exists: {0}")]
    VaultAlreadyExists(String),
}
