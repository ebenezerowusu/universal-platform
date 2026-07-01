# Sprint 23 Commerce POS Review

## Purpose

This document defines review checks for Sprint 23.

Sprint 23 implements the Commerce and POS foundation.

## Review 1: Scope

Sprint 23 may include:

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
- audit records for important actions
- API and Flutter foundations

Sprint 23 should not include:

- public marketplace storefront
- delivery/rider workflows
- multi-warehouse optimization
- vendor marketplace onboarding
- advanced discounts or promotions
- direct payment provider calls inside Commerce Domain
- live provider credentials in repository
- unrelated finance/accounting expansion

## Review 2: Domain Boundaries

Confirm:

- Commerce Domain owns product, inventory, and sale workflows
- Payment Engine owns online payment behavior where used
- Finance Domain may consume summarized sales records later
- Platform Core does not contain commerce-specific business rules

## Review 3: Inventory Rules

Confirm:

- stock is organization-owned
- inventory movements are recorded
- sale confirmation deducts stock where tracking is enabled
- negative stock behavior is controlled by policy
- manual stock adjustments are auditable

## Review 4: Sale Rules

Confirm:

- sales are organization-owned
- sale items preserve unit price at sale time
- receipts are unique and traceable
- payment status is clear
- sale cancellation where supported is auditable

## Review 5: Reconciliation

Confirm:

- confirmed sale without stock deduction can be reviewed
- paid sale without receipt can be reviewed
- stock movement without sale can be reviewed
- manual review actions are auditable

## Review 6: API Contract

Confirm implementation aligns with:

```text
docs/12-api/commerce-pos-api-contract.md
```

## Review 7: Flutter Contract

Confirm screens align with:

```text
docs/11-ui-ux/commerce-pos-flutter-contract.md
```

## Review 8: Test Coverage

Tests should cover where practical:

- product category create/update
- product create/update
- stock adjustment
- sale creation
- insufficient stock behavior
- inventory deduction after sale
- receipt issuing
- payment initiation boundary where used
- permission denied behavior
- reconciliation item behavior

## Final Rule

Sprint 23 is complete when organizations can manage products, track basic inventory, create POS sales, issue receipts, and reconcile stock movement through auditable workflows.
