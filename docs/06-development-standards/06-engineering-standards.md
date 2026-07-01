# Engineering Standards

## Purpose

This document defines how Universal Platform should be engineered.

The goal is to make the system maintainable, testable, modular, and understandable by both human engineers and AI coding agents.

## Architecture Style

The backend should follow:

- Clean Architecture
- Domain-Driven Design
- Modular monolith first
- Event-driven communication where useful
- Infrastructure adapter pattern

The system may evolve into distributed services later, but the first implementation should be a disciplined modular monolith.

## Backend Direction

Primary backend stack:

- Rust
- Axum
- Tokio
- PostgreSQL
- Redis
- Docker Compose

Expected Rust principles:

- Strong typing
- Explicit error handling
- Clear module boundaries
- Small focused crates/modules
- No business logic in handlers
- No provider calls inside domains
- Repository interfaces where needed
- DTOs separated from domain models

## Frontend Direction

Primary frontend stack:

- Flutter
- Shared codebase for mobile and desktop
- Role-based routing
- Module-aware navigation
- Platform-specific layouts where needed
- Localization support from the beginning

Flutter must not contain core business logic.

The frontend should render based on:

- Authenticated user
- Organization context
- Installed modules
- Permissions
- Feature flags
- Country/language configuration

## API Standards

APIs should be:

- Versioned
- Consistent
- Secure
- Documented through OpenAPI
- Pagination-aware
- Filter-aware
- Tenant-aware

Handlers should do minimal work:

1. Authenticate request.
2. Validate input.
3. Resolve tenant context.
4. Call application service.
5. Return standardized response.

## Error Handling

Errors should be structured and consistent.

Expected error fields:

- code
- message
- details
- traceId

Do not expose internal stack traces or provider secrets to clients.

## Testing Standards

Minimum test types:

- Unit tests for domain logic
- Application service tests
- Repository/integration tests
- API contract tests
- Permission tests
- Tenant isolation tests
- Provider adapter tests with mocks

Tenant isolation tests are mandatory for any feature touching organization-owned data.

## Git Workflow

Use branches for meaningful work.

Suggested branch format:

```text
feature/<short-description>
docs/<short-description>
fix/<short-description>
```

Commit messages should be clear and intentional.

Examples:

```text
Add platform constitution
Add PostgreSQL database strategy
Define SMS engine provider abstraction
```

## Code Review Rules

Every review should check:

- Does this violate the Platform Constitution?
- Is business logic in the correct layer?
- Is the feature tenant-aware?
- Does it use capability engines instead of direct providers?
- Is it configurable where necessary?
- Are permissions enforced?
- Are tests included?
- Are errors safe and structured?

## Documentation Rules

Every major feature should have documentation.

Major architecture decisions require ADRs.

Documentation should be updated before or alongside implementation.

## AI Agent Rules

AI coding agents must not start coding from vague prompts.

They must read:

1. Platform Vision
2. Platform Constitution
3. Architecture Overview
4. Relevant engine/domain specs
5. Database strategy
6. AI coding-agent rules

If a requested implementation conflicts with the constitution, the agent must stop and explain the conflict.

## Non-Negotiable Engineering Rule

Do not optimize for speed at the cost of architectural damage.

Fast implementation is valuable only when it preserves the platform foundation.
