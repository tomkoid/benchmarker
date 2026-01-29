FROM rust:1.93 AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    mkdir migrations

# Build dependencies - this layer will be cached
RUN cargo build --release && \
    rm -rf src target/release/deps/utilbench_backend* target/release/utilbench-backend*

# Copy the actual source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/utilbench-backend /app/utilbench-backend
COPY --from=builder /app/migrations /app/migrations

ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/utilbench-backend"]
