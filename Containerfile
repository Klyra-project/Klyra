#syntax=docker/dockerfile-upstream:1.4


# Base image for builds and cache
ARG RUSTUP_TOOLCHAIN
FROM docker.io/lukemathwalker/cargo-chef:latest-rust-${RUSTUP_TOOLCHAIN}-buster as cargo-chef
WORKDIR /build


# Stores source cache and cargo chef recipe
FROM cargo-chef as planner
WORKDIR /src
COPY . .
# Select only the essential files for copying into next steps
# so that changes to miscellaneous files don't trigger a new cargo-chef cook.
# Beware that .dockerignore filters files before they get here.
RUN find . \( \
    -name "*.rs" -or \
    -name "*.toml" -or \
    -name "Cargo.lock" -or \
    -name "*.sql" -or \
    # Used for local TLS testing, as described in admin/README.md
    -name "*.pem" -or \
    -name "ulid0.so" \
    \) -type f -exec install -D \{\} /build/\{\} \;
WORKDIR /build
RUN cargo chef prepare --recipe-path /recipe.json
# TODO upstream: Reduce the cooking by allowing multiple --bin args to prepare, or like this https://github.com/LukeMathWalker/cargo-chef/issues/181


# Builds crate according to cargo chef recipe.
# This step is skipped if the recipe is unchanged from previous build (no dependencies changed).
FROM cargo-chef AS builder
ARG CARGO_PROFILE
COPY --from=planner /recipe.json /
# https://i.imgflip.com/2/74bvex.jpg
RUN cargo chef cook \
    --all-features \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --recipe-path /recipe.json
COPY --from=planner /build .
# Building all at once to share build artifacts in the "cook" layer
RUN cargo build \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --bin klyra-auth \
    --bin klyra-deployer \
    --bin klyra-gateway \
    --bin klyra-logger \
    --bin klyra-provisioner \
    --bin klyra-resource-recorder \
    --bin klyra-next -F next


# Base image for running each "klyra-..." binary
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-buster as klyra-crate-base
ARG CARGO_PROFILE
ARG folder
ARG crate
ARG prepare_args
# Fixes some dependencies compiled with incompatible versions of rustc
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}
# Used as env variable in prepare script
ARG PROD

# Some crates need additional libs
COPY ${folder}/*.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/

COPY --from=builder /build/target/${CARGO_PROFILE}/${crate} /usr/local/bin/service
ENTRYPOINT ["/usr/local/bin/service"]


# Targets for each crate

FROM klyra-crate-base AS klyra-auth
FROM klyra-auth AS klyra-auth-dev

FROM klyra-crate-base AS klyra-deployer
ARG CARGO_PROFILE
COPY --from=builder /build/target/${CARGO_PROFILE}/klyra-next /usr/local/cargo/bin/
COPY deployer/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"
FROM klyra-deployer AS klyra-deployer-dev
# Source code needed for compiling with [patch.crates-io]
COPY --from=planner /build /usr/src/klyra/

FROM klyra-crate-base AS klyra-gateway
FROM klyra-gateway AS klyra-gateway-dev
# For testing certificates locally
COPY --from=planner /build/*.pem /usr/src/klyra/

FROM klyra-crate-base AS klyra-logger
FROM klyra-logger AS klyra-logger-dev

FROM klyra-crate-base AS klyra-provisioner
FROM klyra-provisioner AS klyra-provisioner-dev

FROM klyra-crate-base AS klyra-resource-recorder
FROM klyra-resource-recorder AS klyra-resource-recorder-dev
