# Sprint 25 Order Fulfillment Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 25.

Sprint 25 prepares order fulfillment and pickup workflows after Marketplace and Order foundation.

## Prompt

```text
You are implementing Sprint 25 for Universal Platform.

Your task is to implement the Order Fulfillment and Pickup foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/commerce-domain.md
3. docs/12-api/marketplace-order-api-contract.md
4. docs/05-infrastructure/marketplace-order-reconciliation-playbook.md
5. docs/14-roadmap/sprint-25-order-fulfillment-roadmap.md
6. docs/13-module-sdk/order-fulfillment-readiness.md
7. docs/12-api/order-fulfillment-api-contract.md
8. docs/11-ui-ux/order-fulfillment-ui-contract.md
9. docs/05-infrastructure/order-fulfillment-reconciliation-playbook.md
10. docs/16-quality/sprint-25-order-fulfillment-review.md

Implement only:
- fulfillment module/feature registration foundation
- fulfillment status foundation
- pickup preparation foundation
- order packing/checklist foundation where practical
- customer notification trigger foundation where practical
- fulfillment note/history foundation
- order cancellation reason foundation where practical
- stock release/deduction confirmation direction where practical
- permission and feature checks
- audit records for important fulfillment actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- rider dispatch
- live delivery tracking
- route optimization
- driver payouts
- third-party logistics integrations
- marketplace commission settlement
- advanced returns/refunds
- direct payment provider calls inside Commerce Domain
- unrelated POS expansion

Recommended branch:
feature/order-fulfillment-foundation

Recommended PR title:
feat: add order fulfillment foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- order/fulfillment boundaries followed
- stock and notification rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused order fulfillment foundation for order processing, pickup readiness, status tracking, notes, notifications, and reconciliation.

## Final Rule

Fulfillment must make every order status change explainable, auditable, permission-protected, and connected to marketplace order records.
