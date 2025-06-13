# Build stage
FROM rust:1.87-slim-bullseye AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY migration ./migration

# Create a dummy main.rs to build dependencies
# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

COPY src ./src

RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

COPY --from=builder /app/target/release/rust-api-boilerplate /app/rust-api-boilerplate
COPY --from=builder /app/migration /app/migration

RUN chown -R appuser:appuser /app
USER appuser

EXPOSE ${SERVER_PORT}

CMD [ "/app/rust-api-boilerplate" ]
# CMD [ "/bin/sh" ]
