# Sprint 23 Commerce POS Foundation Roadmap

## Purpose

This document defines Sprint 23 for Commerce and POS foundation.

Sprint 23 should create the first reusable commerce layer for product catalog, inventory, POS sales, receipts, and payment-aware sale records without building the full marketplace yet.

## Sprint Goal

Enable organizations to define products, track basic inventory, create POS sales, issue receipts, and reconcile stock movement with sale records.

## Why This Sprint

The platform vision includes Commerce, Marketplace, POS, and future logistics.

Before marketplace expansion, the platform needs a controlled POS foundation:

- products
- categories
- stock movements
- cart/sale records
- receipts
- payment status
- sales reconciliation

## Work Package 1: Product Categories

Prepare organization-owned product categories such as:

```text
general
food
books
clothing
equipment
services
custom category
```

Labels should be configurable.

## Work Package 2: Product Catalog

Prepare product records with:

- name
- category
- SKU optional
- barcode optional
- price
- currency
- status
- track inventory flag

## Work Package 3: Inventory Foundation

Prepare stock records and movements:

- opening stock
- stock adjustment
- sale deduction
- returned stock placeholder
- damaged/lost stock placeholder

## Work Package 4: POS Sale Foundation

Prepare POS sales with:

- cart items
- quantity
- unit price
- total amount
- payment status
- sale status
- cashier/user reference

## Work Package 5: Receipt Foundation

Prepare receipt records with:

- receipt number
- sale reference
- items summary
- total amount
- currency
- issued at

## Work Package 6: Payment Engine Link

Where online payment is used, initiate through Payment Engine.

Commerce Domain must not call payment providers directly.

## Work Package 7: Reconciliation

Prepare reconciliation between:

- sale record
- payment status
- inventory deduction
- receipt record
- sales ledger entry where available

## Work Package 8: Flutter Foundation

Prepare screens or placeholders for:

- POS dashboard
- product categories
- product catalog
- stock movements
- create sale/cart
- receipt view
- sales history

## Out of Scope

Do not implement:

- public marketplace storefront
- delivery/rider flows
- multi-vendor marketplace
- advanced discounts/promotions
- complex inventory valuation
- multi-warehouse optimization
- full accounting expansion

## Completion Standard

Sprint 23 is complete when organizations can define products, track basic stock, create POS sales, issue receipts, and reconcile inventory movement with sales.

## Final Rule

Commerce/POS foundation should be reusable beyond churches. It must support shops, marts, malls, organizations, and future marketplace features.
