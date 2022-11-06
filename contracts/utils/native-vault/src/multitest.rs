#![cfg(test)]
use crate::error::ContractError;
use crate::msg::DenomResponse;
use cosmwasm_std::{coins, Addr, Coin, Empty, Uint128};
use cw20_base;
use cw_multi_test::{
    App, AppBuilder, BankSudo, Contract, ContractWrapper, Executor, SudoMsg as CwSudoMsg,
};

const NATIVE_DENOM: &str = "ucosm";
const UUSD_DENOM: &str = "uusd";
const DEPLOYER_FUNDS: u128 = 10_000_000_000_000;
const DEPLOYER: &str = "deployer";
const INITIAL_BALANCE: u128 = 1_000_000_000_000;
const CREATION_FEE: u128 = 1_000_000_000;
const ROB: &str = "rob";
const STEVE: &str = "steve";

fn custom_mock_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(DEPLOYER),
                vec![
                    Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(DEPLOYER_FUNDS),
                    },
                    Coin {
                        denom: UUSD_DENOM.to_string(),
                        amount: Uint128::new(DEPLOYER_FUNDS),
                    },
                ],
            )
            .unwrap();
    })
}

pub fn contract_native_vault() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::execute::execute,
        crate::instantiate::instantiate,
        crate::query::query,
    );
    Box::new(contract)
}

pub fn contract_cw20_base() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

// Instantiates all needed contracts for testing
fn setup_contracts(router: &mut App, deployer: &Addr) -> Result<(Addr, Addr, Addr), ContractError> {
    let _block_time = router.block_info().time;

    // Store the cw20 base contract
    let cw20_code_id = router.store_code(contract_cw20_base());

    // Store the native vault contract
    let native_vault_code_id = router.store_code(contract_native_vault());

    // Instantiate the native_vault contract
    let instantiate_msg = crate::msg::InstantiateMsg {};
    let native_vault = router
        .instantiate_contract(
            native_vault_code_id,
            deployer.clone(),
            &instantiate_msg,
            &coins(CREATION_FEE, NATIVE_DENOM),
            "Native Vault",
            None,
        )
        .unwrap();

    // Instantiate a cw20 ucosm contract with native_vault as the minter
    let instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: "ucosm".to_string(),
        symbol: "ATOM".to_string(),
        decimals: 6,
        initial_balances: vec![],
        marketing: None,
        mint: Some(cw20::MinterResponse {
            minter: native_vault.clone().to_string(),
            cap: None,
        }),
    };
    let cw20_ucosm = router
        .instantiate_contract(
            cw20_code_id,
            deployer.clone(),
            &instantiate_msg,
            &coins(CREATION_FEE, NATIVE_DENOM),
            "ucosm",
            None,
        )
        .unwrap();

    // Instantiate a cw20 uusd contract with native_vault as the minter
    let instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: "uusd".to_string(),
        symbol: "HUSD".to_string(),
        decimals: 6,
        initial_balances: vec![],
        marketing: None,
        mint: Some(cw20::MinterResponse {
            minter: native_vault.clone().to_string(),
            cap: None,
        }),
    };
    let cw20_uusd = router
        .instantiate_contract(
            cw20_code_id,
            deployer.clone(),
            &instantiate_msg,
            &coins(CREATION_FEE, NATIVE_DENOM),
            "ucosm",
            None,
        )
        .unwrap();

    // Add the cw20 ucosm contract to the native_vault contract
    let msg = crate::msg::ExecuteMsg::AddVault {
        denom: "ucosm".to_string(),
        address: cw20_uusd.clone().to_string(),
    };
    router
        .execute_contract(deployer.clone(), native_vault.clone(), &msg, &[])
        .unwrap();

    // Add the cw20 uusd contract to the native_vault contract
    let msg = crate::msg::ExecuteMsg::AddVault {
        denom: "uusd".to_string(),
        address: cw20_uusd.clone().to_string(),
    };
    router
        .execute_contract(deployer.clone(), native_vault.clone(), &msg, &[])
        .unwrap();

    Ok((native_vault, cw20_ucosm, cw20_uusd))
}

// Initializes accounts with some tokens
fn setup_accounts(router: &mut App) -> Result<(Addr, Addr, Addr), ContractError> {
    // Fund accounts with ucosm and uusd
    let mut funds: Vec<Coin> = coins(INITIAL_BALANCE, NATIVE_DENOM);
    funds.extend(coins(INITIAL_BALANCE, UUSD_DENOM));

    let deployer = Addr::unchecked(DEPLOYER);
    let rob: Addr = Addr::unchecked(ROB);
    let steve: Addr = Addr::unchecked(STEVE);

    router
        .sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: rob.to_string(),
                amount: funds.clone(),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();
    router
        .sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: steve.to_string(),
                amount: funds.clone(),
            }
        }))
        .map_err(|err| println!("{:?}", err))
        .ok();

    // Check native balances
    let rob_native_balances = router.wrap().query_all_balances(rob.clone()).unwrap();
    assert_eq!(rob_native_balances, funds);
    let steve_native_balances = router.wrap().query_all_balances(steve.clone()).unwrap();
    assert_eq!(steve_native_balances, funds);

    Ok((deployer, rob, steve))
}

#[test]
fn test_create_vault() {
    let mut router = custom_mock_app();
    let (deployer, _rob, _steve) = setup_accounts(&mut router).unwrap();
    let (native_vault, _cw20_ucosm, _cw20_uusd) = setup_contracts(&mut router, &deployer).unwrap();

    // Get native vault address
    let query_denoms = crate::msg::QueryMsg::DenomList {};
    let res: DenomResponse = router
        .wrap()
        .query_wasm_smart(native_vault.clone(), &query_denoms)
        .unwrap();
    assert_eq!(res.denoms.len(), 2);
}
