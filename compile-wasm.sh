#!/bin/sh

# helper script for checking that all contracts and debug projects compile

cd adder
cargo build --bin adder --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/adder.wasm adder.wasm
wasm-snip adder.wasm -o adder.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd crypto-bubbles
cargo build --bin crypto-bubbles --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/crypto-bubbles.wasm crypto-bubbles.wasm
wasm-snip crypto-bubbles.wasm -o crypto-bubbles.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd factorial
cargo build --bin factorial --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/factorial.wasm factorial.wasm
wasm-snip factorial.wasm -o factorial.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd features
cargo build --bin features --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/features.wasm features.wasm
wasm-snip features.wasm -o features.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd features/async/alice
cargo build --bin alice --target=wasm32-unknown-unknown --release
cd ../../..
mv target/wasm32-unknown-unknown/release/alice.wasm alice.wasm
wasm-snip alice.wasm -o alice.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd features/async/bob
cargo build --bin bob --target=wasm32-unknown-unknown --release
cd ../../..
mv target/wasm32-unknown-unknown/release/bob.wasm bob.wasm
wasm-snip bob.wasm -o bob.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd simple-coin
cargo build --bin simple-coin --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/simple-coin.wasm simple-coin.wasm
wasm-snip simple-coin.wasm -o simple-coin.wasm --snip-rust-fmt-code --snip-rust-panicking-code
