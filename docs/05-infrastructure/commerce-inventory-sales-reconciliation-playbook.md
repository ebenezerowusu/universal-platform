# Commerce Inventory Sales Reconciliation Playbook

## Purpose

This document defines how Commerce/POS sales, inventory movement, receipts, payment status, and sales ledger records should be reconciled.

The goal is to keep stock and sales history explainable.

## Principle

Every confirmed sale should explain what was sold, what stock changed, what was paid, and what receipt was issued.

## Reconciliation Inputs

Use these sources:

- product category
- product
- stock item
- inventory movement
- POS sale
- POS sale item
- payment reference where linked
- receipt record
- ledger entry where implemented
- audit records
- support/admin review actions

## Recommended Flow

```text
sale created
  -> stock availability checked
    -> sale confirmed
      -> inventory deducted
        -> payment status recorded
          -> receipt issued
            -> sales ledger entry recorded where implemented
              -> reconciliation status updated
```

Cash/manual sales should still produce receipt and inventory behavior where required.

## Status Review

Review records in these states:

```text
confirmed sale without stock deduction
paid sale without receipt
receipt issued without sale confirmation
stock deduction without matching sale
payment confirmed but sale not marked paid
manual review
```

## Inventory Rules

When product tracks inventory:

- check available stock before confirming sale
- deduct stock after sale confirmation
- record inventory movement
- prevent negative stock unless organization policy allows it

When product does not track inventory:

- skip stock deduction
- still record sale item and receipt

## Duplicate Prevention

Before issuing receipt or creating ledger movement, check:

- sale id
- receipt record
- inventory movement reference
- payment reference where available
- idempotency key where available

## Manual Review Direction

Support/admin review should capture:

```text
reviewed_by
reviewed_at
sale_id optional
product_id optional
issue_summary
action_taken
notes
```

## Audit Direction

Audit should record:

- stock adjusted
- sale confirmed
- sale cancelled where supported
- inventory movement created
- receipt issued
- payment status changed
- reconciliation item resolved

## Final Rule

Commerce reconciliation should make every sale and stock change explainable after the fact.
