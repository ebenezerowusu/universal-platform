# ADR-014: API Contract First Delivery

## Status

Accepted

## Context

Universal Platform will serve multiple clients, including Flutter mobile, Flutter desktop/admin, future web clients, public APIs, and integrations.

If backend implementation moves faster than API contract documentation, clients and coding agents may build against inconsistent assumptions.

## Decision

Use API contract-first delivery for important route groups.

Before a route becomes permanent, it should have documented:

- path
- method
- request shape
- response shape
- error behavior
- pagination behavior where applicable
- owning engine or domain
- related permission where applicable
- related module where applicable

## Consequences

### Positive

- Better backend/frontend alignment
- Easier client development
- Easier testing
- Better OpenAPI generation later
- Fewer hidden route behavior changes
- Better guidance for coding agents

### Cost

- More documentation before implementation
- Requires discipline when routes change
- Requires review to keep docs and code aligned

## Implementation Direction

Use the API docs as the planning source before writing route handlers.

When implementation differs from the docs, update the docs in the same change.

API route owners should maintain route contracts.

## Client Direction

Flutter clients should build from documented API behavior and standard response shapes.

When backend contracts are unstable, Flutter screens should remain placeholders or use mock data.

## Final Rule

Important API behavior must be documented before it becomes permanent implementation.
