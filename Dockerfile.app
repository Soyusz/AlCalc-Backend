FROM rustup/rust:nightly-2020-06 as builder

RUN rustup default nightly-2020-06

RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && touch src/lib.rs src/main.rs

RUN cargo build --release && rm -rf src

COPY . .
RUN cargo build --release

RUN apt-get update && apt-get install -y \
    cargo \
    && cargo install diesel_cli --no-default-features --features postgres \
    && rm -rf /root/.cargo/registry/cache

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/alcalc_backend /app/alcalc_backend
COPY --from=builder /app/target/release/diesel /usr/local/bin/diesel
COPY --from=builder /app/migrations /app/migrations
COPY diesel.toml .env ./
COPY docker-entrypoint.sh /app/docker-entrypoint.sh

RUN chmod +x /app/docker-entrypoint.sh

ENV ROCKET_ENV=production
ENV DATABASE_URL=postgres://alcalc_user:alcalc_pass@db:5432/alcalc_db

EXPOSE 8000

ENTRYPOINT ["/app/docker-entrypoint.sh"]