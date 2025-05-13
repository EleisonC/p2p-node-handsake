# Use a lightweight Rust image with Alpine for the chef stage
FROM rust:1.82-alpine AS chef
USER root
# Install musl-dev for compiling Rust and cargo-chef for dependency caching
RUN apk add --no-cache musl-dev && cargo install cargo-chef
WORKDIR /app

# Planner stage: Generate dependency recipe
FROM chef AS planner
COPY . .
# Create recipe.json for dependency caching
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage: Compile dependencies and the application
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies (cached layer)
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source code and build the application
COPY . .
RUN cargo build --release --bin p2p-node-handshake

# Runtime stage: Minimal image with the compiled binary
FROM debian:buster-slim AS runtime
WORKDIR /app
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/p2p-node-handshake /usr/local/bin
# Set the entrypoint to run the binary
ENTRYPOINT ["/usr/local/bin/p2p-node-handshake"]