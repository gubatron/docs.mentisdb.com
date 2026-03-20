# === Builder stage ===
FROM rust:1.86-alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    perl \
    pkgconfig \
    build-base \
    clang \
    lld \
    mold \
    nodejs \
    npm

ENV OPENSSL_NO_VENDOR=1

RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-leptos --locked
RUN cargo install wasm-bindgen-cli --version 0.2.114 --locked

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN mkdir -p src && echo "" > src/lib.rs

RUN cargo fetch

COPY . .

ENV RUST_BACKTRACE=full
RUN cargo leptos build --release -vv

# === Runner stage ===
FROM alpine:3.20 AS runner

RUN apk add --no-cache \
    libstdc++ \
    openssl \
    ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/docs-mentisdb-com /usr/local/bin/app
COPY --from=builder /app/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="docs-mentisdb-com"
ENV PORT=8080

EXPOSE 8080

CMD ["/bin/sh", "-c", "LEPTOS_SITE_ADDR=0.0.0.0:${PORT:-8080} /usr/local/bin/app"]
