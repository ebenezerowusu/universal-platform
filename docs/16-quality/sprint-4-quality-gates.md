# Sprint 4 Quality Gates

## Purpose

This document defines quality gates for Sprint 4 implementation.

Sprint 4 should establish Module Registry, Subscription/Plan foundations, and shared engine skeletons before domain implementation begins.

## Gate 1: Scope Control

Sprint 4 may include:

- Module Registry migrations
- module catalog repository foundation
- organization module repository foundation
- module seed foundation
- module install/activate/deactivate use cases
- module availability check foundation
- Subscription/Plan migrations
- plan feature and limit repository foundation
- feature availability check foundation
- shared engine interface skeletons
- local/mock engine implementations where useful
- tests for module availability and feature checks

Sprint 4 must not include:

- Religious Domain features
- live payment provider integration
- live SMS provider integration
- full billing lifecycle
- full workflow builder
- Flutter UI
- advanced provider configuration UI

## Gate 2: Module Registry Boundary Check

Confirm:

- module state is owned by Module Registry
- domain code does not create its own module availability logic
- organization module state is organization-scoped
- important module state changes can be audited

## Gate 3: Subscription Boundary Check

Confirm:

- feature availability is checked through Subscription Engine or related foundation service
- plan features and limits are represented clearly
- domains do not duplicate commercial access logic
- usage limit foundation is prepared without overbuilding advanced billing

## Gate 4: Engine Skeleton Boundary Check

Confirm:

- Payment, SMS, Storage, Notification, Reporting, and Workflow interfaces exist where planned
- no live provider is required for Sprint 4
- provider-specific code is not placed inside domains
- local/mock implementations are clearly marked

## Gate 5: API Contract Check

Confirm Sprint 4 routes align with:

- `docs/12-api/module-registry-api-contract.md`
- `docs/12-api/subscription-feature-api-contract.md`
- `docs/12-api/standard-response-contract.md`

## Gate 6: Feature Availability Flow Check

Confirm implementation follows:

- `docs/08-security/feature-availability-decision-flow.md`
- `docs/08-security/authorization-decision-flow.md`

## Gate 7: Data Model Check

Confirm migrations follow:

- `docs/07-database/initial-migration-order.md`
- `docs/07-database/database-naming-conventions.md`
- `docs/07-database/database-relationship-map.md`

## Gate 8: Test Check

Minimum tests should cover:

- module seed repeatability
- module can be installed for an organization
- module can be activated
- inactive module is unavailable
- feature is available when plan allows it
- feature is unavailable when plan does not allow it
- engine interface can be called through local/mock implementation

## Gate 9: Documentation Check

If implementation differs from documented API, data model, or feature availability flow, update the relevant docs in the same PR.

## Final Rule

Sprint 4 is complete only when modules, plan-based feature checks, and engine boundaries are stable enough for Religious Domain MVP work to build on them.
