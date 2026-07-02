# Sprint 44 User Membership Access Lifecycle Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 44.

Sprint 44 prepares the User Invitation, Membership, Role Assignment, and Access Lifecycle foundation after organization lifecycle/provisioning, identity, permission, audit, support, and governance foundations have introduced controlled user access needs.

## Prompt

```text
You are implementing Sprint 44 for Universal Platform.

Your task is to implement the User Invitation, Membership, Role Assignment, and Access Lifecycle foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/identity-api-contract.md
3. docs/12-api/organization-api-contract.md
4. docs/12-api/permission-api-contract.md
5. docs/08-security/authentication-context-flow.md
6. docs/08-security/authorization-decision-flow.md
7. docs/14-roadmap/sprint-44-user-membership-access-lifecycle-roadmap.md
8. docs/13-module-sdk/user-membership-access-lifecycle-readiness.md
9. docs/12-api/user-membership-access-lifecycle-api-contract.md
10. docs/11-ui-ux/user-membership-access-lifecycle-ui-contract.md
11. docs/05-infrastructure/user-membership-access-lifecycle-guide.md
12. docs/16-quality/sprint-44-user-membership-access-lifecycle-review.md

Implement only:
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
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- hidden platform-operator impersonation
- unsafe cross-organization membership changes
- direct bypass of Permission Engine
- password reset/authentication provider implementation
- full HR employee lifecycle automation
- biometric identity verification
- automatic privilege escalation
- unrelated Commerce/POS expansion

Recommended branch:
feature/user-membership-access-lifecycle-foundation

Recommended PR title:
feat: add user membership access lifecycle foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Identity/Organization/Permission/Audit boundaries followed
- membership and access lifecycle rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused access lifecycle foundation for invitations, memberships, role assignments, permission group placeholders, access history, offboarding placeholders, access review linkage, notifications, and audit-safe access changes.

## Final Rule

User access must be organization-scoped, permission-protected, auditable, and controlled through Identity, Organization, and Permission Engine boundaries. No domain should grant access independently.
