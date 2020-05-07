#!/bin/sh

# helper script for checking that all contracts and debug projects compile

cd adder
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin adder --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/adder.wasm adder.wasm
# wasm-snip target/wasm32-unknown-unknown/release/adder.wasm -o adder/output/adder.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd crypto-bubbles
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin crypto-bubbles --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/crypto-bubbles.wasm crypto-bubbles.wasm
# wasm-snip target/wasm32-unknown-unknown/release/crypto-bubbles.wasm -o crypto-bubbles.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd factorial
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin factorial --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/factorial.wasm factorial.wasm
# wasm-snip target/wasm32-unknown-unknown/release/factorial.wasm -o factorial.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd features
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin features --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/features.wasm features.wasm
# contains a panic so cannot remove panic code (fmt code also seems to be needed)
# wasm-snip features.wasm -o features.wasm #--snip-rust-fmt-code --snip-rust-panicking-code

cd features/async/alice
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin alice --target=wasm32-unknown-unknown --release
cd ../../..
mv target/wasm32-unknown-unknown/release/alice.wasm alice.wasm
# wasm-snip target/wasm32-unknown-unknown/release/alice.wasm -o alice.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd features/async/bob
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin bob --target=wasm32-unknown-unknown --release
cd ../../..
mv target/wasm32-unknown-unknown/release/bob.wasm bob.wasm
# wasm-snip target/wasm32-unknown-unknown/release/bob.wasm -o bob.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd simple-coin
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin simple-coin --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/simple-coin.wasm simple-coin.wasm
# wasm-snip target/wasm32-unknown-unknown/release/simple-coin.wasm -o simple-coin.wasm --snip-rust-fmt-code --snip-rust-panicking-code
