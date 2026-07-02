# Sprint 43 Organization Lifecycle Provisioning Review

## Purpose

This document defines review checks for Sprint 43.

Sprint 43 implements Organization Lifecycle, Provisioning, and Tenant Operations foundation.

## Review 1: Scope

Sprint 43 may include:

- organization lifecycle feature registration foundation
- organization provisioning request foundation
- organization lifecycle status foundation
- activation/suspension/reactivation placeholder foundation
- organization owner/admin bootstrap foundation
- default regional settings bootstrap foundation
- module bootstrap placeholder where practical
- billing/subscription linkage placeholder where practical
- onboarding checklist linkage placeholder where practical
- tenant isolation validation placeholder where practical
- provisioning event/history foundation
- permission and feature checks
- audit records for lifecycle/provisioning actions
- API and UI foundations

Sprint 43 should not include:

- unsafe cross-tenant data access
- destructive organization deletion
- automatic production data migration
- direct payment provider calls during provisioning
- country-specific provisioning logic inside Platform Core
- provider-specific infrastructure provisioning scripts inside domain modules
- hidden platform-operator access to tenant data
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Organization Lifecycle Foundation owns lifecycle statuses, provisioning requests, lifecycle history, and tenant-readiness checks
- Identity Foundation owns users and organization membership
- Module Registry owns module installation/availability
- Billing Foundation owns subscription status and entitlement history
- Regional Operations owns country, currency, timezone, language, and formatting defaults
- Support Operations owns onboarding checklist and support follow-up
- Platform Core does not contain country-specific or domain-specific provisioning rules

## Review 3: Tenant Boundaries

Confirm:

- lifecycle records are organization-scoped
- admin bootstrap records are organization-scoped
- regional bootstrap records are organization-scoped
- module bootstrap records are organization-scoped
- billing linkage cannot reference another organization
- support/onboarding linkage cannot cross organizations

## Review 4: Lifecycle Rules

Confirm:

- activation requires required readiness checks where practical
- suspension/reactivation actions preserve reason and history
- destructive organization deletion is not available in MVP
- lifecycle status changes are audited
- invalid status transitions are rejected where practical

## Review 5: Bootstrap Rules

Confirm warnings exist where practical for:

- missing owner/admin
- missing regional settings
- missing module bootstrap
- module access without entitlement
- billing linkage mismatch
- active organization with blocked onboarding
- failed tenant validation

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/organization-lifecycle-provisioning-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/organization-lifecycle-provisioning-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- provisioning request creation
- provisioning approval/rejection placeholder
- lifecycle activation placeholder
- suspension/reactivation placeholder
- admin bootstrap validation
- regional bootstrap validation
- module entitlement warning
- billing linkage organization boundary
- tenant validation run placeholder
- permission denied behavior
- audit record creation

## Final Rule

Sprint 43 is complete when organization provisioning requests, lifecycle statuses, activation/suspension placeholders, admin bootstrap, regional bootstrap, module/billing/onboarding linkage, tenant validation, and lifecycle audit history are available through tenant-safe, permission-protected foundations.
