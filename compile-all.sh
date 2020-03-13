#!/bin/sh

# helper script for checking that all contracts and debug projects compile

cd adder
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd debug
cargo build
cd ../..

cd crypto-bubbles
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd debug
cargo build
cd ../..

cd factorial
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd debug
cargo build
cd ../..

cd features
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd debug
cargo build
cd ../async/alice
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd ../bob
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd ../../..

cd simple-coin
cargo build --bin wasm --target=wasm32-unknown-unknown --release
cd debug
cargo build
cd ../..
