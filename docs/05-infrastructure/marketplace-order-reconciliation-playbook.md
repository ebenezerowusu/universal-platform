# Marketplace Order Reconciliation Playbook

## Purpose

This document defines how storefront orders, carts, checkout intent, payment status, stock movement, and order confirmations should be reconciled.

The goal is to keep customer orders explainable and connected to Commerce/POS foundations.

## Principle

Every order should explain what was ordered, what was paid, what stock changed, and what confirmation was issued.

## Reconciliation Inputs

Use these sources:

- storefront product visibility
- cart and cart items
- checkout intent
- order and order items
- payment reference where linked
- stock reservation or deduction
- order confirmation or receipt
- audit records
- support/admin review actions

## Recommended Flow

```text
cart created
  -> checkout intent created
    -> order created
      -> stock availability checked
        -> payment status tracked where applicable
          -> order confirmed
            -> stock reserved or deducted according to policy
              -> order confirmation issued
                -> reconciliation status updated
```

## Status Review

Review records in these states:

```text
checkout intent without order
order without payment status where payment is required
paid order not confirmed
confirmed order without stock action where required
order confirmation missing
manual review
```

## Stock Rules

Where product tracks stock:

- check stock before order confirmation
- reserve stock where supported
- deduct stock according to policy
- release reservation when order expires or is cancelled

Where product does not track stock:

- skip stock reservation
- still record order item and confirmation

## Payment Rules

Where online payment is used:

- initiate through Payment Engine
- track normalized payment status
- do not mark paid without confirmed payment or audited manual action

## Duplicate Prevention

Before confirming order or issuing confirmation, check:

- cart id
- checkout intent id
- order id
- payment reference where available
- existing confirmation record
- stock movement reference where available
- idempotency key where available

## Audit Direction

Audit should record:

- checkout initiated
- order created
- payment status changed
- order status changed
- stock reserved or released where supported
- order cancelled
- reconciliation item resolved

## Final Rule

Marketplace order reconciliation should make every customer order, payment status, and stock decision explainable after the fact.
