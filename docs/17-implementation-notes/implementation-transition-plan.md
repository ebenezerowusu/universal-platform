# Implementation Transition Plan

## Purpose

This document converts the completed architecture documentation track into an execution plan.

The goal is to start building the Universal Platform foundation immediately and avoid further architecture drift.

## Current State

The repository now contains documentation for:

- platform vision and constitution
- core architecture
- domains and capability engines
- deployment and infrastructure
- development standards
- database strategy
- security and privacy
- audit and compliance
- module registry and subscription foundations
- religious MVP foundations
- Flutter client foundations
- SMS/payment/finance/commerce/logistics/HR foundations
- configuration, background jobs, provider connections
- data quality, lineage, portability, organization networks
- governance, risk, policy, knowledge base, communication
- feedback, analytics, experimentation, customer success
- implementation delivery, solution blueprints, partners, referrals

This is enough for the first implementation stage.

## Immediate Implementation Branch

```text
chore/backend-scaffold
```

## Immediate PR Title

```text
chore: initialize backend scaffold
```

## Files To Add First

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

## First Backend Behavior

The first backend implementation should expose:

```text
GET /health
GET /ready
```

The health endpoint should confirm the API process is alive.

The readiness endpoint may initially return a placeholder until database and Redis checks are wired.

## Environment Variables

```env
APP_ENV=local
APP_NAME=universal-platform
API_HOST=0.0.0.0
API_PORT=8080
DATABASE_URL=postgres://platform:platform@localhost:5432/platform_db
REDIS_URL=redis://localhost:6379
RUST_LOG=info
```

## Local Services

Use Docker Compose for:

- PostgreSQL
- Redis

Do not require cloud services for first implementation.

## Quality Gates

The first implementation PR should pass:

- cargo fmt
- cargo check
- cargo test where tests exist
- backend CI workflow

## Architecture Boundaries To Preserve

- Platform Core must remain domain-agnostic.
- Provider-specific logic must stay behind adapters.
- API handlers should stay thin.
- Future business logic should live in application/domain services.
- Configuration should be environment-driven.
- Tenant isolation must be designed from the beginning.
- Audit and permission boundaries must not be bypassed when introduced.

## Implementation Order After Scaffold

1. Identity foundation
2. Organization/tenant foundation
3. Permission foundation
4. Audit foundation
5. Module registry foundation
6. Configuration and feature flags foundation
7. First Religious MVP slice
8. Flutter client integration

## Documentation Rule During Implementation

Only add docs when code reveals a concrete missing decision or boundary.

Do not expand future architecture by default.

## Final Rule

The project is ready to build.

Start with the backend scaffold and prove the architecture through code.
