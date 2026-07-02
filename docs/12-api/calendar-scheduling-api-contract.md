# Calendar Scheduling API Contract

## Purpose

This document defines the MVP API contract for Organization Calendar and Scheduling foundation.

The API should support calendars, scheduled items, domain references, reminders, and conflict warnings while preserving organization boundaries and timezone correctness.

## Owner

```text
Scheduling Foundation
Notification Engine where reminders are used
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/calendar
```

## Calendars

```text
GET /api/v1/organizations/{organizationId}/calendar/calendars
POST /api/v1/organizations/{organizationId}/calendar/calendars
GET /api/v1/organizations/{organizationId}/calendar/calendars/{calendarId}
PATCH /api/v1/organizations/{organizationId}/calendar/calendars/{calendarId}
```

### Create Calendar Request

```json
{
  "name": "Main Organization Calendar",
  "type": "general",
  "visibility": "internal",
  "timezone": "Africa/Accra",
  "status": "active"
}
```

## Scheduled Items

```text
GET /api/v1/organizations/{organizationId}/calendar/scheduled-items
POST /api/v1/organizations/{organizationId}/calendar/scheduled-items
GET /api/v1/organizations/{organizationId}/calendar/scheduled-items/{scheduledItemId}
PATCH /api/v1/organizations/{organizationId}/calendar/scheduled-items/{scheduledItemId}
```

### Create Scheduled Item Request

```json
{
  "calendarId": "...",
  "title": "Sunday Service",
  "description": "Optional description",
  "startsAt": "2026-07-05T09:00:00Z",
  "endsAt": "2026-07-05T11:00:00Z",
  "timezone": "Africa/Accra",
  "status": "scheduled",
  "domainReference": {
    "type": "religious_event",
    "id": "optional"
  }
}
```

## Domain References

```text
GET /api/v1/organizations/{organizationId}/calendar/scheduled-items/{scheduledItemId}/domain-reference
PATCH /api/v1/organizations/{organizationId}/calendar/scheduled-items/{scheduledItemId}/domain-reference
```

## Reminder Placeholder

```text
GET /api/v1/organizations/{organizationId}/calendar/scheduled-items/{scheduledItemId}/reminders
POST /api/v1/organizations/{organizationId}/calendar/scheduled-items/{scheduledItemId}/reminders
```

### Create Reminder Request

```json
{
  "type": "before_start",
  "offsetMinutes": 60,
  "channel": "notification_placeholder",
  "status": "active"
}
```

## Conflict Warnings

```text
POST /api/v1/organizations/{organizationId}/calendar/conflict-check
```

### Request Direction

```json
{
  "calendarId": "...",
  "startsAt": "2026-07-05T09:00:00Z",
  "endsAt": "2026-07-05T11:00:00Z",
  "resourceReference": "optional",
  "participantReference": "optional"
}
```

## Dashboard Summary

```text
GET /api/v1/organizations/{organizationId}/calendar/dashboard-summary
```

### Query Parameters

```text
startDate
endDate
calendarId
status
domainType
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- calendar feature availability
- required permission
- organization ownership

Domain references should not allow cross-organization links.

## Status Direction

Suggested calendar statuses:

```text
active
inactive
archived
```

Suggested scheduled item statuses:

```text
scheduled
in_progress_placeholder
completed_placeholder
cancelled
manual_review
```

Suggested visibility values:

```text
private
internal
public_placeholder
```

## Timezone Direction

Every scheduled item should store timezone or resolve from organization timezone.

Do not rely only on client local time.

## Audit Direction

Audit should be written for:

- calendar created or updated
- scheduled item created or updated
- scheduled item cancelled
- reminder created or updated
- domain reference linked or changed

## Error Direction

Use standard response shape.

Expected error areas:

- calendar not found
- scheduled item not found
- invalid time range
- invalid timezone
- domain reference invalid
- conflict detected where blocking is enabled later
- permission denied
- organization not found

## Final Rule

Calendar APIs must preserve organization scope, timezone correctness, permission checks, and auditability.
