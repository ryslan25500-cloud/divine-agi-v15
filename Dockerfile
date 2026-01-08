# Divine AGI V16 (Kernel V4) — Railway Dockerfile
# ROSETTA STONE: Uses Rust nightly to support edition2024
FROM rustlang/rust:nightly AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY Cargo.toml ./
COPY rust-toolchain.toml ./
COPY src/ ./src/

# Build release with nightly
RUN cargo +nightly build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/divine-agi /app/divine-agi

# Expose port
EXPOSE 8080

# Environment
ENV PORT=8080
ENV RUST_LOG=info

# Run with PORT from Railway
CMD ["sh", "-c", "./divine-agi server --port ${PORT:-8080}"]
