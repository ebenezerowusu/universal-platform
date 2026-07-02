# Staff Attendance and Leave API Contract

## Purpose

This document defines the MVP API contract for staff attendance and leave foundation.

The API should support attendance sessions, check-in/check-out, manual adjustments, leave requests, approval placeholders, and attendance summaries while preserving organization boundaries.

## Owner

```text
HR Domain
Workflow Engine where leave approval is used
Audit Engine
```

## Base Path

```text
/api/v1/organizations/{organizationId}/hr/attendance
```

## Attendance Sessions

```text
GET /api/v1/organizations/{organizationId}/hr/attendance/sessions
POST /api/v1/organizations/{organizationId}/hr/attendance/sessions
GET /api/v1/organizations/{organizationId}/hr/attendance/sessions/{sessionId}
PATCH /api/v1/organizations/{organizationId}/hr/attendance/sessions/{sessionId}
```

### Create Session Request

```json
{
  "name": "Morning Shift",
  "date": "2026-07-02",
  "startTime": "08:00",
  "endTime": "17:00",
  "departmentId": "optional",
  "status": "open"
}
```

## Check-In

```text
POST /api/v1/organizations/{organizationId}/hr/attendance/sessions/{sessionId}/check-in
```

### Request Direction

```json
{
  "profileId": "...",
  "source": "manual",
  "notes": "Optional note"
}
```

## Check-Out

```text
POST /api/v1/organizations/{organizationId}/hr/attendance/records/{recordId}/check-out
```

### Request Direction

```json
{
  "notes": "Optional note"
}
```

## Attendance Records

```text
GET /api/v1/organizations/{organizationId}/hr/attendance/records
GET /api/v1/organizations/{organizationId}/hr/attendance/records/{recordId}
```

## Manual Adjustments

```text
POST /api/v1/organizations/{organizationId}/hr/attendance/records/{recordId}/adjustments
```

### Request Direction

```json
{
  "type": "missed_checkout",
  "correctedValue": "17:00",
  "reason": "Forgot to check out"
}
```

## Leave Requests

```text
GET /api/v1/organizations/{organizationId}/hr/leave-requests
POST /api/v1/organizations/{organizationId}/hr/leave-requests
GET /api/v1/organizations/{organizationId}/hr/leave-requests/{leaveRequestId}
PATCH /api/v1/organizations/{organizationId}/hr/leave-requests/{leaveRequestId}
```

### Create Leave Request

```json
{
  "profileId": "...",
  "leaveType": "annual_leave_placeholder",
  "startDate": "2026-08-01",
  "endDate": "2026-08-05",
  "reason": "Optional reason"
}
```

## Leave Approval Placeholder

```text
POST /api/v1/organizations/{organizationId}/hr/leave-requests/{leaveRequestId}/approve
POST /api/v1/organizations/{organizationId}/hr/leave-requests/{leaveRequestId}/reject
```

## Attendance Summary

```text
GET /api/v1/organizations/{organizationId}/hr/attendance/summary
```

### Query Parameters

```text
startDate
endDate
departmentId
profileId
```

## Required Checks

Routes should check:

- authenticated user
- organization membership
- HR/workforce feature availability
- required permission
- organization ownership

## Status Direction

Suggested attendance session statuses:

```text
open
closed
cancelled
manual_review
```

Suggested attendance record statuses:

```text
checked_in
checked_out
missing_checkout
adjusted
excused
manual_review
```

Suggested leave request statuses:

```text
submitted
approved
rejected
cancelled
manual_review
```

## Audit Direction

Audit should be written for:

- attendance session created or updated
- check-in recorded
- check-out recorded
- manual adjustment created
- leave request created
- leave request approved or rejected

## Error Direction

Use standard response shape.

Expected error areas:

- session not found
- attendance record not found
- profile not found
- duplicate check-in
- session closed
- leave request not found
- invalid approval action
- permission denied
- organization not found

## Final Rule

Attendance and leave APIs must preserve workforce ownership, permission checks, and auditability.
