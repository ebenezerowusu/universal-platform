# Storefront Order UI Contract

## Purpose

This document defines the MVP UI contract for storefront and order flows.

The screens should support product browsing, cart, checkout, order confirmation, and organization-side order review.

## Product List

Should show:

- product name
- price
- currency
- image placeholder where available
- availability status where practical
- product detail action

## Product Detail

Should show:

- product name
- description where available
- price
- currency
- availability status
- quantity selector
- add to cart action

## Cart

Should show:

- cart items
- quantity
- unit price
- line total
- subtotal
- currency
- update quantity action
- checkout action

## Checkout

Should support:

- customer name optional
- customer phone optional
- customer email optional
- payment mode selection where supported
- order note optional
- place order action

## Order Confirmation

Should show:

- order number
- order status
- payment status
- total amount
- currency
- next steps text

## Organization Order Review

Admin/order manager screens should show:

- order list
- order detail
- customer safe label
- item summary
- payment status
- order status
- update status action where permitted

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 24:

- delivery screens
- rider screens
- multi-vendor screens
- advanced promotions
- customer loyalty screens
- returns screens beyond placeholders
- provider setup screens

## Final Rule

Storefront and order flows should reuse Commerce APIs and avoid duplicating product, stock, and payment behavior in the client.
