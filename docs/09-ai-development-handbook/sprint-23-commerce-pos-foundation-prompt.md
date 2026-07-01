# Sprint 23 Commerce POS Foundation Prompt

## Purpose

This document is a copy-paste-ready prompt for a coding agent to begin Sprint 23.

Sprint 23 prepares the Commerce and POS foundation after finance reporting has established basic visibility over income and expenses.

## Prompt

```text
You are implementing Sprint 23 for Universal Platform.

Your task is to implement the Commerce and POS foundation.

Required reading before coding:
1. CLAUDE.md
2. docs/03-domains/commerce-domain.md
3. docs/03-domains/pos-domain.md
4. docs/04-capability-engines/payment-engine.md
5. docs/04-capability-engines/audit-engine.md
6. docs/14-roadmap/sprint-23-commerce-pos-foundation-roadmap.md
7. docs/13-module-sdk/commerce-pos-foundation-readiness.md
8. docs/12-api/commerce-pos-api-contract.md
9. docs/11-ui-ux/commerce-pos-flutter-contract.md
10. docs/05-infrastructure/commerce-inventory-sales-reconciliation-playbook.md
11. docs/16-quality/sprint-23-commerce-pos-review.md

Implement only:
- Commerce/POS module registration foundation
- product category foundation
- product catalog foundation
- stock item and inventory movement foundation
- POS sale/cart/order foundation
- receipt foundation
- payment status placeholder or Payment Engine link where practical
- inventory deduction after sale confirmation
- sales ledger foundation where practical
- permission and feature checks
- audit records for important commerce actions
- API routes matching the contract
- Flutter placeholders or screens where practical
- tests where practical

Do not implement:
- public marketplace storefront
- delivery/rider workflows
- multi-warehouse optimization
- vendor marketplace onboarding
- advanced discounts or promotions
- direct payment provider calls inside Commerce Domain
- live provider credentials in repository
- unrelated finance/accounting expansion

Recommended branch:
feature/commerce-pos-foundation

Recommended PR title:
feat: add Commerce POS foundation

Before finishing, run the relevant build/test/format checks available in the project.

Final response must include:
- files changed
- migrations added
- API routes added
- Flutter screens or placeholders added
- tests added
- checks run
- Commerce/Payment boundaries followed
- inventory rules followed
- out-of-scope items avoided
- known limitations
- follow-up work
```

## Expected Output

The coding agent should produce a focused Commerce and POS foundation for product catalog, basic inventory, POS sales, receipts, and payment status tracking.

## Final Rule

Commerce/POS must be modular, organization-scoped, inventory-aware, auditable, and provider-agnostic.
