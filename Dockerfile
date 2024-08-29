FROM rust:latest as builder

WORKDIR /usr/src/

RUN rustup target add aarch64-unknown-linux-gnu

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY .env .env

RUN cargo build --target aarch64-unknown-linux-gnu

FROM alt:p11

COPY --from=builder /usr/src/target/aarch64-unknown-linux-gnu/debug/pkg-cmp /usr/local/bin/pkg-cmp
