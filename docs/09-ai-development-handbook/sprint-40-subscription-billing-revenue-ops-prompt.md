# Sprint 40 Subscription Billing Revenue Operations Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 40.

Sprint 40 prepares the Subscription Billing and Revenue Operations foundation after module registry, feature entitlement, SMS package payments, finance, support operations, reporting, and platform monitoring have introduced commercial operations needs.

## Prompt

```text
You are implementing Sprint 40 for Universal Platform.

Your task is to implement the Subscription Billing and Revenue Operations foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/12-api/subscription-feature-api-contract.md
3. docs/04-capability-engines/payment-engine.md
4. docs/04-capability-engines/audit-engine.md
5. docs/04-capability-engines/permission-engine.md
6. docs/14-roadmap/sprint-40-subscription-billing-revenue-ops-roadmap.md
7. docs/13-module-sdk/subscription-billing-revenue-ops-readiness.md
8. docs/12-api/subscription-billing-revenue-ops-api-contract.md
9. docs/11-ui-ux/subscription-billing-revenue-ops-ui-contract.md
10. docs/05-infrastructure/subscription-billing-revenue-ops-guide.md
11. docs/16-quality/sprint-40-subscription-billing-revenue-ops-review.md

Implement only:
- subscription billing feature registration foundation
- plan catalog metadata foundation
- subscription account foundation
- subscription status foundation
- trial period placeholder where practical
- invoice metadata foundation
- billing cycle metadata foundation
- payment intent/reference placeholder through Payment Engine
- entitlement change history foundation
- renewal and cancellation placeholder where practical
- revenue operation notes where practical
- permission and feature checks
- audit records for billing and entitlement actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- direct payment provider calls inside billing domain
- tax calculation engine
- full accounting ledger
- automatic card charging
- revenue recognition engine
- external billing provider integration
- cross-organization billing visibility
- unrelated Commerce/POS expansion

Recommended branch:
feature/subscription-billing-revenue-ops-foundation

Recommended PR title:
feat: add subscription billing revenue operations foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Billing/Payment/Permission boundaries followed
- entitlement and audit rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused billing foundation for plan catalog metadata, subscriptions, invoices, payment references, billing cycles, entitlement changes, trials, renewals, cancellations, and revenue operation notes.

## Final Rule

Billing must be organization-scoped, permission-protected, auditable, provider-agnostic, and entitlement-aware. Billing should request payments through Payment Engine and should never call payment providers directly.
