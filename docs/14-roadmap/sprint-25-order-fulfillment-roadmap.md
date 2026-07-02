# Sprint 25 Order Fulfillment Roadmap

## Purpose

This document defines Sprint 25 for Order Fulfillment and Pickup foundation.

Sprint 25 should create the controlled operational layer after Marketplace order creation, without building full rider or logistics operations yet.

## Sprint Goal

Enable organizations to process marketplace orders, prepare items for pickup, update fulfillment status, notify customers, and reconcile fulfillment with order, payment, and stock state.

## Why This Sprint

Sprint 24 prepared storefront, cart, checkout intent, and order foundation.

The next layer is operational order handling:

- order review
- preparation
- packing/checklist
- ready for pickup
- completed
- cancelled with reason
- customer status visibility

## Work Package 1: Fulfillment Status Foundation

Prepare statuses such as:

```text
pending_review
accepted
preparing
ready_for_pickup
completed
cancelled
manual_review
```

## Work Package 2: Pickup Foundation

Prepare pickup details:

- pickup location label
- pickup instructions
- pickup window placeholder
- customer contact safe label
- order confirmation reference

## Work Package 3: Packing or Preparation Checklist

Prepare simple checklist foundation:

- order item
- expected quantity
- prepared quantity
- status
- note

## Work Package 4: Customer Notification Trigger

Prepare notification trigger direction for:

- order accepted
- order ready for pickup
- order completed
- order cancelled

Use Notification Engine where practical.

## Work Package 5: Cancellation Direction

Prepare cancellation reason foundation.

Cancellation should be auditable and should trigger stock release where stock reservation exists.

## Work Package 6: Stock Confirmation Direction

Where stock reservation/deduction is supported:

- confirm reservation before fulfillment
- deduct stock according to order policy
- release stock on cancellation where supported

## Work Package 7: Flutter Foundation

Prepare screens or placeholders for:

- fulfillment dashboard
- order queue
- order detail
- preparation checklist
- ready for pickup action
- fulfillment history

## Out of Scope

Do not implement:

- rider dispatch
- live delivery tracking
- route optimization
- delivery pricing
- driver payouts
- third-party logistics integration
- advanced returns/refunds

## Completion Standard

Sprint 25 is complete when organizations can review orders, move them through fulfillment statuses, prepare pickup, notify customers, and reconcile fulfillment with order and stock state.

## Final Rule

Fulfillment should make order processing reliable before delivery operations are introduced.
