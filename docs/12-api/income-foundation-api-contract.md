# Income Foundation API Contract

## Purpose

This document defines the MVP API contract for organization income foundation.

The API should support income categories, commitments, income records, payment-linked income, receipts, and reconciliation while preserving Payment Engine boundaries.

## Owner

```text
Finance Domain
Payment Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/finance
```

## Categories

```text
GET /api/v1/organizations/{organizationId}/finance/income-categories
POST /api/v1/organizations/{organizationId}/finance/income-categories
GET /api/v1/organizations/{organizationId}/finance/income-categories/{categoryId}
PATCH /api/v1/organizations/{organizationId}/finance/income-categories/{categoryId}
```

### Create Category Request

```json
{
  "name": "General Giving",
  "code": "general_giving",
  "status": "active"
}
```

## Commitments

```text
GET /api/v1/organizations/{organizationId}/finance/income-commitments
POST /api/v1/organizations/{organizationId}/finance/income-commitments
GET /api/v1/organizations/{organizationId}/finance/income-commitments/{commitmentId}
PATCH /api/v1/organizations/{organizationId}/finance/income-commitments/{commitmentId}
```

### Create Commitment Request

```json
{
  "categoryId": "...",
  "amount": 500,
  "currency": "GHS",
  "personReference": "optional",
  "dueDate": "2026-12-31",
  "notes": "Optional pledge or commitment note"
}
```

## Income Records

```text
GET /api/v1/organizations/{organizationId}/finance/income-records
POST /api/v1/organizations/{organizationId}/finance/income-records
GET /api/v1/organizations/{organizationId}/finance/income-records/{incomeRecordId}
```

### Create Manual Income Request

```json
{
  "categoryId": "...",
  "amount": 100,
  "currency": "GHS",
  "receivedAt": "2026-07-01T10:00:00Z",
  "source": "manual",
  "payerReference": "optional",
  "anonymous": false,
  "notes": "Optional note"
}
```

## Initiate Payment-linked Income

```text
POST /api/v1/organizations/{organizationId}/finance/income-records/{incomeRecordId}/initiate-payment
```

### Direction

This route should call Payment Engine.

Finance Domain must not call payment providers directly.

## Income Payment Status

```text
GET /api/v1/organizations/{organizationId}/finance/income-records/{incomeRecordId}/payment-status
```

## Receipts

```text
GET /api/v1/organizations/{organizationId}/finance/receipts
GET /api/v1/organizations/{organizationId}/finance/receipts/{receiptId}
```

### Receipt Response Direction

```json
{
  "success": true,
  "data": {
    "receipt": {
      "id": "...",
      "receiptNumber": "RCPT-0001",
      "amount": 100,
      "currency": "GHS",
      "categoryName": "General Giving",
      "issuedAt": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Reconciliation Review

```text
GET /api/v1/system/finance/income-reconciliation-items
POST /api/v1/system/finance/income-reconciliation-items/{itemId}/resolve
```

## Required Checks

Organization routes should check:

- authenticated user
- organization membership
- Finance module availability
- required permission
- feature availability
- organization ownership

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested income statuses:

```text
draft
pending_payment
received
failed
cancelled
manual_review
```

Suggested commitment statuses:

```text
open
partially_fulfilled
fulfilled
cancelled
expired
```

## Audit Direction

Audit should be written for:

- category created or updated
- commitment created or updated
- income record created or adjusted
- payment initiated
- payment status changed
- receipt issued
- reconciliation resolved

## Error Direction

Use standard response shape.

Expected error areas:

- category not found
- category inactive
- commitment not found
- income record not found
- payment initiation failed
- permission denied
- organization not found

## Final Rule

Finance APIs must preserve Payment Engine boundaries and organization data ownership. No payment provider logic should appear in Finance Domain routes.
