#!/usr/bin/env sh

scp ubuntu@18.133.52.140:/opt/klyra/user-data/users/users.toml users.toml
cargo run -- users.toml > users.sql


scp users.sql controller.klyra.internal:~/users.sql
ssh controller.klyra.internal "cat ~/users.sql | sudo sqlite3 /var/lib/docker/volumes/klyra-dev_gateway-vol/_data/gateway.sqlite"
