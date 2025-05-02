#!/usr/bin/env sh

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################

# Patch crates to be on same versions
mkdir -p $CARGO_HOME
if [[ $PROD != "true" ]]; then
    echo '[patch.crates-io]
    klyra-service = { path = "/usr/src/klyra/service" }
    klyra-runtime = { path = "/usr/src/klyra/runtime" }

    klyra-aws-rds = { path = "/usr/src/klyra/resources/aws-rds" }
    klyra-persist = { path = "/usr/src/klyra/resources/persist" }
    klyra-shared-db = { path = "/usr/src/klyra/resources/shared-db" }
    klyra-secrets = { path = "/usr/src/klyra/resources/secrets" }
    klyra-static-folder = { path = "/usr/src/klyra/resources/static-folder" }

    klyra-axum = { path = "/usr/src/klyra/services/klyra-axum" }
    klyra-actix-web = { path = "/usr/src/klyra/services/klyra-actix-web" }
    klyra-next = { path = "/usr/src/klyra/services/klyra-next" }
    klyra-poem = { path = "/usr/src/klyra/services/klyra-poem" }
    klyra-poise = { path = "/usr/src/klyra/services/klyra-poise" }
    klyra-rocket = { path = "/usr/src/klyra/services/klyra-rocket" }
    klyra-salvo = { path = "/usr/src/klyra/services/klyra-salvo" }
    klyra-serenity = { path = "/usr/src/klyra/services/klyra-serenity" }
    klyra-thruster = { path = "/usr/src/klyra/services/klyra-thruster" }
    klyra-tide = { path = "/usr/src/klyra/services/klyra-tide" }
    klyra-tower = { path = "/usr/src/klyra/services/klyra-tower" }
    klyra-warp = { path = "/usr/src/klyra/services/klyra-warp" }' > $CARGO_HOME/config.toml
else
    touch $CARGO_HOME/config.toml
fi

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
registry = "sparse+http://panamax:8080/index/"
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
