#!/bin/bash

echo "=== Docker Container Debug Script ==="

# Build the image
echo "Building Docker image..."
docker build -t rust-api-debug .

# Run container interactively to debug
echo "Running container interactively..."
docker run -it --rm \
  -e DATABASE_URL="postgres://admin:password@host.docker.internal:5432/database?sslmode=disable" \
  -e RUST_LOG=debug \
  -e RUST_BACKTRACE=1 \
  --network host \
  rust-api-debug /bin/bash

# Alternative: Run with logs
echo "=== Running with logs ==="
docker run --rm \
  -e DATABASE_URL="postgres://admin:password@host.docker.internal:5432/database?sslmode=disable" \
  -e RUST_LOG=debug \
  -e RUST_BACKTRACE=1 \
  --network host \
  rust-api-debug

