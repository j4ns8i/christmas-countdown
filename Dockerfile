FROM rust:1.66 as builder

WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl

RUN cargo new christmas-countdown
WORKDIR /usr/src/christmas-countdown
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/christmas-countdown /usr/local/bin/christmas-countdown
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ["/usr/local/bin/christmas-countdown"]
