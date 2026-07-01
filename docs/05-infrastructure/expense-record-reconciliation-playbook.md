# Expense Record Reconciliation Playbook

## Purpose

This document defines how expense requests, approvals, expense records, budget lines, and ledger entries should be reconciled.

The goal is to make spending history explainable and controlled.

## Principle

Every expense should be traceable to a request, an approval decision, a manual record, or an audited correction.

## Reconciliation Inputs

Use these sources:

- expense category
- budget line
- expense request
- approval decision
- expense record
- payment reference where applicable
- attachment or receipt reference where available
- ledger entry
- audit records
- support/admin review actions

## Recommended Flow

```text
expense request created
  -> request submitted
    -> request approved or rejected
      -> approved request becomes expense record when spent or paid
        -> ledger entry recorded
          -> reconciliation status updated
```

Manual expense records should still produce audit and ledger behavior where required.

## Status Review

Review records in these states:

```text
approved but no expense record
expense recorded without approved request
paid but not reconciled
ledger entry missing
manual review
```

## Budget Review Direction

Where budget lines are used, compare:

- budget amount
- requested amount
- approved amount
- recorded amount
- remaining balance where practical

Do not block all expenses only because a budget line is absent in MVP.

## Duplicate Prevention

Before creating ledger entry or final expense record, check:

- expense request id
- expense record id
- existing ledger entry
- payment reference where available
- idempotency key where available

## Manual Review Direction

Support/admin review should capture:

```text
reviewed_by
reviewed_at
expense_record_id optional
expense_request_id optional
issue_summary
action_taken
notes
```

## Audit Direction

Audit should record:

- expense request adjusted
- approval decision changed where allowed
- expense record adjusted
- ledger entry created
- reconciliation item resolved
- duplicate action prevented where useful

## Final Rule

Expense reconciliation should make every spending record explainable after the fact.
