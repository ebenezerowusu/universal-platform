# ADR-003: Use Rust and Axum for the Backend

## Status

Accepted

## Context

Universal Platform requires a backend that is secure, reliable, performant, and suitable for a long-term modular platform.

The backend will support multi-tenancy, authentication, permissions, modules, payment flows, SMS, attendance, marketplace features, finance, HR, and future domains.

The backend must be disciplined, type-safe, and maintainable.

## Decision

Use Rust with Axum as the backend foundation.

Recommended supporting technologies:

- Tokio for async runtime
- Tower / tower-http for middleware
- Serde for serialization
- SQLx or SeaORM for database access after evaluation
- Utoipa or similar for OpenAPI documentation
- Tracing for structured logs
- Config/dotenvy for environment configuration

## Alternatives Considered

### Node.js / NestJS

NestJS is productive and familiar, but Rust offers stronger type safety, performance, and long-term reliability for a platform foundation.

### Go

Go is simple and production-proven, but Rust provides stronger compile-time safety and better control for correctness-focused architecture.

## Consequences

### Positive

- Strong type safety.
- Good performance.
- Memory safety.
- Good fit for reliable backend services.
- Encourages disciplined architecture.

### Negative / Cost

- Slower initial development than some dynamic stacks.
- Requires stronger Rust expertise.
- AI coding agents must follow strict project structure and patterns.

## Architecture Requirement

Rust handlers must not contain business logic.

Expected layering:

```text
API Handlers
  -> Application Services
    -> Domain Logic
      -> Repository/Engine Interfaces
        -> Infrastructure Adapters
```

The backend must follow the Platform Constitution and Clean Architecture rules.
