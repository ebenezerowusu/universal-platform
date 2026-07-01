# Commerce POS Flutter Contract

## Purpose

This document defines the MVP Flutter screen contract for Commerce and POS foundation.

The screens should help organization users manage products, stock, POS sales, receipts, and sales history without exposing unnecessary finance or payment-provider complexity.

## Navigation Direction

Commerce/POS screens should appear when:

- user is authenticated
- organization is selected
- Commerce/POS module is available
- user has required permission
- feature availability allows the section

## POS Dashboard

Should show:

- recent sales
- low stock warning where practical
- quick action to create sale
- quick link to product catalog
- quick link to sales history

## Product Categories Screen

Should show:

- category name
- code or safe label
- status
- edit action where permitted

## Product Catalog Screen

Should show:

- product name
- category
- price
- currency
- stock level where tracked
- status
- edit action where permitted

## Product Form

Should support:

- category selection
- name
- SKU optional
- barcode optional
- price
- currency
- track inventory flag
- status

## Inventory Movement Screen

Should support:

- product selection
- movement type
- quantity
- reason
- notes

## Create Sale / Cart Screen

Should support:

- product search or selection
- quantity
- unit price from product
- cart total
- payment mode
- confirm sale action

## Receipt Screen

Should show:

- receipt number
- sale reference
- item summary
- total amount
- currency
- issued date

## Sales History Screen

Should show:

- receipt number where available
- sale status
- payment status where available
- total amount
- currency
- sale date

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 23:

- public marketplace storefront
- delivery/rider screens
- multi-vendor marketplace screens
- advanced promotion screens
- multi-warehouse screens
- payment provider setup screens

## Final Rule

Flutter should make POS workflows fast and clear while keeping Commerce modular and payment-provider details hidden behind APIs.
