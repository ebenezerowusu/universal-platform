# Marketplace Order Foundation Readiness

## Purpose

This document prepares the Marketplace and Order foundation for implementation.

It ensures the customer-facing marketplace reuses Commerce/POS product, inventory, and payment foundations instead of duplicating them.

## Wave Summary

Build foundations for public catalog visibility, storefront product listing, customer cart, order creation, checkout intent, order confirmation, and order reconciliation.

## Problem Being Solved

Organizations need a way to expose selected products to customers and receive orders beyond internal POS sales.

This prepares the platform for future marketplace, delivery, logistics, and commission workflows.

## Evidence

This wave is supported by:

- Commerce/POS foundation direction
- future marketplace roadmap
- product catalog and inventory foundations
- Payment Engine abstraction
- future delivery/logistics opportunity

## Primary Users

- customer or guest buyer
- organization owner/admin
- commerce manager
- order manager
- platform support/admin user

## MVP Scope

Included:

- product storefront visibility foundation
- public catalog list/detail foundation
- cart foundation
- order foundation
- checkout intent foundation
- order confirmation foundation
- order status foundation
- inventory reservation/deduction direction
- payment status placeholder or Payment Engine link where practical

Excluded:

- delivery/rider workflows
- multi-vendor marketplace onboarding
- commission settlement
- advanced promotions
- returns/refunds beyond placeholders
- loyalty programs

## Domain Ownership

Commerce Domain owns storefront and order workflows.

Payment Engine owns payment initiation and normalized payment status where payment is used.

Inventory behavior should reuse Commerce/POS inventory foundations.

Platform Core must not own marketplace-specific business rules.

## Required Engines

- Permission Engine for admin/order management
- Audit Engine
- Payment Engine where online payment is used
- Notification Engine later for order updates
- Feature Flag or License Engine
- Reporting Engine later

## Suggested Permissions

```text
commerce.storefront.manage
commerce.orders.view
commerce.orders.manage
commerce.orders.fulfill
commerce.orders.cancel
commerce.orders.reconciliation.review
```

Public customer catalog routes may not require organization staff permissions but must still enforce safe visibility and organization context.

## Data Ownership

Organization-owned records should include:

- storefront visibility settings
- cart
- cart item
- order
- order item
- checkout intent
- order confirmation/receipt
- order reconciliation item

## API Direction

Planned API groups:

- storefront catalog
- product detail
- cart
- checkout intent
- orders
- order status
- order management
- reconciliation review

## Flutter Direction

Planned screens:

- storefront product list
- product detail
- cart
- checkout
- order confirmation
- order history
- order management placeholder

## Audit Direction

Audit important actions:

- product made public or hidden
- order created
- checkout initiated
- payment status changed
- order cancelled
- stock reserved or released where supported
- reconciliation resolved

## Revenue Direction

This wave supports future marketplace commission, online sales, premium commerce module subscription, and delivery/logistics expansion.

## Risks

- duplicating product logic outside Commerce/POS
- exposing inactive or internal-only products publicly
- overselling inventory
- payment success not reflected on order
- building delivery too early

## Readiness Decision

```text
ready for implementation planning
```

## Final Rule

Marketplace must extend Commerce/POS foundations. Product, stock, and payment behavior should remain shared and reusable.
