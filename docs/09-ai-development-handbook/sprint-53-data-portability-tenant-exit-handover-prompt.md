# Sprint 53 Data Portability Tenant Exit Handover Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 53.

Sprint 53 prepares the Data Portability, Tenant Exit, and Organization Data Handover foundation after privacy governance, import/export, lineage, backup/restore, retention, organization lifecycle, billing, support operations, and data quality foundations have introduced safe tenant exit and data handover needs.

## Prompt

```text
You are implementing Sprint 53 for Universal Platform.

Your task is to implement the Data Portability, Tenant Exit, and Organization Data Handover foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/import-export-data-migration-api-contract.md
3. docs/12-api/data-privacy-consent-governance-api-contract.md
4. docs/12-api/data-lineage-provenance-record-history-api-contract.md
5. docs/12-api/audit-compliance-retention-api-contract.md
6. docs/14-roadmap/sprint-53-data-portability-tenant-exit-handover-roadmap.md
7. docs/13-module-sdk/data-portability-tenant-exit-handover-readiness.md
8. docs/12-api/data-portability-tenant-exit-handover-api-contract.md
9. docs/11-ui-ux/data-portability-tenant-exit-handover-ui-contract.md
10. docs/05-infrastructure/data-portability-tenant-exit-handover-guide.md
11. docs/16-quality/sprint-53-data-portability-tenant-exit-handover-review.md

Implement only:
- data portability feature registration foundation
- tenant export package request foundation
- export scope manifest placeholder foundation
- organization handover checklist foundation
- exit readiness review placeholder foundation
- data ownership confirmation placeholder where practical
- privacy/export review linkage where practical
- lineage/package manifest linkage where practical
- archive/retention linkage where practical
- billing/support/offboarding linkage placeholders where practical
- permission and feature checks
- audit records for tenant export, handover, and exit actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- destructive tenant deletion automation
- forced account closure workflow
- legal advice engine
- unrestricted full-database dump download
- cross-organization export package visibility
- bypass of privacy/export review rules
- provider-specific storage export logic inside domains
- unrelated Commerce/POS expansion

Recommended branch:
feature/data-portability-tenant-exit-handover-foundation

Recommended PR title:
feat: add data portability tenant exit handover foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Portability/Privacy/Lineage/Retention boundaries followed
- export package and handover rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused tenant portability foundation for export package requests, export scope manifests, handover checklists, exit readiness reviews, ownership confirmations, privacy/export review linkage, lineage package manifests, archive/retention linkage, billing/support/offboarding placeholders, and audit-safe tenant exit operations.

## Final Rule

Tenant exit must be controlled, auditable, privacy-aware, non-destructive by default, and respectful of organization ownership. Data handover should be package-based, reviewed, and traceable, not a raw database dump.
