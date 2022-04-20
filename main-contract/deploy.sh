#!/bin/bash
set -e

sh ./build.sh
near delete x.dao-sosmed.testnet dao-sosmed.testnet || :
near create-account x.dao-sosmed.testnet --masterAccount dao-sosmed.testnet --initialBalance 10
near deploy --wasmFile res/main_contract.wasm --accountId x.dao-sosmed.testnet --initFunction new --initArgs '{}'