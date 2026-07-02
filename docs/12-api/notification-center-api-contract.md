# Notification Center API Contract

## Purpose

This document defines the MVP API contract for Notification Center and User Preferences foundation.

The API should support in-app notification records, read/archive actions, preferences, delivery logs, template metadata, and domain trigger references while preserving organization and user boundaries.

## Owner

```text
Notification Center
Notification Engine
SMS Engine where SMS is used
Permission Engine
Audit Engine where policy requires it
```

## Base Path

```text
/api/v1/organizations/{organizationId}/notifications
```

## Notification Inbox

```text
GET /api/v1/organizations/{organizationId}/notifications
GET /api/v1/organizations/{organizationId}/notifications/{notificationId}
```

### Query Parameters

```text
status
category
unreadOnly
archived
page
perPage
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "notifications": [
      {
        "id": "...",
        "title": "Order ready for pickup",
        "summary": "Your order is ready",
        "category": "order_update",
        "status": "unread",
        "createdAt": "..."
      }
    ]
  },
  "meta": {
    "traceId": "...",
    "page": 1,
    "perPage": 20
  }
}
```

## Notification Actions

```text
POST /api/v1/organizations/{organizationId}/notifications/{notificationId}/mark-read
POST /api/v1/organizations/{organizationId}/notifications/{notificationId}/mark-unread
POST /api/v1/organizations/{organizationId}/notifications/{notificationId}/archive
```

## Create Notification Request

Internal/domain-facing foundation:

```text
POST /api/v1/organizations/{organizationId}/notifications/requests
```

### Request Direction

```json
{
  "recipientUserId": "optional",
  "category": "order_update",
  "title": "Order update",
  "summary": "Safe summary",
  "domainReference": {
    "type": "order_status_change",
    "id": "optional"
  }
}
```

Domain modules should call Notification Engine or application service boundaries, not delivery providers directly.

## User Preferences

```text
GET /api/v1/organizations/{organizationId}/notifications/preferences/me
PATCH /api/v1/organizations/{organizationId}/notifications/preferences/me
```

### Request Direction

```json
{
  "categories": [
    {
      "category": "order_update",
      "inAppEnabled": true,
      "smsEnabled": false,
      "muted": false
    }
  ],
  "quietHoursPlaceholder": {
    "enabled": false
  }
}
```

## Organization Default Preferences

```text
GET /api/v1/organizations/{organizationId}/notifications/default-preferences
PATCH /api/v1/organizations/{organizationId}/notifications/default-preferences
```

Admin only.

## Template Metadata

```text
GET /api/v1/organizations/{organizationId}/notifications/templates
POST /api/v1/organizations/{organizationId}/notifications/templates
PATCH /api/v1/organizations/{organizationId}/notifications/templates/{templateId}
```

## Delivery Logs

```text
GET /api/v1/organizations/{organizationId}/notifications/delivery-logs
GET /api/v1/organizations/{organizationId}/notifications/delivery-logs/{deliveryLogId}
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- notification feature availability
- required permission
- recipient ownership or admin permission
- source domain access where domain reference is returned

## Status Direction

Suggested notification statuses:

```text
unread
read
archived
expired_placeholder
manual_review
```

Suggested delivery statuses:

```text
created
requested_placeholder
sent_placeholder
failed
skipped_by_preference
cancelled
```

## Result Safety Direction

Notification titles and summaries must be safe.

Do not include sensitive domain details unless the recipient has access.

## Audit Direction

Audit may be written for:

- organization default preferences changed
- notification template changed
- broadcast-like request created
- delivery failure manually resolved
- user preference changed where policy requires it

## Error Direction

Use standard response shape.

Expected error areas:

- notification not found
- access denied
- invalid category
- template not found
- preference update invalid
- delivery log not found
- notification feature unavailable
- organization not found

## Final Rule

Notification APIs must respect recipient boundaries, preferences, domain access rules, and provider abstraction.
