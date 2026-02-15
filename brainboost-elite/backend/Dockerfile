# Stage 1: Build
FROM rust:1.76-slim-bookworm AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# Build dependencies first (cache layer)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src/ src/
COPY migrations/ migrations/
COPY .sqlx/ .sqlx/
RUN touch src/main.rs && cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates libssl3 curl && rm -rf /var/lib/apt/lists/*

RUN groupadd -r brainboost && useradd -r -g brainboost brainboost

COPY --from=builder /app/target/release/brainboost-elite-backend /usr/local/bin/server
COPY --from=builder /app/migrations /app/migrations

RUN chown -R brainboost:brainboost /app
USER brainboost

ENV RUST_LOG=info
ENV PORT=8080

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -f http://localhost:8080/api/health || exit 1

CMD ["server"]
