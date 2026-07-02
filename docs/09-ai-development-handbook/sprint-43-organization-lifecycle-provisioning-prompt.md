# Sprint 43 Organization Lifecycle Provisioning Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 43.

Sprint 43 prepares the Organization Lifecycle, Provisioning, and Tenant Operations foundation after country/regional settings, billing, payment operations, support, monitoring, module registry, and onboarding foundations have introduced safe tenant activation needs.

## Prompt

```text
You are implementing Sprint 43 for Universal Platform.

Your task is to implement the Organization Lifecycle, Provisioning, and Tenant Operations foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/organization-api-contract.md
3. docs/12-api/module-registry-api-contract.md
4. docs/12-api/subscription-feature-api-contract.md
5. docs/12-api/country-localization-regional-ops-api-contract.md
6. docs/14-roadmap/sprint-43-organization-lifecycle-provisioning-roadmap.md
7. docs/13-module-sdk/organization-lifecycle-provisioning-readiness.md
8. docs/12-api/organization-lifecycle-provisioning-api-contract.md
9. docs/11-ui-ux/organization-lifecycle-provisioning-ui-contract.md
10. docs/05-infrastructure/organization-lifecycle-provisioning-guide.md
11. docs/16-quality/sprint-43-organization-lifecycle-provisioning-review.md

Implement only:
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
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- unsafe cross-tenant data access
- destructive organization deletion
- automatic production data migration
- direct payment provider calls during provisioning
- country-specific provisioning logic inside Platform Core
- provider-specific infrastructure provisioning scripts inside domain modules
- hidden platform-operator access to tenant data
- unrelated Commerce/POS expansion

Recommended branch:
feature/organization-lifecycle-provisioning-foundation

Recommended PR title:
feat: add organization lifecycle provisioning foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Organization/Module/Billing/Regional boundaries followed
- tenant isolation and lifecycle rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused tenant lifecycle foundation for provisioning requests, organization statuses, activation/suspension/reactivation placeholders, admin bootstrap, regional defaults, module bootstrap, subscription linkage, onboarding linkage, tenant isolation checks, and lifecycle audit history.

## Final Rule

Organization provisioning must be tenant-safe, auditable, configuration-driven, and integrated with regional settings, module registry, billing, support, and permission boundaries without exposing cross-tenant data.
