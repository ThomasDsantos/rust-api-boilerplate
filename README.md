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
cd go-api-boilerplate/rust
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your database configuration
```

3. Run database migrations:
```bash
cargo run --bin migration
```

4. Start the server:
```bash
cargo run
```

The API will be available at `http://localhost:8080`

## API Endpoints

### Health Check
- `GET /v1/health` - Check service health

### Greetings
- `GET /v1/greet/{name}` - Get a personalized greeting

### Visits
- `GET /v1/visits` - Get all visits
- `POST /v1/visits` - Create a new visit

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
rust/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Application setup
│   ├── config.rs        # Configuration management
│   ├── database.rs      # Database connection
│   ├── error.rs         # Error handling
│   ├── handlers.rs      # Request handlers
│   ├── middleware.rs    # Middleware setup
│   ├── models.rs        # API models/DTOs
│   ├── routes.rs        # Route definitions
│   └── entities/        # Database entities
│       ├── mod.rs
│       └── visit.rs
├── migration/           # Database migrations
│   ├── src/
│   │   ├── lib.rs
│   │   └── m20220101_000001_create_visit_table.rs
│   └── Cargo.toml
├── Cargo.toml
└── .env
```

