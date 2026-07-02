# Sprint 41 Payment Operations Reconciliation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 41.

Sprint 41 prepares the Payment Provider Operations and Reconciliation foundation after subscription billing, SMS package payments, finance, commerce, marketplace orders, giving/income, and reporting have introduced payment-status integrity needs.

## Prompt

```text
You are implementing Sprint 41 for Universal Platform.

Your task is to implement the Payment Provider Operations and Reconciliation foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/04-capability-engines/payment-engine.md
3. docs/04-capability-engines/audit-engine.md
4. docs/04-capability-engines/permission-engine.md
5. docs/12-api/subscription-billing-revenue-ops-api-contract.md
6. docs/14-roadmap/sprint-41-payment-operations-reconciliation-roadmap.md
7. docs/13-module-sdk/payment-operations-reconciliation-readiness.md
8. docs/12-api/payment-operations-reconciliation-api-contract.md
9. docs/11-ui-ux/payment-operations-reconciliation-ui-contract.md
10. docs/05-infrastructure/payment-operations-reconciliation-guide.md
11. docs/16-quality/sprint-41-payment-operations-reconciliation-review.md

Implement only:
- payment operations feature registration foundation
- payment provider metadata foundation
- payment attempt foundation
- payment callback/webhook event metadata foundation
- payment status history foundation
- payment reconciliation job foundation
- reconciliation mismatch foundation
- settlement batch placeholder where practical
- refund/dispute placeholder where practical
- payment reference linking to billing, commerce, SMS, finance, and income domains
- permission and feature checks
- audit records for sensitive payment operations
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- direct payment provider SDK calls inside domain modules
- real provider credential management UI
- automatic refunds
- chargeback automation
- full accounting settlement ledger
- cross-organization payment visibility
- tax calculation engine
- unrelated Commerce/POS expansion

Recommended branch:
feature/payment-operations-reconciliation-foundation

Recommended PR title:
feat: add payment operations reconciliation foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Payment/Permission/Audit boundaries followed
- reconciliation and provider abstraction rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused payment operations foundation for provider metadata, attempts, callback events, status history, reconciliation jobs, mismatches, settlement placeholders, and refund/dispute placeholders.

## Final Rule

Payment operations must be organization-scoped, provider-agnostic, auditable, reconciliation-first, and isolated behind Payment Engine. Domain modules should reference payment records instead of calling providers directly.
