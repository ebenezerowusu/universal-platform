# Sprint 36 Audit Compliance Retention Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 36.

Sprint 36 prepares the Audit, Compliance, and Data Retention foundation after import/export, workflows, reporting, documents, notifications, HR, finance, commerce, and religious modules have introduced sensitive operational actions.

## Prompt

```text
You are implementing Sprint 36 for Universal Platform.

Your task is to implement the Audit, Compliance, and Data Retention foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/audit-engine.md
3. docs/04-capability-engines/permission-engine.md
4. docs/08-security/08-security-standards.md
5. docs/14-roadmap/sprint-36-audit-compliance-retention-roadmap.md
6. docs/13-module-sdk/audit-compliance-retention-readiness.md
7. docs/12-api/audit-compliance-retention-api-contract.md
8. docs/11-ui-ux/audit-compliance-retention-ui-contract.md
9. docs/05-infrastructure/audit-compliance-retention-operations-guide.md
10. docs/16-quality/sprint-36-audit-compliance-retention-review.md

Implement only:
- audit/compliance feature registration foundation
- audit event metadata foundation
- audit event query foundation
- sensitive action category foundation
- access review placeholder where practical
- retention policy metadata foundation
- retention action placeholder where practical
- safe deletion/archive request placeholder where practical
- compliance flag/checklist placeholder where practical
- permission and feature checks
- audit records for compliance/retention actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- legal advice engine
- automatic destructive deletion
- full GRC/compliance suite
- government filing workflows
- automated regulatory classification
- cross-organization audit visibility
- provider-specific archive storage calls inside domain modules
- unrelated Commerce/POS expansion

Recommended branch:
feature/audit-compliance-retention-foundation

Recommended PR title:
feat: add audit compliance retention foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Audit/Permission/Storage boundaries followed
- retention and safe deletion rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused audit/compliance foundation for audit search, sensitive-action categories, access reviews, retention policies, archive/delete placeholders, compliance flags, and auditable retention actions.

## Final Rule

Audit and retention must be organization-scoped, permission-aware, non-destructive by default, and explainable. Domain modules should emit audit events and retention candidates instead of deleting or hiding records silently.
