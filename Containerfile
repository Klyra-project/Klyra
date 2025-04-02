#syntax=docker/dockerfile-upstream:1.4.0-rc1
FROM rust:1.63.0-buster as klyra-build
RUN apt-get update &&\
    apt-get install -y curl
# download protoc binary and unzip it in usr/bin
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip &&\
    unzip -o protoc-21.9-linux-x86_64.zip -d /usr bin/protoc &&\
    unzip -o protoc-21.9-linux-x86_64.zip -d /usr/ 'include/*' &&\
    rm -f protoc-21.9-linux-x86_64.zip
RUN cargo install cargo-chef
WORKDIR /build

FROM klyra-build as cache
WORKDIR /src
COPY . .
RUN find ${SRC_CRATES} \( -name "*.proto" -or -name "*.rs" -or -name "*.toml" -or -name "README.md" -or -name "*.sql" \) -type f -exec install -D \{\} /build/\{\} \;

FROM klyra-build AS planner
COPY --from=cache /build .
RUN cargo chef prepare --recipe-path recipe.json

FROM klyra-build AS builder
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY --from=cache /build .
ARG crate
RUN cargo build --bin ${crate}

FROM rust:1.63.0-buster as klyra-common
RUN apt-get update &&\
    apt-get install -y curl
RUN rustup component add rust-src
COPY --from=cache /build/ /usr/src/klyra/

FROM klyra-common
ARG crate
SHELL ["/bin/bash", "-c"]
RUN mkdir -p $CARGO_HOME; \
echo $'[patch.crates-io] \n\
klyra-service = { path = "/usr/src/klyra/service" } \n\
klyra-aws-rds = { path = "/usr/src/klyra/resources/aws-rds" } \n\
klyra-persist = { path = "/usr/src/klyra/resources/persist" } \n\
klyra-shared-db = { path = "/usr/src/klyra/resources/shared-db" } \n\
klyra-secrets = { path = "/usr/src/klyra/resources/secrets" }' > $CARGO_HOME/config.toml
COPY --from=builder /build/target/debug/${crate} /usr/local/bin/service
ENTRYPOINT ["/usr/local/bin/service"]
