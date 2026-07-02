# Sprint 27 HR Workforce Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 27.

Sprint 27 prepares HR and Workforce foundation for staff, volunteers, departments, roles, status history, and basic HR visibility.

## Prompt

```text
You are implementing Sprint 27 for Universal Platform.

Your task is to implement the HR and Workforce foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/hr-domain.md
3. docs/04-capability-engines/permission-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/04-capability-engines/workflow-engine.md
6. docs/14-roadmap/sprint-27-hr-workforce-roadmap.md
7. docs/13-module-sdk/hr-workforce-readiness.md
8. docs/12-api/hr-workforce-api-contract.md
9. docs/11-ui-ux/hr-workforce-ui-contract.md
10. docs/05-infrastructure/hr-workforce-data-governance-playbook.md
11. docs/16-quality/sprint-27-hr-workforce-review.md

Implement only:
- HR module registration foundation
- workforce person/profile foundation
- department/team foundation
- position/role assignment foundation
- worker status history foundation
- staff/volunteer type foundation
- HR document/reference placeholder where practical
- attendance integration hook direction where practical
- leave/request placeholder where practical
- permission and feature checks
- audit records for important HR actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- payroll
- tax filing
- benefits administration
- complex performance management
- biometric attendance
- direct payment provider calls inside HR Domain
- sensitive document processing beyond placeholders
- unrelated Commerce/POS expansion

Recommended branch:
feature/hr-workforce-foundation

Recommended PR title:
feat: add HR workforce foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- HR boundaries followed
- privacy and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused HR foundation for organization workforce records, departments, assignments, statuses, and basic HR history.

## Final Rule

HR records must be organization-scoped, privacy-aware, permission-protected, and auditable.
