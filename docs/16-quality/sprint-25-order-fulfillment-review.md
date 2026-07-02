# Sprint 25 Order Fulfillment Review

## Purpose

This document defines review checks for Sprint 25.

Sprint 25 implements Order Fulfillment and Pickup foundation.

## Review 1: Scope

Sprint 25 may include:

- fulfillment module/feature registration foundation
- fulfillment status foundation
- pickup preparation foundation
- order packing/checklist foundation where practical
- customer notification trigger foundation where practical
- fulfillment note/history foundation
- order cancellation reason foundation where practical
- stock release/deduction confirmation direction where practical
- permission and feature checks
- audit records for important fulfillment actions
- API and UI foundations

Sprint 25 should not include:

- rider dispatch
- live delivery tracking
- route optimization
- driver payouts
- third-party logistics integrations
- marketplace commission settlement
- advanced returns/refunds
- direct payment provider calls inside Commerce Domain
- unrelated POS expansion

## Review 2: Domain Boundaries

Confirm:

- Commerce Domain owns fulfillment workflows
- Notification Engine handles customer updates where used
- stock behavior reuses Commerce/POS inventory foundations
- Platform Core does not contain fulfillment-specific business rules

## Review 3: Status Rules

Confirm:

- order status transitions are controlled
- invalid transitions are rejected
- status history is recorded
- cancellation reason is captured where cancellation is supported
- customer-facing status exposes safe fields only

## Review 4: Preparation Rules

Confirm:

- checklist items are linked to order items
- prepared quantities do not exceed expected quantities unless policy allows it
- item unavailable status is handled where practical
- checklist updates are auditable

## Review 5: Stock and Notification Rules

Confirm:

- reserved stock is released on cancellation where supported
- stock deduction is clear when order is completed
- notifications are not duplicated unnecessarily
- notification failures do not silently hide fulfillment status

## Review 6: Reconciliation

Confirm:

- paid order not accepted can be reviewed
- ready order without required notification can be reviewed
- completed order with unresolved stock action can be reviewed
- cancelled order with unreleased stock can be reviewed
- manual review actions are auditable

## Review 7: API Contract

Confirm implementation aligns with:

```text
docs/12-api/order-fulfillment-api-contract.md
```

## Review 8: UI Contract

Confirm screens align with:

```text
docs/11-ui-ux/order-fulfillment-ui-contract.md
```

## Review 9: Test Coverage

Tests should cover where practical:

- fulfillment queue filtering
- accept order transition
- invalid status transition
- checklist update
- ready for pickup action
- complete order action
- cancel order and stock release behavior
- permission denied behavior
- reconciliation item behavior

## Final Rule

Sprint 25 is complete when organizations can process marketplace orders through auditable fulfillment and pickup workflows without introducing full delivery operations.
