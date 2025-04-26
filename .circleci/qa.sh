#! /usr/bin/env sh

set -ue

# Prepare directory
mkdir -p /tmp/qa-linux
cd /tmp/qa-linux

# Init app
cargo klyra init --name qa-linux --axum

# Start locally
cargo klyra run &
sleep 70

echo "Testing local hello endpoint"
output=$(curl --silent localhost:8000/hello)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

killall cargo-klyra

cargo klyra project start

cargo klyra deploy --allow-dirty

echo "Testing remote hello endpoint"
output=$(curl --silent https://qa-linux.unstable.klyraapp.rs/hello)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

cargo klyra project stop

exit 0
