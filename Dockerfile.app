# ==================== Builder Stage ====================
FROM rust:1.85-bookworm AS builder

# Use a much newer nightly that supports edition 2021 (and most features from 2020-era projects)
RUN rustup toolchain install nightly-2021-10-21 --profile minimal \
    && rustup default nightly-2021-10-21

# Install system dependencies needed for building
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy source to build dependencies only
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release && rm -rf src

# Copy the actual source code
COPY . .

# Build the real project
RUN cargo build --release

# Install diesel_cli with postgres support
RUN cargo install diesel_cli --version 2.0.1 --no-default-features --features postgres

# ==================== Runtime Stage ====================
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/alcalc_backend /app/alcalc_backend

# Copy diesel CLI
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy migrations and config
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/diesel.toml /app/diesel.toml
COPY --from=builder /app/.env /app/.env

# Copy entrypoint
COPY --from=builder /app/docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

ENV ROCKET_ENV=production
ENV DATABASE_URL=postgres://alcalc_user:alcalc_pass@db:5432/alcalc_db

EXPOSE 443

ENTRYPOINT ["/app/docker-entrypoint.sh"]
