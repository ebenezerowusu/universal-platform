# Religious Structure and Attendance API Contract

## Purpose

This document defines the MVP Religious Structure and Attendance API contract.

Structure covers congregations, services, and groups.

Attendance covers manual session creation and attendance marking.

## Owner

```text
Religious Domain
```

## Congregation Routes

```text
GET  /api/v1/organizations/{organizationId}/religious/congregations
POST /api/v1/organizations/{organizationId}/religious/congregations
GET  /api/v1/organizations/{organizationId}/religious/congregations/{congregationId}
PATCH /api/v1/organizations/{organizationId}/religious/congregations/{congregationId}
```

### Create Congregation Request

```json
{
  "name": "Adults",
  "description": "Adult congregation",
  "status": "active"
}
```

## Service Routes

```text
GET  /api/v1/organizations/{organizationId}/religious/services
POST /api/v1/organizations/{organizationId}/religious/services
GET  /api/v1/organizations/{organizationId}/religious/services/{serviceId}
PATCH /api/v1/organizations/{organizationId}/religious/services/{serviceId}
```

### Create Service Request

```json
{
  "congregationId": "...",
  "name": "First Service",
  "dayOfWeek": "sunday",
  "startTime": "07:00",
  "endTime": "09:00",
  "status": "active"
}
```

## Group Routes

```text
GET  /api/v1/organizations/{organizationId}/religious/groups
POST /api/v1/organizations/{organizationId}/religious/groups
GET  /api/v1/organizations/{organizationId}/religious/groups/{groupId}
PATCH /api/v1/organizations/{organizationId}/religious/groups/{groupId}
POST /api/v1/organizations/{organizationId}/religious/groups/{groupId}/members
DELETE /api/v1/organizations/{organizationId}/religious/groups/{groupId}/members/{memberId}
```

### Create Group Request

```json
{
  "congregationId": "...",
  "name": "Choir",
  "type": "department",
  "status": "active"
}
```

## Attendance Session Routes

```text
GET  /api/v1/organizations/{organizationId}/religious/attendance/sessions
POST /api/v1/organizations/{organizationId}/religious/attendance/sessions
GET  /api/v1/organizations/{organizationId}/religious/attendance/sessions/{sessionId}
PATCH /api/v1/organizations/{organizationId}/religious/attendance/sessions/{sessionId}
```

### Create Attendance Session Request

```json
{
  "targetType": "service",
  "targetId": "...",
  "sessionDate": "2026-07-01",
  "title": "First Service Attendance",
  "status": "open"
}
```

## Attendance Record Routes

```text
GET  /api/v1/organizations/{organizationId}/religious/attendance/sessions/{sessionId}/records
POST /api/v1/organizations/{organizationId}/religious/attendance/sessions/{sessionId}/records
```

### Mark Attendance Request

```json
{
  "records": [
    {
      "memberId": "...",
      "status": "present"
    }
  ]
}
```

## Attendance Statuses

MVP statuses:

```text
present
absent
late
excused
```

## Required Checks

Structure and attendance routes should check:

- authenticated user
- organization membership
- Religious module availability
- required permission
- feature availability where applicable

## Audit Direction

Audit should be written for important changes such as:

- group created
- member added to group
- attendance session created
- attendance records submitted or updated

## MVP Exclusions

Do not implement in Sprint 5:

- QR attendance
- GPS attendance
- offline sync
- advanced Bible Study tree attendance
- attendance approval workflow

## Error Direction

Use the standard error response contract:

```text
docs/12-api/standard-response-contract.md
```

Expected error areas:

- organization not found
- Religious module unavailable
- permission denied
- congregation not found
- service not found
- group not found
- member not found
- attendance session not found
- duplicate attendance record
- invalid input

## Final Rule

Religious structure and attendance must stay inside Religious Domain while relying on Core, Permission, Audit, Module Registry, and feature availability checks.
