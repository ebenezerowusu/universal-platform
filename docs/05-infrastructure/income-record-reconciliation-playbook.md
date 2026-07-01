# Income Record Reconciliation Playbook

## Purpose

This document defines how income records, payment records, receipts, and ledger entries should be reconciled.

The goal is to protect financial accuracy and make income history explainable.

## Principle

Every received amount should be traceable to a manual record, confirmed payment, or audited correction.

## Reconciliation Inputs

Use these sources:

- income record
- income category
- commitment record where linked
- Payment Engine record where linked
- normalized payment status
- receipt record
- ledger entry
- audit records
- support/admin review actions

## Recommended Flow

```text
income record created
  -> payment initiated when needed
    -> payment status received
      -> income marked received when confirmed
        -> receipt issued
          -> ledger entry recorded
            -> reconciliation status updated
```

Manual income records should still produce auditable ledger and receipt behavior where required.

## Status Review

Review records in these states:

```text
pending payment too long
payment confirmed but income not marked received
receipt missing
ledger entry missing
manual review
```

## Receipt Rules

Receipt can be issued when:

- income is marked received
- required category and amount are present
- duplicate receipt is prevented

Receipt should not be issued when:

- payment is still pending
- income record is cancelled
- amount is invalid

## Duplicate Prevention

Before issuing receipt or ledger entry, check:

- income record id
- payment reference where available
- existing receipt
- existing ledger entry
- idempotency key where available

## Manual Review Direction

Support/admin review should capture:

```text
reviewed_by
reviewed_at
income_record_id
payment_reference optional
issue_summary
action_taken
notes
```

## Audit Direction

Audit should record:

- income record adjusted
- payment status changed
- receipt issued
- ledger entry created
- reconciliation item resolved
- duplicate action prevented where useful

## Final Rule

Income reconciliation should make every received amount, receipt, and ledger entry explainable after the fact.
