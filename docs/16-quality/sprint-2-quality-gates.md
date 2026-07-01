# Sprint 2 Quality Gates

## Purpose

This document defines quality gates for Sprint 2 implementation.

Sprint 2 should establish Identity and Organization foundations without expanding into authorization, modules, or domains.

## Gate 1: Scope Control

Sprint 2 may include:

- Identity migrations
- user repository foundation
- credential/session foundation
- login use case foundation
- current user endpoint
- Organization migrations
- organization repository foundation
- create organization use case
- list current user organizations use case
- get organization detail use case
- first Identity and Organization tests

Sprint 2 must not include:

- Permission Engine implementation
- Audit Engine implementation
- Module Registry implementation
- Subscription Engine implementation
- Religious Domain implementation
- provider integrations
- Flutter UI

## Gate 2: Identity Boundary Check

Confirm:

- credential handling stays inside Identity foundation
- API responses return safe user summaries only
- credential internals are not returned
- credential values are not written to logs

## Gate 3: Organization Boundary Check

Confirm:

- organization membership is represented
- user can belong to many organizations
- organization records are not domain-specific
- organization context is reusable by later engines and domains

## Gate 4: API Contract Check

Confirm Sprint 2 routes align with:

- `docs/12-api/identity-api-contract.md`
- `docs/12-api/organization-api-contract.md`
- `docs/12-api/standard-response-contract.md`

## Gate 5: Data Model Check

Confirm migrations follow:

- `docs/07-database/initial-migration-order.md`
- `docs/07-database/database-naming-conventions.md`
- `docs/07-database/database-relationship-map.md`

## Gate 6: Test Check

Minimum tests should cover:

- login success where implemented
- login failure where implemented
- current user response
- create organization
- list current user organizations
- get organization detail
- organization membership creation

## Gate 7: Documentation Check

If implementation differs from documented API or data model direction, update the relevant docs in the same PR.

## Final Rule

Sprint 2 is complete only when Identity and Organization are stable enough for Permission Engine and Audit Engine to build on them.
