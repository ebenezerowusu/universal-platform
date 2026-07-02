# Payment Operations Reconciliation API Contract

## Purpose

This document defines the MVP API contract for Payment Provider Operations and Reconciliation foundation.

The API should support payment operations dashboards, provider metadata, payment attempts, callback event metadata, status history, reconciliation jobs, mismatches, settlement placeholders, and refund/dispute placeholders while preserving organization boundaries.

## Owner

```text
Payment Engine
Permission Engine
Audit Engine
Workflow Engine where refund/dispute approvals are used later
```

## Base Path

```text
/api/v1/organizations/{organizationId}/payment-operations
```

## Payment Operations Dashboard

```text
GET /api/v1/organizations/{organizationId}/payment-operations/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "successfulPayments": 120,
    "pendingPayments": 8,
    "failedPayments": 3,
    "openMismatches": 2,
    "lastReconciliationStatus": "completed_placeholder"
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Provider Metadata

```text
GET /api/v1/organizations/{organizationId}/payment-operations/providers
POST /api/v1/organizations/{organizationId}/payment-operations/providers
GET /api/v1/organizations/{organizationId}/payment-operations/providers/{providerKey}
PATCH /api/v1/organizations/{organizationId}/payment-operations/providers/{providerKey}
```

Provider create/update should be platform-admin or payment-admin only where applicable.

## Payment Attempts

```text
GET /api/v1/organizations/{organizationId}/payment-operations/attempts
POST /api/v1/organizations/{organizationId}/payment-operations/attempts
GET /api/v1/organizations/{organizationId}/payment-operations/attempts/{paymentAttemptId}
PATCH /api/v1/organizations/{organizationId}/payment-operations/attempts/{paymentAttemptId}
```

### Create Payment Attempt Request

```json
{
  "amount": 250.00,
  "currency": "GHS",
  "providerKey": "hubtel_placeholder",
  "paymentMethodPlaceholder": "mobile_money",
  "sourceReference": {
    "domain": "billing",
    "type": "invoice",
    "id": "..."
  }
}
```

## Callback Event Metadata

```text
GET /api/v1/organizations/{organizationId}/payment-operations/callback-events
POST /api/v1/organizations/{organizationId}/payment-operations/callback-events
GET /api/v1/organizations/{organizationId}/payment-operations/callback-events/{callbackEventId}
POST /api/v1/organizations/{organizationId}/payment-operations/callback-events/{callbackEventId}/process-placeholder
```

## Payment Status History

```text
GET /api/v1/organizations/{organizationId}/payment-operations/attempts/{paymentAttemptId}/status-history
POST /api/v1/organizations/{organizationId}/payment-operations/attempts/{paymentAttemptId}/status-history
```

## Reconciliation Jobs

```text
GET /api/v1/organizations/{organizationId}/payment-operations/reconciliation-jobs
POST /api/v1/organizations/{organizationId}/payment-operations/reconciliation-jobs
GET /api/v1/organizations/{organizationId}/payment-operations/reconciliation-jobs/{jobId}
PATCH /api/v1/organizations/{organizationId}/payment-operations/reconciliation-jobs/{jobId}
```

### Create Reconciliation Job Request

```json
{
  "providerKey": "hubtel_placeholder",
  "periodStart": "2026-07-01",
  "periodEnd": "2026-07-31",
  "notes": "Optional note"
}
```

## Reconciliation Mismatches

```text
GET /api/v1/organizations/{organizationId}/payment-operations/mismatches
GET /api/v1/organizations/{organizationId}/payment-operations/mismatches/{mismatchId}
POST /api/v1/organizations/{organizationId}/payment-operations/mismatches/{mismatchId}/mark-reviewed
```

## Settlement Placeholder

```text
GET /api/v1/organizations/{organizationId}/payment-operations/settlement-batches
POST /api/v1/organizations/{organizationId}/payment-operations/settlement-batches
GET /api/v1/organizations/{organizationId}/payment-operations/settlement-batches/{settlementBatchId}
PATCH /api/v1/organizations/{organizationId}/payment-operations/settlement-batches/{settlementBatchId}
```

## Refund and Dispute Placeholders

```text
GET /api/v1/organizations/{organizationId}/payment-operations/refund-requests
POST /api/v1/organizations/{organizationId}/payment-operations/refund-requests
GET /api/v1/organizations/{organizationId}/payment-operations/disputes
POST /api/v1/organizations/{organizationId}/payment-operations/disputes
```

## Required Checks

Routes should check:

- authenticated user
- organization membership or permitted platform payment role
- payment operations feature availability
- required permission
- organization ownership
- source reference access where source details are returned
- provider metadata availability

Payment records must not cross organization boundaries.

## Status Direction

Suggested payment attempt statuses:

```text
initiated
pending
successful
failed
cancelled
reversed_placeholder
manual_review
```

Suggested callback event statuses:

```text
received
processed_placeholder
ignored_placeholder
failed
manual_review
```

Suggested reconciliation statuses:

```text
queued_placeholder
running_placeholder
completed_placeholder
failed
manual_review
```

Suggested mismatch statuses:

```text
open
reviewing_placeholder
resolved_placeholder
ignored_placeholder
manual_review
```

## Audit Direction

Audit should be written for:

- provider metadata changed
- payment attempt created or updated
- callback event processed placeholder
- payment status changed
- reconciliation job created/completed
- mismatch reviewed
- settlement placeholder updated
- refund/dispute placeholder created

## Error Direction

Use standard response shape.

Expected error areas:

- provider not found
- payment attempt not found
- callback event not found
- reconciliation job not found
- mismatch not found
- source reference unavailable
- payment permission denied
- organization not found

## Final Rule

Payment operations APIs must be organization-scoped, provider-agnostic, reconciliation-aware, permission-protected, and auditable.
