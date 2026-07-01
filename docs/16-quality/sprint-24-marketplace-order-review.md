# Sprint 24 Marketplace Order Review

## Purpose

This document defines review checks for Sprint 24.

Sprint 24 implements Marketplace and Order foundation.

## Review 1: Scope

Sprint 24 may include:

- Marketplace module registration foundation
- public catalog visibility foundation
- storefront product listing foundation
- customer cart foundation
- order creation foundation
- order item foundation
- checkout intent foundation
- payment status placeholder or Payment Engine link where practical
- stock reservation or deduction direction where practical
- order receipt or confirmation foundation
- permission and feature checks
- audit records for important order actions
- API and UI foundations

Sprint 24 should not include:

- delivery/rider workflows
- multi-vendor onboarding
- marketplace commission settlement
- advanced discounts or promotions
- returns/refunds beyond placeholders
- direct payment provider calls inside Commerce Domain
- live provider credentials in repository
- unrelated POS expansion

## Review 2: Domain Boundaries

Confirm:

- Commerce Domain owns storefront and order workflows
- Payment Engine owns online payment behavior where used
- inventory behavior reuses Commerce/POS foundations
- Platform Core does not contain marketplace-specific business rules

## Review 3: Storefront Visibility

Confirm:

- only active products can be public
- internal-only products are not exposed
- hidden products are not exposed
- customer-facing responses expose safe fields only

## Review 4: Cart and Order Rules

Confirm:

- carts are scoped to organization/storefront
- order items preserve unit price at order time
- order totals are calculated safely
- payment status is clear
- order confirmation is unique and traceable

## Review 5: Stock Rules

Confirm:

- stock is checked where tracking is enabled
- reservation or deduction behavior is clear
- reservations can be released where supported
- overselling behavior is controlled by policy

## Review 6: Reconciliation

Confirm:

- checkout intent without order can be reviewed
- paid order not confirmed can be reviewed
- confirmed order without stock action can be reviewed where required
- missing confirmation can be reviewed
- manual review actions are auditable

## Review 7: API Contract

Confirm implementation aligns with:

```text
docs/12-api/marketplace-order-api-contract.md
```

## Review 8: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/storefront-order-ui-contract.md
```

## Review 9: Test Coverage

Tests should cover where practical:

- public catalog visibility
- hidden/internal product exclusion
- cart creation
- order creation
- unit price snapshot
- insufficient stock behavior
- payment initiation boundary where used
- order status update permission
- reconciliation item behavior

## Final Rule

Sprint 24 is complete when storefront catalog, cart, checkout intent, and order foundations reuse Commerce/POS product, stock, and payment foundations safely.
