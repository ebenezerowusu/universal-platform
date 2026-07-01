# ADR-010: Use Redis for Cache and Queue Foundation in MVP

## Status

Accepted

## Context

Universal Platform needs cache and background job support for the MVP.

Initial background work may include SMS sending, notifications, report exports, imports, and scheduled tasks.

Introducing a more complex message broker too early would increase operational burden.

## Decision

Use Redis as the MVP cache and queue foundation.

Redis will initially run with the application infrastructure and can move later through configuration.

## Consequences

### Positive

- Simple MVP infrastructure
- Supports cache needs
- Supports queue foundation
- Works well with Docker Compose
- Can move to dedicated infrastructure later

### Cost

- Redis is not the long-term answer for every messaging pattern.
- Queue behavior must be implemented carefully.
- Critical business truth must remain in PostgreSQL.

## Implementation Rules

- Use Redis through an abstraction.
- Do not hardcode Redis location.
- Do not store critical source-of-truth business records only in Redis.
- Use PostgreSQL for durable business records.
- Use job records where visibility and auditability are needed.

## Future Direction

If queue complexity grows, the platform may introduce a dedicated message broker later.

Possible future candidates can be evaluated through a new ADR.

## Final Rule

Redis is the MVP cache and queue tool, not the permanent architecture boundary.
