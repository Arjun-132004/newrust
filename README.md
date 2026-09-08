# axum-nextjs

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![Next JS](https://img.shields.io/badge/Next-black?style=for-the-badge&logo=next.js&logoColor=white)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Node Version](https://img.shields.io/badge/node-18.0+-green.svg)](https://nodejs.org)

**Version:** 0.0.1  
**Created:** August 2025  
**Last Updated:** August 2025

A full-stack project combining [Axum](https://github.com/tokio-rs/axum) (Rust backend) and [Next.js](https://nextjs.org/) (React frontend), orchestrated with [Tokio](https://tokio.rs/) for async runtime.

## File Structure

```
axum-nextjs/
├── backend/           # Rust backend (Axum)
│   ├── src/
│   │   ├── handlers/  # API route handlers
│   │   ├── models/    # Database models
│   │   ├── middleware/# Auth & CORS middleware
│   │   ├── services/  # Business logic
│   │   ├── utils/     # Utilities (JWT, etc.)
│   │   └── config/    # Database configuration
│   ├── migrations/    # Database migrations
│   ├── Cargo.toml
│   └── .gitignore
├── frontend/          # React frontend (Next.js)
│   ├── src/
│   │   └── app/       # Next.js App Router
│   ├── public/        # Static assets
│   ├── package.json
│   └── .gitignore
├── docker-compose.yml # PostgreSQL setup
├── README.md
└── .gitignore
```

- **backend/**: Contains the Rust Axum API server.
- **frontend/**: Contains the Next.js React application.

## Features

- 🦀 **Rust Backend**: Fast and safe API server built with Axum
- ⚛️ **Next.js Frontend**: Modern React framework with TypeScript
- 🔐 **JWT Authentication**: Secure user authentication system
- 🗃️ **PostgreSQL**: Robust relational database with SQLx
- 🐳 **Docker Support**: Easy development setup with Docker Compose
- 🔄 **Database Migrations**: Version-controlled schema changes
- 🛡️ **Middleware**: CORS, authentication, and request tracing
- 📝 **Type Safety**: End-to-end type safety with Rust and TypeScript

## Technology Stack

### Backend

- **Framework**: Axum 0.8+
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with bcrypt
- **Logging**: Tracing with structured logging
- **Validation**: Validator crate for input validation

### Frontend

- **Framework**: Next.js 14+ (App Router)
- **Language**: TypeScript
- **Styling**: CSS Modules / Tailwind CSS
- **Build Tool**: Turbopack

### DevOps

- **Database**: PostgreSQL via Docker
- **Environment**: Docker Compose for development

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Node.js](https://nodejs.org/) 18.0+ (includes pnpm)
- [Docker](https://www.docker.com/) & Docker Compose
- [PostgreSQL](https://www.postgresql.org/) (via Docker or local installation)

### Steps

1. **Clone the repository:**

```sh
git clone https://github.com/yourusername/axum-nextjs.git
cd axum-nextjs
```

2. **Install backend dependencies:**

```sh
cd backend
cargo build
```

3. **Install frontend dependencies:**

```sh
cd ../frontend
pnpm install
```

## Running the Project

### Start the backend (Axum):

```sh
cd backend
cargo run
```

### Run Migrations:

```sh
cargo install sqlx-cli --no-default-features --features rustls,postgres
sqlx migrate run
```

### COMPLETE PROCESS IN ONE /():

```sh
cd backend

# 2. Install SQLx CLI
cargo install sqlx-cli --no-default-features --features rustls,postgres

# 3. Make sure PostgreSQL is running
docker-compose up -d postgres

# 4. Create .env file with DATABASE_URL
echo "DATABASE_URL=postgresql://postgres:password@localhost:5432/myapp" > .env
echo "JWT_SECRET=your-super-secret-jwt-key-change-this-in-production" >> .env
echo "RUST_LOG=debug" >> .env
echo "PORT=3001" >> .env

# 5. Create the database
sqlx database create

# 6. Run migrations
sqlx migrate run

# 7. Start the application
cargo run
```

### Start the frontend (Next.js):

```sh
cd frontend
pnpm run dev
```

The backend will typically run on `localhost:3001` and the frontend on `localhost:3000`.

## API Endpoints

The backend provides the following REST API endpoints:

### Public Routes

- `GET /api/health` - Health check endpoint
- `POST /api/auth/register` - User registration
- `POST /api/auth/login` - User login

### Protected Routes (requires JWT token)

- `GET /api/users/me` - Get current user profile
- `POST /api/users/me` - Update current user profile

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Authors

- Cesare Montedonico

## Official Documentation

- [Axum](https://docs.rs/axum/) - Web framework for Rust
- [Tokio](https://docs.rs/tokio/) - Asynchronous runtime for Rust
- [Next.js](https://nextjs.org/docs) - React framework for production
- [SQLx](https://docs.rs/sqlx/) - Async SQL toolkit for Rust
- [PostgreSQL](https://www.postgresql.org/docs/) - Advanced open source database

---

## License

This project is licensed under the MIT License - see below for details.

```
MIT License

Copyright (c) 2025 Cesare Montedonico

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

**Made with ❤️ using Rust and TypeScript**
