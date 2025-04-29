#! /usr/bin/env sh

set -ue

# Install the WASM target
rustup target add wasm32-wasi

# Install wasm runtime from checked out code
cargo install klyra-runtime --path runtime --bin klyra-next --features next

# cd into the WASM example
cd examples/next/hello-world

# Start locally
cargo klyra run &
sleep 70

echo "Testing local wasm endpoint"
output=$(curl --silent localhost:8000)
[ "$output" != "Hello, World!" ] && ( echo "Did not expect output: $output"; exit 1 )

killall cargo-klyra
