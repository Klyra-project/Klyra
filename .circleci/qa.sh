#! /usr/bin/env sh

set -ue

# Prepare directory
mkdir -p /tmp/qa-$1
cd /tmp/qa-$1

# Init app
cargo klyra init --name qa-$1 --axum

# Start locally
cargo klyra run &
sleep 150

echo "Testing local hello endpoint"
output=$(curl --silent localhost:8000)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

killall cargo-klyra

cargo klyra project start

cargo klyra deploy --allow-dirty

echo "Testing remote hello endpoint"
output=$(curl --silent https://qa-$1.unstable.klyraapp.rs)
[ "$output" != "Hello, world!" ] && ( echo "Did not expect output: $output"; exit 1 )

cargo klyra project stop

exit 0
