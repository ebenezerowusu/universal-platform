# Delivery Logistics Reconciliation Playbook

## Purpose

This document defines how delivery requests, fee rules, order state, delivery status, proof-of-delivery placeholders, customer notifications, and reconciliation items should be reviewed.

The goal is to make delivery history explainable from request to completion or cancellation.

## Principle

Every delivery action should explain which order was affected, who changed the delivery state, what fee was applied, and what customer-visible status was updated.

## Reconciliation Inputs

Use these sources:

- order
- fulfillment record
- delivery zone
- delivery fee rule
- delivery address
- delivery request
- delivery assignment placeholder
- delivery status history
- proof-of-delivery placeholder
- notification trigger reference
- audit records
- support/admin review actions

## Recommended Flow

```text
order marked eligible for delivery
  -> delivery request created
    -> delivery fee determined
      -> delivery accepted
        -> assignment placeholder recorded where used
          -> delivery status updated
            -> proof-of-delivery recorded where supported
              -> delivery completed or failed
                -> reconciliation status updated
```

## Status Review

Review records in these states:

```text
delivery request without eligible order
delivery fee missing where required
delivery marked delivered without proof where policy requires proof
failed delivery without reason
cancelled delivery without customer-safe status
manual review
```

## Fee Direction

Delivery fee should be traceable to:

- zone
- fee rule
- manual quote placeholder where supported
- currency
- order reference

Do not silently change delivery fees after order confirmation without audit.

## Notification Direction

Notifications may be triggered for:

- delivery accepted
- delivery assigned placeholder
- delivered
- failed
- cancelled

Notifications should use Notification Engine where practical.

## Duplicate Prevention

Before delivery status update or proof record, check:

- current delivery status
- requested transition
- existing proof record where relevant
- existing notification trigger where relevant
- idempotency key where available

## Manual Review Direction

Support/admin review should capture:

```text
reviewed_by
reviewed_at
order_id
delivery_request_id
issue_summary
action_taken
notes
```

## Audit Direction

Audit should record:

- delivery request created
- delivery fee applied
- assignment placeholder changed
- delivery status changed
- proof-of-delivery added
- customer notification triggered
- delivery failed or cancelled
- reconciliation item resolved

## Final Rule

Delivery reconciliation should make every delivery fee, status change, and customer-visible delivery result explainable after the fact.
