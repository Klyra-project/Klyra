#!/bin/bash

###############################################################################
# This file is used by our common Containerfile incase the container for this #
# service might need some extra preparation steps for its final image         #
###############################################################################


# Stuff that depends on local source files
if [ "$1" = "--after-src" ]; then

    # Install the klyra runtime
    cargo install klyra-runtime --path "/usr/src/klyra/runtime" --bin klyra-next --features next

    exit 0
fi


# Patch crates to be on same versions
mkdir -p $CARGO_HOME
touch $CARGO_HOME/config.toml
if [[ $PROD != "true" ]]; then
    echo '
    [patch.crates-io]
    klyra-service = { path = "/usr/src/klyra/service" }
    klyra-runtime = { path = "/usr/src/klyra/runtime" }

    klyra-aws-rds = { path = "/usr/src/klyra/resources/aws-rds" }
    klyra-persist = { path = "/usr/src/klyra/resources/persist" }
    klyra-shared-db = { path = "/usr/src/klyra/resources/shared-db" }
    klyra-secrets = { path = "/usr/src/klyra/resources/secrets" }
    klyra-static-folder = { path = "/usr/src/klyra/resources/static-folder" }

    klyra-actix-web = { path = "/usr/src/klyra/services/klyra-actix-web" }
    klyra-axum = { path = "/usr/src/klyra/services/klyra-axum" }
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
fi

# Add the wasm32-wasi target
rustup target add wasm32-wasi

while getopts "p," o; do
    case $o in
        "p") # if panamax is used, the '-p' parameter is passed
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

# Install common build tools for external crates
# The image should already have these: https://github.com/docker-library/buildpack-deps/blob/65d69325ad741cea6dee20781c1faaab2e003d87/debian/buster/Dockerfile
apt update
apt install -y curl llvm-dev libclang-dev clang cmake

# Install protoc since some users may need it
ARCH="linux-x86_64" && \
VERSION="22.2" && \
curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$VERSION/protoc-$VERSION-$ARCH.zip" && \
    unzip -o "protoc-$VERSION-$ARCH.zip" bin/protoc "include/*" -d /usr/local && \
    rm -f "protoc-$VERSION-$ARCH.zip"
