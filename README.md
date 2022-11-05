# CTF CosmWasm

Date: 2022-11-03

## Introduction

This is a CTF (Capture the Flag) challenge for the [CosmWasm](https://cosmwasm.com/) ecosystem. It is meant to be a fun way to learn about the ecosystem and the tools available. It is not a full introduction to the ecosystem, but rather a challenge to get you to learn and explore.

## Prerequisites

The following software is required to complete this challenge:

* [Go 1.18+](https://golang.org/dl/)
* [Wasmd 0.14.0](https://github.com/CosmWasm/wasmd/tree/v0.14.0)
* [Rust 1.55+](https://www.rust-lang.org/tools/install)

## Build the contracts

The contracts are written in Rust and compiled to WebAssembly. You can build them with the following command:

```bash
cargo build
cargo wasm
```

If everything is building correctly, you can then build the optimized WASM binaries with:

```bash
./scripts/optimize.sh # or `./scripts/optimize-arm.sh` if you are on an ARM machine
```

That will output the optimized WASM binaries in the `artifacts` directory.

## Run a local network

The first step is to run a local network. This is possible with the `wasmd` binary, that we need to clone, and build. We will use the `v0.14.0` tag, which is the latest stable release (on November 3rd 2022).

```bash
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd
git checkout v0.14.0 # if you are on arm, that version is not supported yet -- use the origin/main branch
make install
```

The binary should be found in your local `~/go/bin` folder, make sure it is in your `$PATH`. We can now start a local network of a simple node with the following script:

```bash
./run_localnet.sh
```

That will create a `testnet` folder holding all the data for the local network. You can stop the network with `Ctrl+C` and restart it with the same command.

## Run the live tests

The tests are written is JavaScript, and use the [CosmJS](https://github.com/cosmos/cosmjs) library. You need to install the dependencies with `npm install` and then deploy the contracts with `npm run deploy`. Modify the `index.js` file to hack the contracts.

> Before running the deployment script, make sure to have the local network running with `./run_localnet.sh`, and wait a few seconds for the node to be ready.

```bash
cd tests
npm install
npm run deploy
```

> If you are using the `./scripts/optimize-arm.sh` script, you need to modify the names of the WASM files on the javascript files, as the WASM files do not have the same name and now have a `-aarch64` suffix.

That will deploy the contracts, run some tests, and save the code ID and addresses of the contracts in the `tests/receipt.json` file.

## Hack the contracts

There you go ;)
