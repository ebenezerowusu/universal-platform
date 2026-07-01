# Sprint 5 Quality Gates

## Purpose

This document defines quality gates for Sprint 5 implementation.

Sprint 5 should establish the Religious MVP foundation without expanding into advanced Religious features, providers, or Flutter UI.

## Gate 1: Scope Control

Sprint 5 may include:

- Religious module registration checks
- Religious settings foundation
- Religious member foundation
- Religious visitor foundation
- congregations, services, and groups foundation
- group membership foundation
- manual attendance foundation
- basic Religious report foundation
- permission/module/feature checks through existing engines
- audit writes for important changes
- tests for Religious MVP flows

Sprint 5 must not include:

- QR attendance
- GPS attendance
- live SMS provider integration
- live payment provider integration
- full giving module
- full family/household module
- full care/counseling/welfare module
- advanced branch governance
- Flutter UI
- offline sync

## Gate 2: Core Boundary Check

Confirm:

- Platform Core does not contain Religious business terms
- Religious business logic stays inside Religious Domain
- Core concepts remain generic

## Gate 3: Module and Feature Check

Confirm:

- Religious use cases check required module availability
- feature availability is checked where foundation exists
- Religious routes fail safely when the module is unavailable

## Gate 4: Permission Check

Confirm:

- protected Religious actions use Permission Engine
- API handlers do not duplicate permission logic
- permission keys follow the documented permission catalog direction

## Gate 5: Audit Check

Confirm important changes are auditable, such as:

- member created
- member status changed
- visitor created
- visitor converted
- group created
- attendance session created
- attendance records submitted

## Gate 6: API Contract Check

Confirm Sprint 5 routes align with:

- `docs/12-api/religious-settings-api-contract.md`
- `docs/12-api/religious-member-visitor-api-contract.md`
- `docs/12-api/religious-structure-attendance-api-contract.md`
- `docs/12-api/religious-mvp-api-contracts.md`
- `docs/12-api/standard-response-contract.md`

## Gate 7: Data Model Check

Confirm migrations follow:

- `docs/07-database/religious-domain-data-model.md`
- `docs/07-database/religious-mvp-migration-plan.md`
- `docs/07-database/database-naming-conventions.md`
- `docs/07-database/database-relationship-map.md`

## Gate 8: Test Check

Minimum tests should cover:

- create Religious settings or read defaults
- create member
- list members by organization
- create visitor
- convert visitor to member
- create congregation
- create service
- create group
- add member to group
- create attendance session
- mark attendance record
- deny access when module is unavailable where practical
- deny access when permission is missing where practical

## Gate 9: Report Foundation Check

Confirm basic reports are organization-scoped.

Recommended report foundations:

- member count
- visitor count
- group count
- attendance session summary

## Gate 10: Documentation Check

If implementation differs from documented API, data model, or Religious implementation flow, update the relevant docs in the same PR.

## Final Rule

Sprint 5 is complete only when the Religious MVP proves the platform architecture instead of bypassing it.
