import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { readFileSync, writeFileSync } from "fs";

const artifactsPath = process.cwd() + "/../artifacts/";
const receiptsPath = process.cwd() + "/receipts.json";
const mnemonic = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
const rpcEndpoint = "http://127.0.0.1:26657/";
const prefix = "wasm";

const wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, { prefix });
const [deployer] = await wallet.getAccounts();

const client = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, {
    gasPrice: "0.1ucosm", // this is needed because we use "auto" gas estimation in the transactions
});

// --------------------------------------------- 
// Query accounts
// ---------------------------------------------

// Get balance
const account = await client.getAccount(deployer.address);
console.log("Account:", account);

// Account: {
//     address: 'wasm1cyyzpxplxdzkeea7kwsydadg87357qna465cff',
//     pubkey: null,
//     accountNumber: 1,
//     sequence: 0
// }

const balance = await client.getBalance(deployer.address, "ucosm");
console.log("Balance:", balance);

// Balance: { denom: 'ucosm', amount: '1000000000' }

// --------------------------------------------- 
// Deploy bytecode
// ---------------------------------------------

const uploadCW20Result = await client.upload(deployer.address, readFileSync(artifactsPath + "cw20.wasm"), "auto");
console.log("Upload CW20 result:", uploadCW20Result);

// Upload result: {
//     originalSize: 323365,
//     originalChecksum: '3d44caecf41cbb7cf5404f86e03269a87e6cc271db1f093e0a74c2e81458ad41',
//     compressedSize: 103031,
//     compressedChecksum: '32ab1f812102e93ac551278a076d450e34e837ff7861bf6a2a49d89fca315d26',
//     codeId: 1,
//     logs: [ { msg_index: 0, log: '', events: [Array] } ],
//     height: 21,
//     transactionHash: 'AD37BF690E80B938118EB4D3119D051789016B4E7F464CCC3983AE5EEE3D9019',
//     gasWanted: 2697614,
//     gasUsed: 2089475
// }

const uploadNativeVaultResult = await client.upload(deployer.address, readFileSync(artifactsPath + "native_vault.wasm"), "auto");
console.log("Upload NativeVault result:", uploadNativeVaultResult);

// Upload result: {
//     originalSize: 229428,
//     originalChecksum: 'bf8f7e1f4110aaac010916be9c270bae3bea46f02a0063ee8dd07a2f753853ce',
//     compressedSize: 76085,
//     compressedChecksum: '44444f68555ffc64a128f1e1741e170b8424c4053b5561bac7e663002fb55eaf',
//     codeId: 2,
//     logs: [ { msg_index: 0, log: '', events: [Array] } ],
//     height: 22,
//     transactionHash: '3163334AA27D5C289A29208DE56F6913704978F30D77B1C646592CC78CE1C9BF',
//     gasWanted: 1967190,
//     gasUsed: 1527597
// }

const instantiateNativeVaultResult = await client.instantiate(
    deployer.address, uploadNativeVaultResult.codeId, {}, "Initial native vault", "auto");
console.log("Instantiate NativeVault result:", instantiateNativeVaultResult);

// Instantiate result: {
//     contractAddress: 'wasm1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqhs9hr8',
//     logs: [ { msg_index: 0, log: '', events: [Array] } ],
//     height: 4,
//     transactionHash: '6AE8E734FB425B54383179A084AB79D248066435260DBA82F5197300396D3DCB',
//     gasWanted: 187749,
//     gasUsed: 158757
// }

let queryDenomListResult = await client.queryContractSmart(instantiateNativeVaultResult.contractAddress, { denom_list: {} });
console.log("Query DenomList result:", queryDenomListResult);

// Query result: { denom_list: [] }

// --------------------------------------------- 
// Create CW20 tokens
// ---------------------------------------------

console.log(instantiateNativeVaultResult.contractAddress);
const instantiateAtomCW20Result = await client.instantiate(
    deployer.address, uploadCW20Result.codeId, {
    name: "Atom",
    symbol: "ATOM",
    decimals: 6,
    initial_balances: [],
    mint: {
        minter: instantiateNativeVaultResult.contractAddress,
        cap: null
    },
}, "Initial ATOM", "auto");

console.log("Instantiate ATOM CW20 result:", instantiateAtomCW20Result);
console.log("");

const instantiateHusdCW20Result = await client.instantiate(
    deployer.address, uploadCW20Result.codeId, {
    name: "Husd",
    symbol: "HUSD",
    decimals: 6,
    initial_balances: [],
    mint: {
        minter: instantiateNativeVaultResult.contractAddress,
    },
}, "Initial HUSD", "auto");

console.log("Instantiate HUSD CW20 result:", instantiateAtomCW20Result);
console.log("");

// --------------------------------------------- 
// Add the CW20 tokens to the native vault
// ---------------------------------------------

const executeAddAtomResult = await client.execute(
    deployer.address, instantiateNativeVaultResult.contractAddress, {
    add_vault: {
        denom: "ucosm",
        address: instantiateAtomCW20Result.contractAddress,
    },
}, "auto");

console.log("Execute AddAtom result:", executeAddAtomResult);
console.log("");

const executeAddHusdResult = await client.execute(
    deployer.address, instantiateNativeVaultResult.contractAddress, {
    add_vault: {
        denom: "uusd",
        address: instantiateHusdCW20Result.contractAddress,
    },
}, "auto");

console.log("Execute AddHusd result:", executeAddAtomResult);
console.log("");

// --------------------------------------------- 
// Some tests
// ---------------------------------------------

queryDenomListResult = await client.queryContractSmart(instantiateNativeVaultResult.contractAddress, { denom_list: {} });
console.log("Query DenomList result:", queryDenomListResult);

// Query result: { denoms: [ 'ucosm', 'uusd' ] }

// Query the address of the CW20 contract for the given denom
const queryATOMDenomInfoResult = await client.queryContractSmart(instantiateNativeVaultResult.contractAddress, { vault_address: { denom: "ucosm" } });
console.log("Query ATOM DenomInfo result:", queryATOMDenomInfoResult);

// Query ATOM DenomInfo result: wasm1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrss5maay

const queryHUSDDenomInfoResult = await client.queryContractSmart(instantiateNativeVaultResult.contractAddress, { vault_address: { denom: "uusd" } });
console.log("Query HUSD DenomInfo result:", queryHUSDDenomInfoResult);

// Query HUSD DenomInfo result: wasm1yyca08xqdgvjz0psg56z67ejh9xms6l436u8y58m82npdqqhmmtqas0cl7

// --------------------------------------------- 
// Keep receipts
// ---------------------------------------------

// Write receipts code ID and addresses to file
const receipts = {
    cw20: uploadCW20Result.codeId,
    native_vault: {
        code_id: uploadNativeVaultResult.codeId,
        address: instantiateNativeVaultResult.contractAddress,
    },
    atom_cw20: instantiateAtomCW20Result.contractAddress,
    husd_cw20: instantiateHusdCW20Result.contractAddress,
};
writeFileSync(receiptsPath, JSON.stringify(receipts, null, 2));