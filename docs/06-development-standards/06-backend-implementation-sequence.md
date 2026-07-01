# Backend Implementation Sequence

## Purpose

This document defines the recommended order for backend development after Phase 0 foundation documentation.

The platform must not start with domain-specific features. It must start with the Platform Core foundation.

## Backend Stack

Accepted direction:

- Rust
- Axum
- Tokio
- PostgreSQL
- Redis
- Docker Compose
- Nginx in deployment

## Implementation Principle

Build the stable platform foundation first.

Domains such as Religious, Commerce, POS, Finance, and HR should be installed on top of the core foundation.

## Stage 1: Repository and Workspace Foundation

Create the backend workspace structure.

Expected direction:

```text
backend/
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

This structure may evolve, but the separation must remain clear.

## Stage 2: Runtime Foundation

Implement:

- Application entrypoint
- Axum router setup
- Configuration loader
- Environment handling
- Structured logging/tracing
- Request ID middleware
- Health check endpoint
- Graceful shutdown

Health endpoint example:

```text
GET /health
```

## Stage 3: Database Foundation

Implement:

- PostgreSQL connection pool
- Migration tooling
- Database health check
- Base repository patterns
- Transaction handling pattern

No domain tables should be created before core identity/organization foundations are understood.

## Stage 4: API Foundation

Implement:

- Standard response format
- Standard error format
- Validation approach
- API version prefix
- Authentication middleware placeholder
- Tenant context middleware placeholder
- Permission middleware placeholder

## Stage 5: Identity Foundation

Implement:

- Users
- Authentication
- Password hashing
- Login
- Token/session strategy
- Password reset foundation later

Identity must be platform-level, not organization-specific only.

A user may belong to multiple organizations.

## Stage 6: Organization / Tenant Foundation

Implement:

- Organizations
- Organization users
- Tenant context resolution
- Organization settings foundation
- Organization status

Every tenant-owned feature depends on this.

## Stage 7: Permission Engine Foundation

Implement:

- Permissions
- Roles
- Role permissions
- Organization user roles
- Scope model
- Permission checking service

Every protected action must eventually use this engine.

## Stage 8: Audit Engine Foundation

Implement:

- Audit log table
- Audit service
- Actor tracking
- Organization tracking
- Entity/action metadata

Sensitive actions should call the audit service.

## Stage 9: Module Registry Foundation

Implement:

- Module definitions
- Organization module installations
- Module activation status
- Feature availability checks

Domains should not be usable by organizations unless installed and active.

## Stage 10: Subscription Foundation

Implement:

- Plans
- Plan features
- Plan limits
- Organization subscriptions
- Feature checks
- Limit checks

This may be basic at first but must exist before paid modules/features grow.

## Stage 11: Provider Registry Foundation

Implement:

- Provider definitions
- Country/provider mappings
- Provider credentials placeholder strategy
- Provider status

Actual provider adapters can come later, but the model must support provider abstraction.

## Stage 12: First Capability Engines

After the foundation:

1. SMS Engine skeleton
2. Payment Engine skeleton
3. Notification Engine skeleton
4. Storage Engine skeleton
5. Workflow Engine skeleton

Start with interfaces and internal mock adapters before production providers.

## Stage 13: First Domain Installation

Only after the above should the Religious Domain begin.

Religious Domain first implementation order:

1. Religious profile/settings
2. Branch hierarchy
3. Member management
4. Congregations/services
5. Groups
6. Attendance integration
7. Visitors
8. Communication integration
9. Giving/payment integration

## Testing Requirements by Stage

Each stage must include tests.

Mandatory early tests:

- Health endpoint works
- Configuration loads correctly
- Database connection works
- User auth works
- Tenant isolation works
- Permission checks work
- Organization user cannot access another organization
- Disabled module blocks access

## What Not to Do

Do not start by building member management.

Do not create Religious Domain tables before identity, organization, permissions, audit, and module registry are planned.

Do not integrate Hubtel or Arkessel directly into domain logic.

Do not build Flutter screens before backend contracts are stable.

## Final Rule

Core first. Engines second. Domains third. UI after stable APIs.
