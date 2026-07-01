# Sprint 22 Finance Reporting Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 22.

Sprint 22 prepares finance reporting and cash-position visibility after the income and expense foundations.

## Prompt

```text
You are implementing Sprint 22 for Universal Platform.

Your task is to implement finance reporting and cash-position foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/finance-domain.md
3. docs/14-roadmap/sprint-22-finance-reporting-roadmap.md
4. docs/13-module-sdk/finance-reporting-readiness.md
5. docs/12-api/finance-reporting-api-contract.md
6. docs/11-ui-ux/finance-reporting-flutter-contract.md
7. docs/05-infrastructure/finance-reporting-data-quality-playbook.md
8. docs/16-quality/sprint-22-finance-reporting-review.md
9. docs/12-api/income-foundation-api-contract.md
10. docs/12-api/expense-foundation-api-contract.md

Implement only:
- finance dashboard summary foundation
- period filter foundation
- income summary foundation
- expense summary foundation
- net position summary foundation
- category breakdown foundation
- budget comparison foundation where practical
- export request foundation where practical
- data-quality warnings for incomplete records
- permission and feature checks
- audit records for report exports where practical
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- full accounting statements
- tax reporting
- payroll reports
- external accounting integrations
- complex forecasting
- unrestricted exports without permission checks
- reports that bypass organization boundaries
- unrelated Commerce/POS analytics

Recommended branch:
feature/finance-reporting-foundation

Recommended PR title:
feat: add finance reporting foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added if any
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- reporting boundaries followed
- organization scoping confirmed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused foundation for finance summaries and cash-position visibility using existing income and expense records.

## Final Rule

Finance reports must be organization-scoped, permission-protected, and based on traceable income and expense records.
