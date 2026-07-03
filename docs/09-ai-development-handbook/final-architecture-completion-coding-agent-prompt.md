# Final Architecture Completion Coding Agent Prompt

## Purpose

This document gives coding agents the final instruction after the architecture completion wave.

The platform has enough architecture documentation to begin implementation. Do not continue expanding documentation unless a concrete implementation task reveals a real gap.

## Required Reading Before Implementation

1. CLAUDE.md
2. docs/00-vision/00-platform-vision.md
3. docs/01-platform-constitution/01-platform-constitution.md
4. docs/02-architecture/02-platform-architecture.md
5. docs/06-development-standards/06-engineering-standards.md
6. docs/07-database/07-database-strategy.md
7. docs/08-security/08-security-standards.md
8. docs/09-ai-development-handbook/09-ai-coding-agent-rules.md
9. docs/09-ai-development-handbook/sprint-1-coding-agent-prompt.md
10. docs/10-adr/ADR-015-phase-0-documentation-complete-start-implementation.md
11. docs/10-adr/ADR-016-architecture-documentation-closed-start-implementation.md
12. Relevant API/domain/engine docs for the task being implemented

## Immediate Instruction

```text
Stop creating more architecture documentation by default.
Start implementation.

Begin with the backend scaffold on branch:
chore/backend-scaffold

Recommended PR title:
chore: initialize backend scaffold
```

## First Implementation Scope

Implement only:

- Rust workspace scaffold
- Axum API crate
- platform-core crate
- platform-config crate
- platform-infra crate
- health endpoint
- readiness endpoint placeholder
- AppConfig loading from environment
- standard error type placeholders
- ID/newtype primitives
- PostgreSQL and Redis adapter trait placeholders
- Docker Compose for local PostgreSQL and Redis
- .env.example
- backend CI workflow
- implementation log

## Do Not Implement Yet

Do not implement yet:

- authentication flows
- tenant creation flows
- payment providers
- SMS providers
- religious domain features
- commerce/POS features
- partner features
- marketplace features
- AI agents
- frontend screens
- production deployment automation

## Implementation Guardrails

Follow these rules:

- Core remains domain-agnostic.
- Business logic does not live in API handlers.
- Every future organization-owned record must be tenant-aware.
- Every protected action must pass permission checks once identity/permission foundation exists.
- Every sensitive action must be auditable once Audit Engine exists.
- External services must sit behind adapters.
- Configuration should control provider and country behavior.

## Completion Response Format

When implementation is complete, respond with:

```text
Files changed:

Migrations added:

API routes added:

Tests added:

Checks run:

Architecture boundaries followed:

Known limitations:

Next implementation step:
```

## Final Rule

The architecture documentation track is complete enough. Future work should prove the architecture through code.
