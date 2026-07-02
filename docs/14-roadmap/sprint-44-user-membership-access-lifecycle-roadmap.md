# Sprint 44 User Membership Access Lifecycle Roadmap

## Purpose

This document defines Sprint 44 for User Invitation, Membership, Role Assignment, and Access Lifecycle foundation.

Sprint 44 should create a reusable access lifecycle layer for invitations, membership statuses, role assignments, permission group placeholders, access change history, offboarding placeholders, access review linkage, and audit-safe user access changes.

## Sprint Goal

Enable organizations to invite users, manage membership status, assign roles, track access changes, deactivate/offboard users, and link privileged users to access reviews through permission-protected and auditable workflows.

## Why This Sprint

Organization provisioning creates tenants and bootstraps first admins, but ongoing operations need controlled user access:

- inviting organization admins and staff
- adding finance, HR, commerce, religious, and support users
- changing roles safely
- removing access when users leave
- reviewing privileged permissions
- preventing cross-tenant membership mistakes
- auditing sensitive access changes

Without a shared access lifecycle foundation, each module may start creating its own user-access rules.

## Work Package 1: Invitation Requests

Prepare invitation records with:

- organization
- invited email or phone placeholder
- invited role placeholder
- invited by
- status
- expiry placeholder
- accepted by optional

## Work Package 2: Membership Status

Prepare membership statuses such as:

```text
invited
active
inactive
suspended_placeholder
removed
offboarding_placeholder
manual_review
```

## Work Package 3: Role Assignment Foundation

Prepare role assignment records with:

- user
- organization
- role
- assigned by
- assigned at
- status
- reason optional

## Work Package 4: Permission Group Placeholder

Prepare permission group assignment placeholder for future bundles.

Permission evaluation still belongs to Permission Engine.

## Work Package 5: Access Change History

Prepare access history for:

- invitation created
- invitation accepted
- membership activated
- role assigned
- role removed
- membership suspended
- membership removed
- access review linked

## Work Package 6: Offboarding Placeholder

Prepare offboarding placeholder records for:

- access removal
- pending handover placeholder
- replacement owner placeholder
- open task warning placeholder
- audit note

## Work Package 7: Access Review Linkage

Prepare linkage to governance access review placeholders for:

- admin users
- finance users
- export users
- HR/workforce users
- support/internal users

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- user access dashboard
- invitation list
- invitation detail
- organization member list
- member detail
- role assignment panel
- access history
- offboarding placeholder
- privileged access review summary

## Out of Scope

Do not implement:

- hidden platform-operator impersonation
- unsafe cross-organization membership changes
- direct bypass of Permission Engine
- password reset/authentication provider implementation
- full HR employee lifecycle automation
- biometric identity verification
- automatic privilege escalation

## Completion Standard

Sprint 44 is complete when invitations, membership statuses, role assignments, access history, offboarding placeholders, and access review linkage are available through organization-scoped, permission-protected, and auditable foundations.

## Final Rule

User access must be controlled centrally through Identity, Organization, and Permission boundaries, not by individual domains.
