# ==================== Builder Stage ====================
FROM rust:1.85-bookworm AS builder

# Install the specific old nightly toolchain from June 2020
RUN rustup toolchain install nightly-2020-06-18 --profile minimal \
    && rustup default nightly-2020-06-18

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy source files so cargo can build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies only (cached layer)
RUN cargo build --release && rm -rf src

# Now copy the real source code
COPY . .

# Build the actual project
RUN cargo build --release

# Install diesel CLI with postgres support
RUN cargo install diesel_cli --no-default-features --features postgres

# ==================== Runtime Stage ====================
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/alcalc_backend /app/alcalc_backend

# Copy diesel CLI
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy migrations and config files
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/diesel.toml /app/diesel.toml
COPY --from=builder /app/.env /app/.env

# Copy and make entrypoint executable
COPY --from=builder /app/docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

ENV ROCKET_ENV=production
ENV DATABASE_URL=postgres://alcalc_user:alcalc_pass@db:5432/alcalc_db

EXPOSE 443

ENTRYPOINT ["/app/docker-entrypoint.sh"]
