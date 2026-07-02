# Sprint 44 User Membership Access Lifecycle Review

## Purpose

This document defines review checks for Sprint 44.

Sprint 44 implements User Invitation, Membership, Role Assignment, and Access Lifecycle foundation.

## Review 1: Scope

Sprint 44 may include:

- user access lifecycle feature registration foundation
- invitation request foundation
- invitation acceptance/expiry placeholder foundation
- organization membership status foundation
- role assignment foundation
- permission group assignment placeholder where practical
- access change history foundation
- offboarding/deactivation placeholder foundation
- access review linkage placeholder where practical
- notification trigger placeholder where practical
- permission and feature checks
- audit records for invitations, membership, role, and access changes
- API and UI foundations

Sprint 44 should not include:

- hidden platform-operator impersonation
- unsafe cross-organization membership changes
- direct bypass of Permission Engine
- password reset/authentication provider implementation
- full HR employee lifecycle automation
- biometric identity verification
- automatic privilege escalation
- unrelated Commerce/POS expansion

## Review 2: Engine Boundaries

Confirm:

- Identity Foundation owns user identities and authentication linkage
- Organization Foundation owns organization membership
- Permission Engine owns role and permission evaluation
- Access Lifecycle Foundation owns invitations, membership state changes, role assignment records, offboarding placeholders, and access history
- Audit Engine records sensitive access changes
- Governance/Access Review owns review workflows
- domain modules do not grant access independently

## Review 3: Organization Boundaries

Confirm:

- invitations are organization-scoped
- memberships are organization-scoped
- role assignments are organization-scoped
- access history is organization-scoped
- offboarding placeholders are organization-scoped
- cross-organization membership changes are rejected

## Review 4: Access Rules

Confirm:

- actor cannot grant roles beyond allowed scope
- privileged role assignment requires stronger permission and audit
- inactive/removed members cannot retain active roles
- invalid role assignment is rejected
- Permission Engine is the source of permission evaluation

## Review 5: Lifecycle Rules

Confirm warnings exist where practical for:

- expired pending invitation
- user already invited
- user already member
- inactive user with active role
- privileged user without recent access review
- offboarding user with active permissions

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/user-membership-access-lifecycle-api-contract.md
```

## Review 7: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/user-membership-access-lifecycle-ui-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- invitation creation
- duplicate invitation prevention
- invitation cancel/accept placeholder
- membership status update
- role assignment
- privileged role permission denied behavior
- role removal
- access history creation
- offboarding placeholder creation
- organization boundary enforcement
- audit record creation

## Final Rule

Sprint 44 is complete when invitations, memberships, role assignments, access history, offboarding placeholders, and privileged access review summaries are available through organization-scoped, permission-protected, and auditable foundations.
