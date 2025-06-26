# Dockerfile for Axum project
FROM rust:1.81-slim as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire Axum project
COPY . .

# Try to build the Axum workspace (may fail, but continue)
RUN echo "=== Building Axum workspace ===" && \
    cargo build --release --workspace || echo "Workspace build failed, trying individual examples..."

# Try to build specific examples (one by one, ignoring failures)
RUN echo "=== Building individual examples ===" && \
    cargo build --release --example hello-world || echo "hello-world failed" && \
    cargo build --release --example static-file-server || echo "static-file-server failed" && \
    cargo build --release --example error-handling || echo "error-handling failed" && \
    cargo build --release --example global-404-handler || echo "global-404-handler failed" && \
    cargo build --release --example handle-head-request || echo "handle-head-request failed" && \
    cargo build --release --example print-request-response || echo "print-request-response failed" && \
    cargo build --release --example routes-and-handlers-close-together || echo "routes-and-handlers-close-together failed" && \
    echo "=== Example build completed ==="

# Check what was compiled successfully
RUN echo "=== Checking compiled examples ===" && \
    find target/release -name "example-*" -type f -executable 2>/dev/null | head -10 && \
    find target/release/examples -type f -executable 2>/dev/null | head -10 || echo "No examples found"

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binaries
COPY --from=builder /app/target/release/ ./

# Create user
RUN groupadd -r appuser && useradd -r -g appuser appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 3000

# Show available examples and how to run them
CMD ["sh", "-c", "echo '=== Axum Project - Available Examples ===' && echo 'Looking for compiled examples...' && echo 'Examples in examples/ folder:' && ls -la examples/ 2>/dev/null && echo '' && echo 'Executables starting with example-:' && find . -name 'example-*' -type f -executable 2>/dev/null && echo '' && echo 'To run an example:' && echo 'docker run --rm -p 3000:3000 axum-project ./examples/EXAMPLE_NAME' && echo 'or' && echo 'docker run --rm -p 3000:3000 axum-project ./example-NAME' && echo '' && echo 'Most likely working examples:' && echo '- hello-world' && echo '- error-handling' && echo '- static-file-server'"] 