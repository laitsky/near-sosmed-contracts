#!/bin/bash
set -e

sh ./build.sh
near delete post.dao-sosmed.testnet dao-sosmed.testnet || :
near create-account post.dao-sosmed.testnet --masterAccount dao-sosmed.testnet --initialBalance 10
near deploy --wasmFile res/post_feature.wasm --accountId post.dao-sosmed.testnet --initFunction new --initArgs '{}'