#!/bin/sh

# helper script for checking that all contracts and debug projects compile

rm adder.wasm
rm crypto-bubbles.wasm
rm factorial.wasm
rm simple-coin.wasm
rm features.wasm
rm alice.wasm
rm bob.wasm

cargo clean
