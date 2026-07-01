# ADR-004: Start with a Modular Monolith

## Status

Accepted

## Context

Universal Platform is large and will support many domains and modules.

A microservices architecture may eventually become useful, but starting with distributed services too early would increase operational complexity, deployment burden, debugging difficulty, and development cost.

The early platform needs strong module boundaries, not premature distribution.

## Decision

Start with a disciplined modular monolith.

The backend should be one deployable system with strong internal boundaries between:

- Platform Core
- Capability Engines
- Domains
- Infrastructure Adapters

The codebase must be structured so future service extraction is possible if needed.

## Why Not Microservices First

Microservices require mature DevOps, service discovery, distributed tracing, network fault handling, multiple deployments, versioned service contracts, and operational monitoring.

Those costs are not justified at MVP stage.

## Consequences

### Positive

- Faster development
- Easier deployment
- Easier debugging
- Simpler local development
- Strong consistency through PostgreSQL transactions
- Lower infrastructure cost
- Better fit for small early team and AI coding agents

### Negative / Cost

- Requires discipline to avoid internal coupling
- Boundaries must be enforced by architecture and reviews
- Future service extraction requires planning

## Boundary Rule

Even inside a modular monolith, modules must not access each other's private internals directly.

Use application services, events, and explicit contracts.

## Future Direction

If one part of the platform grows independently, it may later be extracted into a service.

Possible future service candidates:

- SMS sending workers
- Payment processing
- Reporting/analytics
- Notification delivery
- File processing

Do not extract services before clear operational need exists.
