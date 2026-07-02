# Payment Operations Reconciliation Guide

## Purpose

This document defines operating guidance for payment provider metadata, payment attempts, callback events, payment status history, reconciliation jobs, mismatches, settlement placeholders, and refund/dispute placeholders.

The goal is to keep provider state, platform payment state, and source-domain outcomes aligned.

## Principle

A payment is not complete just because one system says it is complete.

Payment operations should reconcile provider state, platform state, and source-domain state.

## Data Sources

Payment operations may use:

- payment provider metadata
- payment attempt
- callback/webhook event metadata
- payment status history
- reconciliation job
- reconciliation mismatch
- settlement batch placeholder
- refund request placeholder
- dispute placeholder
- audit records

## Provider Metadata Direction

Provider metadata should capture:

```text
provider_key
country_region_placeholder
currency_support_placeholder
payment_method_type
status
configuration_reference_placeholder
```

Provider credentials should not be exposed through normal operations screens.

## Payment Attempt Direction

Payment attempts should capture:

```text
organization_id
amount
currency
provider_key
payment_method_placeholder
source_domain
source_type
source_id
status
provider_reference_placeholder
created_at
```

## Callback Event Direction

Callback event metadata should capture:

```text
provider_key
provider_reference
event_type
received_at
processing_status
safe_payload_summary_placeholder
```

Raw payload storage must be controlled by security policy and should not be displayed by default.

## Status History Direction

Every important status change should capture:

```text
old_status
new_status
reason optional
source
actor optional
created_at
```

## Reconciliation Direction

Reconciliation jobs should compare:

- provider records
- platform payment attempts
- source-domain paid status
- amount and currency
- duplicate provider references
- missing source records

## Mismatch Direction

Mismatch records should capture:

```text
mismatch_type
provider_reference optional
payment_attempt_id optional
source_reference optional
severity_placeholder
status
notes optional
```

## Settlement Direction

Settlement placeholders may capture:

- provider
- settlement period
- expected amount placeholder
- received amount placeholder
- mismatch count
- status
- notes

This is not a full accounting settlement ledger in MVP.

## Refund and Dispute Direction

Refund/dispute placeholders should be workflow-ready.

Do not implement automatic refunds or chargeback automation in MVP.

## Audit Direction

Audit should record:

- provider metadata changed
- payment attempt created
- payment status changed
- callback event processed placeholder
- reconciliation job started/completed
- mismatch reviewed
- settlement placeholder changed
- refund/dispute placeholder created

## Data Quality Warnings

Warn or flag when:

- provider successful but platform pending
- platform successful but provider missing
- amount mismatch
- currency mismatch
- duplicate provider reference
- payment has no source record
- source record marked paid without successful payment attempt

## Final Rule

Payment operations must keep money movement explainable and must never let domains decide provider payment status independently.
