# Sprint 24 Marketplace Order Foundation Roadmap

## Purpose

This document defines Sprint 24 for Marketplace and Order foundation.

Sprint 24 should create the first customer-facing commerce layer for storefront catalog, cart, checkout intent, orders, and order confirmations without adding delivery or multi-vendor complexity yet.

## Sprint Goal

Enable organizations to expose approved products to a customer-facing catalog, receive customer orders, track order status, and connect order records to payment and inventory foundations.

## Why This Sprint

Sprint 23 prepared internal Commerce/POS.

Marketplace needs to build on that foundation instead of duplicating product and inventory logic.

The next layer is:

- public catalog visibility
- storefront product listing
- cart
- order creation
- checkout intent
- order status
- order confirmation

## Work Package 1: Public Catalog Visibility

Prepare product visibility rules:

```text
hidden
internal_only
public_storefront
```

Only approved, active products should appear in public storefront views.

## Work Package 2: Storefront Product Listing

Prepare customer-facing product list and detail foundation.

Use existing product catalog records and expose only safe fields.

## Work Package 3: Cart Foundation

Prepare cart records with:

- customer/session reference
- items
- quantity
- unit price snapshot
- currency
- status

## Work Package 4: Order Foundation

Prepare order records with:

- customer reference or guest contact
- organization
- order items
- total amount
- currency
- order status
- payment status

## Work Package 5: Checkout Intent

Prepare checkout intent to connect orders to Payment Engine where online payment is supported.

Commerce Domain must not call payment providers directly.

## Work Package 6: Inventory Direction

Prepare rules for:

- checking availability
- reserving stock where supported
- deducting stock after confirmed order or payment policy
- releasing reserved stock when order expires or is cancelled

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- storefront product list
- product detail
- cart
- checkout
- order confirmation
- order history

## Out of Scope

Do not implement:

- delivery/rider workflows
- multi-vendor marketplace onboarding
- marketplace commission settlement
- advanced promotions
- complex returns/refunds
- full customer loyalty system

## Completion Standard

Sprint 24 is complete when approved products can appear in a storefront, customers can create cart/order records, checkout intent can be prepared, and orders can be reconciled with inventory and payment status.

## Final Rule

Marketplace must reuse Commerce/POS product and inventory foundations. Do not fork product logic for storefront use.
