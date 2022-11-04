import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { coins, DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { readFileSync } from "fs";

const artifactsPath = process.cwd() + "/../artifacts/";
const receiptsPath = process.cwd() + "/receipts.json";
const rob_mnemonic = "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty"
const steve_mnemonic = "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb"
const rpcEndpoint = "http://127.0.0.1:26657/";
const prefix = "wasm";

const rob_wallet = await DirectSecp256k1HdWallet.fromMnemonic(rob_mnemonic, { prefix });
const [rob] = await rob_wallet.getAccounts();
const steve_wallet = await DirectSecp256k1HdWallet.fromMnemonic(steve_mnemonic, { prefix });
const [steve] = await steve_wallet.getAccounts();

const rob_client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, rob_wallet, {
    gasPrice: "0.1ustake", // this is needed because we use "auto" gas estimation in the transactions
});
const steve_client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, steve_wallet, {
    gasPrice: "0.1ustake", // this is needed because we use "auto" gas estimation in the transactions
});

// Read receipts file
const receipts = JSON.parse(readFileSync(receiptsPath, "utf8"));
const nativeVaultAddress = receipts.native_vault.address;
const hustCW20Address = receipts.husd_cw20;
const atomCW20Address = receipts.atom_cw20;
console.log("Native Vault address:", nativeVaultAddress);
console.log("ATOM CW20 address:", atomCW20Address);
console.log("HUSD CW20 address:", hustCW20Address);
console.log("");

// Native Vault address: wasm1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqhs9hr8

const robAccount = await rob_client.getAccount(rob.address);
console.log("Rob's account:", robAccount);
console.log("");


// Rob's account: {
//     address: 'wasm18s5lynnmx37hq4wlrw9gdn68sg2uxp5r23gln4',
//     pubkey: null,
//     accountNumber: 2,
//     sequence: 0
// }

const steveAccount = await steve_client.getAccount(steve.address);
console.log("Steve's account:", steveAccount);
console.log("");

// Steve's account: {
//     address: 'wasm18s5lynnmx37hq4wlrw9gdn68sg2uxp5r23gln4',
//     pubkey: null,
//     accountNumber: 2,
//     sequence: 0
// }

// Rob's balance before
const robBalanceBefore = await rob_client.getBalance(rob.address, "ucosm");
console.log("Rob's balance before:", robBalanceBefore);
console.log("");

// Rob deposits 1000 ucosm to the native vault
const executeBondResult = await rob_client.execute(rob.address, nativeVaultAddress, { bond: { denom: "ucosm", amount: "1000" } }, "auto", "bond 1000 ucosm", coins(1000, "ucosm"));
console.log("Rob's bond 1000ucosm result:", executeBondResult);
console.log("");

// Rob's bond result: {
//     logs: [ { msg_index: 0, log: '', events: [Array] } ],
//     height: 145,
//     transactionHash: '10E280A7CAFF8BDC6D0E2DA2011C3126E32F71B0D72B57045A3B7565B81910E3',
//     gasWanted: 262968,
//     gasUsed: 216619
// }

// Rob's balance should be 1000 ucosm less
const robBalanceAfter = await rob_client.getBalance(rob.address, "ucosm");
console.log("Rob's balance after:", robBalanceAfter);
console.log("");

// Query the cw20 ucosm of Rob
const robAtomCW20Balance = await rob_client.queryContractSmart(atomCW20Address, { balance: { address: rob.address } });
console.log("Rob's ATOM CW20 balance:", robAtomCW20Balance);
console.log("");

// Increase allowance from Rob to the native vault
const executeIncreaseAllowanceResult = await rob_client.execute(rob.address, atomCW20Address, { increase_allowance: { spender: nativeVaultAddress, amount: "1000", expires: null } }, "auto", "set allowance 1000");
console.log("Rob's set allowance 1000 result:", executeIncreaseAllowanceResult);
console.log("");

// Unbond 1000 ucosm from the native vault
const executeUnbondResult = await rob_client.execute(rob.address, nativeVaultAddress, { unbond: { denom: "ucosm", amount: "1000" } }, "auto", "unbond 1000 ucosm");
console.log("Rob's unbond 1000ucosm result:", executeUnbondResult);
console.log("");


// Rob's balance should be 1000 ucosm more
const robBalanceAfterUnbond = await rob_client.getBalance(rob.address, "ucosm");
console.log("Rob's balance after unbond:", robBalanceAfterUnbond);
console.log("");

// Query the cw20 ucosm of Rob
const robAtomCW20BalanceAfterUnbond = await rob_client.queryContractSmart(atomCW20Address, { balance: { address: rob.address } });
console.log("Rob's ATOM CW20 balance after unbond:", robAtomCW20BalanceAfterUnbond);
console.log("");
