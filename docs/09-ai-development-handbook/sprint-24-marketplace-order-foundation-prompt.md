# Sprint 24 Marketplace Order Foundation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 24.

Sprint 24 prepares the customer-facing Marketplace and Order foundation after the internal Commerce/POS foundation.

## Prompt

```text
You are implementing Sprint 24 for Universal Platform.

Your task is to implement the Marketplace and Order foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/commerce-domain.md
3. docs/03-domains/pos-domain.md
4. docs/12-api/commerce-pos-api-contract.md
5. docs/14-roadmap/sprint-24-marketplace-order-foundation-roadmap.md
6. docs/13-module-sdk/marketplace-order-foundation-readiness.md
7. docs/12-api/marketplace-order-api-contract.md
8. docs/11-ui-ux/marketplace-order-flutter-contract.md
9. docs/05-infrastructure/marketplace-order-reconciliation-playbook.md
10. docs/16-quality/sprint-24-marketplace-order-review.md

Implement only:
- Marketplace module registration foundation
- public catalog visibility foundation
- storefront product listing foundation
- customer cart foundation
- order creation foundation
- order item foundation
- checkout intent foundation
- payment status placeholder or Payment Engine link where practical
- inventory reservation/deduction direction where practical
- order receipt/confirmation foundation
- permission and feature checks
- audit records for important order actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- delivery/rider workflows
- multi-vendor onboarding
- marketplace commission settlement
- advanced discounts or promotions
- returns/refunds beyond placeholders
- direct payment provider calls inside Commerce Domain
- live provider credentials in repository
- unrelated POS expansion

Recommended branch:
feature/marketplace-order-foundation

Recommended PR title:
feat: add marketplace order foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- marketplace/order boundaries followed
- inventory/order rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused Marketplace and Order foundation for customer-facing catalog, cart, checkout intent, order records, and basic order status.

## Final Rule

Marketplace must extend Commerce/POS foundations without duplicating product, inventory, or payment logic.
