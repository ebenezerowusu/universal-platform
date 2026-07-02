# Sprint 26 Delivery Logistics Roadmap

## Purpose

This document defines Sprint 26 for Delivery and Logistics foundation.

Sprint 26 should create the first delivery layer after fulfillment and pickup, without building live rider dispatch or ride-hailing operations yet.

## Sprint Goal

Enable organizations to configure delivery zones, define basic fee rules, accept delivery requests linked to orders, track delivery status, notify customers, and reconcile delivery actions with order fulfillment.

## Why This Sprint

Sprint 25 prepared order fulfillment and pickup.

Some marketplace orders will need delivery, but the platform should first build a controlled delivery foundation before advanced logistics.

The next layer is:

- delivery zones
- delivery fee rules
- customer delivery address
- delivery request
- assignment placeholder
- delivery status
- proof-of-delivery placeholder

## Work Package 1: Delivery Zones

Prepare organization-owned delivery zones with:

- name
- description
- area label
- status
- fee rule link optional

## Work Package 2: Delivery Fee Rules

Prepare simple fee rules:

- flat fee
- free delivery threshold placeholder
- manual quote placeholder
- currency
- status

## Work Package 3: Delivery Address

Prepare customer delivery address foundation:

- recipient name optional
- phone optional
- address line
- landmark optional
- area label optional
- notes optional

## Work Package 4: Delivery Request

Prepare delivery request linked to order:

- order reference
- delivery address
- fee amount
- status
- requested at
- assigned actor placeholder

## Work Package 5: Delivery Status

Prepare statuses such as:

```text
requested
accepted
preparing_for_dispatch
assigned_placeholder
out_for_delivery_placeholder
delivered
failed
cancelled
manual_review
```

## Work Package 6: Proof of Delivery Direction

Prepare placeholder for proof of delivery:

- recipient confirmation note
- image/file reference later
- delivered timestamp
- delivered by placeholder

## Work Package 7: Customer Notifications

Prepare notification trigger direction for:

- delivery accepted
- delivery dispatched placeholder
- delivered
- failed or cancelled

Use Notification Engine where practical.

## Out of Scope

Do not implement:

- live rider tracking
- route optimization
- driver payouts
- ride-hailing workflows
- third-party logistics integrations
- complex delivery pricing
- advanced returns/refunds

## Completion Standard

Sprint 26 is complete when organizations can define delivery zones and fees, create delivery requests from orders, update delivery status, and reconcile delivery with fulfillment and customer notification state.

## Final Rule

Delivery foundation should prepare logistics without forcing the platform into full ride-hailing too early.
