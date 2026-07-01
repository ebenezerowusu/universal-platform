# Sprint 17 Provider Readiness Review

## Purpose

This document defines review checks for Sprint 17.

Sprint 17 prepares SMS provider adapter readiness, delivery tracking, and reconciliation.

## Review 1: Scope

Sprint 17 may include:

- SMS provider adapter interface refinements
- local/mock provider behavior improvements
- provider send result normalization
- delivery status foundation
- callback or status update foundation where practical
- reconciliation job foundation
- provider health check foundation
- system/admin operation notes or endpoints
- audit records for manual operations

Sprint 17 should not include:

- provider calls directly inside Communication Domain
- provider details in Flutter screens
- organization-managed provider credentials
- live provider credentials in repository
- real payment collection
- WhatsApp or email campaigns
- complex marketing automation

## Review 2: Provider Abstraction

Confirm:

- SMS Engine owns the provider contract
- provider adapters map provider-specific payloads internally
- platform statuses are normalized
- provider errors are mapped safely
- provider secrets are not exposed in logs or responses

## Review 3: Delivery Status

Confirm platform statuses are supported:

```text
queued
submitted
sent
delivered
failed
expired
unknown
```

## Review 4: Reconciliation

Confirm:

- wallet reservation behavior is clear
- final deduction behavior is clear
- failed send behavior is clear
- unknown status behavior is clear
- reconciliation job direction exists
- manual correction process is auditable

## Review 5: Admin Operations

Confirm system/admin operations support:

- provider health review
- delivery issue review
- reconciliation item review
- manual correction review
- retry review where safe

## Review 6: API Direction

Confirm implementation aligns with:

```text
docs/12-api/sms-provider-admin-api-contract.md
```

## Review 7: Test Coverage

Tests should cover where practical:

- send result normalization
- provider error normalization
- status mapping
- wallet reservation finalization
- unknown status reconciliation
- provider health result mapping

## Final Rule

Sprint 17 is complete when provider integration can be added safely behind SMS Engine without leaking provider complexity into organization workflows.
