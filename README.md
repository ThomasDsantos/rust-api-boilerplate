# Rust API Boilerplate

A modern REST API boilerplate built with Rust, featuring:

- **Axum** - Fast and ergonomic web framework
- **SeaORM** - Modern async ORM for Rust
- **Utoipa** - OpenAPI 3.0 documentation generation
- **PostgreSQL** - Database backend
- **Docker** - Containerization support

## Features

- ✅ RESTful API endpoints
- ✅ OpenAPI/Swagger documentation
- ✅ Database migrations with SeaORM
- ✅ Request logging and error handling
- ✅ CORS support
- ✅ Health check endpoint
- ✅ Environment-based configuration

## Quick Start

### Prerequisites

- Rust 1.70+ 
- PostgreSQL 13+
- Docker (optional)

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-api-boilerplate
```

2. Start the application with Docker Compose:
```bash
make compose
```

The API will be available at `http://localhost:8080`


## API Documentation

Once the server is running, you can access the interactive API documentation at:
- Swagger UI: `http://localhost:8080/swagger-ui/`
- OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`

## Database Schema

The application uses a simple `visit` table:

```sql
CREATE TABLE visit (
  ip inet not null,
  name varchar not null,
  visited_at timestamptz not null,
  primary key (ip, visited_at)
);
```

## Configuration

The application can be configured via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://admin:password@localhost:5432/database?sslmode=disable` | PostgreSQL connection string |
| `SERVER_PORT` | `8080` | Server port |
| `SERVER_HOST` | `0.0.0.0` | Server host |
| `SERVER_API_BASE_PATH` | `/v1` | API base path |
| `SERVER_SERVICE_NAME` | `rust-api` | Service name |
| `SERVER_ENVIRONMENT` | `local` | Environment name |
| `RUST_LOG` | `debug` | Log level |

## Development

### Running Tests
```bash
cargo test
```

### Building for Production
```bash
cargo build --release
```

### Database Migrations

To create a new migration:
```bash
sea-orm-cli migrate generate <migration_name>
```

To run migrations:
```bash
sea-orm-cli migrate up
```

## Docker Support

Build and run with Docker:
```bash
docker build -t rust-api .
docker run -p 8080:8080 rust-api
```

## Project Structure

```
rust-api-boilerplate/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Application setup
│   ├── middleware.rs        # Middleware setup
│   ├── config/              # Configuration management
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   └── database.rs
│   ├── database/            # Database connection and operations
│   │   ├── mod.rs
│   │   ├── connection.rs
│   │   ├── health.rs
│   │   └── migrate.rs
│   ├── entities/            # Database entities
│   │   ├── mod.rs
│   │   └── visit.rs
│   ├── error/               # Error handling
│   │   ├── mod.rs
│   │   └── app_error.rs
│   ├── handlers/            # Request handlers
│   │   ├── mod.rs
│   │   ├── greeting.rs
│   │   ├── health.rs
│   │   └── visit.rs
│   ├── models/              # API models/DTOs
│   │   ├── mod.rs
│   │   ├── common.rs
│   │   ├── greeting.rs
│   │   └── visit.rs
│   ├── routes/              # Route definitions
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── services/            # Business logic services
│   │   ├── mod.rs
│   │   ├── greeting.rs
│   │   └── visit.rs
│   └── utils/               # Utility functions
│       ├── mod.rs
│       └── extractors.rs
├── migration/               # Database migrations
│   ├── src/
│   │   ├── lib.rs
│   │   └── m20220101_000001_create_visit_table.rs
│   └── Cargo.toml
├── tests/                   # Test files
├── Cargo.toml
├── Dockerfile
├── compose.yaml
├── Makefile
└── .env
```

