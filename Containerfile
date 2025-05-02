#syntax=docker/dockerfile-upstream:1.4.0-rc1
ARG RUSTUP_TOOLCHAIN
FROM docker.io/library/rust:${RUSTUP_TOOLCHAIN}-buster as klyra-build
RUN apt-get update &&\
    apt-get install -y curl

# download protoc binary and unzip it in usr/bin
ARG PROTOC_ARCH
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-${PROTOC_ARCH}.zip &&\
    unzip -o protoc-21.9-linux-${PROTOC_ARCH}.zip -d /usr bin/protoc &&\
    unzip -o protoc-21.9-linux-${PROTOC_ARCH}.zip -d /usr/ 'include/*' &&\
    rm -f protoc-21.9-linux-${PROTOC_ARCH}.zip

RUN cargo install cargo-chef
WORKDIR /build

FROM klyra-build as cache
WORKDIR /src
COPY . .
RUN find ${SRC_CRATES} \( -name "*.proto" -or -name "*.rs" -or -name "*.toml" -or -name "Cargo.lock" -or -name "README.md" -or -name "*.sql" \) -type f -exec install -D \{\} /build/\{\} \;
# This is used to carry over in the docker images any *.pem files from klyra root directory, to be used for TLS testing, as described
# here in the admin README.md.
RUN [ "$CARGO_PROFILE" != "release" ] && find ${SRC_CRATES} -name "*.pem" -type f -exec install -D \{\} /build/\{\} \;

FROM klyra-build AS planner
COPY --from=cache /build .
RUN cargo chef prepare --recipe-path recipe.json

FROM klyra-build AS builder
COPY --from=planner /build/recipe.json recipe.json
ARG CARGO_PROFILE
RUN cargo chef cook $(if [ "$CARGO_PROFILE" = "release" ]; then echo --${CARGO_PROFILE}; fi) --recipe-path recipe.json
COPY --from=cache /build .
ARG folder
# if CARGO_PROFILE is release, pass --release, else use default debug profile
RUN cargo build --bin klyra-${folder} $(if [ "$CARGO_PROFILE" = "release" ]; then echo --${CARGO_PROFILE}; fi)

ARG RUSTUP_TOOLCHAIN
FROM rust:${RUSTUP_TOOLCHAIN}-buster as klyra-common
RUN apt-get update &&\
    apt-get install -y curl
# download protoc binary and unzip it in usr/bin
ARG PROTOC_ARCH
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-${PROTOC_ARCH}.zip &&\
    unzip -o protoc-21.9-linux-${PROTOC_ARCH}.zip -d /usr/ bin/protoc &&\
    unzip -o protoc-21.9-linux-${PROTOC_ARCH}.zip -d /usr/ 'include/*' &&\
    rm -f protoc-21.9-linux-${PROTOC_ARCH}.zip
RUN rustup component add rust-src

COPY --from=cache /build/ /usr/src/klyra/

FROM klyra-common
ARG folder
ARG prepare_args
ARG PROD
COPY ${folder}/prepare.sh /prepare.sh
RUN /prepare.sh "${prepare_args}"
ARG CARGO_PROFILE
COPY --from=builder /build/target/${CARGO_PROFILE}/klyra-${folder} /usr/local/bin/service
ARG RUSTUP_TOOLCHAIN
ENV RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}
ENTRYPOINT ["/usr/local/bin/service"]
