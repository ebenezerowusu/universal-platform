# ADR-008: Core-First Development Sequence

## Status

Accepted

## Context

Universal Platform has multiple planned domains, including Religious, Commerce, POS, Finance, and HR.

It may be tempting to start by building the first visible product feature, such as Religious member management. That would create a weak foundation and increase the risk of domain-specific logic leaking into Platform Core.

## Decision

Development must begin with the Platform Core and shared engines before product domains.

Implementation order:

1. Runtime foundation
2. API foundation
3. Database foundation
4. Identity Engine
5. Organization Engine
6. Permission Engine
7. Audit Engine
8. Module Registry
9. Subscription Engine
10. First engine skeletons
11. First domain implementation

## Rationale

All domains depend on:

- Identity
- Organizations
- Permissions
- Tenant isolation
- Module availability
- Subscription access
- Audit logs
- Shared API standards

Building domain features before these foundations would create rework.

## Consequences

### Positive

- Cleaner architecture
- Better tenant isolation
- Reusable engines
- Easier future domains
- Better AI coding-agent guidance

### Cost

- Fewer visible features at the beginning
- More upfront architecture work
- Requires discipline from all contributors

## Final Rule

The first working milestone should prove the platform foundation, not a product feature.
