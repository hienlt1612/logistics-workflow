# ── Build Stage ──────────────────────────────────────────────
FROM rust:1-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy sources
COPY . .

# Build release (web-only, no Qt6 GUI deps)
RUN cargo build --release --locked && \
    cp target/release/logistics-workflow /app/logistics-workflow && \
    strip /app/logistics-workflow

# ── Runtime Stage ────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/logistics-workflow /app/logistics-workflow

EXPOSE 19876

HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD curl -f http://localhost:19876/health || exit 1

ENTRYPOINT ["/app/logistics-workflow"]
