# ADR-012: Use Application Service and Repository Pattern

## Status

Accepted

## Context

Universal Platform will contain Platform Core, shared engines, multiple domains, infrastructure adapters, and API clients.

Without clear application boundaries, business logic may spread into route handlers, database queries, infrastructure adapters, or UI code.

## Decision

Use application services to coordinate use cases and repositories to provide data access boundaries.

API handlers should stay thin.

Application services should coordinate validation, authorization, repositories, engines, events, and audit where needed.

Repositories should own database access patterns and protect organization-scoped queries.

## Consequences

### Positive

- Cleaner separation of concerns
- Easier testing
- Safer tenant-aware data access
- Less business logic in API handlers
- Easier future refactoring
- Better guidance for coding agents

### Cost

- More structure than simple route-handler code
- Requires discipline around boundaries
- Some small use cases may feel more verbose

## Implementation Direction

Use this general flow:

```text
API Handler
  -> Application Service
    -> Repository / Engine Interface
      -> Infrastructure Implementation
```

Do not place domain business rules directly in route handlers.

Do not scatter raw database queries across the API layer.

## Final Rule

Handlers expose routes. Application services execute use cases. Repositories protect data access boundaries.
