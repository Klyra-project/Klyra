#syntax=docker/dockerfile-upstream:1.4


# Base image for builds and cache
ARG RUSTUP_TOOLCHAIN
FROM lukemathwalker/cargo-chef:latest-rust-${RUSTUP_TOOLCHAIN}-buster as klyra-build
WORKDIR /build


# Stores source cache
FROM klyra-build as cache
ARG PROD
WORKDIR /src
COPY . .
RUN find ${SRC_CRATES} \( -name "*.proto" -or -name "*.rs" -or -name "*.toml" -or -name "Cargo.lock" -or -name "README.md" -or -name "*.sql" -or -name "ulid0.so" \) -type f -exec install -D \{\} /build/\{\} \;
# This is used to carry over in the docker images any *.pem files from klyra root directory,
# to be used for TLS testing, as described here in the admin README.md.
RUN if [ "$PROD" != "true" ]; then \
    find ${SRC_CRATES} -name "*.pem" -type f -exec install -D \{\} /build/\{\} \;; \
    fi


# Stores cargo chef recipe
FROM klyra-build AS planner
COPY --from=cache /build .
RUN cargo chef prepare --recipe-path recipe.json


# Builds crate according to cargo chef recipe
FROM klyra-build AS builder
ARG folder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY --from=cache /build .
RUN cargo build --bin klyra-${folder} --release


# The final image for this "klyra-..." crate
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-buster as klyra-crate
ARG folder
ARG prepare_args
# used as env variable in prepare script
ARG PROD
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}

COPY ${folder}/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"

COPY --from=cache /build /usr/src/klyra/

# Any prepare steps that depend on the COPY from src cache.
# In the deployer klyra-next is installed and the panamax mirror config is added in this step.
RUN /prepare.sh --after-src "${prepare_args}"

COPY --from=builder /build/target/release/klyra-${folder} /usr/local/bin/service
ENTRYPOINT ["/usr/local/bin/service"]
