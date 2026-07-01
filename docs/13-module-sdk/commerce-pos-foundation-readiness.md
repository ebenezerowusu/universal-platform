# Commerce POS Foundation Readiness

## Purpose

This document prepares the Commerce and POS foundation for implementation.

It ensures product catalog, inventory, POS sales, receipts, and payment status are implemented as reusable platform capabilities.

## Wave Summary

Build foundations for product categories, products, stock records, inventory movement, POS sales, receipts, and reconciliation.

## Problem Being Solved

Organizations need a simple way to sell items, track stock, and record sale receipts.

This also prepares the platform for future marketplace, delivery, and commerce expansion.

## Evidence

This wave is supported by:

- Commerce Domain direction
- POS Domain direction
- revenue roadmap
- finance reporting direction
- future marketplace needs
- existing Payment Engine abstraction

## Primary Users

- organization owner/admin
- POS cashier
- inventory manager
- finance/admin user
- platform support/admin user

## MVP Scope

Included:

- product category foundation
- product catalog foundation
- stock item foundation
- inventory movement foundation
- POS sale foundation
- receipt foundation
- payment status placeholder or Payment Engine link where practical
- sales reconciliation foundation

Excluded:

- public marketplace storefront
- delivery/rider workflows
- multi-vendor marketplace
- advanced promotions
- multi-warehouse optimization
- full accounting expansion

## Domain Ownership

Commerce Domain owns product, inventory, and sale workflows.

Payment Engine owns payment initiation and normalized payment status where payment is used.

Finance Domain may consume summarized sales records later.

Platform Core must not own commerce-specific business rules.

## Required Engines

- Permission Engine
- Audit Engine
- Payment Engine where online payment is used
- Storage Engine where product images are used later
- Reporting Engine later
- Feature Flag or License Engine

## Suggested Permissions

```text
commerce.products.view
commerce.products.manage
commerce.inventory.view
commerce.inventory.adjust
commerce.pos.view
commerce.pos.sell
commerce.sales.view
commerce.receipts.view
commerce.reconciliation.review
```

## Data Ownership

Organization-owned records should include:

- product category
- product
- stock item
- inventory movement
- POS sale
- POS sale item
- payment reference link
- receipt record
- sales reconciliation item

## API Direction

Planned API groups:

- product categories
- products
- stock items
- inventory movements
- POS sale/cart
- receipts
- sales history
- reconciliation review

## Flutter Direction

Planned screens:

- POS dashboard
- product category list
- product catalog
- inventory movements
- create sale/cart
- receipt details
- sales history

## Audit Direction

Audit important actions:

- product created or updated
- product price changed
- stock adjusted
- sale created
- sale cancelled where supported
- receipt issued
- reconciliation resolved

## Revenue Direction

This wave supports future Commerce/POS module subscription, transaction fees, marketplace commission, and premium inventory/reporting features.

## Risks

- inventory deducted before sale confirmation
- sale payment status unclear
- product price changes without audit trail
- stock adjustments without reason
- overbuilding marketplace too early

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Commerce/POS foundation should be simple, reusable, auditable, and ready for future marketplace expansion.
