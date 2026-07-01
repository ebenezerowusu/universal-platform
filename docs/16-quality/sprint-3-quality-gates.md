# Sprint 3 Quality Gates

## Purpose

This document defines quality gates for Sprint 3 implementation.

Sprint 3 should establish Permission and Audit foundations without expanding into modules, subscriptions, domains, or provider integrations.

## Gate 1: Scope Control

Sprint 3 may include:

- Permission migrations
- role and permission repository foundation
- permission seed foundation
- role assignment foundation
- permission check service foundation
- Audit migration
- audit repository foundation
- audit service foundation
- permission allow/deny tests
- audit write tests

Sprint 3 must not include:

- Module Registry implementation
- Subscription Engine implementation
- Religious Domain implementation
- payment provider integration
- SMS provider integration
- Flutter UI
- advanced scope implementation beyond the foundation

## Gate 2: Permission Boundary Check

Confirm:

- permission checks live in Permission Engine
- API handlers do not manually inspect permission data
- application services call Permission Engine for protected actions
- stable permission keys are used

## Gate 3: Role Assignment Check

Confirm:

- roles are organization-aware
- organization user role assignment is represented
- role permission assignment is represented
- seed behavior avoids duplicate permission records

## Gate 4: Audit Boundary Check

Confirm:

- audit writes go through Audit Engine
- audit records include actor where available
- audit records include organization where applicable
- audit metadata avoids unnecessary sensitive values

## Gate 5: API Contract Check

Confirm Sprint 3 routes align with:

- `docs/12-api/permission-api-contract.md`
- `docs/12-api/audit-api-contract.md`
- `docs/12-api/standard-response-contract.md`

## Gate 6: Security Flow Check

Confirm implementation follows:

- `docs/08-security/authorization-decision-flow.md`
- `docs/08-security/authentication-context-flow.md`
- `docs/08-security/mvp-security-checklist.md`

## Gate 7: Data Model Check

Confirm migrations follow:

- `docs/07-database/initial-migration-order.md`
- `docs/07-database/database-naming-conventions.md`
- `docs/07-database/database-relationship-map.md`

## Gate 8: Test Check

Minimum tests should cover:

- seeded permission exists
- role can receive permission
- organization user can receive role
- permission check allows when assigned
- permission check denies when missing
- audit record can be written
- audit metadata shape is safe

## Gate 9: Documentation Check

If implementation differs from documented API, data model, or authorization flow, update the relevant docs in the same PR.

## Final Rule

Sprint 3 is complete only when authorization and accountability foundations are stable enough for Module Registry, Subscription, and Religious Domain work to build on them.
