# Native Vault

This contract is a simple wrapper around native tokens to make them compatible with CW20. It is a simple contract that is meant to be a bridge between CW20 and native tokens: by bonding native tokens, you can mint CW20 tokens, and by unbonding CW20 tokens, you can burn them and get back native tokens.

## Messages

### Instantiate

The instantiate message is empty. Nothing to see here :)

### Execute

#### Bond

The bond message allows to bond native tokens to the contract. It will mint the same amount of cw20 tokens to the sender on the corresponding cw20 contract.

> Note: you need to send funds with the message.

```rust
{
    "bond": { "denom": String, "amount": Uint128 }
}
```

#### Unbond

The unbond message allows to unbond native tokens from the contract. It will burn the same amount of cw20 tokens from the sender on the corresponding cw20 contract and refund the native tokens.

> Note: you need to increase your allowance on the cw20 contract for NativeVault as a spender before calling this message.

```rust
{
    "unbond": { "denom": String, "amount": Uint128 }
}
```

#### AddVault

The add_vault message will bind a cw20 contract to the native vault contract. This will allow to bond and unbond cw20 tokens to the native vault contract.

```rust
{
    "add_vault": { "denom": String, "address": String }
}
```

#### UpdateConfig

The update_config message allows to update the configuration of the native vault contract.

```rust
{
    "update_config": {
        "admin": Option<String>,
    }
}
```