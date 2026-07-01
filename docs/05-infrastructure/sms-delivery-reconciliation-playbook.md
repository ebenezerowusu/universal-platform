# SMS Delivery Reconciliation Playbook

## Purpose

This document defines how SMS delivery, wallet usage, and provider results should be reconciled.

The goal is to keep organization wallet balances trustworthy even when provider delivery results arrive later.

## Principle

Wallet accuracy matters.

Message sending, provider acceptance, delivery status, and wallet transactions should be traceable.

## Reconciliation Inputs

Use these sources:

- message send request
- recipient count
- estimated units
- wallet reservation
- provider send result
- provider delivery callback or status lookup
- wallet transaction ledger
- support/admin correction records

## Recommended Flow

```text
message send requested
  -> estimate units
    -> reserve wallet units
      -> call SMS Engine provider adapter
        -> record provider result
          -> update message status
            -> finalize wallet deduction or release unused reservation
              -> record transaction ledger
```

## Status Mapping

Use platform statuses:

```text
queued
submitted
sent
delivered
failed
expired
unknown
```

Provider-specific statuses must be mapped into platform statuses.

## Wallet Reservation Direction

Before sending, reserve estimated units where practical.

After provider result:

- deduct accepted units
- release unused reservation
- record failure where send was rejected
- keep uncertain items pending reconciliation

## Reconciliation Job Direction

A background job should review:

- messages stuck in queued/submitted state
- messages with unknown provider result
- wallet reservations not finalized
- provider results received without matching message

## Manual Review Direction

Support/admin review may be needed when:

- provider result is unclear
- wallet balance mismatch occurs
- callback cannot be matched
- message status remains unknown too long

## Audit Direction

Audit should record:

- manual wallet correction
- manual message status correction
- retry decision where supported
- provider configuration change

## Safety Rules

Do not:

- silently change wallet balances
- delete transaction records to fix balance
- trust provider callbacks without validation where validation is supported
- show provider secrets in logs

## Final Rule

SMS delivery reconciliation should make wallet changes explainable after the fact.
