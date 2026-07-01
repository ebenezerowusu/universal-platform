# First Implementation Task

## Purpose

This document is the exact first coding-agent handoff for Universal Platform implementation.

It tells Claude Code or any implementation agent what to build first and what not to build yet.

## Task Name

```text
Sprint 1: Backend Scaffold and Health Endpoint
```

## Required Reading

Before coding, read in this order:

1. `CLAUDE.md`
2. `docs/01-platform-constitution/01-platform-constitution.md`
3. `docs/02-architecture/02-platform-architecture.md`
4. `docs/06-development-standards/backend-crate-boundaries.md`
5. `docs/06-development-standards/rust-backend-structure.md`
6. `docs/06-development-standards/application-service-pattern.md`
7. `docs/06-development-standards/rust-error-handling-standards.md`
8. `docs/06-development-standards/backend-scaffold-acceptance-criteria.md`
9. `docs/14-roadmap/sprint-1-issue-breakdown.md`

## Objective

Create the first Rust backend foundation.

This task should prove that the backend can start, load configuration, return standard responses, expose a health endpoint, and run first tests.

## Strict Scope

Implement only:

- backend workspace structure
- Axum API shell
- runtime configuration foundation
- standard response model
- standard error model
- health endpoint
- first tests
- local development notes if needed

## Out of Scope

Do not implement:

- Identity
- login
- organizations
- permissions
- audit
- module registry
- subscriptions
- Religious Domain
- Payment Engine
- SMS Engine
- Flutter UI
- live providers

## Expected Structure Direction

Use the repository structure target as the guide.

Expected backend areas:

```text
backend/
  crates/
    api/
    shared/
    core/
    engines/
    domains/
    infrastructure/
  migrations/
  tests/
```

The exact Rust crate setup may be adjusted if needed, but architecture boundaries must remain clear.

## Implementation Steps

1. Create backend workspace.
2. Create initial crate or module structure.
3. Add Axum application shell.
4. Add configuration loading foundation.
5. Add shared success response structure.
6. Add shared error response structure.
7. Add health route.
8. Add first tests.
9. Update local development notes if needed.

## Acceptance Criteria

The task is complete when:

- backend project builds
- app can start locally
- health endpoint returns standard success response
- standard error model exists
- configuration foundation exists
- tests exist for health/response behavior
- no domain feature is implemented
- no real secret is committed

## Required Final Summary

The coding agent must report:

```text
What changed:
Files changed:
Tests added:
Checks run:
Architecture rules followed:
Out of scope items avoided:
Known limitations:
Follow-up work:
```

## Final Rule

This first task proves the foundation. Do not rush into product features before the backend base is stable.
