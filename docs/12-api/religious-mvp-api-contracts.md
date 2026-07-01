# Religious MVP API Contracts

## Purpose

This document defines planning-level API contracts for the first Religious Domain MVP.

Exact fields may evolve during implementation, but the route style, response format, tenant context, and permission requirements should remain consistent.

## Member List

```text
GET /api/v1/organizations/{organizationId}/religious/members
```

Query parameters:

- page
- perPage
- search
- status
- branchId
- congregationId
- groupId

Response data:

```json
{
  "items": [],
  "pagination": {
    "page": 1,
    "perPage": 20,
    "total": 0
  }
}
```

Required permission:

```text
religious.members.view
```

## Create Member

```text
POST /api/v1/organizations/{organizationId}/religious/members
```

Request example:

```json
{
  "firstName": "Ama",
  "lastName": "Mensah",
  "phone": "0240000000",
  "email": "ama@example.com",
  "gender": "female",
  "dateOfBirth": "1995-01-01",
  "branchId": "...",
  "congregationId": "..."
}
```

Required permission:

```text
religious.members.create
```

Events:

- ReligiousMemberCreated

## Visitor Registration

```text
POST /api/v1/organizations/{organizationId}/religious/visitors
```

Request example:

```json
{
  "firstName": "Kwame",
  "lastName": "Boateng",
  "phone": "0240000001",
  "invitedByMemberId": "...",
  "branchId": "..."
}
```

Required permission:

```text
religious.visitors.create
```

Events:

- ReligiousVisitorRegistered

## Convert Visitor to Member

```text
POST /api/v1/organizations/{organizationId}/religious/visitors/{visitorId}/convert
```

Request example:

```json
{
  "memberStatus": "active",
  "congregationId": "..."
}
```

Required permission:

```text
religious.visitors.convert
```

Events:

- ReligiousVisitorConverted
- ReligiousMemberCreated

## Congregations

```text
GET  /api/v1/organizations/{organizationId}/religious/congregations
POST /api/v1/organizations/{organizationId}/religious/congregations
```

Create request:

```json
{
  "name": "Adult Congregation",
  "description": "Main adult congregation"
}
```

Required permissions:

- religious.congregations.view
- religious.congregations.manage

## Services

```text
GET  /api/v1/organizations/{organizationId}/religious/services
POST /api/v1/organizations/{organizationId}/religious/services
```

Create request:

```json
{
  "congregationId": "...",
  "name": "First Service",
  "dayOfWeek": "sunday",
  "startTime": "07:00",
  "endTime": "09:00",
  "venue": "Main Auditorium"
}
```

Required permissions:

- religious.services.view
- religious.services.manage

## Groups

```text
GET  /api/v1/organizations/{organizationId}/religious/groups
POST /api/v1/organizations/{organizationId}/religious/groups
```

Create request:

```json
{
  "congregationId": "...",
  "name": "Choir",
  "groupType": "department"
}
```

Required permissions:

- religious.groups.view
- religious.groups.manage

## Attendance Session

```text
POST /api/v1/organizations/{organizationId}/religious/attendance/sessions
```

Request example:

```json
{
  "targetType": "service",
  "targetId": "...",
  "title": "Sunday First Service Attendance",
  "sessionDate": "2026-07-01"
}
```

Required permission:

```text
religious.attendance.mark
```

Events:

- ReligiousAttendanceSessionCreated

## Mark Attendance

```text
POST /api/v1/organizations/{organizationId}/religious/attendance/sessions/{sessionId}/records
```

Request example:

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

Required permission:

```text
religious.attendance.mark
```

Events:

- ReligiousAttendanceMarked

## Final Rule

Religious MVP APIs must remain organization-scoped, permission-protected, and consistent with the shared API standards.
