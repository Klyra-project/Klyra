#!/usr/bin/env sh

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################

# Patch crates to be on same versions
mkdir -p $CARGO_HOME; \
echo '[patch.crates-io]
klyra-service = { path = "/usr/src/klyra/service" }
klyra-runtime = { path = "/usr/src/klyra/runtime" }
klyra-aws-rds = { path = "/usr/src/klyra/resources/aws-rds" }
klyra-persist = { path = "/usr/src/klyra/resources/persist" }
klyra-shared-db = { path = "/usr/src/klyra/resources/shared-db" }
klyra-secrets = { path = "/usr/src/klyra/resources/secrets" }
klyra-static-folder = { path = "/usr/src/klyra/resources/static-folder" }' > $CARGO_HOME/config.toml

# Add the wasm32-wasi target
rustup target add wasm32-wasi

# Install the klyra runtime
cargo install klyra-runtime --path "/usr/src/klyra/runtime" --bin klyra-next --features next

while getopts "p," o; do
    case $o in
        "p")
            # Make future crates requests to our own mirror
            echo '
[source.klyra-crates-io-mirror]
registry = "http://panamax:8080/git/crates.io-index"
[source.crates-io]
replace-with = "klyra-crates-io-mirror"' >> $CARGO_HOME/config.toml
            ;;
        *)
            ;;
    esac
done

# Prefetch crates.io index from our mirror
# TODO: restore when we know how to prefetch from our mirror
# cd /usr/src/klyra/service
# cargo fetch
