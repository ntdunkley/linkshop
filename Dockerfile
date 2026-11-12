# Multi-stage Docker build for linkshop
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copy dependency definitions and sources
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release --bin linkshop

# Minimal runtime image
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Run as non-root user
RUN useradd -m -u 1000 appuser
USER appuser

COPY --from=builder /app/target/release/linkshop /usr/local/bin/linkshop

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

ENTRYPOINT ["linkshop"]
