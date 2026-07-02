# Sprint 54 Organization Network Data Sharing Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 54.

Sprint 54 prepares the Organization Network, Branch Relationships, and Data Sharing Governance foundation after organization lifecycle, tenant exit/handover, privacy governance, data portability, audit, role/permission, and reporting foundations have introduced inter-organization relationship and data visibility needs.

## Prompt

```text
You are implementing Sprint 54 for Universal Platform.

Your task is to implement the Organization Network, Branch Relationships, and Data Sharing Governance foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/organization-api-contract.md
3. docs/12-api/organization-lifecycle-provisioning-api-contract.md
4. docs/12-api/user-membership-access-lifecycle-api-contract.md
5. docs/12-api/data-portability-tenant-exit-handover-api-contract.md
6. docs/14-roadmap/sprint-54-organization-network-data-sharing-roadmap.md
7. docs/13-module-sdk/organization-network-data-sharing-readiness.md
8. docs/12-api/organization-network-data-sharing-api-contract.md
9. docs/11-ui-ux/organization-network-data-sharing-ui-contract.md
10. docs/05-infrastructure/organization-network-data-sharing-guide.md
11. docs/16-quality/sprint-54-organization-network-data-sharing-review.md

Implement only:
- organization network feature registration foundation
- organization relationship metadata foundation
- parent/child branch connection request foundation
- branch approval/rejection placeholder foundation
- branch disconnect/remove request placeholder foundation
- relationship status history foundation
- hierarchy path/depth metadata placeholder where practical
- data sharing policy placeholder foundation
- historical access policy placeholder where practical
- cross-organization reporting visibility placeholder where practical
- permission and feature checks
- audit records for organization relationship and data sharing actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- unrestricted cross-tenant data browsing
- automatic branch takeover
- destructive branch disconnection
- hidden parent access to child data
- bypass of child organization ownership
- forced relationship changes without audit
- full legal governance engine
- unrelated Commerce/POS expansion

Recommended branch:
feature/organization-network-data-sharing-foundation

Recommended PR title:
feat: add organization network data sharing foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Organization/Permission/Audit/Privacy boundaries followed
- branch relationship and data sharing rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused organization network foundation for parent/child relationships, connection requests, approvals, disconnect requests, relationship status history, hierarchy metadata, data sharing policies, historical access policies, cross-organization reporting placeholders, and audit-safe branch governance actions.

## Final Rule

Organization networks must preserve tenant ownership. Parent-child relationships may grant scoped visibility only through explicit policy, permission checks, and audit history. No parent organization should receive hidden or unlimited access to child data.
