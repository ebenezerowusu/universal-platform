# SMS Package Reconciliation Playbook

## Purpose

This document defines how SMS package purchases, payment records, and wallet credit records should be reconciled.

The goal is to protect revenue and wallet accuracy.

## Principle

Every wallet credit from a package purchase should be traceable to a confirmed payment or an audited manual action.

## Reconciliation Inputs

Use these sources:

- package purchase request
- selected SMS package
- Payment Engine record
- payment provider normalized status
- wallet credit transaction
- SMS wallet balance
- audit records
- support/admin review actions

## Recommended Flow

```text
package purchase requested
  -> payment initiated through Payment Engine
    -> payment status received
      -> payment confirmed
        -> wallet credited through SMS Engine
          -> wallet transaction recorded
            -> purchase marked completed
```

## Status Review

Review purchases in these states:

```text
pending too long
paid but not credited
credit failed
credited without confirmed payment
manual review
```

## Wallet Credit Rules

Wallet credit is allowed when:

- Payment Engine confirms payment
- authorized system/admin manual credit is recorded
- non-production mock payment is explicitly marked successful

Wallet credit is not allowed when:

- payment is pending
- payment failed
- payment status is unknown
- duplicate credit would occur

## Duplicate Credit Prevention

Before crediting wallet, check:

- purchase id
- payment reference
- existing wallet credit transaction
- idempotency key where available

## Manual Review Direction

Support/admin review should capture:

```text
reviewed_by
reviewed_at
purchase_id
payment_reference
issue_summary
action_taken
notes
```

## Audit Direction

Audit should record:

- payment status changed
- wallet credited
- manual credit applied
- reconciliation item resolved
- duplicate credit prevented where useful

## Final Rule

SMS package reconciliation should make every paid package and wallet credit explainable after the fact.
