# ADR-007: Use Docker Compose for MVP Deployment

## Status

Accepted

## Context

Universal Platform will start with a simple VPS deployment.

The MVP needs repeatable service orchestration without introducing the complexity of Kubernetes or advanced orchestration too early.

The initial deployment may include:

- Rust API
- PostgreSQL
- Redis
- Background worker
- Reverse proxy

## Decision

Use Docker Compose for the MVP deployment.

Docker Compose will manage local and early production service composition.

## Alternatives Considered

### Manual Process Management

Rejected as the main direction because service setup becomes less repeatable and harder for new developers or AI agents to understand.

### Kubernetes

Deferred because it adds operational complexity that is not needed at MVP stage.

Kubernetes may be considered later when the platform has multiple services, teams, or scaling requirements.

## Consequences

### Positive

- Simple deployment model.
- Repeatable local development.
- Easier onboarding.
- Clear service definitions.
- Easier path to separate services later.

### Cost

- Not as powerful as Kubernetes for large-scale orchestration.
- Requires careful environment and volume management.
- Production usage must be configured carefully.

## Implementation Rules

- Keep Compose files in the infrastructure folder.
- Do not commit production secrets.
- Use environment variables.
- Define service health checks where practical.
- Keep PostgreSQL and Redis internal to the Compose network during MVP.

## Final Rule

Docker Compose is the MVP orchestration tool. The architecture must still allow future migration to more advanced infrastructure when needed.
