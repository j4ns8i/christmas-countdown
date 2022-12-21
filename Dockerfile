# syntax=docker/dockerfile:1.3
FROM rust:1.66 as builder

# By default, app is built in debug mode
ARG CC_CARGO_BUILD_FLAGS=""
ARG CC_CARGO_INSTALL_FLAGS="--debug"

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo new christmas-countdown
WORKDIR /usr/src/christmas-countdown
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=./target \
    cargo build $CC_CARGO_BUILD_FLAGS

COPY src ./src
RUN --mount=type=cache,target=./target \
    cargo install $CC_CARGO_INSTALL_FLAGS --target x86_64-unknown-linux-musl --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/christmas-countdown /usr/local/bin/christmas-countdown
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ["/usr/local/bin/christmas-countdown"]
