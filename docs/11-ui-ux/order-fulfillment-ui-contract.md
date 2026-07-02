# Order Fulfillment UI Contract

## Purpose

This document defines the MVP UI contract for Order Fulfillment and Pickup foundation.

The screens should help organization users process marketplace orders, prepare items, mark orders ready for pickup, complete orders, and review fulfillment history.

## Navigation Direction

Fulfillment screens should appear under Commerce when:

- user is authenticated
- organization is selected
- Commerce/Marketplace module is available
- user has required permission
- feature availability allows fulfillment

## Fulfillment Dashboard

Should show:

- pending orders
- preparing orders
- ready for pickup orders
- completed orders for selected period
- manual review count where available

## Order Queue

Should show:

- order number
- customer safe label
- order total
- payment status
- fulfillment status
- created date
- action to view detail

## Fulfillment Detail

Should show:

- order number
- customer safe label
- item summary
- payment status
- fulfillment status
- pickup details where available
- notes/history where available

## Preparation Checklist

Should show:

- order item
- expected quantity
- prepared quantity
- item status
- note field where permitted

## Ready For Pickup Action

Should support:

- pickup location label
- pickup instructions
- optional note
- confirmation action

## Complete Order Action

Should support:

- completion note optional
- final confirmation action

## Cancel Order Action

Should support:

- cancellation reason
- stock release option where supported
- optional note
- confirmation action

## Customer Status View

Customer-facing order status should show only safe fields:

- order number
- order status
- payment status
- pickup instructions where available
- next steps text

## UI State Requirements

Every screen should support:

- loading
- empty
- error
- unavailable
- submit progress
- success feedback

## MVP Exclusions

Do not implement in Sprint 25:

- rider dispatch screens
- live delivery tracking screens
- route optimization screens
- driver payout screens
- third-party logistics setup screens
- advanced returns/refunds screens

## Final Rule

Fulfillment UI should make order processing clear and safe without introducing full delivery operations too early.
