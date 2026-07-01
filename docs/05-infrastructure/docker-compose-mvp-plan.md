# Docker Compose MVP Plan

## Purpose

This document defines the high-level Docker Compose plan for the MVP.

Docker Compose should make local development and early VPS deployment consistent and understandable.

## Services

The MVP Compose setup should include:

- api
- postgres
- redis
- worker
- reverse_proxy later or in deployment-specific compose file

## API Service

The API service runs the Rust Axum backend.

Responsibilities:

- HTTP API
- Authentication routes
- Organization routes
- Engine endpoints
- Domain endpoints later

## PostgreSQL Service

PostgreSQL is the primary database.

Used for:

- Platform Core data
- Engine data
- Domain data
- Audit logs
- Configuration records

The application should connect through `DATABASE_URL`.

## Redis Service

Redis supports:

- Cache
- Queue foundation
- Rate-limit support
- Temporary state where appropriate

Redis must not be the source of truth for critical business records.

## Worker Service

Workers process background jobs.

Examples later:

- SMS sending
- Report exports
- Import jobs
- Notification delivery
- Payment reconciliation checks

## Environment Files

Use example environment files for development.

Production secrets must not be committed.

Suggested files:

```text
.env.example
.env.development.example
```

## Volumes

Use volumes for local development persistence:

- PostgreSQL data
- Redis data where useful
- Local uploads

## Health Checks

Add health checks where practical:

- API health endpoint
- Database readiness
- Redis readiness

## Migration Flow

Migrations should run as a controlled command.

Do not hide migration behavior in a way that developers cannot reason about.

## Local Development Goal

A developer should be able to:

1. Copy example env file.
2. Start services.
3. Run migrations.
4. Open health endpoint.
5. Run tests.

## Future Evolution

Docker Compose is the MVP orchestration tool.

The architecture should still allow future movement to more advanced infrastructure when needed.

## Final Rule

Compose defines service wiring. Application code must still depend on environment configuration, not fixed container names.
