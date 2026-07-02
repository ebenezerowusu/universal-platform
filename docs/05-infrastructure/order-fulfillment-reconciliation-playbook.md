# Order Fulfillment Reconciliation Playbook

## Purpose

This document defines how order fulfillment status, pickup readiness, customer notifications, stock release/deduction, and order completion should be reconciled.

The goal is to make order processing explainable from order creation to completion or cancellation.

## Principle

Every fulfillment action should explain who changed the order, what changed, when it changed, and what stock or customer notification was affected.

## Reconciliation Inputs

Use these sources:

- order
- order item
- fulfillment record
- fulfillment status history
- preparation checklist
- pickup instruction
- notification trigger reference
- stock reservation or deduction
- cancellation reason
- audit records
- support/admin review actions

## Recommended Flow

```text
order created
  -> order accepted for fulfillment
    -> preparation checklist updated
      -> order marked ready for pickup
        -> customer notification triggered where supported
          -> order completed or cancelled
            -> stock and reconciliation status finalized
```

## Status Review

Review records in these states:

```text
paid order not accepted
accepted order not prepared
ready order without notification where required
completed order with unresolved stock action
cancelled order with reserved stock not released
manual review
```

## Stock Direction

Where stock reservation exists:

- confirm reservation before fulfillment completion
- deduct stock according to order policy
- release reserved stock when order is cancelled
- record stock action reference where practical

## Notification Direction

Notifications may be triggered for:

- order accepted
- order ready for pickup
- order completed
- order cancelled

Notifications should use Notification Engine where practical.

## Duplicate Prevention

Before status update or notification, check:

- current order status
- requested transition
- existing notification trigger where relevant
- stock movement reference where relevant
- idempotency key where available

## Manual Review Direction

Support/admin review should capture:

```text
reviewed_by
reviewed_at
order_id
issue_summary
action_taken
notes
```

## Audit Direction

Audit should record:

- order accepted
- checklist item updated
- ready for pickup marked
- customer notification triggered
- order completed
- order cancelled
- stock released or deducted where supported
- reconciliation item resolved

## Final Rule

Order fulfillment reconciliation should make every order status change, stock decision, and customer notification explainable after the fact.
