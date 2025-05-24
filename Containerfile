#syntax=docker/dockerfile-upstream:1.4


# Base image for builds and cache
ARG RUSTUP_TOOLCHAIN
FROM docker.io/lukemathwalker/cargo-chef:latest-rust-${RUSTUP_TOOLCHAIN}-buster as cargo-chef
WORKDIR /build


# Stores source cache and cargo chef recipe
FROM cargo-chef as chef-planner
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
    -name "README.md" -or \
    # Used for local TLS testing, as described in admin/README.md
    -name "*.pem" \
    \) -type f -exec install -D \{\} /build/\{\} \;
WORKDIR /build
RUN cargo chef prepare --recipe-path /recipe.json
# TODO upstream: Reduce the cooking by allowing multiple --bin args to prepare, or like this https://github.com/LukeMathWalker/cargo-chef/issues/181


# Builds crate according to cargo chef recipe.
# This step is skipped if the recipe is unchanged from previous build (no dependencies changed).
FROM cargo-chef AS chef-builder
ARG CARGO_PROFILE
COPY --from=chef-planner /recipe.json /
# https://i.imgflip.com/2/74bvex.jpg
RUN cargo chef cook \
    --all-features \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --recipe-path /recipe.json
COPY --from=chef-planner /build .
# Building all at once to share build artifacts in the "cook" layer
RUN cargo build \
    $(if [ "$CARGO_PROFILE" = "release" ]; then echo --release; fi) \
    --bin klyra-auth \
    --bin klyra-builder \
    --bin klyra-deployer \
    --bin klyra-gateway \
    --bin klyra-logger \
    --bin klyra-provisioner \
    --bin klyra-resource-recorder \
    --bin klyra-next -F next


# Base image for running each "klyra-..." binary
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-buster as klyra-crate-base
ARG folder
# Some crates need additional libs
COPY ${folder}/*.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/
ENTRYPOINT ["/usr/local/bin/service"]


# Targets for each crate
# Copying of each binary is non-DRY to allow other steps to be cached

FROM klyra-crate-base AS klyra-auth
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-auth /usr/local/bin/service
FROM klyra-auth AS klyra-auth-dev

FROM klyra-crate-base AS klyra-builder
ARG CARGO_PROFILE
ARG prepare_args
COPY builder/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-builder /usr/local/bin/service
FROM klyra-builder AS klyra-builder-dev

FROM klyra-crate-base AS klyra-deployer
ARG CARGO_PROFILE
ARG prepare_args
# Fixes some dependencies compiled with incompatible versions of rustc
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}
# Used as env variable in prepare script
ARG PROD
# Easy way to check if you are running in Klyra's container
ARG klyra=true
COPY deployer/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-deployer /usr/local/bin/service
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-next /usr/local/cargo/bin/
ARG TARGETPLATFORM
RUN for target_platform in "linux/arm64" "linux/arm64/v8"; do \
    if [ "$TARGETPLATFORM" = "$target_platform" ]; then \
      mv /usr/lib/ulid0_aarch64.so /usr/lib/ulid0.so; fi; done
FROM klyra-deployer AS klyra-deployer-dev
# Source code needed for compiling with [patch.crates-io]
COPY --from=chef-planner /build /usr/src/klyra/
FROM klyra-crate-base AS klyra-gateway
ARG CARGO_PROFILE
ARG folder
# Some crates need additional libs
COPY ${folder}/*.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-gateway /usr/local/bin/service
ARG TARGETPLATFORM
RUN for target_platform in "linux/arm64" "linux/arm64/v8"; do \
    if [ "$TARGETPLATFORM" = "$target_platform" ]; then \
      mv /usr/lib/ulid0_aarch64.so /usr/lib/ulid0.so; fi; done
FROM klyra-gateway AS klyra-gateway-dev
# For testing certificates locally
COPY --from=chef-planner /build/*.pem /usr/src/klyra/

FROM klyra-crate-base AS klyra-logger
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-logger /usr/local/bin/service
FROM klyra-logger AS klyra-logger-dev

FROM klyra-crate-base AS klyra-provisioner
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-provisioner /usr/local/bin/service
FROM klyra-provisioner AS klyra-provisioner-dev

FROM klyra-crate-base AS klyra-resource-recorder
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-resource-recorder /usr/local/bin/service
FROM klyra-resource-recorder AS klyra-resource-recorder-dev
