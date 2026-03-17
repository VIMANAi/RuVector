# rvagent-learning Cloud Run Dockerfile
# Daily Learning Loop with GOAP reasoning

FROM rust:1.77-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/rvAgent/rvagent-learning ./crates/rvAgent/rvagent-learning

# Create dummy workspace members to satisfy Cargo
RUN mkdir -p crates/ruvector-core/src && echo "pub fn dummy() {}" > crates/ruvector-core/src/lib.rs
RUN echo '[package]\nname = "ruvector-core"\nversion = "0.1.0"\nedition = "2021"' > crates/ruvector-core/Cargo.toml

# Build release binary
RUN cargo build --release -p rvagent-learning --bin daily_cycle

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Install gcloud CLI for secret manager access
RUN apt-get update && apt-get install -y curl gnupg \
    && echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] http://packages.cloud.google.com/apt cloud-sdk main" | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list \
    && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key --keyring /usr/share/keyrings/cloud.google.gpg add - \
    && apt-get update && apt-get install -y google-cloud-cli \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/daily_cycle /app/daily_cycle

# Create non-root user
RUN useradd -r -s /bin/false appuser
USER appuser

# Set environment
ENV RUST_LOG=info
ENV PORT=8080

# Health check endpoint would be on the scheduler
EXPOSE 8080

# Run the learning cycle
ENTRYPOINT ["/app/daily_cycle"]
CMD ["--once"]
