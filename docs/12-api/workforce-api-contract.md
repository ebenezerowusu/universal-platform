# Workforce API Contract

## Purpose

This document defines the MVP API contract for workforce records.

The API should support departments, people records, assignments, status history, and basic references while preserving organization boundaries.

## Owner

```text
HR Domain
Permission Engine
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/workforce
```

## Departments

```text
GET /api/v1/organizations/{organizationId}/workforce/departments
POST /api/v1/organizations/{organizationId}/workforce/departments
GET /api/v1/organizations/{organizationId}/workforce/departments/{departmentId}
PATCH /api/v1/organizations/{organizationId}/workforce/departments/{departmentId}
```

## Workforce Profiles

```text
GET /api/v1/organizations/{organizationId}/workforce/profiles
POST /api/v1/organizations/{organizationId}/workforce/profiles
GET /api/v1/organizations/{organizationId}/workforce/profiles/{profileId}
PATCH /api/v1/organizations/{organizationId}/workforce/profiles/{profileId}
```

### Create Profile Request

```json
{
  "displayName": "Jane Doe",
  "workerType": "staff",
  "status": "active",
  "startDate": "2026-07-01",
  "notes": "Optional note"
}
```

## Assignments

```text
GET /api/v1/organizations/{organizationId}/workforce/assignments
POST /api/v1/organizations/{organizationId}/workforce/assignments
GET /api/v1/organizations/{organizationId}/workforce/assignments/{assignmentId}
PATCH /api/v1/organizations/{organizationId}/workforce/assignments/{assignmentId}
```

### Create Assignment Request

```json
{
  "profileId": "...",
  "departmentId": "...",
  "positionLabel": "Administrator",
  "assignmentType": "primary",
  "startDate": "2026-07-01",
  "status": "active"
}
```

## Status History

```text
GET /api/v1/organizations/{organizationId}/workforce/profiles/{profileId}/status-history
POST /api/v1/organizations/{organizationId}/workforce/profiles/{profileId}/status-history
```

## Basic References

```text
GET /api/v1/organizations/{organizationId}/workforce/profiles/{profileId}/references
POST /api/v1/organizations/{organizationId}/workforce/profiles/{profileId}/references
```

## Dashboard Summary

```text
GET /api/v1/organizations/{organizationId}/workforce/dashboard-summary
```

### Response Direction

```json
{
  "success": true,
  "data": {
    "summary": {
      "activeWorkers": 25,
      "staffCount": 10,
      "volunteerCount": 15,
      "departments": 4
    }
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
- workforce feature availability
- required permission
- organization ownership

## Status Direction

Suggested worker statuses:

```text
active
inactive
on_leave
suspended
ended
manual_review
```

Suggested worker types:

```text
staff
volunteer
contractor_placeholder
intern_placeholder
```

## Audit Direction

Audit should be written for:

- profile created or updated
- department created or updated
- assignment created or ended
- status changed
- reference added or removed

## Error Direction

Use standard response shape.

Expected error areas:

- department not found
- profile not found
- assignment not found
- invalid status transition
- permission denied
- organization not found

## Final Rule

Workforce APIs must preserve organization ownership, permission checks, and auditability.
