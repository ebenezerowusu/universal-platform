# Communication and SMS API Contract

## Purpose

This document defines the MVP API contract for Communication and SMS Wallet foundation.

The API should expose organization communication workflows while keeping provider behavior behind SMS Engine.

## Owner

```text
Communication Domain
SMS Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/communication
```

## Wallet Summary

```text
GET /api/v1/organizations/{organizationId}/communication/sms-wallet
```

### Success Response

```json
{
  "success": true,
  "data": {
    "wallet": {
      "balanceUnits": 500,
      "reservedUnits": 0,
      "availableUnits": 500,
      "currency": "GHS"
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Package Catalog

```text
GET /api/v1/organizations/{organizationId}/communication/sms-packages
```

### Success Response

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "id": "...",
        "name": "Starter SMS",
        "units": 500,
        "price": 50,
        "currency": "GHS",
        "status": "active"
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Package Request

```text
POST /api/v1/organizations/{organizationId}/communication/sms-package-requests
```

### Request

```json
{
  "packageId": "...",
  "notes": "Request starter package"
}
```

### Direction

MVP may create a package request without real payment collection.

Payment collection must be added through Payment Engine when approved.

## Transaction History

```text
GET /api/v1/organizations/{organizationId}/communication/sms-transactions
```

### Query Parameters

```text
page
perPage
type
```

## Compose Message Draft

```text
POST /api/v1/organizations/{organizationId}/communication/messages
```

### Request

```json
{
  "channel": "sms",
  "title": "Service Reminder",
  "body": "Remember our service tomorrow at 8am.",
  "recipientSource": "manual",
  "recipients": [
    "+233000000000"
  ]
}
```

## Send Message

```text
POST /api/v1/organizations/{organizationId}/communication/messages/{messageId}/send
```

### Success Response

```json
{
  "success": true,
  "data": {
    "message": {
      "id": "...",
      "status": "queued",
      "estimatedUnits": 1
    }
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Message History

```text
GET /api/v1/organizations/{organizationId}/communication/messages
```

### Query Parameters

```text
page
perPage
status
channel
```

## Recipient Preview

```text
POST /api/v1/organizations/{organizationId}/communication/recipient-preview
```

### Request

```json
{
  "source": "religious_members",
  "filters": {
    "status": "active"
  }
}
```

### Success Response

```json
{
  "success": true,
  "data": {
    "recipientCount": 120,
    "estimatedUnits": 120
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- Communication module availability
- required permission
- feature availability
- wallet balance where sending is requested

## Audit Direction

Audit should be written for:

- package request created
- wallet adjustment
- message draft created
- message send requested

## Error Direction

Use:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- communication module unavailable
- permission denied
- insufficient SMS units
- invalid recipient list
- message not found
- organization not found

## Final Rule

Communication APIs must not expose provider-specific behavior. Sending must go through SMS Engine.
