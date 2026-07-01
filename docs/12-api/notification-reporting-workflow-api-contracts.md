# Notification, Reporting, and Workflow API Contracts

## Purpose

This document defines planning-level API contracts for Notification Engine, Reporting Engine, and Workflow Engine.

These engines support many domains and should remain generic platform capabilities.

## Notification: List Notifications

```text
GET /api/v1/organizations/{organizationId}/notifications
```

Query parameters:

- page
- perPage
- status
- channel
- dateFrom
- dateTo

Response data:

```json
{
  "items": [],
  "pagination": {
    "page": 1,
    "perPage": 20,
    "total": 0,
    "totalPages": 0
  }
}
```

## Notification: Create Notification Request

```text
POST /api/v1/organizations/{organizationId}/notifications
```

Request example:

```json
{
  "audienceType": "group",
  "audienceId": "...",
  "channels": ["in_app", "sms"],
  "title": "Meeting Reminder",
  "message": "Remember our meeting today.",
  "sendAt": "2026-07-01T09:00:00Z"
}
```

Required checks:

- organization access
- permission for the calling use case
- channel availability
- SMS wallet check when SMS is selected

## Notification: Mark as Read

```text
POST /api/v1/organizations/{organizationId}/notifications/{notificationId}/read
```

## Reporting: List Available Reports

```text
GET /api/v1/organizations/{organizationId}/reports
```

Response data:

```json
{
  "items": [
    {
      "key": "religious.members.summary",
      "name": "Member Summary",
      "moduleKey": "religious.members"
    }
  ]
}
```

## Reporting: Run Report

```text
POST /api/v1/organizations/{organizationId}/reports/{reportKey}/run
```

Request example:

```json
{
  "filters": {
    "dateFrom": "2026-07-01",
    "dateTo": "2026-07-31",
    "branchId": "..."
  }
}
```

Response data:

```json
{
  "reportKey": "religious.attendance.summary",
  "data": {},
  "generatedAt": "2026-07-01T10:00:00Z"
}
```

## Reporting: Export Report

```text
POST /api/v1/organizations/{organizationId}/reports/{reportKey}/export
```

Expected behavior:

- validate permission
- create export job
- return job reference
- audit sensitive exports

## Workflow: List Tasks

```text
GET /api/v1/organizations/{organizationId}/workflow/tasks
```

Query parameters:

- page
- perPage
- status
- assignedTo
- dueFrom
- dueTo

## Workflow: Create Task

```text
POST /api/v1/organizations/{organizationId}/workflow/tasks
```

Request example:

```json
{
  "title": "Follow up with visitor",
  "description": "Call and check on the visitor.",
  "assignedToUserId": "...",
  "dueAt": "2026-07-03T09:00:00Z",
  "sourceType": "religious_visitor",
  "sourceId": "..."
}
```

## Workflow: Complete Task

```text
POST /api/v1/organizations/{organizationId}/workflow/tasks/{taskId}/complete
```

Request example:

```json
{
  "note": "Visitor was contacted successfully."
}
```

## Final Rule

Notification, Reporting, and Workflow APIs must stay generic. Domains provide meaning; engines provide reusable capability.
