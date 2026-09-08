# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

Full-stack application with Rust (Axum) backend and Next.js (TypeScript) frontend. The backend uses Tokio for async runtime, SQLx for database access, and JWT for authentication. The frontend uses Next.js 15 with App Router, Turbo (monorepo), and TanStack Query.

## Development Commands

### Backend (Rust/Axum)
```bash
# From backend/ directory
cargo build                  # Build the backend
cargo run                    # Run the backend server (port 3001)
cargo test                   # Run tests
cargo check                  # Type check without building

# Database setup and migrations
cargo install sqlx-cli --no-default-features --features rustls,postgres
docker-compose up -d postgres    # Start PostgreSQL
sqlx database create             # Create database
sqlx migrate run                 # Run migrations
sqlx migrate add <name>          # Create new migration
```

### Frontend (Next.js)
```bash
# From frontend/ directory
pnpm install                 # Install dependencies
pnpm dev                     # Run all apps in dev mode
pnpm dev:web                 # Run only web app (port 3001)
pnpm build                   # Build all apps
pnpm check-types             # TypeScript type checking
```

### Docker
```bash
# From root directory
docker-compose up -d postgres    # Start PostgreSQL only
docker-compose down              # Stop all services
```

## Architecture

### Backend Structure (Rust/Axum)

The backend follows a layered architecture pattern:

**Entry Point**: `main.rs` initializes tracing, loads environment variables, sets up database pool with connection pooling, runs migrations automatically on startup, and configures the Axum router with middleware layers.

**Routing**: All routes are defined in `main.rs` via `create_app()`. Public routes (health, login, register) are accessible without authentication. Protected routes require JWT tokens and pass through `auth_middleware` which validates tokens and checks user existence.

**Middleware Chain**: Applied in this order - `auth_middleware` (JWT validation, skips public routes), `CorsLayer` (permissive CORS), `TraceLayer` (HTTP request tracing). The auth middleware injects `JwtClaims` into request extensions for downstream handlers.

**State Management**: `PgPool` (SQLx connection pool) is the main application state, shared across all handlers via Axum's state extraction. Pool configuration supports `DB_MAX_CONNECTIONS`, `DB_MIN_CONNECTIONS`, and `DB_CONNECT_TIMEOUT` environment variables.

**Database Access**: SQLx is used for compile-time checked queries. Models in `models/` contain database access methods (e.g., `User::find_by_id`, `User::create`). Connection pool is passed via State extractor to handlers and services.

**Authentication Flow**: Registration hashes passwords with bcrypt (cost 12), stores users with UUID primary keys. Login validates credentials and returns JWT token. JWT tokens contain user ID (`sub`) and expiration (`exp`). Auth middleware validates token and checks user is_active status.

**Error Handling**: Handlers return `Result<Json<ApiResponse<T>>, StatusCode>`. Errors are logged with tracing and converted to appropriate HTTP status codes. `ApiResponse<T>` is a generic wrapper for consistent JSON responses.

### Frontend Structure (Next.js)

**Monorepo**: Turborepo manages multiple apps/packages. Primary app is in `frontend/apps/web/`. Uses pnpm workspaces for dependency management.

**Next.js App**: Uses App Router (Next.js 15). Located in `apps/web/src/app/`. Components in `apps/web/src/components/` including shadcn/ui components in `components/ui/`.

**Styling**: Tailwind CSS v4 with custom configuration. Uses `class-variance-authority` for component variants and `clsx` + `tailwind-merge` for className utilities.

**State Management**: TanStack Query (React Query) for server state. Theme management via `next-themes`.

**Forms**: TanStack Form with Zod validation.

## Environment Configuration

### Required Variables
```bash
DATABASE_URL=postgresql://postgres:password@localhost:5432/myapp
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
RUST_LOG=debug
PORT=3001
```

### Optional Variables
```bash
DB_MAX_CONNECTIONS=20
DB_MIN_CONNECTIONS=5
DB_CONNECT_TIMEOUT=10
```

## Database

**Database**: PostgreSQL 15 via Docker
**ORM**: SQLx (not Diesel) - compile-time verified queries
**Migrations**: Located in `backend/migrations/`, run automatically on backend startup

**Schema**: Users table with UUID primary keys, email (unique, indexed), bcrypt password hashes, first/last name, is_active flag, timestamps. Automatic `updated_at` trigger on updates.

## API Endpoints

### Public Routes
- `GET /api/health` - Health check
- `POST /api/auth/register` - User registration (requires email, password, first_name, last_name)
- `POST /api/auth/login` - User login (returns JWT token)

### Protected Routes (require Authorization: Bearer <token>)
- `GET /api/users/me` - Get current user profile
- `POST /api/users/me` - Update current user profile

## Code Patterns

### Adding New Endpoints

1. Create handler in `backend/src/handlers/<resource>.rs`
2. Add route in `main.rs` `create_app()` function
3. If protected, ensure it's after the auth middleware layer
4. Update `is_public_route()` in `middleware/auth.rs` if route is public

### Database Migrations

Create new migration: `sqlx migrate add <descriptive_name>`
Migrations are SQL files in `backend/migrations/`
Format: `<sequential_number>_<description>.sql`
Always include rollback strategy in comments

### JWT Authentication

JWT secret loaded from `JWT_SECRET` environment variable
Claims include `sub` (user UUID) and `exp` (expiration timestamp)
Tokens are validated by checking signature and expiration
User existence is verified on each authenticated request

### Error Handling

Backend errors are logged with `tracing::error!()` before returning status codes
Prefer returning appropriate HTTP status codes over custom error types in responses
Frontend should handle common error codes (401, 403, 500)

## Tech Stack Notes

**Backend**: Axum 0.8.4 (not 0.7), Tokio 1.47 with "full" features, SQLx 0.8.6 with rustls (not native-tls), Tower HTTP for CORS and tracing

**Frontend**: Next.js 15.3.0, React 19, Tailwind CSS 4.1.10, TanStack Query 5.80+, Radix UI components via shadcn/ui

**Edition**: Cargo.toml specifies Rust edition 2024

## Port Configuration

Backend default: 3001 (configurable via PORT env var)
Frontend web app: 3001 (configured in apps/web/package.json dev script)
PostgreSQL: 5432

Note: Frontend and backend both default to 3001 - adjust one if running simultaneously.
