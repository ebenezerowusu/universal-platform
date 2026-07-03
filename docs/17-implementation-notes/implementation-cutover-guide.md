# Implementation Cutover Guide

## Purpose

This document is the final cutover guide from architecture documentation to code implementation.

It tells engineering agents what to build first, what to avoid, and how to preserve the architecture while creating the first executable foundation.

## Cutover Decision

```text
architecture documentation track complete
implementation should start now
```

Architecture expansion should no longer be the default action.

## Immediate Branch

```text
chore/backend-scaffold
```

## Immediate PR Title

```text
chore: initialize backend scaffold
```

## First Implementation Scope

Create the backend scaffold only:

```text
Cargo.toml
crates/platform-api/Cargo.toml
crates/platform-api/src/main.rs
crates/platform-core/Cargo.toml
crates/platform-core/src/lib.rs
crates/platform-core/src/errors.rs
crates/platform-core/src/ids.rs
crates/platform-config/Cargo.toml
crates/platform-config/src/lib.rs
crates/platform-infra/Cargo.toml
crates/platform-infra/src/lib.rs
crates/platform-infra/src/adapters.rs
.env.example
docker-compose.yml
migrations/.gitkeep
.github/workflows/backend-ci.yml
docs/17-implementation-notes/sprint-1-backend-scaffold-log.md
```

## First API Routes

```text
GET /health
GET /ready
```

Expected behavior:

- `/health` confirms the API process is alive.
- `/ready` may initially return a placeholder until PostgreSQL and Redis checks are wired.

## First Environment Variables

```env
APP_ENV=local
APP_NAME=universal-platform
API_HOST=0.0.0.0
API_PORT=8080
DATABASE_URL=postgres://platform:platform@localhost:5432/platform_db
REDIS_URL=redis://localhost:6379
RUST_LOG=info
```

## First Docker Compose Services

```text
PostgreSQL
Redis
```

Do not require cloud services for the first implementation.

## First Rust Workspace Shape

```text
platform-api
  Axum HTTP entrypoint

platform-core
  shared errors, IDs, core primitives

platform-config
  environment configuration loading

platform-infra
  infrastructure adapter traits and placeholders
```

## What Not To Build In The First PR

Do not implement yet:

```text
authentication flows
tenant creation flows
permission enforcement engine
audit persistence
module registry persistence
payment providers
SMS providers
religious domain features
commerce/POS features
partner features
frontend screens
production deployment automation
```

## Quality Gates

Run where available:

```text
cargo fmt
cargo check
cargo test
```

CI should run the same checks for the backend workspace.

## Architecture Boundaries To Preserve

- Platform Core must remain domain-agnostic.
- API handlers should stay thin.
- Business logic should live in application/domain services once introduced.
- Provider-specific logic must stay behind adapters.
- Configuration should be environment-driven.
- Tenant isolation must be designed before tenant-owned tables are introduced.
- Permission checks must be enforced before protected features are exposed.
- Audit records must be added before sensitive actions are introduced.

## First Implementation Checklist

Before opening the PR, confirm:

```text
[ ] Workspace compiles
[ ] API starts locally
[ ] /health responds
[ ] /ready responds with placeholder or real readiness
[ ] .env.example exists
[ ] Docker Compose starts PostgreSQL and Redis
[ ] CI workflow exists
[ ] Implementation log exists
[ ] No domain-specific code entered Platform Core
[ ] No provider secrets are stored in code
[ ] No speculative modules were implemented
```

## Recommended PR Summary

```text
## Summary
- Initialized Rust workspace
- Added Axum API scaffold
- Added platform-core, platform-config, and platform-infra crates
- Added health/readiness endpoints
- Added local PostgreSQL and Redis Docker Compose
- Added backend CI workflow
- Added implementation log

## Architecture Boundaries
- Core remains domain-agnostic
- Provider logic is not implemented yet
- Tenant/domain features are not implemented yet
- External services are represented only as future adapters

## Checks
- cargo fmt
- cargo check
- cargo test
```

## Next Implementation After Backend Scaffold

Recommended order:

```text
1. Identity foundation
2. Organization/Tenant foundation
3. Permission foundation
4. Audit foundation
5. Module Registry foundation
6. Configuration and Feature Flags foundation
7. Provider Connections foundation
8. Religious MVP first slice
9. Flutter client integration
```

## Documentation Rule After Cutover

Only create new documentation when:

- implementation reveals a real missing decision
- a schema or API contract needs clarification
- a new ADR is required to resolve a concrete conflict
- the user explicitly asks for new architecture expansion

## Final Rule

Stop planning by default.

Start building.
