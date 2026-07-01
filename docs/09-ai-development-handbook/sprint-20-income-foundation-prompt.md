# Sprint 20 Income Foundation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 20.

Sprint 20 prepares the first basic income-tracking foundation for organizations.

## Prompt

```text
You are implementing Sprint 20 for Universal Platform.

Your task is to implement the organization income foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/finance-domain.md
3. docs/04-capability-engines/payment-engine.md
4. docs/14-roadmap/sprint-20-income-foundation-roadmap.md
5. docs/13-module-sdk/income-foundation-readiness.md
6. docs/12-api/income-foundation-api-contract.md
7. docs/11-ui-ux/income-foundation-flutter-contract.md
8. docs/05-infrastructure/income-record-reconciliation-playbook.md
9. docs/16-quality/sprint-20-income-foundation-review.md

Implement only:
- income module registration foundation
- income category foundation
- pledge or commitment foundation where practical
- income record foundation
- Payment Engine initiation foundation where payment is used
- status tracking foundation
- receipt record foundation
- income ledger foundation
- reconciliation between payment records and income records
- permission and feature checks
- audit records for important finance actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- direct provider calls inside domain logic
- provider-specific behavior in screens
- live provider credentials in the repository
- advanced accounting
- payroll
- full expense management
- tax filing
- full invoicing
- unrelated Commerce/POS features

Recommended branch:
feature/income-foundation

Recommended PR title:
feat: add organization income foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Payment Engine boundaries followed
- audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused foundation for organization income tracking while keeping external payment behavior behind Payment Engine and finance records auditable.

## Final Rule

Income records must be accurate, auditable, organization-scoped, and provider-agnostic.
