# Rust Backend Structure Guide

## Purpose

This document defines the recommended backend structure for Universal Platform.

The backend should be a modular monolith first, with strong internal boundaries.

## Recommended Folder Structure

```text
backend/
  Cargo.toml
  crates/
    api/
    core/
    shared/
    infrastructure/
    engines/
    domains/
  migrations/
  tests/
```

## Crate Responsibilities

### api

Responsible for:

- Axum application setup
- HTTP routes
- Middleware wiring
- Request validation boundary
- Response formatting
- OpenAPI generation later

API handlers must not contain business logic.

### core

Responsible for:

- Platform Core models
- Core application services
- Organization foundation
- Identity foundation
- Module registry foundation
- Subscription foundation

The core must remain domain-agnostic.

### shared

Responsible for:

- Shared error types
- Shared result types
- Common utilities
- Time helpers
- ID types
- API response models
- Validation helpers

Shared code must not become a dumping ground for domain logic.

### infrastructure

Responsible for:

- PostgreSQL implementation
- Redis implementation
- Storage adapters
- Provider adapters
- External integration clients
- Configuration loading

Infrastructure implements contracts. It must not own business rules.

### engines

Responsible for reusable platform engines:

- Permission Engine
- Audit Engine
- Payment Engine
- SMS Engine
- Notification Engine
- Storage Engine
- Reporting Engine
- Workflow Engine

### domains

Responsible for business domains:

- Religious Domain
- Commerce Domain
- POS Domain
- Finance Domain
- HR Domain
- Future domains

Domains must use engines and platform contracts.

## Layering Rule

Preferred flow:

```text
API Handler
  -> Application Service
    -> Domain/Core Logic
      -> Repository or Engine Interface
        -> Infrastructure Implementation
```

Do not reverse this dependency.

## Handler Rule

Handlers should only:

1. Extract request data.
2. Validate request shape.
3. Resolve context.
4. Call application service.
5. Return response.

## Naming Direction

Use clear module names.

Examples:

```text
identity
organizations
permissions
audit
modules
subscriptions
```

Domain modules should be namespaced.

Examples:

```text
domains::religious::members
domains::pos::products
domains::finance::accounts
```

## Error Handling

Use a shared application error type and convert to API errors at the edge.

Do not leak infrastructure-specific errors directly to clients.

## Testing Direction

Place tests where they are most useful:

- Unit tests beside domain/core logic
- Integration tests under backend/tests
- Repository tests for database behavior
- API tests for route behavior

## Final Rule

The backend is one deployable system, but it must behave internally like a disciplined platform with strong module boundaries.
