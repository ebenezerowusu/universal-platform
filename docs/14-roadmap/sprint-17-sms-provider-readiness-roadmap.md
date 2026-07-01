# Sprint 17 SMS Provider Readiness Roadmap

## Purpose

This document defines Sprint 17 for SMS provider readiness, delivery tracking, and reconciliation.

Sprint 17 should prepare the platform for real SMS delivery without creating provider lock-in.

## Sprint Goal

Enable SMS Engine to support real provider adapters safely, normalize provider responses, track delivery status, and reconcile wallet usage.

## Why This Sprint

Sprint 16 created the Communication and SMS Wallet foundation.

Sprint 17 prepares the operational layer needed before real SMS delivery can be trusted:

- provider adapter contract
- send result normalization
- delivery status tracking
- provider callbacks or status polling
- reconciliation jobs
- provider health checks
- support/admin operations

## Work Package 1: Provider Adapter Contract

Refine the provider adapter interface for:

- send message
- estimate units where possible
- normalize provider result
- normalize provider error
- provider health check
- delivery status lookup where supported

## Work Package 2: Delivery Status Foundation

Prepare common statuses:

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

## Work Package 3: Callback or Polling Foundation

Prepare support for delivery updates through:

- provider callback/webhook where available
- polling/status lookup where available
- manual reconciliation where needed

## Work Package 4: Reconciliation

Prepare reconciliation between:

- wallet reservation
- sent messages
- provider accepted count
- provider failed count
- wallet final deduction
- transaction ledger

## Work Package 5: Provider Operations

System/admin operations should support:

- provider health review
- provider error review
- manual message status review
- manual wallet adjustment review
- retry review where safe

## Work Package 6: Audit and Support

Audit important actions:

- provider configuration changed
- message delivery status updated by system
- wallet correction applied
- manual retry requested

## Out of Scope

Do not implement:

- organization-managed provider credentials
- provider details in Flutter
- direct provider calls in Communication Domain
- real payment collection
- WhatsApp/email integrations
- advanced campaigns

## Completion Standard

Sprint 17 is complete when real SMS providers can be added behind SMS Engine with normalized send results, delivery tracking, and reconciliation direction.

## Final Rule

Real SMS delivery must be provider-agnostic from the platform user's perspective. Provider complexity belongs behind adapters and system operations.
