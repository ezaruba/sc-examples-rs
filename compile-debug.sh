#!/bin/sh

# helper script for checking that all contracts and debug projects compile

cd adder/debug
cargo build
cd ../..

cd crypto-bubbles/debug
cargo build
cd ../..

cd factorial/debug
cargo build
cd ../..

cd features/debug
cargo build

cd simple-coin
cargo build
cd ../..
