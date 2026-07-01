# Communication Governance API Contract

## Purpose

This document defines the MVP API contract for Communication Governance.

The API supports recipient preferences, templates, recipient segments, preview, and send review checks.

## Owner

```text
Communication Domain
```

## Base Path

```text
/api/v1/organizations/{organizationId}/communication
```

## Preferences

```text
GET /api/v1/organizations/{organizationId}/communication/preferences
```

### Query Parameters

```text
page
perPage
recipientType
recipientId
smsAllowed
```

```text
PATCH /api/v1/organizations/{organizationId}/communication/preferences/{preferenceId}
```

### Request

```json
{
  "smsAllowed": false,
  "reason": "Recipient requested no SMS updates"
}
```

## Templates

```text
GET /api/v1/organizations/{organizationId}/communication/templates
POST /api/v1/organizations/{organizationId}/communication/templates
GET /api/v1/organizations/{organizationId}/communication/templates/{templateId}
PATCH /api/v1/organizations/{organizationId}/communication/templates/{templateId}
```

### Create Template Request

```json
{
  "name": "Service Reminder",
  "channel": "sms",
  "body": "Reminder: service starts at {{time}}.",
  "status": "active"
}
```

## Segments

```text
GET /api/v1/organizations/{organizationId}/communication/segments
POST /api/v1/organizations/{organizationId}/communication/segments
GET /api/v1/organizations/{organizationId}/communication/segments/{segmentId}
PATCH /api/v1/organizations/{organizationId}/communication/segments/{segmentId}
```

### Create Segment Request

```json
{
  "name": "Active Members",
  "source": "religious_members",
  "rules": {
    "status": "active"
  }
}
```

## Recipient Preview

```text
POST /api/v1/organizations/{organizationId}/communication/recipient-preview
```

### Request

```json
{
  "source": "segment",
  "segmentId": "...",
  "applyPreferences": true
}
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "recipientCount": 120,
    "eligibleCount": 115,
    "excludedCount": 5,
    "estimatedUnits": 115
  },
  "meta": {
    "traceId": "..."
  }
}
```

## Send Review

```text
POST /api/v1/organizations/{organizationId}/communication/messages/{messageId}/send-review
```

### Purpose

Run safe-send checks before a message is sent.

### Response Direction

```json
{
  "success": true,
  "data": {
    "canSend": true,
    "recipientCount": 120,
    "eligibleCount": 115,
    "excludedCount": 5,
    "estimatedUnits": 115,
    "warnings": []
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
- organization ownership

## Safe Send Checks

Before send, check:

- sender permission
- module availability
- feature availability
- wallet balance where required
- recipient eligibility
- recipient preferences
- message content validity

## Audit Direction

Audit should be written for:

- preference changed
- template created or updated
- segment created or updated
- send review accepted where implemented

## Error Direction

Use standard response shape.

Expected error areas:

- communication module unavailable
- permission denied
- preference not found
- template not found
- segment not found
- invalid segment rule
- no eligible recipients

## Final Rule

Communication Governance APIs must protect recipient preferences and organization boundaries before messages are sent.
