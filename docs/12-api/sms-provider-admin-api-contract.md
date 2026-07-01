# SMS Provider Admin API Contract

## Purpose

This document defines system/admin API direction for SMS provider operations.

These APIs are for platform administration and support, not organization user workflows.

## Owner

```text
SMS Engine
Infrastructure Adapters
System/Admin Operations
```

## Base Path Direction

```text
/api/v1/system/sms-providers
```

## Provider Health

```text
GET /api/v1/system/sms-providers/health
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "items": [
      {
        "providerKey": "mock",
        "status": "healthy",
        "lastCheckedAt": "..."
      }
    ]
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Message Delivery Review

```text
GET /api/v1/system/sms-providers/messages/{messageId}/delivery
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "messageId": "...",
    "organizationId": "...",
    "platformStatus": "submitted",
    "providerStatus": "...",
    "providerReference": "...",
    "lastUpdatedAt": "..."
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Reconciliation Review

```text
GET /api/v1/system/sms-providers/reconciliation-items
```

### Query Parameters

```text
status
providerKey
organizationId
page
perPage
```

## Manual Reconciliation Action

```text
POST /api/v1/system/sms-providers/reconciliation-items/{itemId}/resolve
```

### Request Direction

```json
{
  "action": "mark_resolved",
  "notes": "Reviewed provider result and wallet ledger."
}
```

## Provider Callback Direction

Provider callbacks should route through provider-specific adapter endpoints internally.

A public callback endpoint may exist, but it must validate provider authenticity where supported.

## Required Checks

System/admin routes should check:

- authenticated platform admin or support role
- required permission
- audit logging for manual actions
- safe redaction of provider secrets

## Audit Direction

Audit should be written for:

- provider configuration change
- manual reconciliation action
- manual wallet correction
- manual message status correction

## Error Direction

Use standard response shape.

Expected error areas:

- provider not found
- message not found
- reconciliation item not found
- invalid manual action
- permission denied

## Final Rule

Provider admin APIs must not become tenant-facing features. They exist to support safe provider operations and delivery reliability.
