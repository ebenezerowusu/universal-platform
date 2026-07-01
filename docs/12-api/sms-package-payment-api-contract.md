# SMS Package Payment API Contract

## Purpose

This document defines the MVP API contract for SMS package payment readiness.

The API should let organizations request SMS packages, initiate payment through Payment Engine, check status, and view wallet credit history.

## Owner

```text
Communication Domain
Payment Engine
SMS Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/communication
```

## Package Catalog

```text
GET /api/v1/organizations/{organizationId}/communication/sms-packages
```

This route may reuse the package catalog from the Communication/SMS API contract.

## Create Purchase Request

```text
POST /api/v1/organizations/{organizationId}/communication/sms-package-purchases
```

### Request

```json
{
  "packageId": "...",
  "paymentMethodHint": "mobile_money",
  "notes": "Purchase starter package"
}
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "purchase": {
      "id": "...",
      "packageId": "...",
      "amount": 50,
      "currency": "GHS",
      "status": "pending",
      "paymentReference": "..."
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Initiate Payment

```text
POST /api/v1/organizations/{organizationId}/communication/sms-package-purchases/{purchaseId}/initiate-payment
```

### Direction

This route should call Payment Engine.

Communication Domain must not call payment providers directly.

## Payment Status

```text
GET /api/v1/organizations/{organizationId}/communication/sms-package-purchases/{purchaseId}
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "purchase": {
      "id": "...",
      "status": "paid",
      "paymentStatus": "confirmed",
      "walletCreditStatus": "credited",
      "units": 500,
      "amount": 50,
      "currency": "GHS"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Wallet Transactions

```text
GET /api/v1/organizations/{organizationId}/communication/sms-wallet/transactions
```

### Query Parameters

```text
page
perPage
type
purchaseId
```

## System/Admin Review

```text
GET /api/v1/system/sms-package-purchases/review-items
```

### Query Parameters

```text
status
organizationId
page
perPage
```

```text
POST /api/v1/system/sms-package-purchases/{purchaseId}/resolve-review
```

### Request Direction

```json
{
  "action": "mark_reviewed",
  "notes": "Payment confirmed and wallet credit verified."
}
```

## Required Checks

Organization routes should check:

- authenticated user
- organization membership
- Communication module availability
- required permission
- feature availability
- package is active

System/admin routes should check:

- authenticated platform admin or support role
- required system permission
- audit logging

## Status Direction

Suggested purchase statuses:

```text
pending
processing
paid
failed
cancelled
expired
manual_review
```

Suggested wallet credit statuses:

```text
not_required
pending
credited
failed
manual_review
```

## Audit Direction

Audit should be written for:

- purchase requested
- payment initiated
- payment status changed
- wallet credited
- manual review resolved
- manual wallet adjustment

## Error Direction

Use standard response shape.

Expected error areas:

- package not found
- package inactive
- purchase not found
- payment initiation failed
- payment not confirmed
- wallet credit failed
- duplicate credit prevented
- permission denied

## Final Rule

SMS package payment APIs must preserve Payment Engine and SMS Engine boundaries. No direct payment provider logic should appear in Communication Domain routes.
