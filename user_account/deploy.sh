#!/bin/bash
set -e

near delete user.dao-sosmed.testnet dao-sosmed.testnet || :
near create-account user.dao-sosmed.testnet --masterAccount dao-sosmed.testnet --initialBalance 10
near deploy --wasmFile res/user_account.wasm --accountId user.dao-sosmed.testnet --initFunction new --initArgs '{}'