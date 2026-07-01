# Architecture Decision Checklist

## Purpose

This checklist helps decide whether a change needs an Architecture Decision Record or documentation update.

It is designed for human developers and AI coding agents.

## When to Create an ADR

Create an ADR when a change affects:

- primary technology choices
- deployment strategy
- database strategy
- authentication strategy
- module architecture
- provider integration pattern
- major domain boundaries
- API versioning
- queue or event architecture
- storage strategy
- security model

## When to Update Existing Docs

Update documentation when a change affects:

- API routes
- database tables
- permission keys
- module manifests
- domain behavior
- engine contracts
- deployment process
- local development setup
- testing expectations

## Decision Questions

Before implementing, ask:

1. Is this Core, Engine, Domain, Infrastructure, or UI?
2. Does it belong in Platform Core?
3. Is it domain-specific?
4. Should this be configurable?
5. Does it require a permission?
6. Does it require tenant isolation?
7. Does it require audit logging?
8. Does it use a provider?
9. Does it affect the database model?
10. Does it affect public API contracts?
11. Does it affect mobile or desktop UI behavior?
12. Does it need tests?

## Red Flags

Stop and review if the change:

- adds domain concepts to Platform Core
- calls a provider directly from a domain
- adds organization-owned data without organization scope
- creates an unbounded list endpoint
- bypasses Permission Engine
- stores secrets in regular records
- changes API response shape without documenting it
- adds a module without a manifest
- creates a database migration without review

## Final Rule

Architecture decisions should be visible. Hidden decisions become future technical debt.
