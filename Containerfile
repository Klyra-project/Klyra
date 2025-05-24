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


####### Targets for each crate

#### AUTH
FROM docker.io/library/debian:bookworm-20230904-slim AS klyra-auth
ARG CARGO_PROFILE
ARG prepare_args
COPY auth/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-auth /usr/local/bin
ENTRYPOINT ["/usr/local/bin/klyra-auth"]
FROM klyra-auth AS klyra-auth-dev


#### BUILDER
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-bookworm AS klyra-builder
ARG CARGO_PROFILE
ARG prepare_args
COPY builder/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-builder /usr/local/bin
ENTRYPOINT ["/usr/local/bin/klyra-builder"]
FROM klyra-builder AS klyra-builder-dev


#### DEPLOYER
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-bookworm AS klyra-deployer
ARG CARGO_PROFILE
ARG prepare_args
# Fixes some dependencies compiled with incompatible versions of rustc
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}
COPY gateway/ulid0.so /usr/lib/
COPY gateway/ulid0_aarch64.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/
ARG TARGETPLATFORM
RUN for target_platform in "linux/arm64" "linux/arm64/v8"; do \
    if [ "$TARGETPLATFORM" = "$target_platform" ]; then \
      mv /usr/lib/ulid0_aarch64.so /usr/lib/ulid0.so; fi; done
# Used as env variable in prepare script
ARG PROD
# Easy way to check if you are running in Klyra's container
ARG klyra=true
ENV klyra=${klyra}
COPY deployer/prepare.sh /prepare.sh
COPY scripts/apply-patches.sh /scripts/apply-patches.sh
COPY scripts/patches.toml /scripts/patches.toml
RUN /prepare.sh "${prepare_args}"
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-deployer /usr/local/bin
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-next /usr/local/cargo/bin
ENTRYPOINT ["/usr/local/bin/klyra-deployer"]
FROM klyra-deployer AS klyra-deployer-dev
# Source code needed for compiling local deploys with [patch.crates-io]
COPY --from=chef-planner /build /usr/src/klyra/


#### GATEWAY
FROM docker.io/library/debian:bookworm-20230904 AS klyra-gateway
ARG CARGO_PROFILE
COPY gateway/ulid0.so /usr/lib/
COPY gateway/ulid0_aarch64.so /usr/lib/
ENV LD_LIBRARY_PATH=/usr/lib/
ARG TARGETPLATFORM
RUN for target_platform in "linux/arm64" "linux/arm64/v8"; do \
    if [ "$TARGETPLATFORM" = "$target_platform" ]; then \
      mv /usr/lib/ulid0_aarch64.so /usr/lib/ulid0.so; fi; done
# curl is needed for health checks
RUN apt update && apt install -y curl
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-gateway /usr/local/bin
ENTRYPOINT ["/usr/local/bin/klyra-gateway"]
FROM klyra-gateway AS klyra-gateway-dev
# For testing certificates locally
COPY --from=chef-planner /build/*.pem /usr/src/klyra/


#### LOGGER
FROM docker.io/library/debian:bookworm-20230904-slim AS klyra-logger
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-logger /usr/local/bin
ENTRYPOINT ["/usr/local/bin/klyra-logger"]
FROM klyra-logger AS klyra-logger-dev


#### PROVISIONER
# for some reason, hyper-rustls 0.24.1 does not work in a plain debian image
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-bookworm AS klyra-provisioner
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-provisioner /usr/local/bin
ENTRYPOINT ["/usr/local/bin/klyra-provisioner"]
FROM klyra-provisioner AS klyra-provisioner-dev


#### RESOURCE RECORDER
FROM docker.io/library/debian:bookworm-20230904-slim AS klyra-resource-recorder
ARG CARGO_PROFILE
COPY --from=chef-builder /build/target/${CARGO_PROFILE}/klyra-resource-recorder /usr/local/bin
ENTRYPOINT ["/usr/local/bin/klyra-resource-recorder"]
FROM klyra-resource-recorder AS klyra-resource-recorder-dev
