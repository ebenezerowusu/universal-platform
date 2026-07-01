# Sprint 21 Expense Foundation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 21.

Sprint 21 prepares basic expense tracking and budget control after the income foundation.

## Prompt

```text
You are implementing Sprint 21 for Universal Platform.

Your task is to implement the organization expense foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/finance-domain.md
3. docs/04-capability-engines/workflow-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/14-roadmap/sprint-21-expense-foundation-roadmap.md
6. docs/13-module-sdk/expense-foundation-readiness.md
7. docs/12-api/expense-foundation-api-contract.md
8. docs/11-ui-ux/expense-foundation-flutter-contract.md
9. docs/05-infrastructure/expense-record-reconciliation-playbook.md
10. docs/16-quality/sprint-21-expense-foundation-review.md

Implement only:
- expense category foundation
- basic budget line foundation
- expense request foundation
- approval workflow foundation where practical
- expense record foundation
- attachment/reference foundation where practical
- payment status placeholder where applicable
- expense ledger foundation
- reconciliation between request, approval, payment, and ledger records
- permission and feature checks
- audit records for important expense actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full accounting
- payroll
- tax filing
- full procurement
- vendor management beyond simple payee notes
- direct payment provider calls inside Finance Domain
- live provider credentials in repository
- unrelated Commerce/POS features

Recommended branch:
feature/expense-foundation

Recommended PR title:
feat: add organization expense foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Workflow/Audit boundaries followed
- finance control rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused foundation for organization expense tracking while keeping approvals, audit, and organization boundaries clear.

## Final Rule

Expense records must be auditable, organization-scoped, permission-protected, and controlled by workflow where approval is required.
