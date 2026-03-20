# === Builder stage ===
FROM rust:slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Download pre-built cargo-leptos (avoids ~2GB RAM compile on CI)
RUN curl -fsSL https://github.com/leptos-rs/cargo-leptos/releases/download/v0.3.5/cargo-leptos-x86_64-unknown-linux-gnu.tar.gz \
    | tar -xzC /tmp \
    && mv /tmp/cargo-leptos-x86_64-unknown-linux-gnu/cargo-leptos /usr/local/bin/cargo-leptos \
    && chmod +x /usr/local/bin/cargo-leptos

RUN cargo install wasm-bindgen-cli --version 0.2.114 --locked

WORKDIR /app

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN mkdir -p src && echo "" > src/lib.rs

RUN cargo fetch

COPY . .

ENV RUST_BACKTRACE=full
RUN cargo leptos build --release

# === Runner stage ===
FROM debian:bookworm-slim AS runner

RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/docs-mentisdb-com /usr/local/bin/app
COPY --from=builder /app/target/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="docs-mentisdb-com"
ENV LEPTOS_SITE_ROOT="site"
ENV PORT=8080

EXPOSE 8080

CMD ["/bin/sh", "-c", "LEPTOS_SITE_ADDR=0.0.0.0:${PORT:-8080} /usr/local/bin/app"]
