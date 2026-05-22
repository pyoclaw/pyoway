# ---- Build Stage ----
FROM rust:1.95-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./
COPY landing-server/Cargo.toml landing-server/
COPY landing-frontend/Cargo.toml landing-frontend/

# Create dummy source files to build dependencies
RUN mkdir -p landing-server/src landing-frontend/src \
    && echo "fn main() {}" > landing-server/src/main.rs \
    && echo "" > landing-frontend/src/lib.rs \
    && cargo build -p landing-server --release 2>/dev/null || true

# Copy actual source and rebuild
COPY . .
RUN cargo build -p landing-server --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary
COPY --from=builder /app/target/release/landing-server /app/landing-server

# Copy the WASM frontend bundle
COPY --from=builder /app/landing-frontend/dist /app/dist

EXPOSE 8080

CMD ["/app/landing-server"]
