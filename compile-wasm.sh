#!/bin/sh

# helper script for checking that all contracts and debug projects compile

cd adder
cargo build --bin adder --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/adder.wasm adder.wasm

cd crypto-bubbles
cargo build --bin crypto-bubbles --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/crypto-bubbles.wasm crypto-bubbles.wasm

cd factorial
cargo build --bin factorial --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/factorial.wasm factorial.wasm

cd features
cargo build --bin features --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/features.wasm features.wasm

cd features/async/alice
cargo build --bin alice --target=wasm32-unknown-unknown --release
cd ../../..
mv target/wasm32-unknown-unknown/release/alice.wasm alice.wasm

cd features/async/bob
cargo build --bin bob --target=wasm32-unknown-unknown --release
cd ../../..
mv target/wasm32-unknown-unknown/release/bob.wasm bob.wasm

cd simple-coin
cargo build --bin simple-coin --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/simple-coin.wasm simple-coin.wasm
