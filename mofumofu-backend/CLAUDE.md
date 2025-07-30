# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based social media backend API (mofu-backend) built with Axum web framework, SeaORM for database operations, and PostgreSQL as the primary database. The project follows a modular architecture with clear separation of concerns.

## Development Commands

### Main Application (Rust)
- `cargo run` - Start the development server
- `cargo build` - Build the application
- `cargo build --release` - Build optimized release version
- `cargo test` - Run tests
- `cargo check` - Quick syntax and type checking
- `cargo clippy` - Run linter for code quality checks
- `cargo fmt` - Format code according to Rust standards

### Database Migrations (SeaORM)
- `cd migration && cargo run` - Run database migrations
- `cd migration && cargo run -- refresh` - Refresh all migrations

### Task Runner (Python/FastAPI)
Located in `tasks/` directory:
- `cd tasks && uv run fastapi dev app/main.py` - Start task runner API
- `cd tasks && uv run ruff check` - Run Python linter

## Architecture Overview

### Core Structure
- **API Layer**: `/src/api/v0/routes/` - RESTful endpoints organized by feature (auth, user, post, follow)
- **Service Layer**: `/src/service/` - Business logic implementation
- **Data Layer**: `/src/entity/` - Database entity definitions using SeaORM
- **DTOs**: `/src/dto/` - Data transfer objects for request/response handling
- **Middleware**: `/src/middleware/` - CORS, authentication, and request processing
- **Configuration**: `/src/config/` - Environment-based configuration management

### Key Components
- **Authentication**: JWT-based with access/refresh token pattern
- **Database**: PostgreSQL with connection pooling via SeaORM
- **API Documentation**: Auto-generated Swagger UI at `/docs` endpoint
- **Error Handling**: Centralized error management with proper HTTP status codes
- **Logging**: Structured logging with tracing crate

### Database Entities
- `users` - User accounts and profiles
- `posts` - User-generated content
- `comments` - Post comments
- `follows` - User following relationships
- `hash_tags` - Content tagging system
- `user_refresh_tokens` - JWT refresh token management

## Environment Setup

Copy `.env.example` to `.env` and configure:
- Database connection (PostgreSQL)
- JWT secret (generate with `openssl rand -base64 32`)
- Server host/port settings
- CORS configuration
- Token expiration times

## API Structure

All API endpoints are versioned under `/v0/`:
- `/v0/auth/*` - Authentication endpoints
- `/v0/users/*` - User management
- `/v0/posts/*` - Content management  
- `/v0/follow/*` - Social following features
- `/docs` - Swagger UI documentation

## Development Notes

- The codebase uses Korean comments in some files
- State management is handled through `AppState` with database connection sharing
- All routes are protected by appropriate middleware (CORS, compression, auth)
- The project includes both synchronous Rust backend and asynchronous Python task runner
- Database migrations are managed separately in the `/migration` directory